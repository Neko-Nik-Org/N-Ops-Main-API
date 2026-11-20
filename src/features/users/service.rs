use crate::security::auth_tools::{verify_password_hash, check_ip_range_allowed, validate_base64_password, generate_password_hash, validate_username};
use super::dto::{LoginReqDto, AuthInfoDbDto, UpdateProfileReqDto, CreateUserReqDto};
use crate::cache::redis_cache::{set_redis_cache, delete_redis_cache};
use crate::errors::{ServiceError, ServiceResult};
use super::model::SessionUser;
use crate::state::AppState;
use super::repository;
use uuid::Uuid;



async fn validate_geo_and_ip_restrictions(state: &AppState, client_ip: &str, user_auth_info: &AuthInfoDbDto) -> ServiceResult<()> {
    // Validate IP address for the client if IP restrictions are enabled for the user
    if user_auth_info.has_ip_restriction {
        // Validate the client's IP address against any allowed IPs for the user
        if !check_ip_range_allowed(&client_ip, &user_auth_info.allowed_ips) {
            return Err(ServiceError::Unauthorized("IP address not allowed".into()));
        }
    }

    // Validate geographical location for the client if geo restrictions are enabled for the user
    if user_auth_info.has_geo_restriction {
        // Fetch the geographical location for the client's IP address
        let (country_iso_code, country_name) = repository::get_ip_location(&state.pg_pool, &client_ip).await?;
    
        // Validate if Geo location is allowed for the user (If any restrictions are set, else allow all)
        if !user_auth_info.allowed_geo_locations.is_empty() && !user_auth_info.allowed_geo_locations.contains(&country_iso_code) {
            return Err(ServiceError::Unauthorized(format!("Geo location {} not allowed", country_name)));
        }
    }

    Ok(())
}


/// Create a new user session for the given login request and return the session user information
pub async fn create_user_session(state: &AppState, login_request: &LoginReqDto, client_ip: &str) -> ServiceResult<AuthInfoDbDto> {
    // Fetch the authentication information for the given user from the repository
    let user_auth_info = repository::get_user_auth_info(
        &state.pg_pool,
        &login_request.user_name
    ).await?;

    // Check if the authentication information was found
    if user_auth_info.is_none() {
        return Err(ServiceError::Unauthorized("User authentication information not found".into()));
    }
    let user_auth_info = user_auth_info.unwrap();

    // Validate the password provided in the login request against the stored authentication information
    if !verify_password_hash(&login_request.password, &user_auth_info.password_hash) {
        return Err(ServiceError::Unauthorized("Invalid password".into()));
    }

    // Check IP and Geo restrictions for the client
    validate_geo_and_ip_restrictions(state, client_ip, &user_auth_info).await?;

    Ok(user_auth_info)
}


/// Refresh an existing user session by creating a new session with updated authentication information
pub async fn refresh_user_session(state: &AppState, session_user: &SessionUser) -> ServiceResult<SessionUser> {
    // Check if the session is allowed to be refreshed
    if !session_user.allow_refresh {
        return Err(ServiceError::BadRequest("Session cannot be refreshed".into()));
    }

    // Generate the key for the old session in Redis before refreshing it
    let current_session_key = format!("session:{}", session_user.session_id);

    // Re-Fetch the user authentication info from the database
    let auth_info = repository::get_user_auth_info(
        &state.pg_pool,
        &session_user.user_name
    ).await?;

    // If the authentication info is not found, the session is invalid or the User/Org is deleted or disabled
    if auth_info.is_none() {
        return Err(ServiceError::Unauthorized("Invalid session".into()));
    }

    // Create a new session user with the refreshed authentication information
    let session_user = SessionUser::new(auth_info.unwrap(), true);

    let new_session_key = format!("session:{}", session_user.session_id);

    // Store the refreshed session in Redis with a 3-hour TTL
    set_redis_cache(&state.redis_cache, &new_session_key, &session_user, 3 * 60 * 60).await?;

    // Delete the old session from Redis
    delete_redis_cache(&state.redis_cache, &current_session_key).await?;

    Ok(session_user)
}


/// Update the password for the given user
pub async fn update_user_password(state: &AppState, organization_id: Uuid, user_id: i64, new_password_base64: &str) -> ServiceResult<()> {
    // Validate the new password against the defined password policy
    validate_base64_password(&new_password_base64)?;

    // Generate the password hash from the new password
    let new_password_hash = generate_password_hash(&new_password_base64);

    // Update the password in the repository
    let result = repository::update_user_password(&state.pg_pool, organization_id, user_id, &new_password_hash).await?;
    if result == 0 {
        return Err(ServiceError::BadRequest("Failed to update user password in the database".into()));
    }

    Ok(())
}


/// Update the username for the given user
pub async fn update_user_username(state: &AppState, user_id: i64, new_username: &str) -> ServiceResult<()> {
    // Validate the new username against the defined username policy
    validate_username(&new_username)?;

    // Check if the new username is available
    let exists = repository::check_username_exists(&state.pg_pool, &new_username).await?;
    if exists {
        return Err(ServiceError::BadRequest("Username is already taken".into()));
    }

    // Update the username in the repository
    let result = repository::update_user_username(&state.pg_pool, user_id, &new_username).await?;
    if result == 0 {
        return Err(ServiceError::BadRequest("Failed to update username in the database".into()));
    }

    Ok(())
}


/// Update the profile for the given user
pub async fn update_user_profile(state: &AppState, user_id: i64, update_request: &UpdateProfileReqDto) -> ServiceResult<()> {
    // Update user display name
    let result = repository::update_user_display_name(&state.pg_pool, user_id, &update_request.display_name).await?;
    if result == 0 {
        return Err(ServiceError::BadRequest("Failed to update user display name in the database".into()));
    }

    // Update user primary email if provided
    if let Some(primary_email) = &update_request.primary_email {
        let result = repository::update_user_primary_email(&state.pg_pool, user_id, primary_email).await?;
        if result == 0 {
            return Err(ServiceError::BadRequest("Failed to update user primary email in the database".into()));
        }
    }

    Ok(())
}


/// Create a new user in the system
pub async fn create_user(state: &AppState, session_user: &SessionUser, create_request: &CreateUserReqDto) -> ServiceResult<()> {
    // Validate the username against the defined username policy
    validate_username(&create_request.user_name)?;

    // Validate the password against the defined password policy
    validate_base64_password(&create_request.password)?;

    // Generate the password hash from the provided password
    let password_hash = generate_password_hash(&create_request.password);

    // Create the user in the repository
    let result = repository::create_user(
        &state.pg_pool,
        session_user.organization_id,
        &password_hash,
        &create_request,
    ).await?;
    if result == 0 {
        return Err(ServiceError::BadRequest("Failed to create user in the database".into()));
    }

    Ok(())
}
