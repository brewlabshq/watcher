mod log_watcher;
mod watcher_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut watcher = log_watcher::LogWatcher::from_file("config.toml")?; // change config.toml
    watcher.watch().await?;
    Ok(())
}
