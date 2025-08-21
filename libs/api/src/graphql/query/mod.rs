pub mod block;

pub use block::BlockQuery;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(BlockQuery);
