use crate::models::weekly::weekly_maps::types::WeeklyMapsRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<WeeklyMapsRow>, SqlxError> {
    sqlx::query_as!(
        WeeklyMapsRow,
        r#"
        SELECT id, weekly_pool_id, beatmap_id, created_at
        FROM weekly_maps
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

