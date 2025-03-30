use crate::adapters::log_adapter::create_log_adapter;
mod adapters;
mod log_config;
mod log_watcher;
mod watcher_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut watcher = log_watcher::LogWatcher::from_file("config.toml")?;

    let config = log_config::LogServiceConfig::build("config.toml")?;

    let log_adapter = create_log_adapter(&config)?;

    watcher.watch(log_adapter.into()).await?;

    Ok(())
}
