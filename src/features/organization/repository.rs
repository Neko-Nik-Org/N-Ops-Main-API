use super::dto::{ListOrgsDto, CreateOrgDto};
use crate::errors::{PgResult, PgPool};
use super::model::OrganizationInfo;



pub async fn get_organization(db_pool: &PgPool, organization_id: uuid::Uuid) -> PgResult<Option<OrganizationInfo>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                organization_id,
                organization_name,
                organization_info,
                billing_info,
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
                billing_info,
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


pub async fn create_organization(db_pool: &PgPool, encryption_key: &str, org_data: &CreateOrgDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO organization (
                encryption_key,
                organization_name,
                organization_info,
                billing_info,
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
                ss_notes_limit
            )
            VALUES ($1, $2, $3, $4, $5, 0::BIGINT, $6, 0::BIGINT, $7, $8, $9, $10, $11, $12, $13)
            "#,
            &[
                &encryption_key,    // This is encrypted with the master encryption key
                &org_data.organization_name,
                &org_data.organization_info,
                &org_data.billing_info,
                &org_data.allocated_ai_credit,
                &org_data.allocated_storage_bytes,
                &org_data.is_active,
                &org_data.user_limit,
                &org_data.server_limit,
                &org_data.workflow_limit,
                &org_data.ss_project_limit,
                &org_data.ss_keys_limit,
                &org_data.ss_notes_limit,
            ],
        )
        .await?;

    Ok(result)
}
