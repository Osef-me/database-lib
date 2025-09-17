use super::query::{by_id::find_by_id, by_score_id::find_by_score_id, insert::insert};
use super::types::ScoreRating;
use bigdecimal::BigDecimal;
use sqlx::PgPool;

impl ScoreRating {
    pub async fn insert(
        pool: &PgPool,
        score_id: i32,
        rating: BigDecimal,
        rating_type: &str,
    ) -> Result<Self, sqlx::Error> {
        insert(pool, score_id, rating, rating_type).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        find_by_id(pool, id).await
    }

    pub async fn find_by_score_id(pool: &PgPool, score_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        find_by_score_id(pool, score_id).await
    }
}
