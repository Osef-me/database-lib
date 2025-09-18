use super::query::{find_by_id, insert};
use super::BeatmapRatingRow;
use sqlx::{Error as SqlxError, PgPool};

impl BeatmapRatingRow {
    pub async fn insert(self, pool: &PgPool) -> Result<i32, SqlxError> {
        insert(pool, self).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Self, SqlxError> {
        find_by_id(pool, id).await?.ok_or(SqlxError::RowNotFound)
    }
}

