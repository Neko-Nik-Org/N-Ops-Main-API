use deadpool_postgres::Pool as PgPool;
use super::dto::VaultProjectDto;
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
