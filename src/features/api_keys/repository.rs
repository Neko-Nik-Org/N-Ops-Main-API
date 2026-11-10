use deadpool_postgres::Pool as PgPool;
use crate::errors::PgResult;



pub async fn clean_expired_api_keys(db_pool: &PgPool) -> PgResult<()> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM api_key
            WHERE expires_at IS NOT NULL AND expires_at <= NOW()
            "#,
            &[],
        )
        .await?;

    tracing::info!("Cleaned up {} expired API keys", result);

    Ok(())
}
