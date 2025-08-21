use orm::{
    db,
    entities::prelude::*,
    entities::{blocks, events, transactions},
};
use utils::{config, error::Error, result::AppResult};

use std::str::FromStr;

use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
    sol_types::SolEvent,
};
use futures_util::stream::StreamExt;
use sea_orm::*;
use serde::{Deserialize, Serialize};

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

impl BlockInfo {
    fn new_with_tag(tag: BlockNumberOrTag) -> BlockInfo {
        BlockInfo {
            tag: Some(tag),
            ..Default::default()
        }
    }

    fn flush(&mut self) {
        self.block_id = None;
        self.tx_info.tx_id = None;
    }

    fn set_block_info(&mut self, log: &alloy_rpc_types_eth::Log) -> AppResult<()> {
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
        let tx_index = log
            .transaction_index
            .ok_or(Error::BlockInfoError("transaction_index".to_string()))
            .unwrap();
        let new_tx_index = i64::try_from(tx_index).unwrap();
        let new_tx_hash = log
            .transaction_hash
            .ok_or(Error::BlockInfoError("transactions_hash".to_string()))
            .unwrap()
            .to_string();

        if self.tx_info.tx_index != new_tx_index || self.tx_info.tx_hash != new_tx_hash {
            self.tx_info.tx_id = None;
            return true;
        }

        false
    }

    pub fn set_block_id(&mut self, block_id: i32) -> AppResult<()> {
        if self.block_id.is_none() {
            self.block_id = Some(block_id);
        }

        Ok(())
    }

    pub fn set_tx_info(&mut self, log: &alloy_rpc_types_eth::Log) -> AppResult<()> {
        let tx_hash = log
            .transaction_hash
            .ok_or(Error::BlockInfoError("transaction_hash".to_string()))?
            .to_string();
        let tx_index = log
            .transaction_index
            .ok_or(Error::BlockInfoError("transaction_index".to_string()))?;
        self.tx_info.tx_hash = tx_hash.clone();
        self.tx_info.tx_index = i64::try_from(tx_index)?;

        Ok(())
    }

    pub fn set_event_info(&mut self, log: &alloy_rpc_types_eth::Log) -> AppResult<()> {
        let decoded_event = Transfer::decode_log(&log.inner)?;
        let log_index = log
            .log_index
            .ok_or(Error::BlockInfoError("log_index".to_string()))?;
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

impl TryInto<blocks::Model> for &mut BlockInfo {
    type Error = Error;

    fn try_into(self) -> AppResult<blocks::Model> {
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

    fn try_into(self) -> AppResult<events::Model> {
        Ok(events::Model {
            id: 0,
            tx_id: self
                .tx_info
                .tx_id
                .ok_or(Error::BlockInfoError("tx_id".to_string()))?,
            from: self.event_info.from.clone().to_lowercase(),
            to: self.event_info.to.clone().to_lowercase(),
            value: self.event_info.value.clone().to_lowercase(),
            log_index: self.event_info.log_index,
        })
    }
}

impl TryInto<transactions::Model> for &mut BlockInfo {
    type Error = Error;

    fn try_into(self) -> AppResult<transactions::Model> {
        Ok(transactions::Model {
            id: 0,
            block_id: self
                .block_id
                .ok_or(Error::BlockInfoError("block_id".to_string()))?,
            hash: self.tx_info.tx_hash.clone(),
            tag_index: self.tx_info.tx_index,
        })
    }
}

sol! {
   #[derive(Debug)]
   event Transfer(address indexed from, address indexed to, uint value);
}

pub async fn run(cfg: config::Indexer, db_connection: db::DB) -> AppResult<()> {
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
    while let Some(log) = stream.next().await {
        block_info.set_block_info(&log)?;
        insert_data(&db_connection, &mut block_info, &log).await?;
        log::info!("Data added into DB:\n{block_info:#?}");
    }

    Ok(())
}

pub async fn insert_data(
    db: &db::DB,
    block_info: &mut BlockInfo,
    log: &alloy_rpc_types_eth::Log,
) -> AppResult<()> {
    if !block_info.has_block_id() {
        let res = add_block_info(db, block_info.try_into()?).await?;
        block_info.set_block_id(res.id)?;
    }

    if !block_info.has_tx_id() || block_info.is_new_tx(log) {
        block_info.set_tx_info(log)?;
        let res = add_tx_info(db, block_info.try_into()?).await?;
        block_info.set_tx_id(res.id);
    }

    block_info.set_event_info(log)?;
    let res = add_event_info(db, block_info.try_into()?).await?;
    block_info.set_event_id(res.id);

    Ok(())
}

async fn add_block_info(db: &db::DB, block_info: blocks::Model) -> AppResult<blocks::Model> {
    let active_model = blocks::ActiveModel {
        block_number: Set(block_info.block_number),
        hash: Set(block_info.hash.to_owned()),
        timestamp: Set(block_info.timestamp),
        tag: Set(block_info.tag.to_owned()),
        is_removed: Set(block_info.is_removed),
        ..Default::default()
    };

    let res = Blocks::insert(active_model)
        .exec(db.get_connection())
        .await?;

    Ok(blocks::Model {
        id: res.last_insert_id,
        ..block_info
    })
}

async fn add_tx_info(db: &db::DB, tx_info: transactions::Model) -> AppResult<transactions::Model> {
    let active_model = transactions::ActiveModel {
        block_id: Set(tx_info.block_id),
        hash: Set(tx_info.hash.clone()),
        tag_index: Set(tx_info.tag_index),
        ..Default::default()
    };

    let res = Transactions::insert(active_model)
        .exec(db.get_connection())
        .await?;

    Ok(transactions::Model {
        id: res.last_insert_id,
        ..tx_info
    })
}

async fn add_event_info(db: &db::DB, event_info: events::Model) -> AppResult<events::Model> {
    let active_model = events::ActiveModel {
        tx_id: Set(event_info.tx_id),
        from: Set(event_info.from.clone()),
        to: Set(event_info.to.clone()),
        value: Set(event_info.value.clone()),
        log_index: Set(event_info.log_index),
        ..Default::default()
    };

    let res = Events::insert(active_model)
        .exec(db.get_connection())
        .await?;

    Ok(events::Model {
        id: res.last_insert_id,
        ..event_info
    })
}
