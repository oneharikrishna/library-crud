use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;

pub async fn get_db_pool() -> MySqlPool {
    let database_url = env::var("DATABASE_URL")
        .expect("Database Url must be set in .env file");

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create MySql Pool")
}