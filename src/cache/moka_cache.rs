use moka::future::Cache;
use std::time::Duration;
use std::sync::Arc;


/// Cache key type
pub type Key = Arc<str>;

/// Typed cache storing shared values
pub type MokaCache<T> = Cache<Key, Arc<T>>;


/// Insert a shared value into the cache
pub async fn set_in_mem_cache<T>(cache: &MokaCache<T>, key: &str, value: Arc<T>)
    where
        T: Send + Sync + 'static,
{
    cache.insert(key.into(), value).await;
}


/// Retrieve a shared value from the cache
pub async fn get_in_mem_cache<T>(cache: &MokaCache<T>, key: &str) -> Option<Arc<T>>
    where
        T: Send + Sync + 'static,
{
    cache.get(key).await
}


/// Remove a value from the cache
pub async fn remove_in_mem_cache<T>(cache: &MokaCache<T>, key: &str)
    where
        T: Send + Sync + 'static,
{
    cache.invalidate(key).await;
}


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
