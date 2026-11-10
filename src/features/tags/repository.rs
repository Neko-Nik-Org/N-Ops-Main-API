use super::dto::{CreateResourceTagDto, EditResourceTagDto};
use deadpool_postgres::Pool as PgPool;
use super::model::ResourceTag;
use crate::errors::PgResult;



pub async fn create_new_tag(db_pool: &PgPool, organization_id: uuid::Uuid, tag_data: &CreateResourceTagDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO resource_tag (organization_id, tag_name, tag_description)
            VALUES ($1, $2, $3)
            "#,
            &[&organization_id, &tag_data.tag_name, &tag_data.tag_description],
        )
        .await?;

    Ok(result)
}


pub async fn update_tag_data(db_pool: &PgPool, organization_id: uuid::Uuid, tag_data: &EditResourceTagDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE resource_tag
            SET tag_name = $1, tag_description = $2
            WHERE organization_id = $3 AND tag_id = $4
            "#,
            &[&tag_data.tag_name, &tag_data.tag_description, &organization_id, &tag_data.tag_id],
        )
        .await?;

    Ok(result)
}


pub async fn delete_tag(db_pool: &PgPool, organization_id: uuid::Uuid, tag_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM resource_tag
            WHERE organization_id = $1 AND tag_id = $2
            "#,
            &[&organization_id, &tag_id],
        )
        .await?;

    Ok(result)
}


pub async fn get_tag(db_pool: &PgPool, organization_id: uuid::Uuid, tag_id: i64) -> PgResult<Option<ResourceTag>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT tag_id, organization_id, tag_name, tag_description
            FROM resource_tag
            WHERE organization_id = $1 AND tag_id = $2
            "#,
            &[&organization_id, &tag_id],
        )
        .await?;

    Ok(row.map(ResourceTag::from))
}
