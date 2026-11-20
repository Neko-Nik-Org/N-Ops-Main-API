use serde::Deserialize;



#[derive(Deserialize)]
pub struct ListOrgsDto {
    pub organization_name: Option<String>,
    pub limit: i64,
    pub offset: i64,
}
