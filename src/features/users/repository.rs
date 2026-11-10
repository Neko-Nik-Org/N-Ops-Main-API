use deadpool_postgres::Pool as PgPool;
use serde_json::Value as JsonValue;
use super::dto::AuthInfoDbDto;
use crate::errors::PgResult;
use std::net::IpAddr;



pub async fn get_user_auth_info(db_pool: &PgPool, user_name: &str) -> PgResult<Option<AuthInfoDbDto>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                -- Identifiers
                u.user_id,
                u.organization_id,

                -- Security and access information
                u.password_hash,
                o.encryption_key,
                u.has_ip_restriction,
                u.has_geo_restriction,
                u.allowed_ips,
                u.allowed_geo_locations,
                u.has_totp_2fa,
                u.has_email_2fa,

                r.permissions,

                -- Role-associated resource tags
                COALESCE(
                    (
                        SELECT ARRAY_AGG(t.tag_name ORDER BY t.tag_name)
                        FROM role_tag rt
                        INNER JOIN resource_tag t
                            ON t.tag_id = rt.tag_id
                            AND t.organization_id = u.organization_id
                        WHERE rt.role_id = r.role_id
                    ),
                    ARRAY[]::TEXT[]
                ) AS tags,

                -- Basic user information
                u.user_name,
                u.display_name,
                o.organization_name,

                -- Contact information
                u.primary_email,
                u.is_email_verified,

                -- Organization limits
                o.user_limit,
                o.server_limit,
                o.ss_project_limit,
                o.ss_keys_limit,
                o.ss_file_limit,
                o.user_notes_limit,

                -- Authentication metadata
                u.last_login_at

            FROM app_user u

            INNER JOIN organization o
                ON o.organization_id = u.organization_id

            INNER JOIN user_role r
                ON r.role_id = u.role_id
                AND r.organization_id = u.organization_id

            WHERE
                u.user_name = $1
                AND u.is_active = TRUE
                AND o.is_active = TRUE
            "#,
            &[&user_name],
        )
        .await?;

    Ok(row.map(AuthInfoDbDto::from))
}


pub async fn get_ip_location(db_pool: &PgPool, client_ip: &str) -> PgResult<(String, String)> {
    let client = db_pool.get().await?;

    let row = client
        .query_one(
            r#"
            SELECT country_iso_code, country_name
            FROM ip_geo_locations
            WHERE network >>= $1
            "#,
            &[&client_ip.parse::<IpAddr>().ok()],
        )
        .await?;

    let country_iso_code = row.get::<&str, String>("country_iso_code");
    let country_name = row.get::<&str, String>("country_name");

    Ok((country_iso_code, country_name))
}


pub async fn fetch_user_ui_info(db_pool: &PgPool, user_id: i64) -> PgResult<JsonValue> {
    let client = db_pool.get().await?;

    let row = client
        .query_one(
            r#"
            SELECT ui_info
            FROM app_user
            WHERE user_id = $1
            "#,
            &[&user_id],
        )
        .await?;

    Ok(row.get("ui_info"))
}


pub async fn reset_user_ui_info(db_pool: &PgPool, user_id: i64, ui_info: &JsonValue) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE app_user
            SET ui_info = $1
            WHERE user_id = $2
            "#,
            &[ui_info, &user_id],
        )
        .await?;

    Ok(result)
}


pub async fn fetch_user_details(db_pool: &PgPool, user_id: i64) -> PgResult<JsonValue> {
    let client = db_pool.get().await?;

    let row = client
        .query_one(
            r#"
            SELECT user_details
            FROM app_user
            WHERE user_id = $1
            "#,
            &[&user_id],
        )
        .await?;

    Ok(row.get("user_details"))
}


pub async fn reset_user_details(db_pool: &PgPool, user_id: i64, user_details: &JsonValue) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE app_user
            SET user_details = $1
            WHERE user_id = $2
            "#,
            &[user_details, &user_id],
        )
        .await?;

    Ok(result)
}
