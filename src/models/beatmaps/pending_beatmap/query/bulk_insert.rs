use crate::models::beatmaps::pending_beatmap::types::PendingBeatmapRow;
use sqlx::{Error as SqlxError, PgPool};
use validator::Validate;

pub async fn bulk_insert(pool: &PgPool, rows: &[PendingBeatmapRow]) -> Result<i64, SqlxError> {
    let filtered: Vec<&PendingBeatmapRow> = rows.iter().filter(|n| n.validate().is_ok()).collect();

    if filtered.is_empty() {
        return Ok(0);
    }
    let hashes: Vec<&str> = filtered.iter().map(|r| r.osu_hash.as_str()).collect();
    let res = sqlx::query(
        "INSERT INTO pending_beatmap (osu_hash)
         SELECT DISTINCT UNNEST($1::text[])
         ON CONFLICT (osu_hash) DO NOTHING",
    )
    .bind(&hashes)
    .execute(pool)
    .await?;
    Ok(res.rows_affected() as i64)
}
