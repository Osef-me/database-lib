use super::query::{find_by_id, find_by_hash, insert};
use super::PendingBeatmapRow;
use sqlx::{Error as SqlxError, PgPool};

impl PendingBeatmapRow {
    pub async fn insert(self, pool: &PgPool) -> Result<i32, SqlxError> {
        insert(pool, self).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Self, SqlxError> {
        find_by_id(pool, id).await?.ok_or(SqlxError::RowNotFound)
    }

    pub async fn find_by_hash(pool: &PgPool, osu_hash: &str) -> Result<Option<Self>, SqlxError> {
        find_by_hash(pool, osu_hash).await
    }
}

