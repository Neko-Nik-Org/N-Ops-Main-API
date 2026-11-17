use super::dto::{CreateNewRoleDto, ListRolesDto, UpdateRoleInfoDto, UpdateRolePermissionsDto};
use deadpool_postgres::Pool as PgPool;
use crate::errors::PgResult;
use super::model::UserRole;
use uuid::Uuid;



pub async fn create_new_role(db_pool: &PgPool, organization_id: Uuid, role_data: &CreateNewRoleDto) -> PgResult<()> {
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


pub async fn list_roles(db_pool: &PgPool, organization_id: Uuid, query: &ListRolesDto) -> PgResult<Vec<UserRole>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT
                ur.role_id,
                ur.role_name,
                ur.role_description,
                ur.permissions,
                ur.allowed_linux_users,
                COALESCE(
                    ARRAY_AGG(rt.tag_id) FILTER (WHERE rt.role_id IS NOT NULL),
                    ARRAY[]::BIGINT[]
                ) AS allowed_tags,
                ur.can_bypass_approvals,
                ur.can_approve_approvals,
                ur.created_at,
                ur.updated_at
            FROM user_role ur
            LEFT JOIN role_tag rt ON ur.role_id = rt.role_id
            WHERE ur.organization_id = $1
                AND (
                    $2::TEXT IS NULL
                    OR ur.role_name ILIKE '%' || $2 || '%'
                )
            GROUP BY ur.role_id
            ORDER BY ur.created_at DESC
            LIMIT $3 OFFSET $4
            "#,
            &[
                &organization_id,
                &query.role_name_search,
                &query.limit,
                &query.offset,
            ],
        )
        .await?;

    Ok(UserRole::from_rows(rows))
}


pub async fn fetch_role(db_pool: &PgPool, organization_id: Uuid, role_id: i64) -> PgResult<UserRole> {
    let client = db_pool.get().await?;

    let row = client
        .query_one(
            r#"
            SELECT
                ur.role_id,
                ur.role_name,
                ur.role_description,
                ur.permissions,
                ur.allowed_linux_users,
                COALESCE(
                    ARRAY_AGG(rt.tag_id) FILTER (WHERE rt.role_id IS NOT NULL),
                    ARRAY[]::BIGINT[]
                ) AS allowed_tags,
                ur.can_bypass_approvals,
                ur.can_approve_approvals,
                ur.created_at,
                ur.updated_at
            FROM user_role ur
            LEFT JOIN role_tag rt ON ur.role_id = rt.role_id
            WHERE ur.organization_id = $1
                AND ur.role_id = $2
            GROUP BY ur.role_id
            "#,
            &[
                &organization_id,
                &role_id,
            ],
        )
        .await?;

    Ok(UserRole::from(row))
}


pub async fn can_delete_safely(db_pool: &PgPool, organization_id: Uuid, role_id: i64) -> PgResult<bool> {
    let client = db_pool.get().await?;

    let row = client
        .query_one(
            r#"
            SELECT COUNT(*) = 0 AS can_delete_safely
            FROM app_user au
            WHERE au.organization_id = $1
                AND au.role_id = $2
            "#,
            &[
                &organization_id,
                &role_id,
            ],
        )
        .await?;

    Ok(row.get("can_delete_safely"))
}


pub async fn delete_role(db_pool: &PgPool, organization_id: Uuid, role_id: i64) -> PgResult<()> {
    let client = db_pool.get().await?;

    client
        .execute(
            r#"
            DELETE FROM user_role
            WHERE organization_id = $1
                AND role_id = $2
            "#,
            &[
                &organization_id,
                &role_id,
            ],
        )
        .await?;

    Ok(())
}


pub async fn update_role_info(db_pool: &PgPool, organization_id: Uuid, role_id: i64, update_data: &UpdateRoleInfoDto) -> PgResult<()> {
    let client = db_pool.get().await?;

    client
        .execute(
            r#"
            UPDATE user_role
            SET role_name = $1,
                role_description = $2,
                updated_at = NOW()
            WHERE organization_id = $3
                AND role_id = $4
            "#,
            &[
                &update_data.role_name,
                &update_data.role_description,
                &organization_id,
                &role_id,
            ],
        )
        .await?;

    Ok(())
}


pub async fn update_role_permissions(
    db_pool: &PgPool,
    organization_id: Uuid,
    role_id: i64,
    update_data: &UpdateRolePermissionsDto,
) -> PgResult<()> {
    let mut client = db_pool.get().await?;

    let transaction = client.transaction().await?;

    // First Update all the basic single table role fields update
    transaction
        .execute(
            r#"
            UPDATE user_role
            SET permissions = $1,
                allowed_linux_users = $2,
                can_approve_approvals = $3,
                can_bypass_approvals = $4,
                updated_at = NOW()
            WHERE organization_id = $5
                AND role_id = $6
            "#,
            &[
                &update_data.permissions,
                &update_data.allowed_linux_users,
                &update_data.can_approve_approvals,
                &update_data.can_bypass_approvals,
                &organization_id,
                &role_id,
            ],
        )
        .await?;

    // For tags, there can be 2 cases
    // One is empty (means super-admin has access to all tags)
    // The other case is when there are specific tags assigned to the role
    // In case of specific tags, make sure only those tags are assigned to the role, else all remove or add accordingly
    // Remove existing tag mappings

    // Remove all existing tag mappings for the role before inserting the new ones
    transaction
        .execute(
            r#"
            DELETE FROM role_tag
            WHERE role_id = $1
            "#,
            &[&role_id],
        )
        .await?;

    // Insert the new tag mappings (if any)
    if !update_data.allowed_tags.is_empty() {
        transaction
            .execute(
                r#"
                INSERT INTO role_tag (role_id, tag_id)
                SELECT $1, rt.tag_id
                FROM resource_tag rt
                WHERE rt.organization_id = $2
                    AND rt.tag_id = ANY($3)
                "#,
                &[&role_id, &organization_id, &update_data.allowed_tags],
            )
            .await?;
    }

    // Commit the transaction after all updates are done
    transaction.commit().await?;

    Ok(())
}
