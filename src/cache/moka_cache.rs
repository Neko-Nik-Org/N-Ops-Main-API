use moka::future::Cache;
use std::time::Duration;
use std::sync::Arc;


/// Cache key type
pub type Key = Arc<str>;

/// Typed cache storing shared values
pub type MokaCache<T> = Cache<Key, Arc<T>>;


/// Create a new Moka cache with the specified size and expiration time
pub fn build_ttl_cache<T: Send + Sync + 'static>(cache_size: u64, expiration_time: Duration) -> MokaCache<T> {
    MokaCache::builder()
        .max_capacity(cache_size)
        .time_to_live(expiration_time)
        .build()
}


pub fn build_idle_policy_cache<T: Send + Sync + 'static>(cache_size: u64, idle_time: Duration) -> MokaCache<T> {
    MokaCache::builder()
        .max_capacity(cache_size)
        .time_to_idle(idle_time)
        .build()
}
