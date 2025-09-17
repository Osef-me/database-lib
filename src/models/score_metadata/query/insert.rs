use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(
    pool: &PgPool,
    skin: Option<&str>,
    pause_count: i32,
    started_at: NaiveDateTime,
    ended_at: NaiveDateTime,
    time_paused: i32,
    score: i32,
    accuracy: BigDecimal,
    max_combo: i32,
    perfect: bool,
    count_300: i32,
    count_100: i32,
    count_50: i32,
    count_miss: i32,
    count_katu: i32,
    count_geki: i32,
) -> Result<i32, SqlxError> {
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
        skin,
        pause_count,
        started_at,
        ended_at,
        time_paused,
        score,
        accuracy,
        max_combo,
        perfect,
        count_300,
        count_100,
        count_50,
        count_miss,
        count_katu,
        count_geki
    )
    .fetch_one(pool)
    .await?
    .id)
}
