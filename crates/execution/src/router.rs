use crate::dry_run::{simulate_fill, Order};
use tracing::info;

pub async fn execute(order: Order) {
    let mode = std::env::var("EXECUTION_MODE").unwrap_or_else(|_| "dry_run".to_string());

    match mode.as_str() {
        "dry_run" | "paper_trade" => {
            let fill = simulate_fill(&order);
            info!("[DRY RUN] Simulated fill for {}: price = {}, qty = {}", order.symbol, fill.fill_price, order.qty);
        }
        "live" => {
            info!("[LIVE] Sending real order for {}: price = {}, qty = {}", order.symbol, order.price, order.qty);
        }
        _ => {
            info!("[SECURITY SAFEGUARD] Unknown EXECUTION_MODE '{}'. Intercepting order.", mode);
        }
    }
}
