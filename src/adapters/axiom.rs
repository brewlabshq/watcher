use axiom_rs::client::Client;
use serde_json::Value;

use super::Adapter;

pub fn init(token: String) -> Result<Client, Box<dyn std::error::Error>> {
    Ok(Client::builder()
        .with_token(token)
        .build()?)
}

pub async fn ingest(
    client: Client,
    dataset: String,
    data: Value
) -> Result<(), Box<dyn std::error::Error>> {
    client
        .ingest(dataset, vec![data])
        .await?;
    Ok(())
}

