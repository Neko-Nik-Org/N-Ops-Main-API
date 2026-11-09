use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use pg_row_derive::RowFrom;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(RowFrom)]
pub struct AuthInfoDbDto {
    pub user_id: i64,
    pub organization_id: Uuid,

    // Security and access information
    pub password_hash: String,
    pub encryption_key: String,

    pub has_ip_restriction: bool,
    pub has_geo_restriction: bool,
    pub allowed_ips: Vec<String>,
    pub allowed_geo_locations: Vec<String>,

    pub has_totp_2fa: bool,
    pub has_email_2fa: bool,

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

    pub last_login_at: ChronoUtc,
}


#[derive(Deserialize)]
pub struct LoginReqDto {
    pub user_name: String,
    pub password: String,    // Base64 encoded password
    pub remember_me: bool,   // Indicates if the user can refresh the session
}
