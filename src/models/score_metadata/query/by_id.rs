use crate::models::score_metadata::types::ScoreMetadataRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<ScoreMetadataRow>, SqlxError> {
    sqlx::query_as!(
        ScoreMetadataRow,
        r#"
        SELECT id, skin, pause_count, started_at, ended_at, time_paused, score, accuracy, 
               max_combo, perfect, count_300, count_100, count_50, 
               count_miss, count_katu, count_geki, created_at
        FROM score_metadata
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
