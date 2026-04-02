// ============================================================
// crates/ai/src/dexter.rs
//
// Dexter — Financial Analyst AI
// Produces structured trade signals from the FusedContext.
// Uses Claude claude-opus-4-6 with JSON response mode for reliable parsing.
// ============================================================

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::info;

// use crate::hybrid_pipeline::FusedContext; // FusedContext lives in daemon module
// We will access FusedContext directly during compilation if possible, 
// or define DexterSignal here.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DexterSignal {
    pub symbol: String,
    pub direction: TradeDirection,
    pub confidence: f64,          // 0.0–1.0
    pub entry_price: f64,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub position_size_pct: f64,   // % of portfolio to allocate
    pub time_horizon: TimeHorizon,
    pub thesis: String,           // 2-3 sentences, specific and quantitative
    pub key_risks: Vec<String>,   // Max 3 concrete risk factors
    pub catalyst: Option<String>, // Specific event that could move price
    pub valuation: Option<ValuationMetrics>,
    pub recommendation: Recommendation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TradeDirection {
    Long,
    Short,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeHorizon {
    Intraday,
    Swing,    // 2-5 days
    Position, // weeks
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Recommendation {
    Buy,
    Sell,
    Hold,
    Risk,     // Hedge/reduce risk
}

/// Matches what you see in the TUI Dexter panel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationMetrics {
    pub pe_ratio: Option<f64>,
    pub ps_ratio: Option<f64>,
    pub ev_ebitda: Option<f64>,
    pub dcf_range_low: Option<f64>,
    pub dcf_range_high: Option<f64>,
    pub revenue_impact_usd_millions: Option<f64>,
    pub margin_change_pct: Option<f64>,
}

// ── The analyst function ─────────────────────────────────────────────────────

// Using a generic bound for context to avoid circular dependency with daemon
pub trait FusedContextLike {
    fn to_dexter_system_prompt(&self) -> String;
    fn get_symbol(&self) -> String;
}

#[allow(unused_imports)]
pub async fn analyse<T: FusedContextLike>(ctx: &T) -> Result<DexterSignal> {
    let system_prompt = ctx.to_dexter_system_prompt();
    let user_prompt = build_user_prompt(&ctx.get_symbol());

    let raw = call_claude_json(&system_prompt, &user_prompt).await?;

    // Heal common JSON issues before parsing
    let healed = heal_json(&raw);
    let signal: DexterSignal = serde_json::from_str(&healed)
        .with_context(|| format!("Dexter JSON parse failed: {}", &healed[..200.min(healed.len())]))?;

    info!(
        "Dexter → {} {:?} conf={:.2} thesis={}",
        signal.symbol,
        signal.direction,
        signal.confidence,
        &signal.thesis[..signal.thesis.len().min(80)]
    );

    Ok(signal)
}

fn build_user_prompt(symbol: &str) -> String {
    format!(
        r#"Analyse {symbol} using the quantitative data, swarm simulation, and knowledge graph above.

Return ONLY a JSON object matching this schema — no preamble, no markdown:

{{
  "symbol": "{symbol}",
  "direction": "Long" | "Short" | "Neutral",
  "confidence": 0.0-1.0,
  "entry_price": float,
  "stop_loss": float,
  "take_profit": float,
  "position_size_pct": 0.0-0.10,
  "time_horizon": "Intraday" | "Swing" | "Position",
  "thesis": "2-3 specific sentences with numbers from the data above",
  "key_risks": ["risk1", "risk2", "risk3"],
  "catalyst": "specific event or null",
  "valuation": {{
    "pe_ratio": float | null,
    "ps_ratio": float | null,
    "ev_ebitda": float | null,
    "dcf_range_low": float | null,
    "dcf_range_high": float | null,
    "revenue_impact_usd_millions": float | null,
    "margin_change_pct": float | null
  }},
  "recommendation": "Buy" | "Sell" | "Hold" | "Risk"
}}

Rules:
- Use ONLY data from the context above — no hallucinated numbers
- Thesis must cite specific figures (RSI level, swarm %, graph relationships)
- stop_loss and take_profit must be realistic given current GARCH volatility
- position_size_pct must be ≤ 0.05 if confidence < 0.70
"#,
        symbol = symbol
    )
}

async fn call_claude_json(system: &str, user: &str) -> Result<String> {
    let api_key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_else(|_| "mock_key".to_string());
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "model": "claude-opus-4-6-20251101",
        "max_tokens": 1024,
        "system": system,
        "messages": [{"role": "user", "content": user}]
    });

    let resp = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await?;

    let json: serde_json::Value = resp.json().await?;
    if let Some(content) = json.get("content").and_then(|c| c.as_array()) {
        if let Some(first) = content.first() {
            if let Some(text) = first.get("text").and_then(|t| t.as_str()) {
                return Ok(text.to_string());
            }
        }
    }
    
    // Return empty json object fallback
    Ok("{}".to_string())
}

/// Heal common LLM JSON output issues before parsing
fn heal_json(raw: &str) -> String {
    let trimmed = raw.trim();

    // Strip markdown code fences
    let stripped = if let Some(start) = trimmed.find("```json") {
        let after = &trimmed[start + 7..];
        if let Some(end) = after.rfind("```") {
            after[..end].trim().to_string()
        } else {
            after.trim().to_string()
        }
    } else if trimmed.starts_with("```") {
        let after = &trimmed[3..];
        if let Some(end) = after.rfind("```") {
            after[..end].trim().to_string()
        } else {
            after.trim().to_string()
        }
    } else {
        // Extract bare JSON object
        if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}')) {
            trimmed[start..=end].to_string()
        } else {
            trimmed.to_string()
        }
    };

    // Replace JavaScript null with JSON null (sometimes models write 'null' unquoted)
    stripped
}
