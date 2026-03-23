#![forbid(unsafe_code)]
// ============================================================
// swarm_sim — Financial Market Swarm Simulation Engine
// Part of RustForge Terminal (rust-finance)
//
// Architecture:
//   AgentPool  ──step()──►  ActionLog  ──aggregate()──►  SwarmSignal
//       ▲                                                      │
//   MarketState ◄──────── price_impact() ◄────────────────────┘
// ============================================================

pub mod agent;
pub mod engine;
pub mod market;
pub mod scenario;
pub mod signal;
pub mod interview;
pub mod persistence;
pub mod config;
pub mod digital_twin;

pub use agent::{Agent, AgentId, TraderType, AgentState};
pub use engine::{SwarmEngine, SwarmStep};
pub use market::{MarketState, OrderBook, PriceLevel};
pub use scenario::{MarketScenario, ScenarioEngine};
pub use signal::{SwarmSignal, SignalDirection, Conviction};
pub use interview::{InterviewEngine, TradeReason};
pub use config::SwarmConfig;

pub fn default_engine(config: SwarmConfig, initial_state: MarketState) -> engine::SwarmEngine {
    engine::SwarmEngine::new(config, initial_state)
}
