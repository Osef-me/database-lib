use crate::models::beatmap::pending_beatmap::types::PendingBeatmapRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<PendingBeatmapRow>, SqlxError> {
    sqlx::query_as!(
        PendingBeatmapRow,
        r#"
        SELECT id, osu_hash, osu_id, created_at
        FROM pending_beatmap
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_by_hash(
    pool: &PgPool,
    osu_hash: &str,
) -> Result<Option<PendingBeatmapRow>, SqlxError> {
    sqlx::query_as!(
        PendingBeatmapRow,
        r#"
        SELECT id, osu_hash, osu_id, created_at
        FROM pending_beatmap
        WHERE osu_hash = $1
        "#,
        osu_hash
    )
    .fetch_optional(pool)
    .await
}
