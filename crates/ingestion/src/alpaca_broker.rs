use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, error};

/// Request structure mapping heavily to Alpaca's JSON schema for `POST /v2/orders`.
/// Note: Extended hours and Trailing Stops are omitted in this basic implementation.
#[derive(Serialize, Debug)]
pub struct AlpacaOrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub side: String,
    pub type_: String,
    pub time_in_force: String,
}

#[derive(Debug, Deserialize)]
pub struct AlpacaOrderResponse {
    pub id: String,
    pub client_order_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub submitted_at: String,
    pub filled_at: Option<String>,
    pub expired_at: Option<String>,
    pub canceled_at: Option<String>,
    pub failed_at: Option<String>,
    pub replaced_at: Option<String>,
    pub replaced_by: Option<String>,
    pub replaces: Option<String>,
    pub asset_id: String,
    pub symbol: String,
    pub asset_class: String,
    pub notional: Option<String>,
    pub qty: Option<String>,
    pub filled_qty: String,
    pub filled_avg_price: Option<String>,
    pub order_class: String,
    pub order_type: String,
    pub side: String,
    pub time_in_force: String,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub status: String,
    pub extended_hours: bool,
    pub legs: Option<Vec<serde_json::Value>>,
    pub trail_percent: Option<String>,
    pub trail_price: Option<String>,
    pub hwm: Option<String>,
}

#[derive(Clone)]
pub struct AlpacaBroker {
    client: Client,
    api_key: String,
    secret_key: String,
    base_url: String,
}

impl AlpacaBroker {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            secret_key,
            base_url: "https://paper-api.alpaca.markets".to_string(), // Defaulting to paper trading for safety
        }
    }

    pub async fn submit_order(&self, request: AlpacaOrderRequest) -> Result<AlpacaOrderResponse> {
        let url = format!("{}/v2/orders", self.base_url);
        
        info!("Submitting Alpaca order: {:?}", request);

        let response = self.client.post(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.secret_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Alpaca order rejected: {}", error_text);
            anyhow::bail!("Order failed: {}", error_text);
        }

        let order_response: AlpacaOrderResponse = response.json().await?;
        Ok(order_response)
    }

    pub async fn get_positions(&self) -> Result<serde_json::Value> {
        let url = format!("{}/v2/positions", self.base_url);
        let response = self.client.get(&url)
            .header("APCA-API-KEY-ID", &self.api_key)
            .header("APCA-API-SECRET-KEY", &self.secret_key)
            .send()
            .await?;
        
        Ok(response.json().await?)
    }
}
