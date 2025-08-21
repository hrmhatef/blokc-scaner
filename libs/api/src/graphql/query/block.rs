use orm::{blocks_query::*, db};

use std::sync::Arc;

use async_graphql::{Context, Object, Result, SimpleObject};

#[derive(Default)]
pub struct BlockQuery;

#[Object]
impl BlockQuery {
    async fn blocks(&self, ctx: &Context<'_>) -> Result<GraphqlResult> {
        let db = ctx.data_unchecked::<Arc<db::DB>>();
        let res = blocks_by_filter(db.clone()).await.unwrap();

        Ok(GraphqlResult { data: res })
    }

    async fn get_total_blocks(&self, ctx: &Context<'_>) -> Result<u64> {
        let db = ctx.data_unchecked::<Arc<db::DB>>();

        Ok(get_total_blocks(db.clone()).await.unwrap())
    }
}

/// The `ShowsPage` result type
#[derive(Clone, SimpleObject)]
pub struct GraphqlResult {
    /// The list of `Shows` returned for the current page
    data: Vec<BlockResult>,
}
