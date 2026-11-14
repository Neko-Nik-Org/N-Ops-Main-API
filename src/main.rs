use routes::{health, auth, users, tags};
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
                .allowed_methods(["GET", "POST", "PUT", "DELETE"])
                .supports_credentials()
                .allow_any_header()
                .max_age(420)
            )
            .service(
                actix_scope("/health")
                .service(health::full_health_check)
            )
            // .service(    // super-admin -> Admin Panel for managing organizations
            //     actix_scope("/super-admin/organization")
            //     .wrap(from_fn(middleware::key_based::auth_check))
            //     .service(organization::create_organization)
            //     .service(organization::update_organization)
            //     .service(organization::delete_organization)
            //     .service(organization::enable_organization)
            //     .service(organization::list_organizations)
            //     .service(organization::get_organization)
            // )
            // .service(
            //     actix_scope("/super-admin/user")
            //     .wrap(from_fn(middleware::key_based::auth_check))
            //     .service(users::admin_create_user)
            //     .service(users::admin_update_user)
            //     .service(users::admin_reset_password)
            //     .service(users::admin_delete_user)
            //     .service(users::admin_list_users)
            //     .service(users::admin_get_user)
            //     .service(users::admin_verify_email)
            // )
            .service(
                actix_scope("/auth")
                .service(auth::user_login)
                // .service(auth::forgot_password)
                // .service(2fa::resend_otp)
                // .service(2fa::verify_otp)
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
                .service(users::get_ui_info)
                .service(users::set_ui_info)
                .service(users::get_org_info)
                .service(users::verify_email)
                .service(users::check_username)
                .service(users::get_user_details)
                .service(users::set_user_details)
                .service(users::send_verification_email)
                // .service(users::change_password)    // Change the current user's password
                // .service(users::update_profile)    // Update the current user's profile (name, email, etc.)
                // .service(users::change_username)    // Change the current user's username
            )
            // .service(
            //     actix_scope("/user/admin")
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(users::create_user)    // Create Any User
            //     .service(users::update_user)    // Update Any User's Details
            //     .service(users::reset_password) // Reset Any User's Password
            //     .service(users::delete_user)    // Delete Any User
            //     .service(users::list_users) // List All Users
            //     .service(users::get_user) // Get Any User info
            //     .service(users::admin_verify_email) // Admin verify any user's email (Bypass user verification)
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
            //     actix_scope("/api-keys")
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(api_keys::create_api_key)    // Create Any API Key (by user tag permission)
            //     .service(api_keys::update_api_key)    // Update Any API Key's Properties (by user tag permission)
            //     .service(api_keys::delete_api_key)    // Delete Any API Key (by user tag permission)
            //     .service(api_keys::list_api_keys) // List All API Keys (by user tag permission)
            //     .service(api_keys::get_api_key) // Get Any API Key info (by user tag permission)
            // )
            // .service(
            //     actix_scope("/user/roles")
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(user_roles::create_user_role)    // Create Any User Role (by user tag permission)
            //     .service(user_roles::update_user_role)    // Update Any User Role's Properties (by user tag permission)
            //     .service(user_roles::delete_user_role)    // Delete Any User Role (by user tag permission)
            //     .service(user_roles::list_user_roles) // List All User Roles (by user tag permission)
            //     .service(user_roles::get_user_role) // Get Any User Role info (by user tag permission)
            // )
            // .service(
            //     actix_scope("/vault/project")
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(vault_project::create_project)
            //     .service(vault_project::update_project)
            //     .service(vault_project::delete_project)
            //     .service(vault_project::list_projects)
            //     .service(vault_project::get_project)
            // )
            // .service(
            //     actix_scope("/vault/project/service")
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(vault_project_service::create_service)
            //     .service(vault_project_service::update_service)
            //     .service(vault_project_service::delete_service)
            //     .service(vault_project_service::list_services)
            //     .service(vault_project_service::get_service)
            // )
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
