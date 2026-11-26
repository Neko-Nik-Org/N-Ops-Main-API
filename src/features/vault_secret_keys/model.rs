use pg_row_derive::RowFrom;
use serde::Serialize;

type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(RowFrom, Serialize)]
pub struct VaultKeysListing {
    pub secret_id: i64,
    pub secret_name: String,

    pub current_version: i64,
    pub is_hidden: bool,    // If the current version is hidden or visible in the UI

    pub created_at: ChronoUtc,
    pub updated_at: ChronoUtc,
}
