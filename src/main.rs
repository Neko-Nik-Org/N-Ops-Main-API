use routes::{health, auth, users, tags, user_roles, vault_project, organization, ar_policy};
use actix_web::web::scope as actix_scope;
use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer};
use std::env::var as env_var;
use actix_cors::Cors;


mod middleware;
mod messaging;
mod database;
mod features;
mod security;
mod errors;
mod routes;
mod tasks;
mod cache;
mod state;
mod utils;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _log_guard = utils::logging::init();

    let app_state = state::initialize().await;

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Cors::default()
                .allowed_origin_fn(state::cors_allowed_origin_fn)
                .allowed_methods(["GET", "POST", "PUT", "PATCH", "DELETE"])
                .supports_credentials()
                .allow_any_header()
                .max_age(420)
            )
            .service(
                actix_scope("/health")
                .service(health::full_health_check)
            )
            .service(
                actix_scope("/super-admin/organization")
                .wrap(from_fn(middleware::key_based::admin_auth_check))
                .service(organization::organization_state_change)
                .service(organization::create_organization)
                .service(organization::delete_organization)
                .service(organization::billing_info_update)
                .service(organization::list_organizations)
                .service(organization::basic_info_update)
                .service(organization::get_organization)
                .service(organization::check_org_name)
                .service(organization::quota_update)
                .service(organization::name_update)
                // .service(invite::admin_invite)  // Create a new full permission invite for an admin user (Send email with invite code)
            )
            .service(
                actix_scope("/auth")
                .service(auth::user_login)
                // .service(auth::forgot_password)  // Send an email with a link to reset password
                // .service(auth::reset_password)  // Reset password using a token sent via email
                // .service(invite::join_invite)  // User registration by link (invitation based)
                // .service(mfa::resend_otp)
                // .service(mfa::verify_otp)
                .service(
                    actix_scope("")
                    .wrap(from_fn(middleware::user_session::auth_check))
                    .service(auth::refresh_session)
                    .service(auth::verify_session)
                    .service(auth::user_logout)
                )
            )
            .service(
                actix_scope("/user/self")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(users::get_profile)
                .service(users::get_ui_info)
                .service(users::set_ui_info)
                .service(users::get_org_info)
                .service(users::verify_email)
                .service(users::self_password)
                .service(users::check_username)
                .service(users::update_profile)
                .service(users::self_deactivate)
                .service(users::get_user_details)
                .service(users::set_user_details)
                .service(users::self_user_name_update)
                .service(users::send_verification_email)
            )
            .service(
                actix_scope("/user/admin")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(users::set_verify_email)
                .service(users::deactivate_user)
                .service(users::change_username)
                .service(users::reset_password)
                .service(users::delete_user)
                .service(users::create_user)
                .service(users::list_users)
                .service(users::get_user)
                // .service(users::update_user)    // Update User's Details like primary_email, display_name, user_details
                // .service(users::assign_role)    // Update User's Role and Geo / IP restrictions or 2FA methods (disable only)
                // .service(users::assign_ar_policy)    // Update User's Role and Geo / IP restrictions or 2FA methods (disable only)
                // .service(users::send_invitation) // Send an invitation to a user to join the organization (email send with link)
            )
            // .service(
            //     actix_scope("/user/2fa")
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(mfa::set_email_2fa)    // Enable/Disable email-based 2FA (Works only if Email is verified)
            //     .service(mfa::set_totp_2fa)    // Enable/Disable TOTP-based 2FA (Works only if at least one TOTP entry exists)
            //     .service(mfa::check_totp_code)    // Check a TOTP code for validity by entering it (for each entry / new entry)
            //     .service(mfa::create_totp_entry)    // Create TOTP entry
            //     .service(mfa::list_totp_entries)    // List all TOTP entries
            //     .service(mfa::delete_totp_entry)    // Delete a TOTP entry (Can not delete the last remaining entry if TOTP-based 2FA is enabled)
            //     .service(mfa::edit_totp_entry)    // Edit a TOTP entry name only
            //     .service(mfa::generate_new_recovery_codes)    // Generate new recovery codes for 2FA (Show only once)
            //     .service(mfa::count_recovery_codes)    // Count remaining recovery codes for 2FA (Just returns the count)
            // )
            .service(
                actix_scope("/tag")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(tags::create_tag)
                .service(tags::update_tag)
                .service(tags::delete_tag)
                .service(tags::list_tags)
                .service(tags::get_tag)
            )
            // .service(
            //     actix_scope("/invite")
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(invite::delete_invite) // Delete an existing invite by ID, send cancellation email if necessary?
            //     .service(invite::resend_invite) // Resend an existing invite email
            //     .service(invite::list_invites)  // List all existing pending invites
            //     .service(invite::new_invite)    // Send email with invite code "{UI}/invite/{invitation_id}/{invitation_code}"
            //     .service(invite::get_invite)    // Get details of a specific invite by ID
            // )
            .service(
                actix_scope("/user/roles")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(user_roles::list_roles)
                .service(user_roles::fetch_role)
                .service(user_roles::create_role)
                .service(user_roles::delete_role)
                .service(user_roles::update_role_info)
                .service(user_roles::update_role_permissions)
            )
            .service(
                actix_scope("/policy/access-restriction")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(ar_policy::fetch_policy)
                .service(ar_policy::list_policies)
                .service(ar_policy::create_policy)
                .service(ar_policy::delete_policy)
                .service(ar_policy::update_policy_info)
                .service(ar_policy::check_policy_association)
                .service(ar_policy::update_policy_enforcement)
            )
            // .service(
            //     actix_scope("/api-keys")
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(api_keys::create_api_key)    // Create Any API Key (by user tag permission)
            //     .service(api_keys::update_api_key)    // Update Any API Key's Properties (by user tag permission)
            //     .service(api_keys::delete_api_key)    // Delete Any API Key (by user tag permission)
            //     .service(api_keys::list_api_keys) // List All API Keys (by user tag permission)
            //     .service(api_keys::get_api_key) // Get Any API Key info (by user tag permission)
            // )
            .service(
                actix_scope("/vault/project")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(vault_project::create_project)
                .service(vault_project::update_project)
                .service(vault_project::delete_project)
                .service(vault_project::list_projects)
                .service(vault_project::get_project)
            )
            // .service(
            //     actix_scope("/vault/secret/keys")
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(vault_secret_keys::create_env_key)
            //     .service(vault_secret_keys::update_env_key) // Create new version of the environment key
            //     .service(vault_secret_keys::delete_env_key) // Delete the environment key along with all its versions
            //     .service(vault_secret_keys::set_key_version) // Set the active current_version of the environment key
            //     .service(vault_secret_keys::get_env_key) // Get the environment key along with its max version count
            //     .service(vault_secret_keys::list_env_keys) // List all environment keys along with their max version count
            //     .service(vault_secret_keys::get_key_version) // Get a specific version of the environment key (with bool decode_and_fetch)
            //     .service(vault_secret_keys::list_key_versions) // List all versions of the environment key
            // )
    })
    .bind(("0.0.0.0", 8686))?
    .workers(env_var("API_WORKERS_COUNT").unwrap_or("4".to_string()).parse().unwrap())
    .run().await
}
