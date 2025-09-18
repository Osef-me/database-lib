use crate::models::weekly::weekly_participants::types::WeeklyParticipantsRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<WeeklyParticipantsRow>, SqlxError> {
    sqlx::query_as!(
        WeeklyParticipantsRow,
        r#"
        SELECT id, weekly_id, user_id, created_at
        FROM weekly_participants
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

