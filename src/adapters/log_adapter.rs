use super::super::log_watcher::LogEntry;
use crate::adapters::axiom::AxiomAdapter;
use crate::adapters::betterstack::BetterStackClient;
use crate::adapters::datadog::DatadogClient;
use crate::log_config::LogServiceConfig;
use std::error::Error;

#[async_trait::async_trait]
pub trait LogAdapter: Send + Sync {
    async fn ingest(&self, log_entry: Vec<LogEntry>) -> Result<(), Box<dyn Error>>;
}

pub fn create_log_adapter(
    config: &LogServiceConfig,
) -> Result<Box<dyn LogAdapter>, Box<dyn Error>> {
    match config.service.as_str() {
        "axiom" => {
            let dataset = config
                .dataset
                .as_ref()
                .ok_or("Dataset is required for Axiom")?;
            let adapter = AxiomAdapter::new(config.api_key.as_str(), dataset.as_str())?;
            Ok(Box::new(adapter))
        }
        "datadog" => {
            if let Some(ingestion_url) = &config.ingestion_url {
                let adapter = DatadogClient::new(config.api_key.clone(), ingestion_url.to_string());

                Ok(Box::new(adapter))
            } else {
                panic!("ingestion_url is required to use BetterStack.");
            }
        }
        "betterstack" => {
            if let Some(ingestion_url) = &config.ingestion_url {
                let adapter =
                    BetterStackClient::new(config.api_key.to_string(), ingestion_url.to_string());

                Ok(Box::new(adapter))
            } else {
                panic!("ingestion_url is required to use BetterStack.");
            }
        }
        _ => Err("Unsupported log service".into()),
    }
}
