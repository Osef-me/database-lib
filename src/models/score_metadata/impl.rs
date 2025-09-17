use super::query::{find_by_id, insert};
use super::types::ScoreMetadataRow;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use sqlx::PgPool;

impl ScoreMetadataRow {
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
    ) -> Result<i32, sqlx::Error> {
        insert(
            pool,
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
            count_geki,
        )
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }
}
