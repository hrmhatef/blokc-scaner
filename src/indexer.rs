use crate::config;
use crate::error::Error;
use crate::orm::db;
use crate::orm::entities::blocks;
use crate::orm::entities::events;
use crate::orm::entities::transactions;

use std::str::FromStr;

use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
    sol_types::SolEvent,
};
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};

sol! {
   #[derive(Debug)]
   event Transfer(address indexed from, address indexed to, uint value);
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventInfo {
    id: i32,
    from: String,
    to: String,
    value: String,
    log_index: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransactionInfo {
    tx_id: Option<i32>,
    tx_hash: String,
    tx_index: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockInfo {
    block_id: Option<i32>,
    block_number: i64,
    block_hash: String,
    timestamp: Option<i64>,
    tag: Option<BlockNumberOrTag>,
    is_removed: bool,
    tx_info: TransactionInfo,
    event_info: EventInfo,
}

impl TryInto<blocks::Model> for &mut BlockInfo {
    type Error = Error;

    fn try_into(self) -> config::AppResult<blocks::Model> {
        Ok(blocks::Model {
            id: 0,
            block_number: self.block_number,
            hash: self.block_hash.clone(),
            timestamp: None,
            tag: self
                .tag
                .ok_or(Error::BlockInfoError("tag".to_string()))?
                .to_string(),
            is_removed: self.is_removed,
        })
    }
}

impl TryInto<events::Model> for &mut BlockInfo {
    type Error = Error;

    fn try_into(self) -> config::AppResult<events::Model> {
        Ok(events::Model {
            id: 0,
            tx_id: self.tx_info.tx_id.ok_or(Error::BlockInfoError("tx_id".to_string()))?,
            from: self.event_info.from.clone(),
            to: self.event_info.to.clone(),
            value: self.event_info.value.clone(),
            log_index: self.event_info.log_index,
        })
    }
}

impl TryInto<transactions::Model> for &mut BlockInfo {
    type Error = Error;

    fn try_into(self) -> config::AppResult<transactions::Model> {
        Ok(transactions::Model {
            id: 0,
            block_id: self.block_id.ok_or(Error::BlockInfoError("block_id".to_string()))?,
            hash: self.tx_info.tx_hash.clone(),
            tag_index: self.tx_info.tx_index,
        })
    }
}

impl BlockInfo {
    fn new_with_tag(tag: BlockNumberOrTag) -> BlockInfo {
        let mut block_info = BlockInfo::default();
        block_info.tag = Some(tag);

        block_info
    }

    fn flush(&mut self) {
        self.block_id = None;
        self.tx_info.tx_id = None;
    }

    fn set_block_info(&mut self, log: &alloy_rpc_types_eth::Log) -> config::AppResult<()> {
        let block_number = log
            .block_number
            .ok_or(Error::BlockInfoError("block_number".to_string()))?;
        let new_block_number = i64::try_from(block_number)?;
        let new_block_hash = log
            .block_hash
            .ok_or(Error::BlockInfoError("block_hash".to_string()))?
            .to_string();
        if self.block_number != new_block_number {
            self.flush();
            self.block_hash = new_block_hash;
            self.block_number = new_block_number;
        }

        Ok(())
    }

    pub fn is_new_tx(&mut self, log: &alloy_rpc_types_eth::Log) -> bool {
        let tx_index = log.transaction_index.ok_or(Error::BlockInfoError("transaction_index".to_string())).unwrap();
        let new_tx_index = i64::try_from(tx_index).unwrap();

        if self.tx_info.tx_index != new_tx_index {
            self.tx_info.tx_id = None;
            return true;
        }

        return false;
    }

    pub fn set_block_id(&mut self, block_id: i32) -> config::AppResult<()> {
        if self.block_id.is_none() {
            self.block_id = Some(block_id);
        }

        Ok(())
    }

    pub fn set_tx_info(&mut self, log: &alloy_rpc_types_eth::Log) -> config::AppResult<()> {
        let tx_hash = log.transaction_hash.ok_or(Error::BlockInfoError("transaction_hash".to_string()))?.to_string();
        let tx_index = log.transaction_index.ok_or(Error::BlockInfoError("transaction_index".to_string()))?;
        self.tx_info.tx_hash = tx_hash.clone();
        self.tx_info.tx_index = i64::try_from(tx_index)?;

        Ok(())
    }

    pub fn set_event_info(&mut self, log: &alloy_rpc_types_eth::Log) -> config::AppResult<()> {
        let decoded_event = Transfer::decode_log(&log.inner)?;
        let log_index = log.log_index.ok_or(Error::BlockInfoError("log_index".to_string()))?;
        self.event_info.from = decoded_event.from.clone().to_string();
        self.event_info.to = decoded_event.to.clone().to_string();
        self.event_info.value = decoded_event.value.clone().to_string();
        self.event_info.log_index = i64::try_from(log_index)?;

        Ok(())
    }

    pub fn has_block_id(&self) -> bool {
        self.block_id.is_some()
    }

    pub fn set_tx_id(&mut self, tx_id: i32) {
        if self.tx_info.tx_id.is_none() {
            self.tx_info.tx_id = Some(tx_id)
        }
    }

    pub fn has_tx_id(&self) -> bool {
        self.tx_info.tx_id.is_some()
    }

    pub fn set_event_id(&mut self, event_id: i32) {
        self.event_info.id = event_id;
    }
}

pub async fn run(cfg: config::Indexer, db_connection: &db::DB) -> config::AppResult<()> {
    let ws = WsConnect::new(cfg.rpc_url.clone());
    let provider = ProviderBuilder::new().connect_ws(ws).await?;
    log::info!("Connect to RPC node: {:}", cfg.rpc_url);

    let contract_address = Address::from_str(cfg.contract_address.as_str())?;
    let block_tag = BlockNumberOrTag::from_str(cfg.block_tag.as_str())?;
    let filter = Filter::new()
        .address(contract_address)
        .event(Transfer::SIGNATURE)
        .from_block(block_tag);

    let sub = provider.subscribe_logs(&filter).await?;
    log::info!(
        "Subscribe to the contract address: {:}",
        cfg.contract_address
    );
    let mut stream = sub.into_stream();
    let mut block_info = BlockInfo::new_with_tag(block_tag);
    let mut counter = 1;
    while let Some(log) = stream.next().await {
        block_info.set_block_info(&log)?;
        db::insert_data(db_connection.get_connection(), &mut block_info, &log).await?;
        counter +=1;
        log::info!("inserted data: {:}", counter);
    }

    Ok(())
}
