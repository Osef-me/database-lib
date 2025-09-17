use sqlx::{Error as SqlxError, PgPool};

pub async fn exists_by_hash(pool: &PgPool, hash: &str) -> Result<bool, SqlxError> {
    let exists: Option<bool> = sqlx::query_scalar(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM score WHERE hash = $1
        )
        "#,
    )
    .bind(hash)
    .fetch_optional(pool)
    .await?;

    Ok(exists.unwrap_or(false))
}
