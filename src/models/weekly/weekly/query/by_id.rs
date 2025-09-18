use crate::models::weekly::weekly::types::WeeklyRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<WeeklyRow>, SqlxError> {
    sqlx::query_as!(
        WeeklyRow,
        r#"
        SELECT id, name, description, start_date, end_date, is_active, created_at
        FROM weekly
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

