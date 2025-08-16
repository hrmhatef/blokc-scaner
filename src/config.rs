use crate::error;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use twelf::{config, Layer};

pub type AppResult<T> = std::result::Result<T, error::Error>;

pub fn load(path: PathBuf) -> AppResult<Config> {
    let path = path.into();
    // Layer from different sources to build configuration. Order matters!
    let conf = Config::with_layers(&[Layer::Yaml(path), Layer::Env(Some(String::from("APP_")))])?;
    Ok(conf)
}

#[config]
#[derive(Debug, Default, Serialize)]
pub struct Config {
    pub log: Log,
    pub api: API,
    pub db: Database,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct API {
    pub port: u16,
}
