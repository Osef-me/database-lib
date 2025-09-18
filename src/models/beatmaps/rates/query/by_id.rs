use crate::models::beatmap::rates::types::RatesRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<RatesRow>, SqlxError> {
    sqlx::query_as!(
        RatesRow,
        r#"
        SELECT id, beatmap_id, osu_hash, centirate, drain_time, total_time, bpm, created_at
        FROM rates
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
