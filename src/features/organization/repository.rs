use crate::errors::{PgResult, PgPool};
use super::model::OrganizationInfo;
use super::dto::ListOrgsDto;



pub async fn get_organization(db_pool: &PgPool, organization_id: uuid::Uuid) -> PgResult<Option<OrganizationInfo>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                organization_id,
                organization_name,
                organization_info,
                allocated_ai_credit,
                utilized_ai_credit,
                allocated_storage_bytes,
                utilized_storage_bytes,
                is_active,
                user_limit,
                server_limit,
                workflow_limit,
                ss_project_limit,
                ss_keys_limit,
                ss_notes_limit,
                created_at
            FROM organization
            WHERE organization_id = $1
            "#,
            &[&organization_id],
        )
        .await?;

    Ok(row.map(OrganizationInfo::from))
}


pub async fn list_organizations(db_pool: &PgPool, query: ListOrgsDto) -> PgResult<Vec<OrganizationInfo>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT
                organization_id,
                organization_name,
                organization_info,
                allocated_ai_credit,
                utilized_ai_credit,
                allocated_storage_bytes,
                utilized_storage_bytes,
                is_active,
                user_limit,
                server_limit,
                workflow_limit,
                ss_project_limit,
                ss_keys_limit,
                ss_notes_limit,
                created_at
            FROM organization
            WHERE (
                $1::TEXT IS NULL
                OR organization_name ILIKE '%' || $1 || '%'
            )
            LIMIT $2
            OFFSET $3
            "#,
            &[
                &query.organization_name,
                &query.limit,
                &query.offset,
            ],
        )
        .await?;

    Ok(OrganizationInfo::from_rows(rows))
}
