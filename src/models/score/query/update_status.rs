use sqlx::{Error as SqlxError, PgPool};

pub async fn update_status(pool: &PgPool, id: i32, status: &str) -> Result<u64, SqlxError> {
    let result = sqlx::query!(
        r#"
        UPDATE score 
        SET status = $1
        WHERE id = $2
        "#,
        status,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn update_status_by_hash(
    pool: &PgPool,
    hash: &str,
    status: &str,
) -> Result<u64, SqlxError> {
    let result = sqlx::query!(
        r#"
        UPDATE score 
        SET status = $1
        WHERE hash = $2
        "#,
        status,
        hash
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
