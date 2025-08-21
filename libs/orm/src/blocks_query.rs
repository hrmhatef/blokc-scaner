use crate::db::DB;
use crate::entities::prelude::*;
use crate::entities::*;

use utils::result::AppResult;

use std::sync::Arc;

use async_graphql::SimpleObject;
use sea_orm::{QueryOrder, prelude::*};
use serde::{Deserialize, Serialize};

pub struct BlockFilters {
    pub block_number: i64,
}

pub struct Filter {
    pub limit: i32,
    pub offset: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, SimpleObject)]
pub struct EventInfo {
    from: String,
    to: String,
    value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, SimpleObject)]
pub struct BlockResult {
    block_number: i64,
    event_info: EventInfo,
    block_hash: String,
    timestamp: Option<i64>,
    tag: String,
    transaction_hash: String,
    tag_index: i64,
    log_index: i64,
    is_removed: bool,
}

pub async fn blocks_by_filter(
    // _query_filter: BlockFilters,
    db: Arc<DB>,
) -> AppResult<Vec<BlockResult>> {
    let items: Vec<(
        blocks::Model,
        Option<transactions::Model>,
        Option<events::Model>,
    )> = Blocks::find()
        .find_also_related(Transactions)
        .and_also_related(Events)
        .order_by_asc(events::Column::Id)
        .filter(blocks::Column::BlockNumber.eq(23174921))
        .all(db.get_connection())
        .await?;

    let mut result: Vec<BlockResult> = vec![];
    for item in items {
        let tx_info = item.1.as_ref().expect("msg");
        let event_info = item.2.as_ref().expect("msg");
        let res = BlockResult {
            block_number: item.0.block_number,
            block_hash: item.0.hash.clone(),
            tag: item.0.tag.clone(),
            transaction_hash: tx_info.hash.clone(),
            tag_index: tx_info.tag_index,
            log_index: event_info.log_index,
            event_info: EventInfo {
                to: event_info.to.clone(),
                from: event_info.from.clone(),
                value: event_info.from.clone(),
            },
            ..Default::default()
        };

        result.push(res);
    }

    Ok(result)
}

pub async fn get_total_blocks(db: Arc<DB>) -> AppResult<u64> {
    Ok(Blocks::find().count(db.get_connection()).await?)
}
