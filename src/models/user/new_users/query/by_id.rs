use crate::models::user::new_users::types::NewUsersRow;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

pub async fn find_by_discord_id(
    pool: &PgPool,
    discord_id: i64,
) -> Result<Option<NewUsersRow>, SqlxError> {
    sqlx::query_as!(
        NewUsersRow,
        r#"
        SELECT discord_id, username, token, created_at
        FROM new_users
        WHERE discord_id = $1
        "#,
        discord_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_by_token(pool: &PgPool, token: Uuid) -> Result<Option<NewUsersRow>, SqlxError> {
    sqlx::query_as!(
        NewUsersRow,
        r#"
        SELECT discord_id, username, token, created_at
        FROM new_users
        WHERE token = $1
        "#,
        token
    )
    .fetch_optional(pool)
    .await
}
