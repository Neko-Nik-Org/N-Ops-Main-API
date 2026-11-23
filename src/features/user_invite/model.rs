use pg_row_derive::RowFrom;
use serde::Serialize;


type ChronoUtc = chrono::DateTime<chrono::Utc>;



#[derive(Serialize, RowFrom)]
pub struct InviteInfo {
    pub invitation_id: i64,
    pub organization_id: uuid::Uuid,

    pub email: String,
    pub invitation_notes: String,

    pub role_id: i64,
    pub access_restriction_policy_id: Option<i64>,

    pub invited_at: ChronoUtc,
    pub expires_at: ChronoUtc,
}
