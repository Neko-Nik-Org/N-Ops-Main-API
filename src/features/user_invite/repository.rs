use super::dto::{ListInvitesDto, NewInviteDto};
use crate::errors::{PgResult, PgPool};
use super::model::InviteInfo;



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


pub async fn create_invite(db_pool: &PgPool, organization_id: uuid::Uuid, invitation_code_hash: String, new_invite: &NewInviteDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
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
