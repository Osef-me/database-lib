use crate::models::users::device_tokens::types::DeviceTokensRow;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

pub async fn find_by_token(pool: &PgPool, token: Uuid) -> Result<Option<DeviceTokensRow>, SqlxError> {
    sqlx::query_as!(
        DeviceTokensRow,
        r#"
        SELECT token, discord_id, device_name, hwid, created_at
        FROM device_tokens
        WHERE token = $1
        "#,
        token
    )
    .fetch_optional(pool)
    .await
}

