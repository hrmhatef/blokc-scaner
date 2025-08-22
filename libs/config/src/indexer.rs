use crate::{Database, Load, Log};

use utils::result::AppResult;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use twelf::{Layer, config};

#[config]
#[derive(Debug, Clone, Default, Serialize)]
pub struct Config {
    pub log: Log,
    pub db: Database,
    pub indexer: Indexer,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Indexer {
    pub rpc_url: String,
    pub contract_address: String,
    pub block_tag: String,
}

impl Load for Config {
    fn load(path: PathBuf) -> AppResult<Self>
    where
        Self: Sized,
    {
        let conf =
            Config::with_layers(&[Layer::Yaml(path), Layer::Env(Some(String::from("APP_")))])?;
        Ok(conf)
    }
}
