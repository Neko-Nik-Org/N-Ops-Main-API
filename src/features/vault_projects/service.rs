use crate::features::users::{SessionUser, fetch_org_info};
use crate::errors::{ServiceError, ServiceResult};
use super::dto::{VaultProjectDto, ListQueryDto};
use super::model::VaultProject;
use crate::state::AppState;
use super::repository;



/// Create a new project using the provided project data
pub async fn create_new_project(state: &AppState, session_user: &SessionUser, project_data: &VaultProjectDto) -> ServiceResult<()> {
    // Fetch organization info to check project quota
    let org_info = fetch_org_info(&state.pg_pool, session_user.organization_id).await?;
    if org_info.ss_project_limit <= org_info.current_ss_project_count {
        return Err(ServiceError::Unprocessable("Organization has reached its project quota".into()));
    }

    // Create the new project in the repository
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

/// List projects based on the provided query
pub async fn list_projects(state: &AppState, session_user: &SessionUser, query: &ListQueryDto) -> ServiceResult<Vec<VaultProject>> {
    let projects = repository::list_projects(
        &state.pg_pool,
        session_user.organization_id,
        query
    ).await?;

    Ok(projects)
}


/// Get the details of a specific project by its ID
pub async fn get_project_detail(state: &AppState, session_user: &SessionUser, project_id: i64) -> ServiceResult<Option<VaultProject>> {
    let project = repository::get_project_detail(
        &state.pg_pool,
        session_user.organization_id,
        project_id
    ).await?;

    if project.is_none() {
        return Err(ServiceError::NotFound("Project not found".into()));
    }

    Ok(project)
}


/// Delete a project by its ID
pub async fn delete_project(state: &AppState, session_user: &SessionUser, project_id: i64) -> ServiceResult<()> {
    let result = repository::delete_project(
        &state.pg_pool,
        session_user.organization_id,
        project_id
    ).await?;

    if result == 0 {
        return Err(ServiceError::Unprocessable("Failed to delete project".into()));
    }

    Ok(())
}
