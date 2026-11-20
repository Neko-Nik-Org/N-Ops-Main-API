use serde_json::Value as JsonValue;
use serde::Deserialize;



#[derive(Deserialize)]
pub struct ListOrgsDto {
    pub organization_name: Option<String>,
    pub limit: i64,
    pub offset: i64,
}


#[derive(Deserialize)]
pub struct CreateOrgDto {
    pub organization_name: String,

    pub organization_info: JsonValue,
    pub billing_info: JsonValue,

    pub is_active: bool,
    
    pub allocated_ai_credit: i64,
    pub allocated_storage_bytes: i64,

    pub user_limit: i32,
    pub server_limit: i32,
    pub workflow_limit: i32,

    pub ss_project_limit: i32,
    pub ss_keys_limit: i32,
    pub ss_notes_limit: i32,
}


#[derive(Deserialize)]
pub struct UpdateOrgQuotaDto {
    pub allocated_ai_credit: i64,
    pub allocated_storage_bytes: i64,

    pub user_limit: i32,
    pub server_limit: i32,
    pub workflow_limit: i32,

    pub ss_project_limit: i32,
    pub ss_keys_limit: i32,
    pub ss_notes_limit: i32,
}
