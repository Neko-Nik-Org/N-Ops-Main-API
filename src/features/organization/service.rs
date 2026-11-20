use crate::errors::{ServiceError, ServiceResult};
use crate::security::encryption;
use crate::state::API_SETTINGS;
use super::dto::{CreateOrgDto};
use crate::state::AppState;
use super::repository;


/// Create a new tag using the provided resource tag data
pub async fn create_organization(state: &AppState, org_data: &CreateOrgDto) -> ServiceResult<()> {
    // Build the encryption key for the organization
    let encryption_key = encryption::generate_encryption_key();

    // Encrypt the encryption key with the master encryption key
    let encrypted_encryption_key = encryption::encrypt(
        &API_SETTINGS.master_encryption_key,
        &encryption_key
    );

    let result = repository::create_organization(&state.pg_pool, &encrypted_encryption_key, org_data).await?;
    if result == 0 {
        return Err(ServiceError::Unprocessable("Failed to create new organization".into()));
    }

    Ok(())
}
