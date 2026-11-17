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


#[derive(Deserialize)]
pub struct UpdateRoleInfoDto {
    pub role_name: String,
    pub role_description: String,
}


#[derive(Deserialize)]
pub struct UpdateRolePermissionsDto {
    pub permissions: Vec<String>,
    pub allowed_linux_users: Vec<String>,
    pub allowed_tags: Vec<i64>,

    pub can_approve_approvals: bool,
    pub can_bypass_approvals: bool,
}


#[derive(Deserialize)]
pub struct ListRolesDto {
    pub role_name_search: Option<String>,
    pub limit: i64,
    pub offset: i64,
}


// ------- Implementations ------- //


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


impl RolePermissions for UpdateRolePermissionsDto {
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
