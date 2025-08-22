use api::server;
use config::{self, IsValid, Load};
use utils::{cmd, result::AppResult};

use std::{path::PathBuf, str::FromStr, sync::Arc};

use env_logger::Builder;
use log::LevelFilter;

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = cmd::parse();
    let cfg: config::server::Config = Load::load(PathBuf::from(args.config_path))?;

    let level = LevelFilter::from_str(&cfg.log.level).expect("failed to parse the log level");

    cfg.is_valid()?;

    Builder::new()
        .filter_level(level)
        .format_timestamp_secs()
        .init();

    log::info!("App will run by the following config:\n{cfg:#?}");

    log::info!(
        "Trying to make a connection with the DB: {:}",
        cfg.db.url.clone()
    );
    let db = orm::db::new(cfg.db.clone()).await?;
    log::info!("Database is connected...");

    let arc_db = Arc::new(db.clone());
    let context = Arc::new(api::server::Context::init(cfg.api.clone(), arc_db).await?);
    server::run(context).await?;

    db.close().await?;
    log::info!("The DB connection is closed");

    Ok(())
}
