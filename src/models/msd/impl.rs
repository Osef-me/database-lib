use super::query::{
    find_all_by_beatmap_id, find_by_beatmap_id, find_by_beatmap_id_and_rate, find_by_id, insert,
};
use super::types::MSDRow;
use bigdecimal::BigDecimal;
use sqlx::PgPool;

impl MSDRow {
    pub async fn insert_into_db(
        pool: &PgPool,
        beatmap_id: i32,
        overall: BigDecimal,
        stream: BigDecimal,
        jumpstream: BigDecimal,
        handstream: BigDecimal,
        stamina: BigDecimal,
        jackspeed: BigDecimal,
        chordjack: BigDecimal,
        technical: BigDecimal,
        rate: BigDecimal,
        main_pattern: Option<String>,
    ) -> Result<i32, sqlx::Error> {
        insert(
            pool,
            beatmap_id,
            overall,
            stream,
            jumpstream,
            handstream,
            stamina,
            jackspeed,
            chordjack,
            technical,
            rate,
            main_pattern,
        )
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }

    pub async fn find_by_beatmap_id(
        pool: &PgPool,
        beatmap_id: i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        find_by_beatmap_id(pool, beatmap_id).await
    }

    pub async fn find_by_beatmap_id_and_rate(
        pool: &PgPool,
        beatmap_id: i32,
        rate: f64,
    ) -> Result<Option<Self>, sqlx::Error> {
        find_by_beatmap_id_and_rate(pool, beatmap_id, rate).await
    }

    pub async fn find_all_by_beatmap_id(
        pool: &PgPool,
        beatmap_id: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        find_all_by_beatmap_id(pool, beatmap_id).await
    }
}
