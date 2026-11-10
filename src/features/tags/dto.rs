use serde::Deserialize;



#[derive(Deserialize)]
pub struct CreateResourceTagDto {
    pub tag_name: String,
    pub tag_description: String,
}


#[derive(Deserialize)]
pub struct EditResourceTagDto {
    pub tag_id: i64,
    pub tag_name: String,
    pub tag_description: String,
}
