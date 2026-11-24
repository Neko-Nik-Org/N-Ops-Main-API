use crate::errors::{ServiceError, ServiceResult};
use crate::security::encryption;
use crate::state::API_SETTINGS;
use super::dto::{CreateOrgDto};
use crate::state::AppState;
use super::repository;
use std::sync::Arc;



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


/// Get Organization Encryption Key (Decrypted)
async fn get_organization_encryption_key(state: &AppState, org_id: &uuid::Uuid) -> ServiceResult<Vec<u8>> {
    // Check if the organization key exists in the In-Memory cache
    let cache_key = org_id.to_string();
    let decrypted_key= state
        .in_mem_cache
        .org_encryption_keys
        .get(cache_key.as_str())
        .await;

    if let Some(key) = decrypted_key {
        return Ok(encryption::decode_encryption_key(&key));
    }

    // Fetch the encrypted encryption key for the organization from the repository
    let encrypted_encryption_key = repository::get_encryption_key(&state.pg_pool, org_id).await?;
    if encrypted_encryption_key.is_none() {
        return Err(ServiceError::NotFound("Organization not found or is inactive".into()));
    }
    let encrypted_encryption_key = encrypted_encryption_key.unwrap();

    // Decrypt the encryption key using the master encryption key
    let decrypted_encryption_key: String = encryption::decrypt(&API_SETTINGS.master_encryption_key, &encrypted_encryption_key);

    // Store the decrypted encryption key in the In-Memory cache for future use
    state
        .in_mem_cache
        .org_encryption_keys
        .insert(
            cache_key.into(),
            Arc::new(decrypted_encryption_key.clone())
        )
        .await;

    Ok(encryption::decode_encryption_key(&decrypted_encryption_key))
}


/// Encrypt the given data using the organization ID
pub async fn encrypt_with_org_key(state: &AppState, org_id: &uuid::Uuid, data: &str) -> ServiceResult<String> {
    let org_encryption_key = get_organization_encryption_key(state, org_id).await?;
    Ok(encryption::encrypt(&org_encryption_key, data))
}


/// Decrypt the given data using the organization ID
pub async fn decrypt_with_org_key(state: &AppState, org_id: &uuid::Uuid, data: &str) -> ServiceResult<String> {
    let org_encryption_key = get_organization_encryption_key(state, org_id).await?;
    Ok(encryption::decrypt(&org_encryption_key, data))
}
