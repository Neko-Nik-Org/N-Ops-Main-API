use serde::Deserialize;



#[derive(Deserialize)]
pub struct ListPolicyDto {
    pub policy_name: Option<String>,    // Filter policies by name (optional)
    pub limit: i64,
    pub offset: i64
}


#[derive(Deserialize)]
pub struct CreatePolicyDto {
    pub policy_name: String,
    pub description: String,

    pub enforce_totp_2fa: bool,
    pub enforce_email_2fa: bool,

    pub set_ip_restriction: bool,
    pub set_geo_restriction: bool,

    pub allowed_ips: Vec<String>,
    pub allowed_geo_locations: Vec<String>
}
