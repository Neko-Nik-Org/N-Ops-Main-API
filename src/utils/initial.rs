use lettre::transport::smtp::authentication::Credentials as SmtpCredentials;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use lettre::message::Mailbox;
use std::env::var as env_var;
use std::time::Duration;


pub const OTP_CHARSET: &str = "ABCDEFGHJKLMNPQRSTWXY3456789";   // Characters used for generating OTPs


pub const ALL_AVAILABLE_PERMISSIONS: [&str; 4] = [
    // Resource Tag's
    "tag:view",
    "tag:create",
    "tag:edit",
    "tag:delete",
];


pub struct PgSettings {
    pub url: String,
    pub conn_timeout: u64,
    pub max_pool_size: usize,
    pub wait_timeout: u64,
    pub new_connection_timeout: u64,
    pub recycle_timeout: u64,
    pub warm_pool: bool,
    pub warm_pool_size: usize,
    pub ssl_accept_invalid_certs: bool,
    pub ssl_accept_invalid_hostnames: bool,
    pub ssl_root_cert_path: Option<String>,
}


pub struct MokaSettings {
    pub cache_size: u64,
    pub expiration_time: Duration,
}


pub struct AppSettings {
    pub pg_settings: PgSettings,
    pub cache_settings: MokaSettings,
    pub redis_url: String,
}


#[derive(Clone)]
pub struct RmqSettings {
    pub domain: String,
    pub auth_token: String, // Base64 encoded string of "username:password"
    pub virtual_host: String,
    pub exchange_name: String,
    pub routing_key: String,
}


pub struct SmtpSettings {
    pub host: String,
    pub from_email: Mailbox,
    pub credentials: SmtpCredentials,
    pub use_tls: bool,
    pub port: u16,
}


pub struct ApiSettings {
    pub allowed_origins: Vec<String>,
    pub self_api_key: String,
    pub master_encryption_key: String,
    // Additional API settings can be added here as needed
}


pub struct CfTurnstileSettings {
    pub api_url: String,
    pub secret_key: String,
}


// ------- Implementations ------- //


impl PgSettings {
    fn from_env() -> Self {
        let url = env_var("POSTGRES_DB_URL").expect("POSTGRES_DB_URL must be set");
        let conn_timeout = env_var("PG_CONN_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_CONN_TIMEOUT must be a positive integer of type u64");
        let max_pool_size = env_var("PG_POOL_MAX_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_MAX_SIZE must be a positive integer of type usize");
        let wait_timeout = env_var("PG_POOL_WAIT_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_WAIT_TIMEOUT must be a positive integer of type u64");
        let new_connection_timeout = env_var("PG_POOL_NEW_CONNECTION_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_NEW_CONNECTION_TIMEOUT must be a positive integer of type u64");
        let recycle_timeout = env_var("PG_POOL_RECYCLE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_RECYCLE_TIMEOUT must be a positive integer of type u64");
        let warm_pool = env_var("PG_POOL_WARM_POOL").expect("PG_POOL_WARM_POOL must be set as true or false");
        let warm_pool = match warm_pool.to_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => panic!("PG_POOL_WARM_POOL must be set as true or false"),
        };
        let warm_pool_size = env_var("PG_POOL_WARM_POOL_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_WARM_POOL_SIZE must be a positive integer of type usize");
        let ssl_accept_invalid_certs = env_var("PG_SSL_ACCEPT_INVALID_CERTS")
            .ok()
            .map(|s| s.to_lowercase())
            .map(|s| match s.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("PG_SSL_ACCEPT_INVALID_CERTS must be true or false"),
            })
            .unwrap_or(false);
        let ssl_accept_invalid_hostnames = env_var("PG_SSL_ACCEPT_INVALID_HOSTNAMES")
            .ok()
            .map(|s| s.to_lowercase())
            .map(|s| match s.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("PG_SSL_ACCEPT_INVALID_HOSTNAMES must be true or false"),
            })
            .unwrap_or(false);
        let ssl_root_cert_path = env_var("PG_SSL_ROOT_CERT_PATH")
            .ok()
            .filter(|s| !s.trim().is_empty());

        // Warm pool size can not go above 128 (if warm pool is enabled)
        if warm_pool_size > max_pool_size {
            panic!("PG_POOL_WARM_POOL_SIZE must be at most PG_POOL_MAX_SIZE, it can not go more than {}", max_pool_size);
        }
        if warm_pool && warm_pool_size > 128 {
            panic!("PG_POOL_WARM_POOL_SIZE must be at most 128, and the optimal size is 64");
        }

        PgSettings {
            url,
            conn_timeout,
            max_pool_size,
            wait_timeout,
            new_connection_timeout,
            recycle_timeout,
            warm_pool,
            warm_pool_size,
            ssl_accept_invalid_certs,
            ssl_accept_invalid_hostnames,
            ssl_root_cert_path,
        }
    }
}


impl MokaSettings {
    fn from_env() -> Self {
        let cache_size = env_var("CACHE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("CACHE_SIZE must be a positive integer of type u64");
        let expiration_time = env_var("CACHE_EXPIRATION_TIME")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("CACHE_EXPIRATION_TIME must be a positive integer of type u64");

        MokaSettings {
            cache_size,
            expiration_time: Duration::from_secs(expiration_time),
        }
    }
}


impl AppSettings {
    pub fn from_env() -> Self {
        let redis_url = env_var("REDIS_URL").expect("REDIS_URL must be set");

        AppSettings {
            pg_settings: PgSettings::from_env(),
            cache_settings: MokaSettings::from_env(),
            redis_url,
        }
    }
}


impl RmqSettings {
    pub fn from_env() -> Self {
        let domain = env_var("RABBITMQ_DOMAIN").expect("RABBITMQ_DOMAIN must be set");
        let user_name = env_var("RABBITMQ_USER_NAME").expect("RABBITMQ_USER_NAME must be set");
        let password = env_var("RABBITMQ_PASSWORD").expect("RABBITMQ_PASSWORD must be set");
        let virtual_host = env_var("RABBITMQ_VIRTUAL_HOST").expect("RABBITMQ_VIRTUAL_HOST must be set");
        let exchange_name = env_var("RABBITMQ_EXCHANGE_NAME").expect("RABBITMQ_EXCHANGE_NAME must be set");
        let routing_key = env_var("RABBITMQ_ROUTING_KEY").expect("RABBITMQ_ROUTING_KEY must be set");

        let auth_token = BASE64_STANDARD.encode(format!("{}:{}", user_name, password));

        RmqSettings {
            domain,
            auth_token,
            virtual_host,
            exchange_name,
            routing_key,
        }
    }
}


impl ApiSettings {
    pub fn from_env() -> Self {
        let allowed_origins = env_var("ALLOWED_ORIGINS")
            .ok()
            .map(|s| {
                s.split(',')
                    .map(|entry| entry.trim())
                    .filter(|entry| !entry.is_empty())
                    .map(|entry| entry.to_string())
                    .collect::<Vec<String>>()
            })
            .filter(|items| !items.is_empty())
            .expect("ALLOWED_ORIGINS must be set as a comma-separated list of allowed origins or '*' for allowing all origins");

        let self_api_key = env_var("SELF_API_KEY").expect("SELF_API_KEY must be set");
        let master_encryption_key = env_var("MASTER_ENCRYPTION_KEY").expect("MASTER_ENCRYPTION_KEY must be set");

        ApiSettings {
            allowed_origins,
            self_api_key,
            master_encryption_key
        }
    }
}


impl CfTurnstileSettings {
    pub fn from_env() -> Self {
        let api_url = env_var("CLOUDFLARE_TURNSTILE_API_URL").expect("CLOUDFLARE_TURNSTILE_API_URL must be set");
        let secret_key = env_var("CLOUDFLARE_TURNSTILE_SECRET_KEY").expect("CLOUDFLARE_TURNSTILE_SECRET_KEY must be set");

        CfTurnstileSettings {
            api_url,
            secret_key,
        }
    }
}


impl SmtpSettings {
    pub fn from_env() -> Self {
        let host = env_var("SMTP_HOST").expect("SMTP_HOST must be set");
        let port = env_var("SMTP_PORT").expect("SMTP_PORT must be set").parse::<u16>().expect("SMTP_PORT must be a valid u16");
        
        let from_email = env_var("SMTP_FROM_EMAIL")
            .expect("SMTP_FROM_EMAIL must be set")
            .parse::<Mailbox>()
            .expect("SMTP_FROM_EMAIL must be a valid email address");

        let user_name = env_var("SMTP_USER_NAME").expect("SMTP_USER_NAME must be set");
        let password = env_var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let credentials = SmtpCredentials::new(user_name, password);

        let use_tls = env_var("SMTP_USE_TLS")
            .ok()
            .map(|s| s.to_lowercase())
            .map(|s| match s.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("SMTP_USE_TLS must be true or false"),
            })
            .unwrap_or(true);

        SmtpSettings {
            host,
            from_email,
            credentials,
            port,
            use_tls,
        }
    }
}
