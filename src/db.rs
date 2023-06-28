use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


pub async fn create_connection_pool() -> Pool<Postgres> {
  let db_url = std::env::var("DATABASE_URL").expect("Failed to get database URL");
  let max_conn_limit = std::env::var("DB_CONN_LIMIT").unwrap_or(1.to_string()).parse().expect("Failed to parse CONN_LIMIT");
  let pool = PgPoolOptions::new()
    .max_connections(max_conn_limit)
    .connect(&db_url)
    .await.expect("Failed to create DB Pool");
  pool
}
