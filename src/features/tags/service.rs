use super::dto::{CreateResourceTagDto, EditResourceTagDto};
use crate::errors::{ServiceError, ServiceResult};
use crate::features::users::SessionUser;
use super::model::ResourceTag;
use crate::state::AppState;
use super::repository;
use std::sync::Arc;



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

    // Delete the cached tags for the organization since a new tag was added
    let cache_key = session_user.organization_id.to_string();
    state
        .in_mem_cache
        .org_tags
        .invalidate(cache_key.as_str())
        .await;

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

    // Delete the cached tags for the organization since a tag was deleted
    let cache_key = session_user.organization_id.to_string();
    state
        .in_mem_cache
        .org_tags
        .invalidate(cache_key.as_str())
        .await;

    Ok(())
}


/// Get an existing tag using the provided tag ID
pub async fn get_tag(state: &AppState, session_user: &SessionUser, tag_id: i64) -> ServiceResult<ResourceTag> {
    if let Some(tag) = repository::get_tag(
        &state.pg_pool,
        session_user.organization_id,
        tag_id
    ).await? {
        Ok(tag)
    } else {
        Err(ServiceError::NotFound("Tag not found".into()))
    }
}


/// List all tags for the session user's organization
pub async fn list_all_tags(state: &AppState, session_user: &SessionUser) -> ServiceResult<Vec<ResourceTag>> {
    let tags = repository::list_all_tags(
        &state.pg_pool,
        session_user.organization_id
    ).await?;

    Ok(tags)
}


/// Fetch all tags for the session user's organization from the in-memory cache
pub async fn fetch_org_tag_ids(state: &AppState, organization_id: uuid::Uuid) -> ServiceResult<Vec<i64>> {
    let cache_key = organization_id.to_string();
    let org_tags = state
        .in_mem_cache
        .org_tags
        .get(cache_key.as_str())
        .await;

    // Return the cached tag IDs for the organization
    if let Some(tags) = org_tags {
        return Ok(tags.to_vec());
    }

    // If the tags are not found in the cache, fetch them from the database
    let tags = repository::list_all_org_tag_ids(&state.pg_pool, organization_id).await?;

    // Cache the fetched tag IDs for the organization
    state
        .in_mem_cache
        .org_tags
        .insert(
            cache_key.into(),
            Arc::new(tags.clone())
        )
        .await;

    Ok(tags)
}
