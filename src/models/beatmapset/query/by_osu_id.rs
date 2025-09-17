use crate::models::beatmapset::BeatmapsetRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_osu_id(
    pool: &PgPool,
    osu_id: i32,
) -> Result<Option<BeatmapsetRow>, SqlxError> {
    sqlx::query_as!(
        BeatmapsetRow,
        "SELECT * FROM beatmapset WHERE osu_id = $1",
        osu_id
    )
    .fetch_optional(pool)
    .await
}
