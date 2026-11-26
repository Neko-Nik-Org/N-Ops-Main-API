use crate::features::organization::{encrypt_with_org_key, decrypt_with_org_key};
use crate::errors::{ServiceError, ServiceResult};
use crate::features::users::SessionUser;
use crate::state::AppState;
use super::dto::SkNewDto;
use super::repository;



/// Create a new vault project secret key
pub async fn create_new_sk(state: &AppState, session_user: &SessionUser, secret_key_data: &SkNewDto) -> ServiceResult<()> {
    // Validate the new secret key data before proceeding
    secret_key_data.validate()?;

    // Encrypt the secret value using the organization's key
    let encrypted_secret_value = encrypt_with_org_key(
        state,
        &session_user.organization_id,
        &secret_key_data.secret_value
    ).await?;

    // Create the new secret key in the vault project
    let result = repository::create_new_secret_key(
        &state.pg_pool,
        session_user.organization_id,
        secret_key_data,
        &encrypted_secret_value
    ).await?;
    if result == 0 {
        return Err(ServiceError::Unprocessable("Failed to create new secret key".into()));
    }

    Ok(())
}
