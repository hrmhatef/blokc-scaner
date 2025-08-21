use crate::result::AppResult;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use twelf::{Layer, config};

/// # Errors
///
/// Will return `Err` if `filename` does not exist or any problem to find the right config.
pub fn load(path: PathBuf) -> AppResult<Config> {
    // Layer from different sources to build configuration. Order matters!
    let conf = Config::with_layers(&[Layer::Yaml(path), Layer::Env(Some(String::from("APP_")))])?;
    Ok(conf)
}

#[config]
#[derive(Debug, Clone, Default, Serialize)]
pub struct Config {
    pub log: Log,
    pub api: API,
    pub db: Database,
    pub indexer: Indexer,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct API {
    pub port: u16,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Indexer {
    pub rpc_url: String,
    pub contract_address: String,
    pub block_tag: String,
}
