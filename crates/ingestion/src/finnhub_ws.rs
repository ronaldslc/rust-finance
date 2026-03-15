use anyhow::Result;
use common::events::BotEvent;
use tokio::sync::mpsc;
use futures_util::{StreamExt, stream::SplitStream};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;
use url::Url;
use tracing::{info, error};

#[derive(serde::Deserialize)]
struct FinnhubTradeMsg {
    r#type: String,
    data: Option<Vec<Trade>>,
}

#[derive(serde::Deserialize)]
struct Trade {
    s: String,
    p: f64,
    v: Option<f64>,
    t: i64,
}

pub struct FinnhubWs {
    api_key: String,
    symbols: Vec<String>,
}

impl FinnhubWs {
    pub fn new(api_key: String, symbols: Vec<String>) -> Self {
        Self { api_key, symbols }
    }

    pub async fn run(&self, tx: mpsc::UnboundedSender<BotEvent>) -> Result<()> {
        let url_str = format!("wss://ws.finnhub.io?token={}", self.api_key);
        let url = Url::parse(&url_str)?;

        info!("Connecting to Finnhub WebSocket...");
        let (mut ws_stream, _) = connect_async(url).await?;
        info!("Finnhub connected.");

        for symbol in &self.symbols {
            let msg = format!(r#"{{"type":"subscribe","symbol":"{}"}}"#, symbol);
            use futures_util::SinkExt;
            ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await?;
        }

        let (_, mut read) = ws_stream.split();

        while let Some(msg) = read.next().await {
            match msg {
                Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                    // Send directly to normalizer or parse it
                    // Mock normalizer action here for brevity
                    if let Ok(msg) = serde_json::from_str::<FinnhubTradeMsg>(&text) {
                        if msg.r#type == "trade" {
                            for t in msg.data.unwrap_or_default() {
                                let ev = BotEvent::MarketEvent {
                                    symbol: t.s,
                                    price: t.p,
                                    timestamp: t.t,
                                    event_type: "trade".to_string(),
                                    volume: t.v,
                                };
                                let _ = tx.send(ev);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Finnhub WebSocket error: {:?}", e);
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
