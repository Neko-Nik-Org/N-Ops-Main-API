use actix_web::web::scope as actix_scope;
use routes::{health, sample_db, auth};
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
                // .allow_any_origin()  // Uncomment this line to allow any origin (not recommended for production)
                .allowed_origin_fn(state::cors_allowed_origin_fn)
                .allowed_methods(["GET", "POST", "DELETE"])
                .supports_credentials()
                .allow_any_header()
                .max_age(420)
            )
            .service(
                actix_scope("/health")
                // .service(health::full_health_check)  // Perform one full health check for all components and DB's
                .service(health::db_health_check)
                .service(health::api_health_check)
                .service(health::redis_cache_health_check)
                .service(health::in_mem_cache_health_check)
            )
            .service(
                actix_scope("/internal")
                .wrap(from_fn(middleware::key_based::auth_check))
                .service(health::db_health_check)
                .service(health::api_health_check)
                .service(health::redis_cache_health_check)
                .service(health::in_mem_cache_health_check)
            )
            .service(
                actix_scope("/sample_db")
                .wrap(from_fn(middleware::user_session::auth_check))
                .service(sample_db::create_note_handler)
                .service(sample_db::list_notes_handler)
            )
            .service(
                actix_scope("/auth")
                // .service(auth::user_login)   // Login endpoint for users (3 Hours session, refreshable)
                // Send/Re-send 2FA code for login
                // Verify 2FA code for login
                .service(auth::create_session_handler)
                .service(
                    actix_scope("")
                    .wrap(from_fn(middleware::user_session::auth_check))
                    // Logout
                    // Refresh session if user is active and the session is still valid
                    // Check session validity
                    .service(auth::delete_session_handler)
                    .service(auth::get_session_handler)
                )
            )
            // .service(
            //     actix_scope("/user")
            //     .wrap(from_fn(middleware::user_session::auth_check))
            //     .service(user::get_ui_details) // Get UI details JSON
            //     .service(user::set_ui_details) // Replace UI details JSON
            //     .service(user::get_user_details) // Get user details JSON
            //     .service(user::set_user_details) // Replace user details JSON
            // )
    })
    .bind(("0.0.0.0", 8686))?
    .workers(env_var("API_WORKERS_COUNT").unwrap_or("4".to_string()).parse().unwrap())
    .run().await
}
