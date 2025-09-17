use super::super::types::ScoreRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_pending_score(pool: &PgPool) -> Result<Option<ScoreRow>, SqlxError> {
    let score = sqlx::query_as!(
        ScoreRow,
        r#"
        SELECT id, user_id, beatmap_id, score_metadata_id, replay_id, rate, 
               hwid, mods, hash, rank, status, created_at
        FROM score 
        WHERE status = 'pending' 
        ORDER BY created_at ASC 
        LIMIT 1
        "#
    )
    .fetch_optional(pool)
    .await?;

    Ok(score)
}
