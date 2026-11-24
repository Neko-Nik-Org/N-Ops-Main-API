use serde::Deserialize;



#[derive(Deserialize)]
pub struct CreateDeviceSessionDto {
    pub device_name: String,
    pub device_type: String,

    pub device_identifier: String,
    pub device_details: serde_json::Value,

    pub session_token_hash: String,
    pub fcm_token: Option<String>,
    pub apns_token: Option<String>,
}
