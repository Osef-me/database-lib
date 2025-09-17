//! Database query operations for the `FailedQuery` model.
//!
//! This module contains all database operations related to failed queries,
//! including CRUD operations and utility functions.
//!
//! # Available Operations
//!
//! - **Insert**: Create a new failed query record
//! - **Find by ID**: Retrieve a failed query by its unique identifier
//! - **Delete by Hash**: Remove failed queries by their hash identifier
//! - **Exists by Hash**: Check if a failed query exists for a given hash
//!
//! # Examples
//!
//! ```rust
//! use sqlx::PgPool;
//! use crate::models::failed_query::query::*;
//!
//! async fn example_operations(pool: &PgPool) -> Result<(), sqlx::Error> {
//!     // Insert a new failed query
//!     let id = insert(pool, "abc123def456").await?;
//!     
//!     // Check if it exists
//!     let exists = exists_by_hash(pool, "abc123def456").await?;
//!     assert!(exists);
//!     
//!     // Find by ID
//!     let failed_query = find_by_id(pool, id).await?;
//!     assert!(failed_query.is_some());
//!     
//!     // Delete by hash
//!     let deleted = delete_by_hash(pool, "abc123def456").await?;
//!     assert_eq!(deleted, 1);
//!     
//!     Ok(())
//! }
//! ```

pub mod by_id;
pub mod delete_by_hash;
pub mod exists_by_hash;
pub mod insert;

pub use by_id::find_by_id;
pub use delete_by_hash::delete_by_hash;
pub use exists_by_hash::exists_by_hash;
pub use insert::insert;
