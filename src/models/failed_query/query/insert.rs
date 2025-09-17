use sqlx::{Error as SqlxError, PgPool};

/// Inserts a new failed query record into the database.
///
/// This function creates a new failed query record with the provided hash
/// and returns the auto-generated ID of the created record.
///
/// # Arguments
///
/// * `pool` - A reference to the PostgreSQL connection pool
/// * `hash` - The hash identifier for the failed query (must be alphanumeric)
///
/// # Returns
///
/// Returns a `Result` containing:
/// - `Ok(i32)` - The ID of the newly created failed query record
/// - `Err(SqlxError)` - If the database operation fails
///
/// # Errors
///
/// This function will return an error if:
/// - The database connection fails
/// - The hash violates database constraints (e.g., too long, invalid format)
/// - A database constraint violation occurs
///
/// # Examples
///
/// ```rust
/// use sqlx::PgPool;
/// use crate::models::failed_query::query::insert;
///
/// async fn create_failed_query(pool: &PgPool) -> Result<(), sqlx::Error> {
///     let id = insert(pool, "abc123def456").await?;
///     println!("Created failed query with ID: {}", id);
///     Ok(())
/// }
/// ```
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
