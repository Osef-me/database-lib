use crate::models::pending_beatmap::PendingBeatmapRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn oldest(pool: &PgPool) -> Result<Option<PendingBeatmapRow>, SqlxError> {
    let row = sqlx::query_as!(
        PendingBeatmapRow,
        r#"
        SELECT id, hash, osu_id, created_at
        FROM pending_beatmap
        ORDER BY created_at ASC, id ASC
        LIMIT 1
        "#
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
