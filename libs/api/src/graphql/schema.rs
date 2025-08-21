use crate::server;

use crate::graphql::query::Query;

use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema(ctx: Arc<server::Context>) -> GraphQLSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(ctx.config.clone())
        .data(ctx.db.clone())
        .finish()
}
