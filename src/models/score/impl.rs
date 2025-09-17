use super::query::{
    by_beatmap_id::find_by_beatmap_id,
    by_id::find_by_id,
    by_pending::find_pending_score,
    by_user_id::find_by_user_id,
    exists::exists_by_hash,
    insert::insert,
    update_status::{update_status, update_status_by_hash},
};
use super::types::ScoreRow;
use bigdecimal::BigDecimal;
use sqlx::PgPool;

impl ScoreRow {
    pub async fn insert(
        pool: &PgPool,
        user_id: i64,
        beatmap_id: i32,
        score_metadata_id: i32,
        replay_id: Option<i32>,
        rate: BigDecimal,
        hwid: Option<&str>,
        mods: i64,
        hash: &str,
        rank: &str,
        status: &str,
    ) -> Result<Self, sqlx::Error> {
        insert(
            pool,
            user_id,
            beatmap_id,
            score_metadata_id,
            replay_id,
            rate,
            hwid,
            mods,
            hash,
            rank,
            status,
        )
        .await
    }

    pub async fn exists_by_hash(pool: &PgPool, hash: &str) -> Result<bool, sqlx::Error> {
        exists_by_hash(pool, hash).await
    }

    pub async fn find_pending_score(pool: &PgPool) -> Result<Option<Self>, sqlx::Error> {
        find_pending_score(pool).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        find_by_user_id(pool, user_id).await
    }

    pub async fn find_by_beatmap_id(
        pool: &PgPool,
        beatmap_id: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        find_by_beatmap_id(pool, beatmap_id).await
    }

    pub async fn update_status(pool: &PgPool, id: i32, status: &str) -> Result<u64, sqlx::Error> {
        update_status(pool, id, status).await
    }

    pub async fn update_status_by_hash(
        pool: &PgPool,
        hash: &str,
        status: &str,
    ) -> Result<u64, sqlx::Error> {
        update_status_by_hash(pool, hash, status).await
    }
}
