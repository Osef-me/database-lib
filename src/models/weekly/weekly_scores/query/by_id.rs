use crate::models::weekly::weekly_scores::types::WeeklyScoresRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<WeeklyScoresRow>, SqlxError> {
    sqlx::query_as!(
        WeeklyScoresRow,
        r#"
        SELECT id, weekly_id, score_id, created_at
        FROM weekly_scores
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

