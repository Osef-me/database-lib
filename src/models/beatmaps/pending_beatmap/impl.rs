use super::query::{delete_by_id, find_by_hash, find_by_id, insert, last_pending_beatmap};
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

    pub async fn last_pending_beatmap(pool: &PgPool) -> Result<Option<Self>, SqlxError> {
        last_pending_beatmap(pool).await
    }

    pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<bool, SqlxError> {
        delete_by_id(pool, id).await
    }
}
