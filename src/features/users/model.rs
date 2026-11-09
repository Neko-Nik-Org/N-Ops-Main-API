use serde::{Deserialize, Serialize};
use super::dto::AuthInfoDbDto;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
pub struct SessionUser {
    // Unique identifiers for the session and user
    pub session_id: Uuid,
    pub user_id: i64,
    pub organization_id: Uuid,

    // Security and access information
    pub allow_refresh: bool,    // Indicates if the session can be refreshed
    pub encryption_key: String,
    pub permissions: Vec<String>,
    pub tags: Vec<String>,

    // Basic user information
    pub user_name: String,
    pub display_name: String,
    pub organization_name: String,

    // Contact information (Use only if it is verified)
    pub primary_email: String,
    pub is_email_verified: bool,

    // Limits for the user or organization
    pub user_limit: i32,
    pub server_limit: i32,
    pub ss_project_limit: i32,
    pub ss_keys_limit: i32,
    pub ss_file_limit: i32,
    pub user_notes_limit: i32,
}


// ------- Implementations ------- //


impl SessionUser {
    /// Creates a new `SessionUser` with a unique session ID
    pub fn new(auth_info: AuthInfoDbDto, allow_refresh: bool) -> Self {
        SessionUser {
            session_id: Uuid::new_v4(),
            user_id: auth_info.user_id,
            organization_id: auth_info.organization_id,

            allow_refresh,
            encryption_key: auth_info.encryption_key,
            permissions: auth_info.permissions,
            tags: auth_info.tags,

            user_name: auth_info.user_name,
            display_name: auth_info.display_name,
            organization_name: auth_info.organization_name,

            primary_email: auth_info.primary_email,
            is_email_verified: auth_info.is_email_verified,

            user_limit: auth_info.user_limit,
            server_limit: auth_info.server_limit,
            ss_project_limit: auth_info.ss_project_limit,
            ss_keys_limit: auth_info.ss_keys_limit,
            ss_file_limit: auth_info.ss_file_limit,
            user_notes_limit: auth_info.user_notes_limit,
        }
    }
}
