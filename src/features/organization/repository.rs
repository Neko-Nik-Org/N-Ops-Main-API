use super::dto::{ListOrgsDto, CreateOrgDto, UpdateOrgQuotaDto};
use crate::errors::{PgResult, PgPool};
use super::model::OrganizationInfo;
use serde_json::Value as JsonValue;
use uuid::Uuid;



pub async fn get_organization(db_pool: &PgPool, organization_id: Uuid) -> PgResult<Option<OrganizationInfo>> {
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
            ORDER BY organization_name ASC
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

    let default_zero: i64 = 0;
    let new_org_id = uuid::Uuid::new_v4();

    let result = client
        .execute(
            r#"
            INSERT INTO organization (
                organization_id,
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
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            "#,
            &[
                &new_org_id,
                &encryption_key,    // This is encrypted with the master encryption key
                &org_data.organization_name,
                &org_data.organization_info,
                &org_data.billing_info,
                &org_data.allocated_ai_credit,
                &default_zero,
                &org_data.allocated_storage_bytes,
                &default_zero,
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


pub async fn delete_organization(db_pool: &PgPool, organization_id: Uuid) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM organization
            WHERE organization_id = $1
            "#,
            &[&organization_id],
        )
        .await?;

    Ok(result)
}


pub async fn organization_state_change(db_pool: &PgPool, organization_id: Uuid, is_active: bool) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE organization
            SET is_active = $1
            WHERE organization_id = $2
            "#,
            &[&is_active, &organization_id],
        )
        .await?;

    Ok(result)
}


pub async fn update_organization_billing_info(db_pool: &PgPool, organization_id: Uuid, billing_info: &JsonValue) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE organization
            SET billing_info = $1
            WHERE organization_id = $2
            "#,
            &[&billing_info, &organization_id],
        )
        .await?;

    Ok(result)
}


pub async fn update_organization_info(db_pool: &PgPool, organization_id: Uuid, organization_info: &JsonValue) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE organization
            SET organization_info = $1
            WHERE organization_id = $2
            "#,
            &[&organization_info, &organization_id],
        )
        .await?;

    Ok(result)
}


pub async fn quota_update(db_pool: &PgPool, organization_id: Uuid, new_quota: &UpdateOrgQuotaDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE organization
            SET
                allocated_ai_credit = $2,
                allocated_storage_bytes = $3,
                user_limit = $4,
                server_limit = $5,
                workflow_limit = $6,
                ss_project_limit = $7,
                ss_keys_limit = $8,
                ss_notes_limit = $9
            WHERE
                organization_id = $1

                -- All quota values must be at least 1
                AND $2 >= 1
                AND $3 >= 1
                AND $4 >= 1
                AND $5 >= 1
                AND $6 >= 1
                AND $7 >= 1
                AND $8 >= 1
                AND $9 >= 1

                -- AI credits cannot go below utilized credits
                AND $2 >= utilized_ai_credit

                -- Storage cannot go below utilized storage
                AND $3 >= utilized_storage_bytes

                -- User limit cannot go below existing users
                AND $4 >= (
                    SELECT COUNT(*)
                    FROM app_user
                    WHERE organization_id = $1
                )

                -- Project limit cannot go below existing projects
                AND $7 >= (
                    SELECT COUNT(*)
                    FROM vault_project
                    WHERE organization_id = $1
                )

                -- TODO: Set similar checks for server_limit, workflow_limit,
                -- ss_keys_limit, and ss_notes_limit to ensure
                -- they do not go below existing usage
            "#,
            &[
                &organization_id,
                &new_quota.allocated_ai_credit,
                &new_quota.allocated_storage_bytes,
                &new_quota.user_limit,
                &new_quota.server_limit,
                &new_quota.workflow_limit,
                &new_quota.ss_project_limit,
                &new_quota.ss_keys_limit,
                &new_quota.ss_notes_limit,
            ],
        )
        .await?;


    Ok(result)
}


pub async fn name_update(db_pool: &PgPool, organization_id: Uuid, new_name: &str) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE organization
            SET organization_name = $2
            WHERE organization_id = $1
            "#,
            &[
                &organization_id,
                &new_name,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn check_org_name(db_pool: &PgPool, organization_name: &str) -> PgResult<bool> {
    let client = db_pool.get().await?;

    let row = client
        .query_one(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM organization
                WHERE organization_name = $1
            )
            "#,
            &[&organization_name],
        )
        .await?;

    Ok(row.get(0))
}


pub async fn get_encryption_key(db_pool: &PgPool, organization_id: &Uuid) -> PgResult<Option<String>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT encryption_key
            FROM organization
            WHERE
                organization_id = $1
                AND is_active = TRUE
            "#,
            &[&organization_id],
        )
        .await?;

    Ok(row.map(|r| r.get("encryption_key")))
}
