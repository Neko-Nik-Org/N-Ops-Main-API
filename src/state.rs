use crate::utils::initial::{ApiSettings, AppSettings, RmqSettings, CfTurnstileSettings, SmtpSettings, OtpSettings, OrgInviteSettings, InMemCacheSettings, DeviceTokenSettings};
use crate::cache::moka_cache::{MokaCache, build_ttl_cache, build_idle_policy_cache};
use crate::database::pool::{init_pg_pool, warm_pool};
use crate::cache::redis_cache::init_redis;
use crate::tasks::start_background_jobs;
use redis::aio::MultiplexedConnection;
use deadpool_postgres::Pool as PgPool;
use actix_web::web::Data as webData;
use std::sync::LazyLock;



pub struct AppState {
    pub pg_pool: PgPool,
    pub redis_cache: MultiplexedConnection,
    pub in_mem_cache: InMemCache,
}


pub struct InMemCache {
    pub org_encryption_keys: MokaCache<String>,
    pub org_tags: MokaCache<Vec<i64>>,
    pub number_based: MokaCache<u64>,
}


pub static API_SETTINGS: LazyLock<ApiSettings> = LazyLock::new(|| {
    ApiSettings::from_env()
});


pub static RMQ_SETTINGS: LazyLock<RmqSettings> = LazyLock::new(|| {
    RmqSettings::from_env()
});


pub static SMTP_SETTINGS: LazyLock<SmtpSettings> = LazyLock::new(|| {
    SmtpSettings::from_env()
});


pub static OTP_SETTINGS: LazyLock<OtpSettings> = LazyLock::new(|| {
    OtpSettings::from_env()
});


pub static DEVICE_TOKEN_SETTINGS: LazyLock<DeviceTokenSettings> = LazyLock::new(|| {
    DeviceTokenSettings::from_env()
});


pub static ORG_INVITE_SETTINGS: LazyLock<OrgInviteSettings> = LazyLock::new(|| {
    OrgInviteSettings::from_env()
});


pub static CLOUDFLARE_TURNSTILE_SETTINGS: LazyLock<CfTurnstileSettings> = LazyLock::new(|| {
    CfTurnstileSettings::from_env()
});


/// Build the in-memory cache (Moka)
fn init_in_mem_cache(in_mem_cache_settings: InMemCacheSettings) -> InMemCache {
    InMemCache {
        org_encryption_keys: build_idle_policy_cache(
            in_mem_cache_settings.org_encryption_keys.cache_size,
            in_mem_cache_settings.org_encryption_keys.idle_time
        ),
        org_tags: build_idle_policy_cache(
            in_mem_cache_settings.org_tags.cache_size,
            in_mem_cache_settings.org_tags.idle_time
        ),
        number_based: build_ttl_cache(
            in_mem_cache_settings.number_based.cache_size,
            in_mem_cache_settings.number_based.expiration_time
        ),
    }
}


/// Determines if the given origin is allowed based on the API settings
pub fn cors_allowed_origin_fn(origin: &actix_web::http::header::HeaderValue, _: &actix_web::dev::RequestHead) -> bool {
    let origin_str = origin.to_str().unwrap_or("");
    let allowed_origins = &API_SETTINGS.allowed_origins;

    if allowed_origins.iter().any(|item| item == "*") {
        return true;
    }

    allowed_origins.iter().any(|item| item == origin_str)
}


/// Initializes the application state, including the Postgres pool, Redis cache, and in-memory cache
pub async fn initialize() -> webData<AppState> {
    // Preparing to start the server by collecting environment variables
    let app_settings: AppSettings = AppSettings::from_env();

    tracing::info!("Starting the server by initializing the application state");

    // Initialize the Postgres client
    let pg_pool = init_pg_pool(&app_settings.pg_settings);

    // Warm the Postgres pool based on the warm pool settings
    warm_pool(&pg_pool, &app_settings.pg_settings).await;

    // Initialize the in-memory cache (Moka)    [You can configure for each type of cache separately also, this is just an example]
    let in_mem_cache = init_in_mem_cache(app_settings.in_mem_cache_settings);

    // Initialize the Redis cache
    let redis_cache = init_redis(&app_settings.redis_url).await;

    // Start background jobs
    start_background_jobs(pg_pool.clone());

    // Wrap the state of the application and share it
    webData::new(AppState {
        pg_pool,
        redis_cache,
        in_mem_cache,
    })
}
