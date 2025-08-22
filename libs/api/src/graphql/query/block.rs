use orm::{blocks_query::*, db};

use std::sync::Arc;

use async_graphql::{Context, Object, Result, SimpleObject};

#[derive(Default)]
pub struct BlockQuery;

#[Object]
impl BlockQuery {
    // returns all events based on the provided blockNumber
    async fn blocks(&self, ctx: &Context<'_>, block_number: i64) -> Result<BlocksInfo> {
        let db = ctx.data_unchecked::<Arc<db::DB>>();

        let res = blocks_by_filter(db.clone(), block_number).await?;

        Ok(BlocksInfo { data: res })
    }

    // returns all events which is the address is equal with From or To
    async fn blocks_by_address(&self, ctx: &Context<'_>, address: String) -> Result<BlocksInfo> {
        let db = ctx.data_unchecked::<Arc<db::DB>>();

        let res = get_block_info_by_address(db.clone(), address).await?;

        Ok(BlocksInfo { data: res })
    }

    // returns all events which is the address is in same place of From and To
    async fn circular_trnasactions(
        &self,
        ctx: &Context<'_>,
        address: String,
    ) -> Result<BlocksInfo> {
        let db = ctx.data_unchecked::<Arc<db::DB>>();

        let res = get_circular_transactions(db.clone(), address).await?;

        Ok(BlocksInfo { data: res })
    }

    // return total blocks stored on the DB
    async fn total_blocks(&self, ctx: &Context<'_>) -> Result<u64> {
        let db = ctx.data_unchecked::<Arc<db::DB>>();

        Ok(get_total_blocks(db.clone()).await?)
    }

    // returns total events of the DB
    async fn total_events(&self, ctx: &Context<'_>) -> Result<u64> {
        let db = ctx.data_unchecked::<Arc<db::DB>>();

        Ok(get_total_events(db.clone()).await?)
    }
}

#[derive(Clone, SimpleObject)]
pub struct BlocksInfo {
    data: Vec<BlockResult>,
}
