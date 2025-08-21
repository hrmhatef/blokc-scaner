pub mod indexer;
pub mod server;

use utils::result::AppResult;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub trait Load {
    fn load(path: PathBuf) -> AppResult<Self> where Self: Sized;
}

pub trait IsValid {
    fn is_valid(&self) -> AppResult<()>;
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Database {
    pub url: String,
}
