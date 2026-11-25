use pg_row_derive::RowFrom;
use serde::Deserialize;



#[derive(Deserialize)]
pub struct DeviceLoginReqDto {
    pub user_id: i64,
    pub device_token: String,

    pub device_name: String,
    pub device_type: String,
    pub device_identifier: String,

    pub device_details: serde_json::Value,
}


#[derive(RowFrom)]
pub struct DeviceSessionDto {
    pub user_name: String,
    pub session_token_hash: String,
}
