use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(pool: &PgPool, hash: &str) -> Result<i32, SqlxError> {
    Ok(sqlx::query!(
        r#"
        INSERT INTO failed_query (hash)
        VALUES ($1)
        RETURNING id
        "#,
        hash
    )
    .fetch_one(pool)
    .await?
    .id)
}
