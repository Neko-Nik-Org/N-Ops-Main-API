use crate::features::api_keys::clean_expired_api_keys;
use deadpool_postgres::Pool as PgPool;
use std::future::Future;
use std::time::Duration;


const API_KEYS_CLEANUP_INTERVAL: u64 = 3 * 60 * 60;


async fn run_every<F, Fut, E>(seconds: u64, mut job: F)
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<(), E>>,
        E: std::fmt::Debug,
{
    let mut interval = tokio::time::interval(Duration::from_secs(seconds));

    loop {
        interval.tick().await;

        if let Err(err) = job().await {
            tracing::error!("Background job failed: {:?}", err);
        }
    }
}


pub async fn clean_expired_api_keys_job(pg_pool: PgPool) {
    tracing::info!("Starting clean_expired_api_keys job");

    run_every(API_KEYS_CLEANUP_INTERVAL, || {
        clean_expired_api_keys(&pg_pool)
    })
    .await;
}
