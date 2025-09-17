use super::super::types::ScoreRatingRow;
use bigdecimal::BigDecimal;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(
    pool: &PgPool,
    score_id: i32,
    rating: BigDecimal,
    rating_type: &str,
) -> Result<ScoreRatingRow, SqlxError> {
    let score_rating = sqlx::query_as!(
        ScoreRatingRow,
        r#"
        INSERT INTO score_rating (score_id, rating, rating_type, created_at)
        VALUES ($1, $2, $3, NOW())
        RETURNING id, score_id, rating, rating_type, created_at
        "#,
        score_id,
        rating,
        rating_type
    )
    .fetch_one(pool)
    .await?;

    Ok(score_rating)
}
