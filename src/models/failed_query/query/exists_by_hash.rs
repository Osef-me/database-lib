use sqlx::{Error as SqlxError, PgPool};

/// Checks if a failed query record exists for the given hash.
///
/// This function performs an efficient existence check using a SQL EXISTS query
/// to determine whether any failed query records exist with the provided hash.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool
/// * `hash` - The hash identifier to check for existence
///
/// # Returns
///
/// Returns a `Result` containing:
/// - `Ok(true)` - If at least one failed query record exists with the given hash
/// - `Ok(false)` - If no failed query records exist with the given hash
/// - `Err(SqlxError)` - If the database operation fails
///
/// # Errors
///
/// This function will return an error if:
/// - The database connection fails
/// - The SQL query execution fails
///
/// # Examples
///
/// ```rust
/// use sqlx::PgPool;
/// use crate::models::failed_query::query::exists_by_hash;
///
/// async fn check_failed_query_exists(pool: &PgPool) -> Result<(), sqlx::Error> {
///     let exists = exists_by_hash(pool, "abc123def456").await?;
///     if exists {
///         println!("Failed query exists for this hash");
///     } else {
///         println!("No failed query found for this hash");
///     }
///     Ok(())
/// }
/// ```
pub async fn exists_by_hash(pool: &PgPool, hash: &str) -> Result<bool, SqlxError> {
    let query = sqlx::query!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM failed_query WHERE hash = $1
        ) as exists
        "#,
        hash
    );
    let result = query.fetch_one(pool).await?;
    Ok(result.exists.unwrap_or(false))
}
