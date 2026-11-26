use crate::errors::{ServiceResult, ServiceError};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use pg_row_derive::RowFrom;



#[derive(Deserialize)]
pub struct SkNewDto {
    pub project_id: i64,
    pub tag_id: i64,

    pub secret_name: String,
    pub secret_value: String,

    pub hide_from_ui: bool, // Determines if the secret should be hidden and accessible only through the API
    pub details: JsonValue, // Basic metadata or additional information about the secret (e.g., description, UI icons, etc.)
}


#[derive(Serialize, Deserialize, RowFrom)]
pub struct SkVersionDto {
    pub project_id: i64,
    pub secret_id: i64,
    pub tag_id: i64,

    pub secret_value: String,

    pub hide_from_ui: bool, // Determines if the secret should be hidden and accessible only through the API
    pub details: JsonValue, // Basic metadata or additional information about the secret (e.g., description, UI icons, etc.)
}


#[derive(Deserialize)]
pub struct SkListingDto {
    pub project_id: i64,
    pub tag_id: i64,

    pub limit: i64,
    pub offset: i64,
}


#[derive(Deserialize)]
pub struct SkListVersionsDto {
    pub project_id: i64,
    pub secret_id: i64,
    pub tag_id: i64,

    pub limit: i64,
    pub offset: i64,
}


#[derive(Deserialize)]
pub struct SkDeleteDto {
    pub project_id: i64,
    pub tag_id: i64,
    pub secret_id: i64,
}


// ------- Implementations ------- //


/// Validate secret value for all secret key DTOs
fn validate_secret_value(secret_value: &str) -> ServiceResult<()> {
    // Secret value cannot be empty
    if secret_value.trim().is_empty() {
        return Err(ServiceError::Unprocessable("Secret value cannot be empty".into()));
    }

    // Maximum length check (e.g., 255 characters)
    if secret_value.len() > 255 {
        return Err(ServiceError::Unprocessable("Secret value cannot exceed 255 characters".into()));
    }

    Ok(())
}

impl SkNewDto {
    fn validate_secret_name(&self) -> ServiceResult<()> {
        // Cant be empty
        if self.secret_name.trim().is_empty() {
            return Err(ServiceError::Unprocessable("Secret name cannot be empty".into()));
        }

        // Secret name should not contain spaces
        if self.secret_name.contains(' ') {
            return Err(ServiceError::Unprocessable("Secret name cannot contain spaces".into()));
        }

        // It can only contain alphanumeric characters, underscores, and hyphens
        if !self.secret_name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(ServiceError::Unprocessable("Secret name can only contain alphanumeric characters, underscores, and hyphens".into()));
        }

        Ok(())
    }

    /// Validate the secret key data for creating a new secret key
    pub fn validate(&self) -> ServiceResult<()> {
        // Validate secret name and value
        self.validate_secret_name()?;
        validate_secret_value(&self.secret_value)?;

        Ok(())
    }
}


impl SkVersionDto {
    /// Validate the secret key data for creating a new secret key version
    pub fn validate(&self) -> ServiceResult<()> {
        // Validate secret value
        validate_secret_value(&self.secret_value)?;

        Ok(())
    }

    /// Replace the secret value with its decrypted version
    pub fn with_decrypted_value(mut self, decrypted_value: String) -> Self {
        self.secret_value = decrypted_value;
        self
    }
}
