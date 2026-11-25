use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use pg_row_derive::RowFrom;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(RowFrom, Serialize, Deserialize)]
pub struct AuthInfoDbDto {
    pub user_id: i64,
    pub organization_id: Uuid,

    // Security and access information
    pub password_hash: String,
    pub encryption_key: String,

    // Policy information
    pub enforce_totp_2fa: bool,
    pub enforce_email_2fa: bool,
    pub allow_device_logins: bool,
    pub has_ip_restriction: bool,
    pub has_geo_restriction: bool,
    pub allowed_ips: Vec<String>,
    pub allowed_geo_locations: Vec<String>,

    pub has_totp_2fa: bool,
    pub has_email_2fa: bool,

    pub permissions: Vec<String>,
    pub tags: Vec<i64>,
    pub can_approve_approvals: bool,
    pub can_bypass_approvals: bool,
    pub allowed_linux_users: Vec<String>,

    // Basic user information
    pub user_name: String,
    pub display_name: String,
    pub organization_name: String,

    // Contact information (Use only if it is verified)
    pub primary_email: String,
    pub is_email_verified: bool,

    pub last_login_at: ChronoUtc,
}


#[derive(Deserialize)]
pub struct LoginReqDto {
    pub user_name: String,
    pub password: String,    // Base64 encoded password
    pub remember_me: bool,   // Indicates if the user can refresh the session
}


#[derive(Deserialize)]
pub struct DeviceRefreshReqDto {
    pub refresh_token: String,
    pub current_session_key: String,
}


#[derive(Deserialize)]
pub struct UpdateProfileReqDto {
    pub display_name: String,
    pub primary_email: Option<String>,  // If given, the new primary email for the user
}


#[derive(Deserialize)]
pub struct UpdateProfileAdminReqDto {
    pub display_name: String,
    pub primary_email: Option<String>,  // If given, the new primary email for the user
    pub user_details: JsonValue,
}


#[derive(Deserialize)]
pub struct ListUsersReqDto {
    pub user_name: Option<String>,  // Filter users by user name (optional)
    pub limit: i64,
    pub offset: i64
}


#[derive(Deserialize)]
pub struct CreateUserReqDto {
    pub user_name: String,
    pub display_name: String,

    pub primary_email: String,
    pub user_details: JsonValue,
    pub is_active: bool,

    pub role_id: i64,
    pub password: String,    // Base64 encoded password

    pub access_restriction_policy_id: Option<i64>,
}


#[derive(Deserialize)]
pub struct AssignArPolicyReqDto {
    pub access_restriction_policy_id: Option<i64>,
}
