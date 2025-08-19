use crate::orm::entities::prelude::*;
use crate::orm::entities::*;
use crate::{config, indexer};

use sea_orm::*;

pub struct DB {
    connection: DatabaseConnection,
}

pub async fn new(db_config: config::Database) -> config::AppResult<DB> {
    let db = Database::connect(db_config.url).await?;
    Ok(DB { connection: db })
}

impl DB {
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub async fn close(&self) -> config::AppResult<()> {
        let _ = &self.connection.close_by_ref().await?;

        Ok(())
    }
}

pub async fn insert_data(
    db: &DatabaseConnection,
    block_info: &mut indexer::BlockInfo,
    log: &alloy_rpc_types_eth::Log,
) -> config::AppResult<()> {
    if !block_info.has_block_id() {
        let res = add_block_info(&db, block_info.try_into()?).await?;
        block_info.set_block_id(res.id)?;
    }

    if !block_info.has_tx_id() || block_info.is_new_tx(&log) {
        block_info.set_tx_info(&log)?;
        let res = add_tx_info(&db, block_info.try_into()?).await?;
        block_info.set_tx_id(res.id);
    }

    block_info.set_event_info(&log)?;
    let res = add_event_info(&db, block_info.try_into()?).await?;
    block_info.set_event_id(res.id);

    Ok(())
}

async fn add_block_info(
    db: &DatabaseConnection,
    block_info: blocks::Model,
) -> config::AppResult<blocks::Model> {
    let active_model = blocks::ActiveModel {
        block_number: Set(block_info.block_number),
        hash: Set(block_info.hash.to_owned()),
        timestamp: Set(block_info.timestamp),
        tag: Set(block_info.tag.to_owned()),
        is_removed: Set(block_info.is_removed),
        ..Default::default()
    };

    let res = Blocks::insert(active_model).exec(db).await?;

    Ok(blocks::Model {
        id: res.last_insert_id,
        ..block_info
    })
}

async fn add_tx_info(
    db: &DatabaseConnection,
    tx_info: transactions::Model,
) -> config::AppResult<transactions::Model> {
    let active_model = transactions::ActiveModel {
        block_id: Set(tx_info.block_id),
        hash: Set(tx_info.hash.clone()),
        tag_index: Set(tx_info.tag_index),
        ..Default::default()
    };

    let res = Transactions::insert(active_model).exec(db).await?;

    Ok(transactions::Model {
        id: res.last_insert_id,
        ..tx_info
    })
}

async fn add_event_info(
    db: &DatabaseConnection,
    event_info: events::Model,
) -> config::AppResult<events::Model> {
    let active_model = events::ActiveModel {
        tx_id: Set(event_info.tx_id),
        from: Set(event_info.from.clone()),
        to: Set(event_info.to.clone()),
        value: Set(event_info.value.clone()),
        log_index: Set(event_info.log_index),
        ..Default::default()
    };

    let res = Events::insert(active_model).exec(db).await?;

    Ok(events::Model {
        id: res.last_insert_id,
        ..event_info
    })
}
