use super::super::types::Score;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Vec<Score>, SqlxError> {
    let scores = sqlx::query_as!(
        Score,
        r#"
        SELECT id, user_id, beatmap_id, score_metadata_id, replay_id, rate, 
               hwid, mods, hash, rank, status, created_at
        FROM score 
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(scores)
}
