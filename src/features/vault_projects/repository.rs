use super::dto::{VaultProjectDto, ListQueryDto};
use deadpool_postgres::Pool as PgPool;
use super::model::VaultProject;
use crate::errors::PgResult;
use uuid::Uuid;



pub async fn create_new_project(db_pool: &PgPool, organization_id: Uuid, project_data: &VaultProjectDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO vault_project (organization_id, project_name, details)
            VALUES ($1, $2, $3)
            "#,
            &[&organization_id, &project_data.project_name, &project_data.details],
        )
        .await?;

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
