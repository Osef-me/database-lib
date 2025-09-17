use crate::models::beatmapset::types::BeatmapsetRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(pool: &PgPool, beatmapset: BeatmapsetRow) -> Result<i32, SqlxError> {
    Ok(sqlx::query!(
        r#"
            INSERT INTO beatmapset (
                osu_id, artist, artist_unicode, title, title_unicode, creator, source,
                tags, has_video, has_storyboard, is_explicit, is_featured,
                cover_url, preview_url, osu_file_url
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
            ON CONFLICT (osu_id) DO UPDATE SET
                artist = EXCLUDED.artist,
                artist_unicode = EXCLUDED.artist_unicode,
                title = EXCLUDED.title,
                title_unicode = EXCLUDED.title_unicode,
                creator = EXCLUDED.creator,
                source = EXCLUDED.source,
                tags = EXCLUDED.tags,
                has_video = EXCLUDED.has_video,
                has_storyboard = EXCLUDED.has_storyboard,
                is_explicit = EXCLUDED.is_explicit,
                is_featured = EXCLUDED.is_featured,
                cover_url = EXCLUDED.cover_url,
                preview_url = EXCLUDED.preview_url,
                osu_file_url = EXCLUDED.osu_file_url,
                updated_at = now()
            RETURNING id
            "#,
        beatmapset.osu_id,
        beatmapset.artist,
        beatmapset.artist_unicode.as_deref(),
        beatmapset.title,
        beatmapset.title_unicode.as_deref(),
        beatmapset.creator,
        beatmapset.source.as_deref(),
        beatmapset.tags.as_deref(),
        beatmapset.has_video,
        beatmapset.has_storyboard,
        beatmapset.is_explicit,
        beatmapset.is_featured,
        beatmapset.cover_url.as_deref(),
        beatmapset.preview_url.as_deref(),
        beatmapset.osu_file_url.as_deref()
    )
    .fetch_one(pool)
    .await?
    .id)
}
