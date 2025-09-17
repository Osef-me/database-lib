use super::super::types::ScoreRatingRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_score_id(
    pool: &PgPool,
    score_id: i32,
) -> Result<Vec<ScoreRatingRow>, SqlxError> {
    let score_ratings = sqlx::query_as!(
        ScoreRatingRow,
        r#"
        SELECT id, score_id, rating, rating_type, created_at
        FROM score_rating 
        WHERE score_id = $1
        ORDER BY created_at DESC
        "#,
        score_id
    )
    .fetch_all(pool)
    .await?;

    Ok(score_ratings)
}
