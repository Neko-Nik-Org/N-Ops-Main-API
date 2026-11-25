use deadpool_postgres::Pool as PgPool;

mod jobs;


pub fn start_background_jobs(pg_pool: PgPool) {
    // Clean expired API keys
    tokio::spawn(jobs::clean_expired_api_keys_job(pg_pool.clone()));

    // Delete Old Invites
    tokio::spawn(jobs::delete_old_invites_job(pg_pool.clone()));

    // Delete expired device sessions
    tokio::spawn(jobs::delete_expired_device_sessions_job(pg_pool.clone()));
}
