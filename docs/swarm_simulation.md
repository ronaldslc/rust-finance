# Swarm Simulation Engine (`swarm_sim`)

The `swarm_sim` crate is a high-performance, concurrent multi-agent market simulation engine built directly into the RustForge Terminal. Drawing inspiration from deep behavioral finance models, the engine replicates micro-market structures natively without external process dependencies.

## Core Capabilities

### 1. Multi-Agent Engine
A highly parallelized loop (via `rayon`) modeling thousands of market agents holding bespoke positions and diverse reaction logic schemas.
- **Trader Types Supported:** Retail, Hedge Funds, Market Makers, Arbitrage Bots, Momentum Traders, and News Traders.
- **Price Impact & Volatility:** Simulates authentic market diffusions using standard Brownian motion augmented by the net flow of order imbalance resulting from the agent action resolution phase.
- **Empirical Benchmark**: Live tests (`cargo test --release benchmark_100k_agents`) demonstrate instantiation of 100,000 agents taking **~7ms**, while parallel order evaluations resolving via lock-free atomics average just **1.91ms** per round (>520 rounds per second).

### 2. Market Scenarios
Capable of imposing severe macro shocks mid-flight using the `ScenarioEngine`. Injected events naturally decay and warp the underlying baseline sentiment of agents.
Examples include:
- `FedRateHike`, `CPI_Surprise`, `EarningsBeat/Miss`
- `FlashCrash`, `LiquidityVacuum`, `ShortSqueeze`

### 3. Explainability via the Interview Engine
At any point, the `InterviewEngine` can cross-examine individual algorithmic agents and extract the *categorical reasons* why a specific order was placed. This outputs the driving factors (e.g., "RSI oversold", "Arbitrage spread > 2bps", "Risk Limit Exceeded") to assist in post-mortem analytics.

### 4. Background Persistence
The `ActionLog` subsystem utilizes `tokio::mpsc::unbounded_channel()` to stream execution data off the main simulation thread to flat JSONL representations synchronously without blocking the hot simulation tier.

## Integration Architecture
1. **Config Initialization**: Loads fractions of trader types and general limits from the `SwarmConfig`.
2. **Tick Advance**: The `SwarmEngine::step_round()` function resolves parallel moves across all agents and aggregates net flow, producing a `SwarmSignal`.
3. **Signal Export**: The `SwarmSignal` natively contains `direction`, `conviction`, and `regime` metadata ready for broadcast directly onto the `event_bus` for broader strategic listening.
