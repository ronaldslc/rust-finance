use tokio::sync::mpsc;
use sqlx::PgPool;
use tracing::{info, error};

pub struct DbEvent {
    pub query: String,
}

pub async fn start_worker(pool: PgPool) -> mpsc::Sender<DbEvent> {
    let (tx, mut rx) = mpsc::channel::<DbEvent>(100000);

    tokio::spawn(async move {
        info!("Async Persistence Worker (TimescaleDB) started.");
        while let Some(event) = rx.recv().await {
            // Decoupled disk writing prevents execution lock-ups
            if let Err(e) = sqlx::query(&event.query).execute(&pool).await {
                error!("Async DbWorker Error: {:?}", e);
            }
        }
    });

    tx
}
