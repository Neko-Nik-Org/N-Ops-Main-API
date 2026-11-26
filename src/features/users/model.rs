use crate::errors::{ServiceError, ServiceResult};
use crate::features::tags::fetch_org_tag_ids;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use super::dto::AuthInfoDbDto;
use pg_row_derive::RowFrom;
use crate::state::AppState;
use uuid::Uuid;

type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(Serialize, Deserialize, Clone)]
pub struct SessionUser {
    // Unique identifiers for the session and user
    pub session_id: Uuid,
    pub user_id: i64,
    pub organization_id: Uuid,
    pub device_session_id: Option<i64>, // Optional ID if the session is associated with a device

    // Basic user information
    pub user_name: String,
    pub display_name: String,
    pub organization_name: String,

    // Contact information (Use only if it is verified)
    pub primary_email: String,
    pub is_email_verified: bool,

    // Security and access information
    pub encryption_key: String,
    pub allow_refresh: bool,    // Indicates if the session can be refreshed
    pub allow_device_logins: bool,

    pub allowed_linux_users: Vec<String>,
    pub permissions: Vec<String>,
    pub tags: Vec<i64>,

    pub can_approve_approvals: bool,
    pub can_bypass_approvals: bool,
}


#[derive(RowFrom, Serialize)]
pub struct OrgInfo {
    pub organization_id: Uuid,
    pub organization_name: String,
    pub organization_info: JsonValue,

    pub allocated_ai_credit: i64,
    pub utilized_ai_credit: i64,

    pub allocated_storage_bytes: i64,
    pub utilized_storage_bytes: i64,

    pub user_limit: i32,
    pub current_user_count: i32,

    pub server_limit: i32,
    // pub current_server_count: i32,

    pub workflow_limit: i32,
    // pub current_workflow_count: i32,

    pub ss_project_limit: i32,
    pub current_ss_project_count: i32,

    // This is per project limit for Vault secret store
    pub ss_keys_limit: i32,
    pub ss_notes_limit: i32,

    pub created_at: ChronoUtc,
}


#[derive(RowFrom, Serialize)]
pub struct UserProfileInfo {
    pub user_id: i64,
    pub organization_id: Uuid,

    // Basic user information
    pub user_name: String,
    pub display_name: String,
    pub organization_name: String,

    // Contact information (Use only if it is verified)
    pub primary_email: String,
    pub is_email_verified: bool,
    pub is_user_active: bool,
    pub is_organization_active: bool,

    // Permissions and access control
    pub allowed_linux_users: Vec<String>,
    pub permissions: Vec<String>,
    pub tags: Vec<i64>,

    pub can_approve_approvals: bool,
    pub can_bypass_approvals: bool,
}


#[derive(RowFrom, Serialize)]
pub struct UserProfileListing {
    pub user_id: i64,

    // Basic user information
    pub user_name: String,
    pub display_name: String,
    pub user_details: JsonValue,

    // Contact information (Use only if it is verified)
    pub primary_email: String,
    pub is_email_verified: bool,
    pub is_active: bool,

    // Security and access control
    pub role_id: i64,

    // Geo and IP restrictions + Enforcements policy
    pub access_restriction_policy_id: Option<i64>,

    // Two-factor authentication status
    pub has_totp_2fa: bool,
    pub has_email_2fa: bool,

    // Timestamps for record tracking
    pub created_at: ChronoUtc,
    pub updated_at: ChronoUtc,
    pub last_login_at: ChronoUtc,
}


// ------- Implementations ------- //


impl SessionUser {
    /// Creates a new `SessionUser` with a unique session ID
    pub fn new(auth_info: AuthInfoDbDto, allow_refresh: bool, device_session_id: Option<i64>) -> Self {
        SessionUser {
            session_id: Uuid::new_v4(),
            user_id: auth_info.user_id,
            organization_id: auth_info.organization_id,

            allow_refresh,
            allow_device_logins: auth_info.allow_device_logins,
            encryption_key: auth_info.encryption_key,
            device_session_id,

            allowed_linux_users: auth_info.allowed_linux_users,
            permissions: auth_info.permissions,
            tags: auth_info.tags,

            can_approve_approvals: auth_info.can_approve_approvals,
            can_bypass_approvals: auth_info.can_bypass_approvals,

            user_name: auth_info.user_name,
            display_name: auth_info.display_name,
            organization_name: auth_info.organization_name,

            primary_email: auth_info.primary_email,
            is_email_verified: auth_info.is_email_verified,
        }
    }

    /// Check if the session has all the required permissions
    pub fn has_all_permissions(&self, required_permissions: &[&str]) -> ServiceResult<()> {
        if required_permissions.iter().all(|&perm| self.permissions.contains(&perm.to_string())) {
            Ok(())
        } else {
            Err(ServiceError::Forbidden("E221: Insufficient permissions".into()))
        }
    }

    /// Check if the session has tag with the specified ID
    pub async fn has_tag_permission(&self, state: &AppState, tag_id: i64) -> ServiceResult<()> {
        // If there are no tags then all tags are considered accessible
        if self.tags.is_empty() {
            // So, fetch all the tags of the organization from the cache or database
            let org_tags = fetch_org_tag_ids(state, self.organization_id).await?;

            // if org_tags are also empty, it means there are no tags in the organization
            if org_tags.is_empty() {
                return Ok(());
            }

            // If there are organization tags, but the user has no specific tags, consider all org tags accessible
            // Check if the requested tag is part of the organization tags
            if org_tags.contains(&tag_id) {
                return Ok(());
            } else {
                return Err(ServiceError::Forbidden("E222: Insufficient tag access".into()));
            }
        }

        // Check if the tag is present in the user's accessible tags
        if self.tags.contains(&tag_id) {
            Ok(())
        } else {
            Err(ServiceError::Forbidden("E222: Insufficient tag access".into()))
        }
    }

    /// Return the tags that the session user has access to only
    pub fn accessible_tags(&self, all_tags: &[i64]) -> Vec<i64> {
        if self.tags.is_empty() {
            return all_tags.to_vec();
        }

        all_tags.iter()
            .filter(|&&tag_id| self.tags.contains(&tag_id))
            .cloned()
            .collect()
    }
}
