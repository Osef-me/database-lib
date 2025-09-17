use super::super::types::ScoreRating;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<ScoreRating>, SqlxError> {
    let score_rating = sqlx::query_as!(
        ScoreRating,
        r#"
        SELECT id, score_id, rating, rating_type, created_at
        FROM score_rating 
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(score_rating)
}
