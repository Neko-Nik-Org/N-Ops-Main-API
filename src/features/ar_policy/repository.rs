use super::dto::{ListPolicyDto, CreatePolicyDto};
use crate::errors::{PgResult, PgPool};
use super::model::PolicyInfo;



pub async fn get_policy(db_pool: &PgPool, organization_id: uuid::Uuid, policy_id: i64) -> PgResult<Option<PolicyInfo>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                policy_id,
                policy_name,
                policy_description,
                enforce_totp_2fa,
                enforce_email_2fa,
                has_ip_restriction,
                has_geo_restriction,
                allowed_ips,
                allowed_geo_locations
            FROM access_restriction_policy
            WHERE
                organization_id = $1
                AND policy_id = $2
            "#,
            &[&organization_id, &policy_id],
        )
        .await?;

    Ok(row.map(PolicyInfo::from))
}


pub async fn list_policies(db_pool: &PgPool, organization_id: uuid::Uuid, list_policy_dto: ListPolicyDto) -> PgResult<Vec<PolicyInfo>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT
                policy_id,
                policy_name,
                policy_description,
                enforce_totp_2fa,
                enforce_email_2fa,
                has_ip_restriction,
                has_geo_restriction,
                allowed_ips,
                allowed_geo_locations
            FROM access_restriction_policy
            WHERE
                organization_id = $1
                AND ($2::text IS NULL OR policy_name ILIKE '%' || $2 || '%')
            LIMIT $3
            OFFSET $4
            "#,
            &[
                &organization_id,
                &list_policy_dto.policy_name,
                &list_policy_dto.limit,
                &list_policy_dto.offset,
            ],
        )
        .await?;

    Ok(PolicyInfo::from_rows(rows))
}


pub async fn create_policy(db_pool: &PgPool, organization_id: uuid::Uuid, create_policy_dto: CreatePolicyDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO access_restriction_policy (
                organization_id,
                policy_name,
                policy_description,
                enforce_totp_2fa,
                enforce_email_2fa,
                has_ip_restriction,
                has_geo_restriction,
                allowed_ips,
                allowed_geo_locations
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            &[
                &organization_id,
                &create_policy_dto.policy_name,
                &create_policy_dto.description,
                &create_policy_dto.enforce_totp_2fa,
                &create_policy_dto.enforce_email_2fa,
                &create_policy_dto.set_ip_restriction,
                &create_policy_dto.set_geo_restriction,
                &create_policy_dto.allowed_ips,
                &create_policy_dto.allowed_geo_locations,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn count_policy_associations(db_pool: &PgPool, organization_id: uuid::Uuid, policy_id: i64) -> PgResult<i64> {
    let client = db_pool.get().await?;

    let row = client
        .query_one(
            r#"
            SELECT COUNT(*) AS association_count
            FROM app_user au
            JOIN access_restriction_policy arp
                ON arp.policy_id = au.access_restriction_policy_id
            WHERE arp.organization_id = $1
              AND arp.policy_id = $2
            "#,
            &[&organization_id, &policy_id],
        )
        .await?;

    Ok(row.get("association_count"))
}


pub async fn delete_policy(db_pool: &PgPool, organization_id: uuid::Uuid, policy_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM access_restriction_policy
            WHERE organization_id = $1
              AND policy_id = $2
            "#,
            &[&organization_id, &policy_id],
        )
        .await?;

    Ok(result)
}
