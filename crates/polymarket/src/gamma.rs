use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GammaMarket {
    pub condition_id: String,
    pub question: String,
    pub tokens: Vec<Token>,
    pub active: bool,
    pub closed: bool,
    pub volume: Option<String>,
    pub volume_24hr: Option<f64>,
    pub outcome_prices: Option<String>,   // JSON string: "[0.65, 0.35]"
    pub neg_risk: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub token_id: String,
    pub outcome: String,  // "Yes" or "No"
    pub price: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct GammaClient {
    http: Client,
    base_url: String,
}

impl GammaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// Fetch active, open markets
    pub async fn get_active_markets(
        &self,
        limit: u32,
        offset: u32,
    ) -> anyhow::Result<Vec<GammaMarket>> {
        let url = format!("{}/markets", self.base_url);
        let resp = self.http
            .get(&url)
            .query(&[
                ("closed", "false"),
                ("active", "true"),
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ])
            .send()
            .await?
            .json::<Vec<GammaMarket>>()
            .await?;
        Ok(resp)
    }

    /// Search markets by query
    pub async fn search_markets(&self, query: &str) -> anyhow::Result<Vec<GammaMarket>> {
        let url = format!("{}/markets", self.base_url);
        let resp = self.http
            .get(&url)
            .query(&[("closed", "false"), ("limit", "20")])
            // Gamma doesn't have a search param directly;
            // filter client-side or use the CLOB search endpoint
            .send()
            .await?
            .json::<Vec<GammaMarket>>()
            .await?;
        Ok(resp
            .into_iter()
            .filter(|m| m.question.to_lowercase().contains(&query.to_lowercase()))
            .collect())
    }
}
