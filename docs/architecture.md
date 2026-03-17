# RustForge System Architecture

RustForge Terminal is a low-latency, modular trading architecture built heavily upon asynchronous message passing channels and lock-free concurrency.

## Core Philosophical Tenets
1. **Zero-Overhead Parsing:** Data ingestion must never block. `ingestion` yields streams into the EventBus.
2. **Actor-like Concurrency:** The `daemon` routes state exclusively through channels (`mpsc`, `broadcast`).
3. **Poka-yoke Safety:** The `KillSwitch` and `RiskManager` exist physically independent of the execution loop so runaway latency cannot block risk cutoff.
4. **Resiliency over Uptime:** Alpaca WebSockets and TCP EventBus connections feature exponential backoffs (`tokio-retry`) to outlast network drops.

## Component Flow
- **Component Flow**: Ingestion -> AI & Strategy -> OMS -> Risk KillSwitch -> Execution/Relay/Polymarket.
- **Relay Router:** Routes transactions via available JSON-RPC nodes.
- **Polymarket Engine:** A custom, lightweight (`reqwest` + `ethers-core`) client connecting natively to Polygon to sign EIP-712 typed orders. By removing the official SDK, the daemon completely avoids deep `solana-sdk` ↔ `alloy` cryptography dependency conflicts.
- **Strategy & AI (Opus 4.6 Routing):** DL models act on the normalized stream. PPO Agents and Statistical Arbitrage evaluate. The daemon implements a gated probability router in `ai_pipeline.rs`.
  - Includes **Mirofish Swarm** for 5,000-agent deterministic probability analysis.
  - Includes server-side **Compaction API** blocks to handle context retention natively.
- **OMS & Compliance Layer:** The Order Management System natively tracks position flipping, unrealised PNL, VWAP, and maintains the order lifecycle. The Order Blotter routes into a simulated compliance engine for educational limit testing.
- **Risk Layer:** Advanced Kill Switch evaluating Historical 95% VaR, Maximum Drawdown Halts, and GARCH(1,1) Volatility Surges atomically prior to simulated order submission.
- **Observability Stack:** OpenTelemetry (OTLP) tracing across components with internal Prometheus metrics tracked on a Grafana Dashboard.
- **Tiered Persistence:** State is persisted into **DragonflyDB** (Redis). An **Async Worker Queue** flushes historically simulated tracks and trades into **PostgreSQL 16** and **TimescaleDB**.

### Execution Path Latency Specifications

| System Layer | Technology | Implementation Crate | Target Latency |
| :--- | :--- | :--- | :--- |
| **In-Process State** | Rust Lock-Free Concurrency | `crates/common` | `~50 ns` |
| **Shared Hot-State** | DragonflyDB via `redis` | `crates/persistence/dragonfly.rs` | `~0.2 - 0.5 ms` |
| **Historical Storage**| PostgreSQL + TimescaleDB | `crates/persistence/db.rs` | `~2 - 5 ms` |

The entire routing path (`Tick Analysis` → `AI Veto Gate` → `Order Fill`) evaluates natively below `1 ms` internally before Solana RPC propagation.

## Quantitative Analytics & Pricing (Phase 5)
RustForge integrates financial engineering formulas directly into the `pricing` and `risk` crates, built to execute rapidly for the terminal display.

### 1. Options Pricing (Black-Scholes-Merton & Heston)
The system incorporates classical BSM with a closed-form Newton-Raphson IV solver, alongside the **Heston Stochastic Volatility Model** which uses the Gil-Pelaez characteristic function inversion to capture the volatility smile/skew that BSM fails to model:

**Heston Dynamics:**
- `dS = μ·S·dt + √v·S·dW₁`
- `dv = κ·(θ - v)·dt + σ_v·√v·dW₂`
- `corr(dW₁, dW₂) = ρ·dt`

### 2. Fixed Income (Hull-White)
To capture interest rate term structures and price bond derivatives, the system implements the **Hull-White One-Factor** model wrapped around a Trinomial Tree algorithm to accurately compute American-style early exercise premiums.

### 3. Risk & Volatility Forecasting (GARCH)
Rather than solely relying on historical standard deviations, the internal Risk Manager utilizes a rolling **GARCH(1,1)** Maximum Likelihood Estimation engine to forecast variance.

**GARCH(1,1) Conditional Variance formulation:**
`σ²_t = ω + α·ε²_{t-1} + β·σ²_{t-1}`
- *Where α + β < 1 guarantees mean-reversion stationarity.*

### 5. Swarm Intelligence Engine (`swarm_sim`)
Inspired by complex sociological models, the terminal natively integrates a highly parallelized (via `rayon`) multi-agent simulator running inside its own isolated `tokio` context.
- **Microstructure Emulation**: Models the behavioral differences and latency constraints between Retail, Hedge Funds, Arbitrage, and Market Maker subsets.
- **Synthetic Scenarios**: Can inject extreme scenarios (Flash Crashes, Oil Supply Shocks, Interest Rate Hikes) to model exactly how a given market structure might collapse or absorb the liquidity vacuum.
- **Explainable Analytics**: Features an `InterviewEngine` which can cross-examine simulation agents mid-flight, converting arbitrary float weights back into categorical natural language reasoning.
