use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;


pub async fn create_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("no DATABASE_URL in .env");

    let pool: PgPool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&db_url)
    .await
    .unwrap();

    pool
}
