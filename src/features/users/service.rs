use crate::security::auth_tools::{verify_password_hash, check_ip_range_allowed};
use crate::errors::{ServiceError, ServiceResult};
use super::dto::{LoginReqDto, AuthInfoDbDto};
use crate::state::AppState;
use super::repository;


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
