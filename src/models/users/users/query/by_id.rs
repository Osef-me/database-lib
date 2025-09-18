use crate::models::users::users::types::UsersRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_discord_id(pool: &PgPool, discord_id: i64) -> Result<Option<UsersRow>, SqlxError> {
    sqlx::query_as!(
        UsersRow,
        r#"
        SELECT discord_id, username, created_at, roles
        FROM users
        WHERE discord_id = $1
        "#,
        discord_id
    )
    .fetch_optional(pool)
    .await
}