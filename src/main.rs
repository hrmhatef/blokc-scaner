mod cmd;
mod error;
mod config;

use std::{path::PathBuf, str::FromStr};

use env_logger::Builder;
use log::LevelFilter;

#[tokio::main]
async fn main() -> config::AppResult<()> {
    let args = cmd::parse();
    let cfg = config::load(PathBuf::from(args.config_path))?;

    let level = LevelFilter::from_str(&cfg.log.level).expect("failed to parse the log level");

    let _logger = Builder::new()
        .filter_level(level)
        .format_timestamp_secs()
        .init();

    log::info!("hello zama");

    Ok(())
}
