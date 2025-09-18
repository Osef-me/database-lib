use super::query::{find_by_id, find_by_osu_id, insert};
use super::BeatmapRow;
use sqlx::{Error as SqlxError, PgPool};

impl BeatmapRow {
    pub async fn insert(self, pool: &PgPool) -> Result<i32, SqlxError> {
        insert(pool, self).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Self, SqlxError> {
        find_by_id(pool, id).await?.ok_or(SqlxError::RowNotFound)
    }

    pub async fn find_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<Option<Self>, SqlxError> {
        find_by_osu_id(pool, osu_id).await
    }
}
