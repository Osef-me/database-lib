use crate::models::score_metadata::types::ScoreMetadataRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(pool: &PgPool, score_metadata: ScoreMetadataRow) -> Result<i32, SqlxError> {
    Ok(sqlx::query!(
        r#"
        INSERT INTO score_metadata (
            skin, pause_count, started_at, ended_at, time_paused, score, accuracy, 
            max_combo, perfect, count_300, count_100, count_50, 
            count_miss, count_katu, count_geki, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, NOW())
        RETURNING id
        "#,
        score_metadata.skin,
        score_metadata.pause_count,
        score_metadata.started_at,
        score_metadata.ended_at,
        score_metadata.time_paused,
        score_metadata.score,
        score_metadata.accuracy,
        score_metadata.max_combo,
        score_metadata.perfect,
        score_metadata.count_300,
        score_metadata.count_100,
        score_metadata.count_50,
        score_metadata.count_miss,
        score_metadata.count_katu,
        score_metadata.count_geki
    )
    .fetch_one(pool)
    .await?
    .id)
}
