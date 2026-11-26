use crate::features::organization::{encrypt_with_org_key, decrypt_with_org_key};
use crate::errors::{ServiceError, ServiceResult};
use crate::features::users::SessionUser;
use crate::state::AppState;
use super::dto::{SkNewDto, SkVersionDto};
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


/// Create a new vault key version for a project secret key
pub async fn make_new_sk_version(state: &AppState, session_user: &SessionUser, secret_key_version_data: &SkVersionDto) -> ServiceResult<()> {
    // Validate the new secret key data before proceeding
    secret_key_version_data.validate()?;

    // Encrypt the secret value using the organization's key
    let encrypted_secret_value = encrypt_with_org_key(
        state,
        &session_user.organization_id,
        &secret_key_version_data.secret_value
    ).await?;

    // Create the new secret key in the vault project
    let result = repository::create_new_sk_version(
        &state.pg_pool,
        session_user.organization_id,
        secret_key_version_data,
        &encrypted_secret_value
    ).await?;
    if result == 0 {
        return Err(ServiceError::Unprocessable("Failed to create new secret key".into()));
    }

    Ok(())
}


/// Read a specific version of a vault project secret key
pub async fn read_secret_key_by_version(state: &AppState, session_user: &SessionUser, secret_id: i64, version_number: i64) -> ServiceResult<SkVersionDto> {
    // Fetch the secret key by its version from the repository
    let secret_key = repository::read_secret_key_by_version(
        &state.pg_pool,
        session_user.organization_id,
        secret_id,
        version_number,
    ).await?;
    if secret_key.is_none() {
        return Err(ServiceError::NotFound("Secret key not found, or not accessible to the user".into()));
    }
    let secret_key = secret_key.unwrap();

    // Check if the secret is hidden from the UI
    if secret_key.hide_from_ui {
        return Err(ServiceError::NotFound("Secret key not found, or not accessible to the user".into()));
    }

    // Check if the user can access the secret key based on the tag
    session_user.has_tag_permission(state, secret_key.tag_id).await?;

    // Since the secret key is not hidden and the user has the necessary tag permission
    // Decrypt the secret value using the organization's key

    let decrypted_secret_value = decrypt_with_org_key(
        state,
        &session_user.organization_id,
        &secret_key.secret_value
    ).await?;

    Ok(secret_key.with_decrypted_value(decrypted_secret_value))
}
