use super::super::types::ScoreRatingRow;
use bigdecimal::BigDecimal;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(
    pool: &PgPool,
    score_id: i32,
    rating: BigDecimal,
    rating_type: &str,
) -> Result<ScoreRatingRow, SqlxError> {
    let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "INSERT INTO score_rating (score_id, rating, rating_type, created_at) VALUES (",
    );
    let mut sep = builder.separated(", ");
    sep.push_bind(score_id);
    sep.push_bind(rating);
    sep.push_bind(rating_type);
    sep.push("NOW()");
    // `sep` drops here naturally; no need to call drop explicitly
    builder.push(") RETURNING id, score_id, rating, rating_type, created_at");

    builder
        .build_query_as::<ScoreRatingRow>()
        .fetch_one(pool)
        .await
}
