use crate::models::scores::score::types::ScoreRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<ScoreRow>, SqlxError> {
    sqlx::query_as!(
        ScoreRow,
        r#"
        SELECT id, user_id, rates_id, score_metadata_id, replay_id, hwid, mods, rank, status, created_at
        FROM score
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
