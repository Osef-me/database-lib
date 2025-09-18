use crate::models::weeklys::weekly_pool::types::WeeklyPoolRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<WeeklyPoolRow>, SqlxError> {
    sqlx::query_as!(
        WeeklyPoolRow,
        r#"
        SELECT id, beatmap_id, weekly_id, vote_count, created_at
        FROM weekly_pool
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
