use crate::models::msd::types::MSDRow;

use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(pool: &PgPool, msd: MSDRow) -> Result<i32, SqlxError> {
    let row = sqlx::query!(
        r#"
            INSERT INTO msd (
                beatmap_id, overall, stream, jumpstream, handstream,
                stamina, jackspeed, chordjack, technical, rate, main_pattern
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
            RETURNING id
            "#,
        msd.beatmap_id,
        msd.overall,
        msd.stream,
        msd.jumpstream,
        msd.handstream,
        msd.stamina,
        msd.jackspeed,
        msd.chordjack,
        msd.technical,
        msd.rate,
        msd.main_pattern
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
