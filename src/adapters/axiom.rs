use super::super::log_watcher::LogEntry;
use crate::adapters::log_adapter::LogAdapter;
use axiom_rs::client::Client;
use std::error::Error;

pub struct AxiomAdapter {
    client: Client,
    dataset: String,
}

impl AxiomAdapter {
    pub fn new(api_key: &str, dataset: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder().with_token(api_key).build()?;
        Ok(AxiomAdapter {
            client,
            dataset: dataset.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl LogAdapter for AxiomAdapter {
    async fn ingest(&self, log_entry: Vec<LogEntry>) -> Result<(), Box<dyn Error>> {
        self.client.ingest(self.dataset.clone(), log_entry).await?;
        Ok(())
    }
}
