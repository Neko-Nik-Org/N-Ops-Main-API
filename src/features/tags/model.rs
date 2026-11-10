use serde::{Deserialize, Serialize};
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
