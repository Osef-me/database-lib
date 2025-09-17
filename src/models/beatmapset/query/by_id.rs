use crate::models::beatmapset::BeatmapsetRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<BeatmapsetRow>, SqlxError> {
    sqlx::query_as!(BeatmapsetRow, "SELECT * FROM beatmapset WHERE id = $1", id)
        .fetch_optional(pool)
        .await
}
