use crate::cache::redis_cache::{delete_redis_cache, get_redis_cache};
use crate::errors::{ServiceError, ServiceResult};
use crate::features::users::SessionUser;
use super::dto::CreateDeviceSessionDto;
use super::model::DeviceSession;
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

    // 3. Generate a new device token for the user

    // 4. Store the generated device token (Hashed) in the Redis-cache (Max 5 Minutes TTL)

    // 5. Return the generated device token to the caller
    todo!("Generate and return the device token");

    Ok("TODO".into())
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
pub async fn check_device_token(state: &AppState, session_user: SessionUser) -> ServiceResult<bool> {
    // Generate the key for the device token in the Redis-cache
    let device_token_key = format!("device_token:{}", session_user.user_id);

    // Check if the device token exists in the Redis-cache
    let device_token: Option<String> = get_redis_cache(&state.redis_cache, &device_token_key).await?;

    Ok(device_token.is_some())
}
