use crate::models::msd::types::MSDRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<MSDRow>, SqlxError> {
    sqlx::query_as!(
        MSDRow,
        r#"
        SELECT * FROM msd WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
