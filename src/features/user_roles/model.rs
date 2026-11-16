use pg_row_derive::RowFrom;
use serde::Serialize;
use uuid::Uuid;



pub trait RolePermissions {
    fn permissions(&self) -> &[String];
    fn allowed_linux_users(&self) -> &[String];
    fn allowed_tags(&self) -> &[i64];

    fn can_bypass_approvals(&self) -> bool;
    fn can_approve_approvals(&self) -> bool;
}


#[derive(Serialize, RowFrom)]
pub struct ResourceTag {
    pub tag_id: i64,
    pub organization_id: Uuid,

    pub tag_name: String,
    pub tag_description: String,
}
