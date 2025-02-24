use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log_service: LogServiceConfig,
}

#[derive(Debug, Deserialize)]
pub struct LogServiceConfig {
    pub service: String,
    pub ingestion_url: Option<String>,
    pub api_key: String,
    pub dataset: Option<String>,
}

impl LogServiceConfig {
    pub fn build(config_path: &str) -> Result<Self, Box<dyn Error>> {
        let config_str = std::fs::read_to_string(config_path)?;

        let config: Config = toml::from_str(&config_str)?;

        Ok(config.log_service)
    }
}
