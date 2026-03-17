use serde::{Serialize, Deserialize};
use compact_str::CompactString;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum BotEvent {
    PositionUpdate {
        token: CompactString,
        size: f64,
        entry: f64,
        price: f64,
    },

    WalletUpdate {
        sol_balance: f64,
        exposure: f64,
    },

    StrategyUpdate {
        buy: f64,
        sell: f64,
        hold: f64,
        confidence: f64,
        reason: String, // Reason stays String as it can be long
    },

    LatencyUpdate {
        rpc: f64,
        decision: f64,
        sign: f64,
        send: f64,
    },

    Feed(String), // Feed can be long

    MarketEvent {
        symbol: CompactString,
        price: f64,
        timestamp: i64,
        event_type: CompactString, // "trade", "quote", etc.
        volume: Option<f64>,
    },

    AISignal {
        symbol: CompactString,
        action: CompactString,
        confidence: f64,
        reason: String, // Reason stays String
    },

    QuoteEvent {
        symbol: CompactString,
        bid_price: f64,
        bid_size: u64,
        ask_price: f64,
        ask_size: u64,
        timestamp: i64,
    },

    ExchangeHeartbeat {
        exchange: CompactString,
        status: CompactString,
        latency_ms: f64,
    },

    // Polymarket events
    PolymarketBookUpdate {
        asset_id: String,
        bids: Vec<(String, String)>,
        asks: Vec<(String, String)>,
    },
    PolymarketPriceChange {
        asset_id: String,
        price: String,
        side: String,
    },
    PolymarketTradeSignal {
        token_id: String,
        side: String,  // "BUY" or "SELL"
        price: String,
        size: String,
        source: String, // "copy_trade", "swarm_sim", "ai_signal"
    },
    PolymarketOrderFilled {
        order_id: String,
        token_id: String,
        fill_price: String,
        fill_size: String,
    },
    PolymarketMarketResolved {
        market: String,
        winning_outcome: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "command")]
pub enum ControlCommand {
    Pause,
    Resume,
    KillSwitch,
    ToggleLive,
    RestartIngestion,
    SwitchStrategy,
    ClosePosition,
    AdjustRisk { delta: f64 },
}
