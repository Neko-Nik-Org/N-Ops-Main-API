use super::dto::{ListInvitesDto, NewInviteDto};
use crate::errors::{PgResult, PgPool, PgError};
use super::model::InviteInfo;



pub async fn list_invites(db_pool: &PgPool, organization_id: uuid::Uuid, list_invites_dto: ListInvitesDto) -> PgResult<Vec<InviteInfo>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT
                invitation_id,
                organization_id,
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


pub async fn create_invite(db_pool: &PgPool, organization_id: uuid::Uuid, invitation_code_hash: String, new_invite: &NewInviteDto) -> PgResult<u64> {
    let mut client = db_pool.get().await?;

    // Start transaction
    let transaction = client.transaction().await?;

    // 1. Ensure the role & access restriction policy belong to this organization
    let belongs_to_org = transaction
        .query_opt(
            r#"
            SELECT 1
            FROM user_role ur
            LEFT JOIN access_restriction_policy arp
                ON arp.organization_id = ur.organization_id
               AND arp.policy_id = $2
            WHERE ur.organization_id = $1
              AND ur.role_id = $3
              AND (
                  $2::BIGINT IS NULL
                  OR arp.policy_id IS NOT NULL
              )
            "#,
            &[
                &organization_id,
                &new_invite.access_restriction_policy_id,
                &new_invite.role_id,
            ],
        )
        .await?;

    if belongs_to_org.is_none() {
        transaction.rollback().await?;

        return Err(PgError::Custom(
            "Role or access restriction policy does not belong to the organization".into(),
        ));
    }

    // 2. Create invitation
    let result = transaction
        .execute(
            r#"
            INSERT INTO organization_invitation (
                organization_id,
                email,
                invitation_notes,
                role_id,
                access_restriction_policy_id,
                invitation_code_hash,
                expires_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            &[
                &organization_id,
                &new_invite.invite_to_email,
                &new_invite.invitation_notes,
                &new_invite.role_id,
                &new_invite.access_restriction_policy_id,
                &invitation_code_hash,
                &new_invite.expiration_datetime,
            ],
        )
        .await?;

    // 3. Commit transaction
    transaction.commit().await?;

    Ok(result)
}


pub async fn delete_old_invites(db_pool: &PgPool) -> PgResult<()> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM organization_invitation
            WHERE expires_at <= NOW()
            "#,
            &[],
        )
        .await?;

    tracing::info!("Cleaned up {} expired invites", result);

    Ok(())
}


pub async fn check_invite_possible(db_pool: &PgPool, email: &str) -> PgResult<bool> {
    let client = db_pool.get().await?;

    let row = client
        .query_one(
            r#"
            SELECT
                NOT EXISTS (
                    SELECT 1
                    FROM app_user
                    WHERE primary_email = $1
                )
                AND NOT EXISTS (
                    SELECT 1
                    FROM organization_invitation
                    WHERE email = $1
                ) AS invite_possible
            "#,
            &[&email],
        )
        .await?;

    Ok(row.get("invite_possible"))
}


pub async fn delete_invitation(db_pool: &PgPool, organization_id: uuid::Uuid, invitation_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM organization_invitation
            WHERE organization_id = $1
                AND invitation_id = $2
            "#,
            &[&organization_id, &invitation_id],
        )
        .await?;

    Ok(result)
}


pub async fn get_invite_details_by_code(db_pool: &PgPool, invitation_code_hash: &str) -> PgResult<Option<InviteInfo>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                invitation_id,
                organization_id,
                email,
                invitation_notes,
                role_id,
                access_restriction_policy_id,
                invited_at,
                expires_at
            FROM organization_invitation
            WHERE invitation_code_hash = $1
                AND expires_at > NOW()
            "#,
            &[&invitation_code_hash],
        )
        .await?;

    Ok(row.map(InviteInfo::from))
}
