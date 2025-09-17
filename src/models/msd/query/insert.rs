use bigdecimal::BigDecimal;

use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(
    pool: &PgPool,
    beatmap_id: i32,
    overall: BigDecimal,
    stream: BigDecimal,
    jumpstream: BigDecimal,
    handstream: BigDecimal,
    stamina: BigDecimal,
    jackspeed: BigDecimal,
    chordjack: BigDecimal,
    technical: BigDecimal,
    rate: BigDecimal,
    main_pattern: Option<String>,
) -> Result<i32, SqlxError> {
    let row = sqlx::query!(
        r#"
            INSERT INTO msd (
                beatmap_id, overall, stream, jumpstream, handstream,
                stamina, jackspeed, chordjack, technical, rate, main_pattern
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
            RETURNING id
            "#,
        beatmap_id,
        overall,
        stream,
        jumpstream,
        handstream,
        stamina,
        jackspeed,
        chordjack,
        technical,
        rate,
        main_pattern
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
