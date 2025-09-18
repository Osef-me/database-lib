use crate::models::rating::score_rating::types::ScoreRatingRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<ScoreRatingRow>, SqlxError> {
    sqlx::query_as!(
        ScoreRatingRow,
        r#"
        SELECT id, score_id, rating, rating_type, created_at
        FROM score_rating
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

