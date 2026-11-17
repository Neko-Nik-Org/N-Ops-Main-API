use serde_json::Value as JsonValue;
use serde::Deserialize;



#[derive(Deserialize)]
pub struct VaultProjectDto {
    pub project_name: String,
    pub details: JsonValue,
}


#[derive(Deserialize)]
pub struct ListQueryDto {
    pub project_name: Option<String>,
    pub limit: i64,
    pub offset: i64
}
