use crate::models::rating::score_mania_rating::types::ScoreManiaRatingRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<ScoreManiaRatingRow>, SqlxError> {
    sqlx::query_as!(
        ScoreManiaRatingRow,
        r#"
        SELECT id, rating_id, stream, jumpstream, handstream, stamina, jackspeed, chordjack, technical, created_at
        FROM score_mania_rating
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

