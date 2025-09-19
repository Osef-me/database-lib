use sqlx::PgPool;

pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM pending_beatmap
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
