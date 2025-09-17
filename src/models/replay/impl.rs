use super::query::insert::insert;
use super::types::ReplayRow;
use sqlx::PgPool;

impl ReplayRow {
    pub async fn insert(pool: &PgPool, hash: &str, replay_path: &str) -> Result<i32, sqlx::Error> {
        insert(pool, hash, replay_path).await
    }
}
