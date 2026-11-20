use serde_json::Value as JsonValue;
use pg_row_derive::RowFrom;
use serde::Serialize;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;



#[derive(Serialize, RowFrom)]
pub struct OrganizationInfo {
    pub organization_id: Uuid,
    pub organization_name: String,

    pub organization_info: JsonValue,
    pub billing_info: JsonValue,
    
    pub allocated_ai_credit: i64,
    pub utilized_ai_credit: i64,

    pub allocated_storage_bytes: i64,
    pub utilized_storage_bytes: i64,

    pub is_active: bool,

    pub user_limit: i32,
    pub server_limit: i32,
    pub workflow_limit: i32,

    pub ss_project_limit: i32,
    pub ss_keys_limit: i32,
    pub ss_notes_limit: i32,

    pub created_at: ChronoUtc,
}
