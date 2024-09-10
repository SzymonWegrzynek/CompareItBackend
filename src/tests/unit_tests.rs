#[cfg(test)]
mod tests {
    use crate::database::conn::DatabaseConn;
    use std::env;
    use dotenv::dotenv;
    use sqlx::query;

    #[tokio::test]
    async fn unit_test_create_pool() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL");
        assert!(db_url.is_ok(), "no DATABASE_URL in .env");

        let db_conn = DatabaseConn::create_pool().await;
        let result = query("SELECT 1").fetch_one(db_conn.pool()).await;
        assert!(result.is_ok(), "failed to create conn poll")
    }
}
