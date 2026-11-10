use super::dto::{CreateResourceTagDto, EditResourceTagDto};
use crate::errors::{ServiceError, ServiceResult};
use crate::features::users::SessionUser;
use crate::state::AppState;
use super::repository;



/// Create a new tag using the provided resource tag data
pub async fn create_new_tag(state: &AppState, session_user: &SessionUser, tag_data: &CreateResourceTagDto) -> ServiceResult<()> {
    let result = repository::create_new_tag(
        &state.pg_pool,
        session_user.organization_id,
        tag_data
    ).await?;

    if result == 0 {
        return Err(ServiceError::Unprocessable("Failed to create new tag".into()));
    }

    Ok(())
}


/// Update an existing tag using the provided resource tag data
pub async fn update_tag_data(state: &AppState, session_user: &SessionUser, tag_data: &EditResourceTagDto) -> ServiceResult<()> {
    let result = repository::update_tag_data(
        &state.pg_pool,
        session_user.organization_id,
        tag_data
    ).await?;

    if result == 0 {
        return Err(ServiceError::Unprocessable("Failed to update tag".into()));
    }

    Ok(())
}


/// Delete an existing tag using the provided tag ID
pub async fn delete_tag(state: &AppState, session_user: &SessionUser, tag_id: i64) -> ServiceResult<()> {
    let result = repository::delete_tag(
        &state.pg_pool,
        session_user.organization_id,
        tag_id
    ).await?;

    if result == 0 {
        return Err(ServiceError::Unprocessable("Failed to delete tag".into()));
    }

    Ok(())
}
