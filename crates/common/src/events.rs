use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum BotEvent {
    PositionUpdate {
        token: String,
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
        reason: String,
    },

    LatencyUpdate {
        rpc: f64,
        decision: f64,
        sign: f64,
        send: f64,
    },

    Feed(String),

    MarketEvent {
        symbol: String,
        price: f64,
        timestamp: i64,
        event_type: String, // "trade", "quote", etc.
        volume: Option<f64>,
    },

    AISignal {
        symbol: String,
        action: String,
        confidence: f64,
        reason: String,
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
