use crate::errors::{ServiceResult, ServiceError};
use serde_json::Value as JsonValue;
use serde::Deserialize;



#[derive(Deserialize)]
pub struct SkNewDto {
    pub project_id: i64,
    pub tag_id: i64,

    pub secret_name: String,
    pub secret_value: String,

    pub hide_from_ui: bool, // Determines if the secret should be hidden and accessible only through the API
    pub details: JsonValue, // Basic metadata or additional information about the secret (e.g., description, UI icons, etc.)
}


// ------- Implementations ------- //


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

    fn validate_secret_value(&self) -> ServiceResult<()> {
        // Secret value cannot be empty
        if self.secret_value.trim().is_empty() {
            return Err(ServiceError::Unprocessable("Secret value cannot be empty".into()));
        }

        // Maximum length check (e.g., 255 characters)
        if self.secret_value.len() > 255 {
            return Err(ServiceError::Unprocessable("Secret value cannot exceed 255 characters".into()));
        }

        Ok(())
    }

    pub fn validate(&self) -> ServiceResult<()> {
        // Validate secret name and value
        self.validate_secret_name()?;
        self.validate_secret_value()?;

        Ok(())
    }
}
