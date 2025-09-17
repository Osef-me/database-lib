use super::query::{exists_by_checksum, find_by_id, get_beatmapset_id, insert::insert};
use super::types::BeatmapRow;
use bigdecimal::BigDecimal;
use sqlx::PgPool;

impl BeatmapRow {
    pub async fn insert_into_db(
        pool: &PgPool,
        osu_id: Option<i32>,
        beatmapset_id: Option<i32>,
        difficulty: String,
        difficulty_rating: BigDecimal,
        count_circles: i32,
        count_sliders: i32,
        count_spinners: i32,
        max_combo: i32,
        drain_time: i32,
        total_time: i32,
        bpm: BigDecimal,
        cs: BigDecimal,
        ar: BigDecimal,
        od: BigDecimal,
        hp: BigDecimal,
        mode: i32,
        status: String,
        file_md5: String,
        file_path: String,
    ) -> Result<i32, sqlx::Error> {
        insert(
            pool,
            osu_id,
            beatmapset_id,
            difficulty,
            difficulty_rating,
            count_circles,
            count_sliders,
            count_spinners,
            max_combo,
            drain_time,
            total_time,
            bpm,
            cs,
            ar,
            od,
            hp,
            mode,
            status,
            file_md5,
            file_path,
        )
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }

    pub async fn exists_by_checksum(pool: &PgPool, checksum: &str) -> Result<bool, sqlx::Error> {
        exists_by_checksum(pool, checksum).await
    }

    pub async fn get_beatmapset_id(
        pool: &PgPool,
        beatmap_id: i32,
    ) -> Result<Option<i32>, sqlx::Error> {
        get_beatmapset_id(pool, beatmap_id).await
    }
}
