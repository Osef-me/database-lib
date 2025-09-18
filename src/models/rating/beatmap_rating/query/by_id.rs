use crate::models::rating::beatmap_rating::types::BeatmapRatingRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<BeatmapRatingRow>, SqlxError> {
    sqlx::query_as!(
        BeatmapRatingRow,
        r#"
        SELECT id, rates_id, rating, rating_type, created_at
        FROM beatmap_rating
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

