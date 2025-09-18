use crate::models::beatmap::beatmap::types::BeatmapRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<BeatmapRow>, SqlxError> {
    sqlx::query_as!(
        BeatmapRow,
        r#"
        SELECT id, osu_id, beatmapset_id, difficulty, count_circles, count_sliders, count_spinners, max_combo, main_pattern, cs, ar, od, hp, mode, status, created_at, updated_at
        FROM beatmap
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<Option<BeatmapRow>, SqlxError> {
    sqlx::query_as!(
        BeatmapRow,
        r#"
        SELECT id, osu_id, beatmapset_id, difficulty, count_circles, count_sliders, count_spinners, max_combo, main_pattern, cs, ar, od, hp, mode, status, created_at, updated_at
        FROM beatmap
        WHERE osu_id = $1
        "#,
        osu_id
    )
    .fetch_optional(pool)
    .await
}
