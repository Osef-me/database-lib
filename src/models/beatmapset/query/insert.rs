use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(
    pool: &PgPool,
    osu_id: Option<i32>,
    artist: String,
    artist_unicode: Option<String>,
    title: String,
    title_unicode: Option<String>,
    creator: String,
    source: Option<String>,
    tags: Option<Vec<String>>,
    has_video: bool,
    has_storyboard: bool,
    is_explicit: bool,
    is_featured: bool,
    cover_url: Option<String>,
    preview_url: Option<String>,
    osu_file_url: Option<String>,
) -> Result<i32, SqlxError> {
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
        osu_id,
        artist,
        artist_unicode.as_deref(),
        title,
        title_unicode.as_deref(),
        creator,
        source.as_deref(),
        tags.as_deref(),
        has_video,
        has_storyboard,
        is_explicit,
        is_featured,
        cover_url.as_deref(),
        preview_url.as_deref(),
        osu_file_url.as_deref()
    )
    .fetch_one(pool)
    .await?
    .id)
}
