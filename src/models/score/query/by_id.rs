use super::super::types::Score;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Score>, SqlxError> {
    let score = sqlx::query_as!(
        Score,
        r#"
        SELECT id, user_id, beatmap_id, score_metadata_id, replay_id, rate, 
               hwid, mods, hash, rank, status, created_at
        FROM score 
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(score)
}
