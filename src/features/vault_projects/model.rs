use pg_row_derive::RowFrom;
use serde::Serialize;
use uuid::Uuid;

type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(RowFrom, Serialize)]
pub struct VaultProject {
    pub project_id: i64,
    pub organization_id: Uuid,

    pub project_name: String,
    pub details: serde_json::Value,

    pub created_at: ChronoUtc,
    pub updated_at: ChronoUtc,
}
