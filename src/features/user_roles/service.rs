use super::dto::{CreateNewRoleDto, UpdateRoleInfoDto, UpdateRolePermissionsDto};
use crate::errors::{ServiceError, ServiceResult};
use crate::features::users::SessionUser;
use super::model::RolePermissions;
use crate::state::AppState;
use super::repository;



fn validate_role_restrictions<T>(session_user: &SessionUser, role: &T) -> ServiceResult<()>
    where T: RolePermissions
{
    // If role permissions are empty, then its a bad request
    if role.permissions().is_empty() {
        return Err(ServiceError::BadRequest("Role permissions cannot be empty".into()));
    }

    // Check if the new role permissions are subset of the session user's permissions
    if !role.permissions().iter().all(|p| session_user.permissions.contains(p)) {
        return Err(ServiceError::Forbidden(
            "Role permissions exceed session user's permissions".into()
        ));
    }

    // Allowed Linux users must be a subset of the session user's allowed Linux users
    // If the session user has no restrictions on allowed Linux users, then allow anything
    // If the session user has restrictions, the role's allowed Linux users must be a subset of them
    if !session_user.allowed_linux_users.is_empty() {
        if role.allowed_linux_users().is_empty()
            || !role.allowed_linux_users()
                .iter()
                .all(|u| session_user.allowed_linux_users.contains(u))
        {
            return Err(ServiceError::Forbidden(
                "Role's allowed linux users exceed session user's allowed linux users".into()
            ));
        }
    }

    // Similar to allowed Linux users restrictions
    if !session_user.tags.is_empty() {
        if role.allowed_tags().is_empty()
            || !role.allowed_tags()
                .iter()
                .all(|t| session_user.tags.contains(t))
        {
            return Err(ServiceError::Forbidden(
                "Role's allowed tags exceed session user's allowed tags".into()
            ));
        }
    }

    // Can set Approve approvals (Means Server Manager Admin guy)
    if role.can_approve_approvals() && !session_user.can_approve_approvals {
        return Err(ServiceError::Forbidden(
            "Session user does not have permission to set approve approvals".into()
        ));
    }

    // Can set Bypass approvals if the session user has the required privileges (Means Unrestricted Admin)
    if role.can_bypass_approvals() && !session_user.can_bypass_approvals {
        return Err(ServiceError::Forbidden(
            "Session user does not have permission to set bypass restrictions".into()
        ));
    }

    Ok(())
}


/// Create a new role using the provided role data
pub async fn create_new_role(state: &AppState, session_user: &SessionUser, role_data: &CreateNewRoleDto) -> ServiceResult<()> {
    // Validate that the role restrictions in the provided data do not exceed the session user's permissions and restrictions
    validate_role_restrictions(session_user, role_data)?;

    // Call the repository function to create the new role in the database
    let _ = repository::create_role(&state.pg_pool, session_user.organization_id, role_data).await?;

    Ok(())
}


/// Delete an existing role using the provided role ID
pub async fn delete_role(state: &AppState, session_user: &SessionUser, role_id: i64) -> ServiceResult<()> {
    // Check if the role can be safely deleted (i.e., no users are associated with it)
    if !repository::can_delete_safely(&state.pg_pool, session_user.organization_id, role_id).await? {
        return Err(ServiceError::PreconditionFailed(
            "Role cannot be deleted because there are users associated with it, delete those users first".into()
        ));
    }

    // First fetch the role to ensure it exists and to perform any necessary validations
    let role = repository::fetch_role(&state.pg_pool, session_user.organization_id, role_id).await?;

    // Validate that the session user has the required permissions to delete this role
    validate_role_restrictions(session_user, &role)?;

    // Call the repository function to delete the role from the database
    repository::delete_role(&state.pg_pool, session_user.organization_id, role_id).await?;

    Ok(())
}


/// Update the information of an existing role using the provided role ID and update data
pub async fn update_role_info(state: &AppState, session_user: &SessionUser, role_id: i64, update_data: &UpdateRoleInfoDto) -> ServiceResult<()> {
    // First fetch the role to ensure it exists and to perform any necessary validations
    let role = repository::fetch_role(&state.pg_pool, session_user.organization_id, role_id).await?;

    // Validate that the session user has the required permissions to update this role
    validate_role_restrictions(session_user, &role)?;

    // Call the repository function to update the role info in the database
    repository::update_role_info(&state.pg_pool, session_user.organization_id, role_id, update_data).await?;

    Ok(())
}


/// Update the permissions of an existing role using the provided role ID and update data
pub async fn update_role_permissions(state: &AppState, session_user: &SessionUser, role_id: i64, update_data: &UpdateRolePermissionsDto) -> ServiceResult<()> {
    // First fetch the role to ensure it exists and to perform any necessary validations
    let role = repository::fetch_role(&state.pg_pool, session_user.organization_id, role_id).await?;

    // Validate that the session user has the required permissions to update this role
    validate_role_restrictions(session_user, &role)?;

    // Validate that the session user with new permission set, still complies with the role restrictions
    validate_role_restrictions(session_user, update_data)?;

    // Call the repository function to update the role permissions in the database
    repository::update_role_permissions(&state.pg_pool, session_user.organization_id, role_id, update_data).await?;

    Ok(())
}
