use crate::models::beatmaps::pending_beatmap::PendingBeatmapRow;
use sqlx::PgPool;

pub async fn last_pending_beatmap(pool: &PgPool) -> Result<Option<PendingBeatmapRow>, sqlx::Error> {
    let result = sqlx::query_as!(
        PendingBeatmapRow,
        r#"
        SELECT id, osu_hash, osu_id, created_at
        FROM pending_beatmap
        ORDER BY created_at ASC, id ASC
        LIMIT 1
        "#
    )
    .fetch_optional(pool)
    .await?;

    Ok(result)
}
