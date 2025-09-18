use crate::models::user::bans::types::BansRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<BansRow>, SqlxError> {
    sqlx::query_as!(
        BansRow,
        r#"
        SELECT id, discord_id, reason, banned_at
        FROM bans
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
