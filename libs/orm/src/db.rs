use crate::entities::prelude::*;
use crate::entities::{blocks, events, transactions};

use utils::{config, result::AppResult};

use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection, EntityTrait};

pub async fn new(db_config: config::Database) -> AppResult<DB> {
    let db = Database::connect(db_config.url).await?;

    Ok(DB { connection: db })
}

#[derive(Debug, Clone)]
pub struct DB {
    connection: DatabaseConnection,
}

impl DB {
    #[must_use]
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub async fn close(&self) -> AppResult<()> {
        let () = &self.connection.close_by_ref().await?;

        Ok(())
    }
}

pub async fn get_blocks_info(db: Arc<DB>, _filter: String) -> AppResult<Vec<blocks::Model>> {
    let res = Blocks::find().all(db.get_connection()).await?;

    Ok(res)
}

pub async fn get_transactions_info(
    db: Arc<DB>,
    _filter: String,
) -> AppResult<Vec<transactions::Model>> {
    let res = Transactions::find().all(db.get_connection()).await?;

    Ok(res)
}

pub async fn get_events_info(db: Arc<DB>, _filter: String) -> AppResult<Vec<events::Model>> {
    let res = Events::find().all(db.get_connection()).await?;

    Ok(res)
}
