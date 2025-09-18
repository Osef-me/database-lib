use crate::models::beatmapset::types::BeatmapsetRow;
use sqlx::QueryBuilder;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(pool: &PgPool, beatmapset: BeatmapsetRow) -> Result<i32, SqlxError> {
    let mut builder = QueryBuilder::<sqlx::Postgres>::new(
        "INSERT INTO beatmapset (osu_id, artist, artist_unicode, title, title_unicode, creator, source, tags, has_video, has_storyboard, is_explicit, is_featured, cover_url, preview_url, osu_file_url) VALUES ("
    );

    let mut separated = builder.separated(", ");
    separated.push_bind(beatmapset.osu_id);
    separated.push_bind(beatmapset.artist);
    separated.push_bind(beatmapset.artist_unicode);
    separated.push_bind(beatmapset.title);
    separated.push_bind(beatmapset.title_unicode);
    separated.push_bind(beatmapset.creator);
    separated.push_bind(beatmapset.source);
    separated.push_bind(beatmapset.tags);
    separated.push_bind(beatmapset.has_video);
    separated.push_bind(beatmapset.has_storyboard);
    separated.push_bind(beatmapset.is_explicit);
    separated.push_bind(beatmapset.is_featured);
    separated.push_bind(beatmapset.cover_url);
    separated.push_bind(beatmapset.preview_url);
    separated.push_bind(beatmapset.osu_file_url);
    // separated will go out of scope here
    builder.push(") ON CONFLICT (osu_id) DO UPDATE SET ");
    builder.push(
        "artist = EXCLUDED.artist, artist_unicode = EXCLUDED.artist_unicode, title = EXCLUDED.title, title_unicode = EXCLUDED.title_unicode, creator = EXCLUDED.creator, source = EXCLUDED.source, tags = EXCLUDED.tags, has_video = EXCLUDED.has_video, has_storyboard = EXCLUDED.has_storyboard, is_explicit = EXCLUDED.is_explicit, is_featured = EXCLUDED.is_featured, cover_url = EXCLUDED.cover_url, preview_url = EXCLUDED.preview_url, osu_file_url = EXCLUDED.osu_file_url, updated_at = NOW() RETURNING id",
    );

    let rec = builder.build().fetch_one(pool).await?;
    use sqlx::Row;
    rec.try_get("id")
}
