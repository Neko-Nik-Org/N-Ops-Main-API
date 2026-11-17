use super::dto::{CreateNewRoleDto, ListRolesDto};
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
