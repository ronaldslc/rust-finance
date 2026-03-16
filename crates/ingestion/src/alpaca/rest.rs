use anyhow::{Result, Context};
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AlpacaConfig {
    pub key_id: String,
    pub secret_key: String,
    pub base_url: String, // e.g., "https://paper-api.alpaca.markets" for sandbox
}

/// Simple REST client for Alpaca Trading API endpoints
pub struct AlpacaRestClient {
    client: Client,
    base_url: String,
}

impl AlpacaRestClient {
    pub fn new(config: AlpacaConfig) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "APCA-API-KEY-ID",
            header::HeaderValue::from_str(&config.key_id)?,
        );
        headers.insert(
            "APCA-API-SECRET-KEY",
            header::HeaderValue::from_str(&config.secret_key)?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to build Alpaca reqwest Client")?;

        Ok(Self {
            client,
            base_url: config.base_url.trim_end_matches('/').to_string(),
        })
    }

    /// Fetches account details including buying power
    pub async fn get_account(&self) -> Result<Account> {
        let url = format!("{}/v2/account", self.base_url);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Fetches all open positions
    pub async fn get_positions(&self) -> Result<Vec<Position>> {
        let url = format!("{}/v2/positions", self.base_url);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Submits a market or limit order
    pub async fn place_order(&self, req: &OrderRequest) -> Result<Order> {
        let url = format!("{}/v2/orders", self.base_url);
        let resp = self.client.post(&url).json(req).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Cancels all open orders
    pub async fn cancel_all_orders(&self) -> Result<()> {
        let url = format!("{}/v2/orders", self.base_url);
        self.client.delete(&url).send().await?.error_for_status()?;
        Ok(())
    }
}

// ── Models ────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: String,
    pub status: String,
    pub currency: String,
    pub buying_power: String,
    pub regt_buying_power: String,
    pub daytrading_buying_power: String,
    pub non_marginable_buying_power: String,
    pub cash: String,
    pub portfolio_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub asset_id: String,
    pub symbol: String,
    pub qty: String,
    pub avg_entry_price: String,
    pub current_price: String,
    pub unrealized_pl: String,
    pub unrealized_plpc: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub side: String, // "buy" or "sell"
    #[serde(rename = "type")]
    pub order_type: String, // "market", "limit", "stop"
    pub time_in_force: String, // "day", "gtc"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub qty: Option<String>,
    pub status: String,
}
