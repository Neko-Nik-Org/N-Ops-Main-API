use routes::{health, auth, users};
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
                .allowed_methods(["GET", "POST", "DELETE"])
                .supports_credentials()
                .allow_any_header()
                .max_age(420)
            )
            // .service(
            //     actix_scope("/health")
            //     .service(health::full_health_check)  // Perform one full health check for all components and DB's
                // .service(health::db_health_check)    // Merge into one
                // .service(health::api_health_check)    // Merge into one
                // .service(health::redis_cache_health_check)    // Merge into one
                // .service(health::in_mem_cache_health_check)    // Merge into one
            // )
            // .service(
            //     actix_scope("/internal")
            //     .wrap(from_fn(middleware::key_based::auth_check))
            //     .service(health::db_health_check)
            //     .service(health::api_health_check)
            //     .service(health::redis_cache_health_check)
            //     .service(health::in_mem_cache_health_check)
            // )
            .service(
                actix_scope("/auth")
                .service(auth::user_login)
                // .service(auth::resend_otp)
                // .service(auth::verify_otp)
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
                // .service(users::get_org_info)   // Get Organization Info for the User (including limits and all)
                .service(users::get_user_details)
                .service(users::set_user_details)
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
            // )
            // .service(
            //     actix_scope("/tag")
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(tags::create_tag)    // Create Any Tag
            //     .service(tags::update_tag)    // Update Any Tag's Details
            //     .service(tags::delete_tag)    // Delete Any Tag
            //     .service(tags::list_tags) // List All Tags
            //     .service(tags::get_tag) // Get Any Tag info
            // )
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
    })
    .bind(("0.0.0.0", 8686))?
    .workers(env_var("API_WORKERS_COUNT").unwrap_or("4".to_string()).parse().unwrap())
    .run().await
}
