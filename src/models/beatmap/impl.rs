use super::query::{exists_by_checksum, find_by_beatmapset_id, find_by_id, insert::insert};
use super::types::BeatmapRow;
use sqlx::PgPool;

impl BeatmapRow {
    pub async fn insert_into_db(self, pool: &PgPool) -> Result<i32, sqlx::Error> {
        insert(pool, self).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }

    pub async fn exists_by_checksum(pool: &PgPool, checksum: &str) -> Result<bool, sqlx::Error> {
        exists_by_checksum(pool, checksum).await
    }

    pub async fn find_by_beatmapset_id(
        pool: &PgPool,
        beatmap_id: i32,
    ) -> Result<Option<i32>, sqlx::Error> {
        find_by_beatmapset_id(pool, beatmap_id).await
    }
}
