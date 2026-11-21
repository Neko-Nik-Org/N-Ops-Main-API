use pg_row_derive::RowFrom;
use serde::Serialize;



#[derive(Serialize, RowFrom)]
pub struct PolicyInfo {
    pub policy_id: i64,

    pub policy_name: String,
    pub policy_description: String,

    pub enforce_totp_2fa: bool,
    pub enforce_email_2fa: bool,

    pub has_ip_restriction: bool,
    pub has_geo_restriction: bool,

    pub allowed_ips: Vec<String>,
    pub allowed_geo_locations: Vec<String>,
}
