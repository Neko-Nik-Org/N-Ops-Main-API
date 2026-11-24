use crate::features::users::{SessionUser, CreateUserReqDto, create_user, fetch_org_info};
use crate::features::user_roles::{CreateNewRoleDto, check_role_exists, create_role};
use crate::utils::initial::ALL_AVAILABLE_PERMISSIONS;
use crate::errors::{ServiceError, ServiceResult};
use crate::security::invite_code;
use super::dto::NewInviteDto;
use crate::messaging::email;
use crate::state::AppState;
use super::repository;
use uuid::Uuid;


const SUPER_ADMIN_ROLE_NAME: &str = "Super Admin - Initial Automated Invite";
const SUPER_ADMIN_INVITE_NOTES: &str = "Happy to welcome you as a super admin to N-Ops! \nPlease use the invitation code sent to your email to complete the registration.";


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

    // Fetch the organization information for the organization associated with the invite
    let org_info = fetch_org_info(&state.pg_pool, invite_details.organization_id).await?;

    // At this point, invite_details contains the valid invitation information
    // Build the request to create a new user based on the invite details
    let create_request = CreateUserReqDto {
        user_name: user_data.user_name,
        display_name: user_data.display_name,
        primary_email: invite_details.email,
        user_details: user_data.user_details,
        is_active: true,
        role_id: invite_details.role_id,
        password: user_data.password,
        access_restriction_policy_id: invite_details.access_restriction_policy_id,
    };

    // Create the user using the constructed request (No need to use the returned anti-phishing secret)
    create_user(&state, invite_details.organization_id, &org_info.organization_name, &create_request).await?;

    // Once created, delete the invite to prevent reuse
    repository::delete_invitation(&state.pg_pool, invite_details.organization_id, invite_details.invitation_id).await?;

    Ok(())
}


/// Send an admin invite to the specified email
pub async fn send_admin_invite(state: &AppState, organization_id: Uuid, email: String) -> ServiceResult<()> {
    // Check if the organization exists and active and fetch its information
    let org_info = fetch_org_info(&state.pg_pool, organization_id).await?;
    if org_info.current_user_count >= org_info.user_limit {
        return Err(ServiceError::BadRequest("User limit reached for the organization.".into()));
    }

    // Check and fetch the existing role for the super admin invite
    let if_existing_role_id = check_role_exists(&state.pg_pool, organization_id, SUPER_ADMIN_ROLE_NAME).await?;
    let role_id: i64;

    // If the role already exists, use its ID; otherwise, create it
    if if_existing_role_id.is_some() {
        role_id = if_existing_role_id.unwrap();
    } else {
        // Create a new role for the admin invite
        let role_data = CreateNewRoleDto {
            role_name: SUPER_ADMIN_ROLE_NAME.into(),
            role_description: "Role created for the initial automated super admin invite with full privileges".into(),
            permissions: ALL_AVAILABLE_PERMISSIONS.map(|p| p.into()).to_vec(),
            allowed_linux_users: vec![],
            allowed_tags: vec![],
            can_approve_approvals: true,
            can_bypass_approvals: true,
        };

        // Create the new role and get its ID
        role_id = create_role(&state.pg_pool, organization_id, &role_data).await?;
    }

    // Generate the invitation code
    let invitation_code = invite_code::generate_invite_code()?;

    // Generate the hash of the invitation code
    let invitation_code_hash = invite_code::hash_invite_code(&invitation_code);

    // Build the invitation record to be saved in the database
    let invite_record = NewInviteDto {
        invite_to_email: email.clone(),
        invitation_notes: SUPER_ADMIN_INVITE_NOTES.into(),
        role_id,
        access_restriction_policy_id: None,
        expiration_datetime: chrono::Utc::now() + chrono::Duration::days(7) // Set expiration to 7 days from now
    };

    // Save the invitation record to the database
    let result = repository::create_invite(
        &state.pg_pool,
        organization_id,
        invitation_code_hash,
        &invite_record
    ).await?;
    if result == 0 {
        return Err(ServiceError::Unprocessable("Failed to create new invite".into()));
    }

    // Send the admin invite email
    email::send_invite_email(
        &invite_record.invite_to_email,
        &invite_record.invitation_notes,
        &invite_record.expiration_datetime,
        &invitation_code,
        "Super Admin (N-Ops)",
        &org_info.organization_name
    ).await?;

    Ok(())
}
