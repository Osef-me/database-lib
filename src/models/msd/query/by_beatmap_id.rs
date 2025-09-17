use crate::models::msd::types::MSDRow;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_beatmap_id(
    pool: &PgPool,
    beatmap_id: i32,
) -> Result<Option<MSDRow>, SqlxError> {
    sqlx::query_as!(
        MSDRow,
        r#"
        SELECT * FROM msd WHERE beatmap_id = $1 ORDER BY created_at DESC LIMIT 1
        "#,
        beatmap_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_by_beatmap_id_and_rate(
    pool: &PgPool,
    beatmap_id: i32,
    rate: f64,
) -> Result<Option<MSDRow>, SqlxError> {
    sqlx::query_as!(
        MSDRow,
        r#"SELECT * FROM msd WHERE beatmap_id = $1 AND rate = $2"#,
        beatmap_id,
        BigDecimal::from_f64(rate).unwrap()
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_all_by_beatmap_id(
    pool: &PgPool,
    beatmap_id: i32,
) -> Result<Vec<MSDRow>, SqlxError> {
    sqlx::query_as!(
        MSDRow,
        r#"SELECT * FROM msd WHERE beatmap_id = $1 ORDER BY created_at DESC"#,
        beatmap_id
    )
    .fetch_all(pool)
    .await
}
