use deadpool_postgres::Pool as PgPool;
use super::dto::{CreateNewRoleDto};
use crate::errors::PgResult;



pub async fn create_new_role(db_pool: &PgPool, organization_id: uuid::Uuid, role_data: &CreateNewRoleDto) -> PgResult<()> {
    let mut client = db_pool.get().await?;

    // Start transaction
    let transaction = client.transaction().await?;

    // 1. Create role
    let row = transaction
        .query_one(
            r#"
            INSERT INTO user_role (
                organization_id,
                role_name,
                role_description,
                permissions,
                can_approve_approvals,
                can_bypass_approvals,
                allowed_linux_users
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING role_id
            "#,
            &[
                &organization_id,
                &role_data.role_name,
                &role_data.role_description,
                &role_data.permissions,
                &role_data.can_approve_approvals,
                &role_data.can_bypass_approvals,
                &role_data.allowed_linux_users,
            ],
        )
        .await?;

    let role_id: i64 = row.get("role_id");

    // 2. Insert all associated tags in one query
    if !role_data.allowed_tags.is_empty() {
        transaction
            .execute(
                r#"
                INSERT INTO role_tag (role_id, tag_id)
                SELECT $1, tag_id
                FROM UNNEST($2::BIGINT[]) AS tags(tag_id)
                ON CONFLICT (role_id, tag_id) DO NOTHING
                "#,
                &[
                    &role_id,
                    &role_data.allowed_tags,
                ],
            )
            .await?;
    }

    // 3. Commit transaction
    transaction.commit().await?;

    Ok(())
}
