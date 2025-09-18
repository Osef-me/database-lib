use crate::models::score::replay::types::ReplayRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<ReplayRow>, SqlxError> {
    sqlx::query_as!(
        ReplayRow,
        r#"
        SELECT id, replay_hash, replay_available, replay_path, created_at
        FROM replay
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
