use super::query::{exists_by_osu_id, find_all, find_by_id, find_by_osu_id, insert, search};
use super::types::BeatmapsetRow;
use sqlx::PgPool;

impl BeatmapsetRow {
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
    ) -> Result<i32, sqlx::Error> {
        insert(
            pool,
            osu_id,
            artist,
            artist_unicode,
            title,
            title_unicode,
            creator,
            source,
            tags,
            has_video,
            has_storyboard,
            is_explicit,
            is_featured,
            cover_url,
            preview_url,
            osu_file_url,
        )
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }

    pub async fn find_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_osu_id(pool, osu_id).await
    }

    pub async fn exists_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<Option<bool>, sqlx::Error> {
        exists_by_osu_id(pool, osu_id).await
    }

    pub async fn search(
        pool: &PgPool,
        term: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        search(pool, term, limit, offset).await
    }

    pub async fn find_all(
        pool: &PgPool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        find_all(pool, limit, offset).await
    }
}
