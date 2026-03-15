use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

pub async fn connect_db(db_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(db_url)
        .await?;
        
    Ok(pool)
}
