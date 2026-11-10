use pg_row_derive::RowFrom;
use serde::Serialize;
use uuid::Uuid;



#[derive(Serialize, RowFrom)]
pub struct ResourceTag {
    pub tag_id: i64,
    pub organization_id: Uuid,

    pub tag_name: String,
    pub tag_description: String,
}
