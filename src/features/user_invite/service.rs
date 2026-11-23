use crate::errors::{ServiceError, ServiceResult};
use crate::features::users::SessionUser;
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
        &invitation_code,
        &session_user.display_name,
        &session_user.organization_name
    ).await?;

    Ok(())
}
