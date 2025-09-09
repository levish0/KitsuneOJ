use redis::aio::ConnectionManager as RedisClient;
use reqwest::Client as HttpClient;
use sea_orm::DatabaseConnection as PostgresqlClient;

#[derive(Clone)]
pub struct AppState {
    pub conn: PostgresqlClient,
    // pub cloudflare_r2: R2Client,
    pub redis: RedisClient,
    pub http_client: HttpClient,
    // pub meilisearch: MeilisearchClient,
}
