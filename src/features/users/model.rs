use crate::{errors::{ServiceError, ServiceResult}, routes::auth};
use serde::{Deserialize, Serialize};
use super::dto::AuthInfoDbDto;
use pg_row_derive::RowFrom;
use uuid::Uuid;



#[derive(Serialize, Deserialize, Clone)]
pub struct SessionUser {
    // Unique identifiers for the session and user
    pub session_id: Uuid,
    pub user_id: i64,
    pub organization_id: Uuid,

    // Basic user information
    pub user_name: String,
    pub display_name: String,
    pub organization_name: String,

    // Contact information (Use only if it is verified)
    pub primary_email: String,
    pub is_email_verified: bool,

    // Security and access information
    pub encryption_key: String,
    // pub anti_phishing_secret: String,    // TODO: Implement anti-phishing secret management
    pub allow_refresh: bool,    // Indicates if the session can be refreshed

    pub allowed_linux_users: Vec<String>,
    pub permissions: Vec<String>,
    pub tags: Vec<i64>,

    pub can_approve_approvals: bool,
    pub can_bypass_approvals: bool,
}


#[derive(RowFrom)]
pub struct OrgQuotaLimits {
    pub organization_id: Uuid,

    pub allocated_ai_credit: i64,
    pub utilized_ai_credit: i64,

    pub allocated_storage_bytes: i64,
    pub utilized_storage_bytes: i64,

    pub user_limit: i32,
    pub server_limit: i32,
    pub workflow_limit: i32,

    pub ss_project_limit: i32,
    pub ss_keys_limit: i32,
    pub ss_notes_limit: i32,
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
            Err(ServiceError::Forbidden("Insufficient permissions".into()))
        }
    }

    /// Check if the session has tag with the specified ID
    pub fn has_tag_permission(&self, tag_id: i64) -> ServiceResult<()> {
        // If there are no tags then all tags are considered accessible
        if self.tags.is_empty() {
            return Ok(());
        }

        // Check if the tag is present in the user's accessible tags
        if self.tags.contains(&tag_id) {
            Ok(())
        } else {
            Err(ServiceError::Forbidden("Insufficient tag access".into()))
        }
    }
}
