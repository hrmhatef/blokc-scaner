mod cmd;
mod error;
mod config;
mod indexer;
mod database;

use std::{path::PathBuf, str::FromStr};

use config::AppResult;
use env_logger::Builder;
use log::LevelFilter;

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = cmd::parse();
    let cfg = config::load(PathBuf::from(args.config_path))?;

    let level = LevelFilter::from_str(&cfg.log.level).expect("failed to parse the log level");

    Builder::new()
        .filter_level(level)
        .format_timestamp_secs()
        .init();

    log::info!("hello zama");

    indexer::run(cfg.indexer).await
}
