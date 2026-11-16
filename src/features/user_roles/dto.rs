use super::model::RolePermissions;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct CreateNewRoleDto {
    pub role_name: String,
    pub role_description: String,

    pub permissions: Vec<String>,
    pub allowed_linux_users: Vec<String>,
    pub allowed_tags: Vec<i64>,

    pub can_approve_approvals: bool,
    pub can_bypass_approvals: bool,
}


impl RolePermissions for CreateNewRoleDto {
    fn permissions(&self) -> &[String] {
        &self.permissions
    }

    fn allowed_linux_users(&self) -> &[String] {
        &self.allowed_linux_users
    }

    fn allowed_tags(&self) -> &[i64] {
        &self.allowed_tags
    }

    fn can_bypass_approvals(&self) -> bool {
        self.can_bypass_approvals
    }

    fn can_approve_approvals(&self) -> bool {
        self.can_approve_approvals
    }
}
