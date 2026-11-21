use serde::Deserialize;



#[derive(Deserialize)]
pub struct ListPolicyDto {
    pub policy_name: Option<String>,    // Filter policies by name (optional)
    pub limit: i64,
    pub offset: i64
}
