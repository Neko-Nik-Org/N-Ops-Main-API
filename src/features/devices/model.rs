use pg_row_derive::RowFrom;
use serde::Serialize;

type ChronoUtc = chrono::DateTime<chrono::Utc>;



#[derive(Serialize, RowFrom)]
pub struct DeviceSession {
    pub session_id: i64,
    
    pub device_name: String,
    pub device_type: String,
    pub device_identifier: String,

    pub device_details: serde_json::Value,

    pub created_at: ChronoUtc,
    pub last_accessed_at: ChronoUtc,
}
