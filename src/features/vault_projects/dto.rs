use serde_json::Value as JsonValue;
use serde::Deserialize;



#[derive(Deserialize)]
pub struct VaultProjectDto {
    pub project_name: String,
    pub details: JsonValue,
}
