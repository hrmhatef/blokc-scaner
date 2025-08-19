mod cmd;
mod config;
mod error;
mod indexer;
mod orm;

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

    log::info!(
        "Trying to make a connection with the DB: {:}",
        cfg.db.url.clone()
    );
    let db = orm::db::new(cfg.db).await?;
    log::info!("Database is connected...");

    indexer::run(cfg.indexer, &db).await?;

    db.close().await?;
    log::info!("The DB connection is closed");

    Ok(())
}
