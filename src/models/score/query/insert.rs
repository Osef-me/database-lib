use super::super::types::ScoreRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(pool: &PgPool, score: ScoreRow) -> Result<ScoreRow, SqlxError> {
    let score = sqlx::query_as!(
        ScoreRow,
        r#"
        INSERT INTO score (
            user_id, beatmap_id, score_metadata_id, replay_id, rate, 
            hwid, mods, hash, rank, status, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW())
        RETURNING id, user_id, beatmap_id, score_metadata_id, replay_id, rate, 
                  hwid, mods, hash, rank, status, created_at
        "#,
        score.user_id,
        score.beatmap_id,
        score.score_metadata_id,
        score.replay_id,
        score.rate,
        score.hwid,
        score.mods,
        score.hash,
        score.rank,
        score.status
    )
    .fetch_one(pool)
    .await?;

    Ok(score)
}
