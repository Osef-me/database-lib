use super::query::{delete_by_hash, exists_by_hash, find_by_id, insert};
use super::FailedQueryRow;
use sqlx::{Error as SqlxError, PgPool};

impl FailedQueryRow {
    pub async fn insert(pool: &PgPool, hash: &str) -> Result<i32, SqlxError> {
        insert(pool, hash).await
    }

    pub async fn exists_by_hash(pool: &PgPool, hash: &str) -> Result<bool, SqlxError> {
        exists_by_hash(pool, hash).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Self, SqlxError> {
        find_by_id(pool, id).await?.ok_or(SqlxError::RowNotFound)

    }

    pub async fn delete_by_hash(pool: &PgPool, hash: &str) -> Result<u64, SqlxError> {
        delete_by_hash(pool, hash).await
    }
}
