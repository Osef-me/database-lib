use super::super::types::Score;
use bigdecimal::BigDecimal;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(
    pool: &PgPool,
    user_id: i64,
    beatmap_id: i32,
    score_metadata_id: i32,
    replay_id: Option<i32>,
    rate: BigDecimal,
    hwid: Option<&str>,
    mods: i64,
    hash: &str,
    rank: &str,
    status: &str,
) -> Result<Score, SqlxError> {
    let score = sqlx::query_as!(
        Score,
        r#"
        INSERT INTO score (
            user_id, beatmap_id, score_metadata_id, replay_id, rate, 
            hwid, mods, hash, rank, status, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW())
        RETURNING id, user_id, beatmap_id, score_metadata_id, replay_id, rate, 
                  hwid, mods, hash, rank, status, created_at
        "#,
        user_id,
        beatmap_id,
        score_metadata_id,
        replay_id,
        rate,
        hwid,
        mods,
        hash,
        rank,
        status
    )
    .fetch_one(pool)
    .await?;

    Ok(score)
}
