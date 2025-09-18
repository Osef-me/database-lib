use crate::models::beatmap::beatmapset::types::BeatmapsetRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<BeatmapsetRow>, SqlxError> {
    sqlx::query_as!(
        BeatmapsetRow,
        r#"
        SELECT id, osu_id, artist, artist_unicode, title, title_unicode, creator, source, tags, has_video, has_storyboard, is_explicit, is_featured, cover_url, preview_url, osu_file_url, created_at, updated_at
        FROM beatmapset
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_by_osu_id(
    pool: &PgPool,
    osu_id: i32,
) -> Result<Option<BeatmapsetRow>, SqlxError> {
    sqlx::query_as!(
        BeatmapsetRow,
        r#"
        SELECT id, osu_id, artist, artist_unicode, title, title_unicode, creator, source, tags, has_video, has_storyboard, is_explicit, is_featured, cover_url, preview_url, osu_file_url, created_at, updated_at
        FROM beatmapset
        WHERE osu_id = $1
        "#,
        osu_id
    )
    .fetch_optional(pool)
    .await
}
