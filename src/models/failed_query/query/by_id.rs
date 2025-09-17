use crate::models::failed_query::types::FailedQuery;
use sqlx::{Error as SqlxError, PgPool};

/// Finds a failed query record by its unique identifier.
///
/// This function retrieves a failed query record from the database using
/// its unique ID. If no record is found with the given ID, `None` is returned.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool
/// * `id` - The unique identifier of the failed query to retrieve
///
/// # Returns
///
/// Returns a `Result` containing:
/// - `Ok(Some(FailedQuery))` - The failed query record if found
/// - `Ok(None)` - If no record exists with the given ID
/// - `Err(SqlxError)` - If the database operation fails
///
/// # Errors
///
/// This function will return an error if:
/// - The database connection fails
/// - The SQL query execution fails
/// - There's a data type mismatch during deserialization
///
/// # Examples
///
/// ```rust
/// use sqlx::PgPool;
/// use crate::models::failed_query::query::find_by_id;
///
/// async fn get_failed_query(pool: &PgPool) -> Result<(), sqlx::Error> {
///     match find_by_id(pool, 1).await? {
///         Some(failed_query) => {
///             println!("Found failed query: {:?}", failed_query);
///         }
///         None => {
///             println!("No failed query found with ID 1");
///         }
///     }
///     Ok(())
/// }
/// ```
pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<FailedQuery>, SqlxError> {
    let row = sqlx::query_as!(
        FailedQuery,
        r#"
        SELECT id, hash, created_at
        FROM failed_query
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
