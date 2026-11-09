use crate::security::auth_tools::verify_password_hash;
use crate::errors::{ServiceError, ServiceResult};
use super::model::SessionUser;
use super::dto::LoginReqDto;
use crate::state::AppState;
use super::repository;



/// Create a new user session for the given login request and return the session user information
pub async fn create_user_session(state: &AppState, login_request: &LoginReqDto, client_ip: &str) -> ServiceResult<SessionUser> {
    // Fetch the authentication information for the given user from the repository
    let fetch_auth_info = repository::get_user_auth_info(
        &state.pg_pool,
        &login_request.user_name
    ).await?;

    // Check if the authentication information was found
    if fetch_auth_info.is_none() {
        return Err(ServiceError::Unauthorized("User authentication information not found".into()));
    }
    let fetch_auth_info = fetch_auth_info.unwrap();

    // Validate the password provided in the login request against the stored authentication information
    if !verify_password_hash(&login_request.password, &fetch_auth_info.password_hash) {
        return Err(ServiceError::Unauthorized("Invalid password".into()));
    }

    // Check if the user has IP or geo restrictions and validate them accordingly

    // Check if the user has any 2FA enabled and validate it accordingly

    Err(ServiceError::NotFound("Cats".into()))
}
