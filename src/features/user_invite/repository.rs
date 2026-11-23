use super::dto::{ListInvitesDto, CreatePolicyDto, UpdatePolicyBaseInfoDto, UpdatePolicyDto};
use crate::errors::{PgResult, PgPool};
use super::model::InviteInfo;



// pub async fn get_policy(db_pool: &PgPool, organization_id: uuid::Uuid, policy_id: i64) -> PgResult<Option<PolicyInfo>> {
//     let client = db_pool.get().await?;

//     let row = client
//         .query_opt(
//             r#"
//             SELECT
//                 policy_id,
//                 policy_name,
//                 policy_description,
//                 enforce_totp_2fa,
//                 enforce_email_2fa,
//                 has_ip_restriction,
//                 has_geo_restriction,
//                 allowed_ips,
//                 allowed_geo_locations
//             FROM access_restriction_policy
//             WHERE
//                 organization_id = $1
//                 AND policy_id = $2
//             "#,
//             &[&organization_id, &policy_id],
//         )
//         .await?;

//     Ok(row.map(PolicyInfo::from))
// }


pub async fn list_invites(db_pool: &PgPool, organization_id: uuid::Uuid, list_invites_dto: ListInvitesDto) -> PgResult<Vec<InviteInfo>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT
                invitation_id,
                email,
                invitation_notes,
                role_id,
                access_restriction_policy_id,
                invited_at,
                expires_at
            FROM organization_invitation
            WHERE
                organization_id = $1
                AND ($2::text IS NULL OR email ILIKE '%' || $2 || '%')
            LIMIT $3
            OFFSET $4
            "#,
            &[
                &organization_id,
                &list_invites_dto.email,
                &list_invites_dto.limit,
                &list_invites_dto.offset,
            ],
        )
        .await?;

    Ok(InviteInfo::from_rows(rows))
}


// pub async fn create_policy(db_pool: &PgPool, organization_id: uuid::Uuid, create_policy_dto: CreatePolicyDto) -> PgResult<u64> {
//     let client = db_pool.get().await?;

//     let result = client
//         .execute(
//             r#"
//             INSERT INTO access_restriction_policy (
//                 organization_id,
//                 policy_name,
//                 policy_description,
//                 enforce_totp_2fa,
//                 enforce_email_2fa,
//                 has_ip_restriction,
//                 has_geo_restriction,
//                 allowed_ips,
//                 allowed_geo_locations
//             ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
//             "#,
//             &[
//                 &organization_id,
//                 &create_policy_dto.policy_name,
//                 &create_policy_dto.description,
//                 &create_policy_dto.enforce_totp_2fa,
//                 &create_policy_dto.enforce_email_2fa,
//                 &create_policy_dto.set_ip_restriction,
//                 &create_policy_dto.set_geo_restriction,
//                 &create_policy_dto.allowed_ips,
//                 &create_policy_dto.allowed_geo_locations,
//             ],
//         )
//         .await?;

//     Ok(result)
// }


// pub async fn count_policy_associations(db_pool: &PgPool, organization_id: uuid::Uuid, policy_id: i64) -> PgResult<i64> {
//     let client = db_pool.get().await?;

//     let row = client
//         .query_one(
//             r#"
//             SELECT COUNT(*) AS association_count
//             FROM app_user au
//             JOIN access_restriction_policy arp
//                 ON arp.policy_id = au.access_restriction_policy_id
//             WHERE arp.organization_id = $1
//               AND arp.policy_id = $2
//             "#,
//             &[&organization_id, &policy_id],
//         )
//         .await?;

//     Ok(row.get("association_count"))
// }


// pub async fn delete_policy(db_pool: &PgPool, organization_id: uuid::Uuid, policy_id: i64) -> PgResult<u64> {
//     let client = db_pool.get().await?;

//     let result = client
//         .execute(
//             r#"
//             DELETE FROM access_restriction_policy
//             WHERE organization_id = $1
//               AND policy_id = $2
//             "#,
//             &[&organization_id, &policy_id],
//         )
//         .await?;

//     Ok(result)
// }


// pub async fn update_policy_base_info(db_pool: &PgPool, organization_id: uuid::Uuid, policy_id: i64, name_info: UpdatePolicyBaseInfoDto) -> PgResult<u64> {
//     let client = db_pool.get().await?;

//     let result = client
//         .execute(
//             r#"
//             UPDATE access_restriction_policy
//             SET policy_name = $1,
//                 policy_description = $2
//             WHERE organization_id = $3
//               AND policy_id = $4
//             "#,
//             &[
//                 &name_info.policy_name,
//                 &name_info.description,
//                 &organization_id,
//                 &policy_id,
//             ],
//         )
//         .await?;

//     Ok(result)
// }


// pub async fn update_full_policy(db_pool: &PgPool, organization_id: uuid::Uuid, policy_id: i64, full_policy_info: UpdatePolicyDto) -> PgResult<u64> {
//     let client = db_pool.get().await?;

//     let result = client
//         .execute(
//             r#"
//             UPDATE access_restriction_policy
//             SET enforce_totp_2fa = $1,
//                 enforce_email_2fa = $2,
//                 set_ip_restriction = $3,
//                 set_geo_restriction = $4,
//                 allowed_ips = $5,
//                 allowed_geo_locations = $6
//             WHERE organization_id = $7
//               AND policy_id = $8
//             "#,
//             &[
//                 &full_policy_info.enforce_totp_2fa,
//                 &full_policy_info.enforce_email_2fa,
//                 &full_policy_info.set_ip_restriction,
//                 &full_policy_info.set_geo_restriction,
//                 &full_policy_info.allowed_ips,
//                 &full_policy_info.allowed_geo_locations,
//                 &organization_id,
//                 &policy_id,
//             ],
//         )
//         .await?;

//     Ok(result)
// }
