use crate::errors::{ServiceError, ServiceResult};
use crate::features::users::SessionUser;
use super::dto::VaultProjectDto;
use super::model::VaultProject;
use crate::state::AppState;
use super::repository;



/// Create a new project using the provided project data
pub async fn create_new_project(state: &AppState, session_user: &SessionUser, project_data: &VaultProjectDto) -> ServiceResult<()> {
    let result = repository::create_new_project(
        &state.pg_pool,
        session_user.organization_id,
        project_data
    ).await?;

    if result == 0 {
        return Err(ServiceError::Unprocessable("Failed to create new project".into()));
    }

    Ok(())
}


/// Update an existing project using the provided project data
pub async fn update_project(state: &AppState, session_user: &SessionUser, project_id: i64, project_data: &VaultProjectDto) -> ServiceResult<()> {
    let result = repository::update_project(
        &state.pg_pool,
        session_user.organization_id,
        project_id,
        project_data
    ).await?;

    if result == 0 {
        return Err(ServiceError::Unprocessable("Failed to update project".into()));
    }

    Ok(())
}
