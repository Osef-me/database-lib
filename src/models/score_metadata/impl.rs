use super::query::{find_by_id, insert};
use super::types::ScoreMetadataRow;
use sqlx::PgPool;

impl ScoreMetadataRow {
    pub async fn insert(self, pool: &PgPool) -> Result<i32, sqlx::Error> {
        insert(pool, self).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }
}
