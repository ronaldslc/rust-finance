#![forbid(unsafe_code)]
// crates/backtest/src/lib.rs
//
// Root module for backtesting logic.

pub mod engine;
pub mod strategy;
pub mod benchmark;
pub mod robustness;

pub use engine::{BacktestEngine, BacktestConfig, BacktestMetrics, Bar};
pub use strategy::{Strategy, StrategySignal, SimpleMovingAverageCrossover, ZScoreMeanReversion};
pub use benchmark::{
    InstitutionalMetrics, BenchmarkThresholds, BenchmarkReport, BenchmarkGrade,
    compute_institutional_metrics, validate_against_institutions, print_benchmark_report,
    walk_forward_backtest, validate_swarm_stylized_facts, SwarmValidationResult,
};
pub use robustness::{
    CostSensitivityReport, CapacityReport, ReproducibilityProof, OverfitReport,
    FullAuditReport, EngineConsistencyResult,
    cost_sensitivity_matrix, capacity_degradation, verify_reproducibility,
    probability_of_backtest_overfitting, engine_consistency_check, run_full_audit,
    print_cost_sensitivity, print_capacity_report, print_reproducibility_proof,
    print_overfit_report,
};
