use pg_row_derive::RowFrom;
use serde::Serialize;


type ChronoUtc = chrono::DateTime<chrono::Utc>;



#[derive(Serialize, RowFrom)]
pub struct UserRole {
    pub role_id: i64,

    pub role_name: String,
    pub role_description: String,

    pub permissions: Vec<String>,
    pub allowed_linux_users: Vec<String>,
    pub allowed_tags: Vec<i64>,

    pub can_bypass_approvals: bool,
    pub can_approve_approvals: bool,

    pub created_at: ChronoUtc,
    pub updated_at: ChronoUtc,
}


pub trait RolePermissions {
    fn permissions(&self) -> &[String];
    fn allowed_linux_users(&self) -> &[String];
    fn allowed_tags(&self) -> &[i64];

    fn can_bypass_approvals(&self) -> bool;
    fn can_approve_approvals(&self) -> bool;
}


// ------- Implementations ------- //


impl RolePermissions for UserRole {
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
