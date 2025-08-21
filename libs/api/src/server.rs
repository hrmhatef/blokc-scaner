use crate::graphql::schema::{build_schema, GraphQLSchema};

use utils::{config, result::AppResult};
use orm::db;

use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

/// Dependencies needed by the resolvers
pub struct Context {
    pub config: config::API,

    /// The database connections
    pub db: Arc<db::DB>,
}

/// Intialize dependencies
impl Context {
    /// Create a new set of dependencies based on the given shared resources
    pub async fn init(config: config::API, db_connection: Arc<db::DB>) -> AppResult<Self> {
        Ok(Self {
            config,
            db: db_connection,
        })
    }
}

async fn graphql_handler(schema: State<GraphQLSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

pub async fn run(ctx: Arc<Context>) -> AppResult<()> {
    let port = ctx.config.port;
    let schema = build_schema(ctx).await;

    let router = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(schema);

    log::info!("Playground: http://localhost:{:}/api/graphql", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{:}", port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
