use deadpool_postgres::Pool as PgPool;
use super::dto::ListPolicyDto;
use super::model::PolicyInfo;
use crate::errors::PgResult;



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
