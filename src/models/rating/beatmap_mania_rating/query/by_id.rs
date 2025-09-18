use crate::models::rating::beatmap_mania_rating::types::BeatmapManiaRatingRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<BeatmapManiaRatingRow>, SqlxError> {
    sqlx::query_as!(
        BeatmapManiaRatingRow,
        r#"
        SELECT id, rating_id, stream, jumpstream, handstream, stamina, jackspeed, chordjack, technical, created_at, updated_at
        FROM beatmap_mania_rating
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
