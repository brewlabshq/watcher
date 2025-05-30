use super::super::log_watcher::LogEntry;
use crate::adapters::log_adapter::LogAdapter;
use reqwest::Client;
use serde::Serialize;
use std::error::Error;

pub struct BetterStackClient {
    client: Client,
    api_key: String,
    ingestion_url: String,
}

impl BetterStackClient {
    pub fn new(api_key: String, ingestion_url: String) -> Self {
        BetterStackClient {
            client: Client::new(),
            api_key,
            ingestion_url,
        }
    }

    pub async fn ingest<I, E>(&self, events: I) -> Result<(), Box<dyn Error>>
    where
        I: IntoIterator<Item = E>,
        E: Serialize,
    {
        let url = &self.ingestion_url;

        let events_vec: Vec<E> = events.into_iter().collect();
        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&events_vec)
            .send()
            .await?;

        if response.status().is_success() {
            println!("Logs successfully ingested to betterstack.");
        } else {
            let err_body = response.text().await?;
            println!("Failed to ingest logs: {}", err_body);
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl LogAdapter for BetterStackClient {
    async fn ingest(&self, data: Vec<LogEntry>) -> Result<(), Box<dyn Error>> {
        self.ingest(data).await?;
        Ok(())
    }
}
