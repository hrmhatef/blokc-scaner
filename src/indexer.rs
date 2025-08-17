use crate::config;
use crate::error::Error;

use std::str::FromStr;

use alloy::{
    sol_types::SolEvent,
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
};
use futures_util::stream::StreamExt;

sol!{
   #[derive(Debug)]
   event Transfer(address indexed from, address indexed to, uint value);
}

pub async fn run(cfg: config::Indexer) -> config::AppResult<()> {
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
    log::info!("Subscribe to the contract address: {:}", cfg.contract_address);
    let mut stream = sub.into_stream();
    while let Some(log) = stream.next().await {
        let decoded_event = Transfer::decode_log(&log.inner)?;
        let block_number = log.block_number.ok_or(Error::BlockNumberError)?;
        log::trace!("The transfer event of block ({:}): {:#?}", block_number, decoded_event);
    };

    Ok(())
}
