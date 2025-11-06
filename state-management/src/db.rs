use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

pub async fn init_db_pool(database_url: &str) -> anyhow::Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5)) // ✅ correct method for connection timeout
        .connect(database_url)
        .await?;

    // Optional: only if you have migrations directory
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
