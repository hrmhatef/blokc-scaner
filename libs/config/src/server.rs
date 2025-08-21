use crate::{Database, Log, Load, IsValid};

use utils::{error::Error, result::AppResult};

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use twelf::{Layer, config};

#[config]
#[derive(Debug, Clone, Default, Serialize)]
pub struct Config {
    pub log: Log,
    pub db: Database,
    pub api: Api,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Api {
    pub port: u16,
}

impl Load for Config {
    fn load(path: PathBuf) -> AppResult<Self> where Self: Sized {
        let conf = Config::with_layers(&[Layer::Yaml(path), Layer::Env(Some(String::from("APP_")))])?;
        Ok(conf)
    }
}


impl IsValid for Config {
    fn is_valid(&self) -> AppResult<()> {
        if self.api.port < 4000 && self.api.port > 9999 {
            return Err(Error::InvalidPortNumber);
        }

        Ok(())
    }
}
