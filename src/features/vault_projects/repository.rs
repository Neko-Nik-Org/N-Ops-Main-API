use super::dto::{VaultProjectDto, ListQueryDto};
use crate::errors::{PgResult, PgError, PgPool};
use super::model::VaultProject;
use uuid::Uuid;



pub async fn create_new_project(db_pool: &PgPool, organization_id: Uuid, project_data: &VaultProjectDto) -> PgResult<u64> {
    let mut client = db_pool.get().await?;
    let transaction = client.transaction().await?;

    // 1. Lock organization and retrieve project quota
    let org = transaction
        .query_opt(
            r#"
            SELECT ss_project_limit
            FROM organization
            WHERE organization_id = $1
            FOR UPDATE
            "#,
            &[&organization_id],
        )
        .await?;

    let Some(org) = org else {
        transaction.rollback().await?;
        return Err(PgError::Custom("Organization not found".into()));
    };

    let project_limit: i32 = org.get("ss_project_limit");

    // 2. Count existing projects
    let current_projects: i64 = transaction
        .query_one(
            r#"
            SELECT COUNT(*)::BIGINT
            FROM vault_project
            WHERE organization_id = $1
            "#,
            &[&organization_id],
        )
        .await?
        .get(0);

    // 3. Check project quota
    if current_projects >= project_limit as i64 {
        transaction.rollback().await?;
        return Err(PgError::Custom("Project limit reached".into()));
    }

    // 4. Insert project
    let result = transaction
        .execute(
            r#"
            INSERT INTO vault_project (
                organization_id,
                project_name,
                details
            )
            VALUES ($1, $2, $3)
            "#,
            &[
                &organization_id,
                &project_data.project_name,
                &project_data.details,
            ],
        )
        .await?;

    // 5. Commit transaction
    transaction.commit().await?;

    Ok(result)
}


pub async fn update_project(db_pool: &PgPool, organization_id: Uuid, project_id: i64, project_data: &VaultProjectDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE vault_project
            SET project_name = $1, details = $2, updated_at = NOW()
            WHERE organization_id = $3 AND project_id = $4
            "#,
            &[&project_data.project_name, &project_data.details, &organization_id, &project_id],
        )
        .await?;

    Ok(result)
}


pub async fn list_projects(db_pool: &PgPool, organization_id: Uuid, query: &ListQueryDto) -> PgResult<Vec<VaultProject>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT
                project_id,
                project_name,
                details,
                created_at,
                updated_at
            FROM vault_project
            WHERE organization_id = $1
                AND (
                    $2::TEXT IS NULL
                    OR project_name ILIKE '%' || $2 || '%'
                )
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
            &[
                &organization_id,
                &query.project_name,
                &query.limit,
                &query.offset,
            ]
        )
        .await?;

    Ok(VaultProject::from_rows(rows))
}


pub async fn get_project_detail(db_pool: &PgPool, organization_id: Uuid, project_id: i64) -> PgResult<Option<VaultProject>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                project_id,
                organization_id,
                project_name,
                details,
                created_at,
                updated_at
            FROM vault_project
            WHERE organization_id = $1 AND project_id = $2
            "#,
            &[&organization_id, &project_id],
        )
        .await?;

    Ok(row.map(VaultProject::from))
}


pub async fn delete_project(db_pool: &PgPool, organization_id: Uuid, project_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM vault_project
            WHERE organization_id = $1 AND project_id = $2
            "#,
            &[&organization_id, &project_id],
        )
        .await?;

    Ok(result)
}
