use crate::graphql::schema::{GraphQLSchema, build_schema};

use orm::db;
use utils::result::AppResult;
use config;

use std::sync::Arc;

use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};

/// Dependencies needed by the resolvers
pub struct Context {
    pub config: config::server::Api,

    /// The database connections
    pub db: Arc<db::DB>,
}

/// Intialize dependencies
impl Context {
    /// Create a new set of dependencies based on the given shared resources
    pub async fn init(config: config::server::Api, db_connection: Arc<db::DB>) -> AppResult<Self> {
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

    log::info!("Playground: http://localhost:{port}/api/graphql");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
