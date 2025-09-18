use crate::models::other::failed_query::types::FailedQueryRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<FailedQueryRow>, SqlxError> {
    sqlx::query_as!(
        FailedQueryRow,
        r#"
        SELECT id, hash, created_at
        FROM failed_query
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

