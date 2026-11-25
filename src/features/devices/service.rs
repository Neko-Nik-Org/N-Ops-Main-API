use crate::cache::redis_cache::{delete_redis_cache, get_redis_cache, set_redis_cache};
use crate::security::device_token::{build_device_token, hash_device_token};
use crate::errors::{ServiceError, ServiceResult};
use crate::features::users::SessionUser;
use super::dto::DeviceLoginReqDto;
use crate::state::AppState;
use super::repository;



/// Generate a new device token for the session user
pub async fn generate_device_token(state: &AppState, session_user: SessionUser) -> ServiceResult<String> {
    // 1. Check if the user is allowed to login from a new device
    if !session_user.allow_device_logins {
        return Err(ServiceError::Forbidden("Device logins are not allowed for your account".into()));
    }

    // 2. Check if there are any existing generated device tokens for the user
    // If so, dont allow generating a new token until the existing token is revoked or expired
    if check_device_token(state, session_user.user_id).await? {
        return Err(ServiceError::Forbidden("An active device token already exists for your account".into()));
    }

    // 3. Generate a new device token for the user
    let device_token = build_device_token(32)?; // Generate a 32-character device token
    let device_token_hashed = hash_device_token(&device_token);

    // 4. Store the generated device token (Hashed) in the Redis-cache (Max 5 Minutes TTL)
    let device_token_key = format!("device_token:{}", session_user.user_id);

    // 5. Save it in the Redis-cache
    set_redis_cache(
        &state.redis_cache,
        &device_token_key,
        &device_token_hashed,
        300
    ).await?;

    Ok(device_token)
}


/// Revoke an existing device token for the session user
pub async fn revoke_device_token(state: &AppState, session_user: SessionUser) -> ServiceResult<()> {
    // Check if the user is allowed to even have device tokens
    if !session_user.allow_device_logins {
        return Err(ServiceError::Forbidden("Device logins are not enabled for your account".into()));
    }

    // Generate the key for the device token in the Redis-cache
    let device_token_key = format!("device_token:{}", session_user.user_id);

    // Revoke the device token for the user by deleting it from the Redis-cache
    delete_redis_cache(&state.redis_cache, &device_token_key).await?;

    Ok(())
}


/// Check if a device token is valid for the session user
pub async fn check_device_token(state: &AppState, user_id: i64) -> ServiceResult<bool> {
    // Generate the key for the device token in the Redis-cache
    let device_token_key = format!("device_token:{}", user_id);

    // Check if the device token exists in the Redis-cache
    let device_token: Option<String> = get_redis_cache(&state.redis_cache, &device_token_key).await?;

    Ok(device_token.is_some())
}


/// Validate a device token, given the user ID and the device token
async fn validate_device_token(state: &AppState, user_id: i64, device_token: &str) -> ServiceResult<bool> {
    // Generate the key for the device token in the Redis-cache
    let device_token_key = format!("device_token:{}", user_id);

    // Fetch the hashed device token from the Redis-cache
    let device_token_hashed: Option<String> = get_redis_cache(&state.redis_cache, &device_token_key).await?;

    
    // Validate the device token
    if let Some(stored_hash) = device_token_hashed {
        // Make sure to delete the device token from the Redis-cache after validation (Even if it is failed)
        delete_redis_cache(&state.redis_cache, &device_token_key).await?;

        Ok(stored_hash == hash_device_token(device_token))
    } else {
        Ok(false)
    }
}


/// Create a new device refresh token for the given login request and return the session user information
pub async fn create_device_refresh_token(state: &AppState, device_login_req: DeviceLoginReqDto) -> ServiceResult<String> {
    // Check if the device login request is valid for the given user
    if !validate_device_token(&state, device_login_req.user_id, &device_login_req.device_token).await? {
        return Err(ServiceError::BadRequest("Invalid device token, regenerate a new one and try again".into()));
    }

    // Create a new device session for the given device details (Refresh Token)
    let refresh_token = uuid::Uuid::new_v4().to_string();
    let refresh_token_hash = hash_device_token(&refresh_token);

    // Create a new device session in the repository
    repository::create_new_device_session(&state.pg_pool, &device_login_req, &refresh_token_hash).await?;

    Ok(refresh_token)
}
