//! Failed Query Model
//! 
//! This module provides functionality for managing failed query records in the database.
//! Failed queries are typically used to track queries that have failed for various reasons,
//! enabling monitoring, debugging, and retry mechanisms.
//! 
//! # Overview
//! 
//! The `FailedQuery` model represents a record of a failed database query operation.
//! It includes a unique identifier, a hash of the failed query, and an optional timestamp
//! indicating when the failure was recorded.
//! 
//! # Features
//! 
//! - **Data Validation**: Comprehensive validation rules for data integrity
//! - **Database Operations**: Full CRUD operations with PostgreSQL
//! - **Hash-based Lookups**: Efficient querying by hash identifier
//! - **Timestamp Tracking**: Optional creation timestamp recording
//! 
//! # Usage
//! 
//! ```rust
//! use crate::models::failed_query::{FailedQuery, query::*};
//! use sqlx::PgPool;
//! 
//! async fn example_usage(pool: &PgPool) -> Result<(), sqlx::Error> {
//!     // Create a new failed query record
//!     let id = insert(pool, "abc123def456").await?;
//!     
//!     // Check if a failed query exists
//!     let exists = exists_by_hash(pool, "abc123def456").await?;
//!     
//!     // Retrieve the failed query
//!     if let Some(failed_query) = find_by_id(pool, id).await? {
//!         println!("Failed query: {:?}", failed_query);
//!     }
//!     
//!     // Clean up
//!     delete_by_hash(pool, "abc123def456").await?;
//!     
//!     Ok(())
//! }
//! ```
//! 
//! # Validation Rules
//! 
//! - **ID**: Must be a positive integer (≥ 1)
//! - **Hash**: Must be 1-255 alphanumeric characters only
//! - **Created At**: Optional timestamp field
//! 
//! # Database Schema
//! 
//! The corresponding database table should have the following structure:
//! 
//! ```sql
//! CREATE TABLE failed_query (
//!     id SERIAL PRIMARY KEY,
//!     hash VARCHAR(255) NOT NULL CHECK (hash ~ '^[a-zA-Z0-9]+$'),
//!     created_at TIMESTAMP
//! );
//! ```

pub mod r#impl;
pub mod query;
pub mod types;

#[cfg(test)]
mod tests;

pub use types::FailedQuery;
