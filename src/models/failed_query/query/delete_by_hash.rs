use sqlx::{Error as SqlxError, PgPool};

/// Deletes failed query records by their hash identifier.
/// 
/// This function removes all failed query records that match the provided hash.
/// It returns the number of records that were deleted.
/// 
/// # Arguments
/// 
/// * `pool` - A reference to the PostgreSQL connection pool
/// * `hash` - The hash identifier of the failed queries to delete
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(u64)` - The number of records that were deleted
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
/// use crate::models::failed_query::query::delete_by_hash;
/// 
/// async fn cleanup_failed_queries(pool: &PgPool) -> Result<(), sqlx::Error> {
///     let deleted_count = delete_by_hash(pool, "abc123def456").await?;
///     println!("Deleted {} failed query records", deleted_count);
///     Ok(())
/// }
/// ```
pub async fn delete_by_hash(pool: &PgPool, hash: &str) -> Result<u64, SqlxError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM failed_query
        WHERE hash = $1
        "#,
        hash
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
