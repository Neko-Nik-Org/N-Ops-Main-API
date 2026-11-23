use crate::features::users::{SessionUser, CreateUserReqDto, create_user};
use crate::errors::{ServiceError, ServiceResult};
use crate::security::invite_code;
use super::dto::NewInviteDto;
use crate::messaging::email;
use crate::state::AppState;
use super::repository;



/// Send a new invite to the specified email with the generated invitation code
pub async fn send_new_invite(state: &AppState, session_user: &SessionUser, invite_data: NewInviteDto) -> ServiceResult<()> {
    // Validate the invite data
    invite_data.validate()?;

    // Check if the user invitation is possible
    if !repository::check_invite_possible(&state.pg_pool, &invite_data.invite_to_email).await? {
        return Err(ServiceError::BadRequest(
            "Invite not possible for this email, it may already be in use or an invite has already been sent.".into()
        ));
    }

    // Generate the invitation code
    let invitation_code = invite_code::generate_invite_code()?;

    // Generate the hash of the invitation code
    let invitation_code_hash = invite_code::hash_invite_code(&invitation_code);

    // Save in DB
    let result = repository::create_invite(
        &state.pg_pool,
        session_user.organization_id,
        invitation_code_hash,
        &invite_data
    ).await?;
    if result == 0 {
        return Err(ServiceError::Unprocessable("Failed to create new invite".into()));
    }

    // Send the email
    email::send_invite_email(
        &invite_data.invite_to_email,
        &invite_data.invitation_notes,
        &invite_data.expiration_datetime,
        &invitation_code,
        &session_user.display_name,
        &session_user.organization_name
    ).await?;

    Ok(())
}


/// Join an invite using the provided invitation code
pub async fn join_invite(state: &AppState, invitation_code: String, user_data: CreateUserReqDto) -> ServiceResult<()> {
    let invitation_code_hash = invite_code::hash_invite_code(&invitation_code);

    // Fetch the invite details using the invitation code hash
    let invite_details = repository::get_invite_details_by_code(&state.pg_pool, &invitation_code_hash).await?;
    if invite_details.is_none() {
        return Err(ServiceError::BadRequest("Invalid or expired invitation code.".into()));
    }
    let invite_details = invite_details.unwrap();

    // At this point, invite_details contains the valid invitation information
    // Build the request to create a new user based on the invite details
    let create_request = CreateUserReqDto {
        user_name: user_data.user_name,
        display_name: user_data.display_name,
        primary_email: invite_details.email,
        user_details: user_data.user_details,
        is_active: user_data.is_active,
        role_id: invite_details.role_id,
        password: user_data.password,
        access_restriction_policy_id: invite_details.access_restriction_policy_id,
    };

    // Create the user using the constructed request
    create_user(&state, invite_details.organization_id, &create_request).await?;

    // Once created, delete the invite to prevent reuse
    repository::delete_invitation(&state.pg_pool, invite_details.organization_id, invite_details.invitation_id).await?;

    Ok(())
}
