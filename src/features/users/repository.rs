use super::dto::{AuthInfoDbDto, ListUsersReqDto, CreateUserReqDto};
use super::model::{OrgInfo, UserProfileInfo, UserProfileListing};
use crate::errors::{PgResult, PgError, PgPool};
use serde_json::Value as JsonValue;
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

                -- Role information
                r.permissions,
                r.can_approve_approvals,
                r.can_bypass_approvals,
                r.allowed_linux_users,

                -- Role-associated resource tag IDs
                COALESCE(
                    (
                        SELECT ARRAY_AGG(t.tag_id ORDER BY t.tag_id)
                        FROM role_tag rt
                        INNER JOIN resource_tag t
                            ON t.tag_id = rt.tag_id
                            AND t.organization_id = u.organization_id
                        WHERE rt.role_id = r.role_id
                    ),
                    ARRAY[]::BIGINT[]
                ) AS tags,

                -- Basic user information
                u.user_name,
                u.display_name,
                o.organization_name,

                -- Contact information
                u.primary_email,
                u.is_email_verified,

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

    Ok((
        row.get("country_iso_code"),
        row.get("country_name"),
    ))
}


pub async fn fetch_user_profile(db_pool: &PgPool, user_id: i64) -> PgResult<Option<UserProfileInfo>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                u.user_id,
                u.organization_id,

                -- Basic user information
                u.user_name,
                u.display_name,
                o.organization_name,

                -- Active status flags
                u.is_active AS is_user_active,
                o.is_active AS is_organization_active,

                -- Contact information
                u.primary_email,
                u.is_email_verified,

                -- Permissions and access control
                r.permissions,
                r.can_approve_approvals,
                r.can_bypass_approvals,
                r.allowed_linux_users,

                -- Role-associated resource tag IDs
                COALESCE(
                    (
                        SELECT ARRAY_AGG(t.tag_id ORDER BY t.tag_id)
                        FROM role_tag rt
                        INNER JOIN resource_tag t
                            ON t.tag_id = rt.tag_id
                            AND t.organization_id = u.organization_id
                        WHERE rt.role_id = r.role_id
                    ),
                    ARRAY[]::BIGINT[]
                ) AS tags
            FROM app_user u
            INNER JOIN organization o
                ON o.organization_id = u.organization_id
            INNER JOIN user_role r
                ON r.role_id = u.role_id
                AND r.organization_id = u.organization_id
            WHERE u.user_id = $1
            "#,
            &[&user_id],
        )
        .await?;

    Ok(row.map(UserProfileInfo::from))
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


pub async fn fetch_org_info(db_pool: &PgPool, organization_id: uuid::Uuid) -> PgResult<OrgInfo> {
    let client = db_pool.get().await?;

    let row = client
        .query_one(
            r#"
            SELECT
                o.organization_id,
                o.organization_name,
                o.organization_info,

                o.allocated_ai_credit,
                o.utilized_ai_credit,

                o.allocated_storage_bytes,
                o.utilized_storage_bytes,

                o.user_limit,
                (
                    SELECT COUNT(*)
                    FROM app_user au
                    WHERE au.organization_id = o.organization_id
                )::INT AS current_user_count,

                o.server_limit,
                o.workflow_limit,

                o.ss_project_limit,
                (
                    SELECT COUNT(*)
                    FROM vault_project vp
                    WHERE vp.organization_id = o.organization_id
                )::INT AS current_ss_project_count,

                o.ss_keys_limit,
                o.ss_notes_limit,

                o.created_at

            FROM organization o
            WHERE o.organization_id = $1
                AND o.is_active = TRUE
            "#,
            &[&organization_id],
        )
        .await?;

    Ok(OrgInfo::from(row))
}


pub async fn update_user_email_verified(db_pool: &PgPool, organization_id: uuid::Uuid, user_id: i64, is_email_verified: bool) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE app_user
            SET is_email_verified = $1
            WHERE organization_id = $2
                AND user_id = $3
            "#,
            &[&is_email_verified, &organization_id, &user_id],
        )
        .await?;

    Ok(result)
}


pub async fn check_username_exists(db_pool: &PgPool, username: &str) -> PgResult<bool> {
    let client = db_pool.get().await?;

    let row = client
        .query_one(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM app_user
                WHERE user_name = $1
            ) AS username_exists
            "#,
            &[&username],
        )
        .await?;

    Ok(row.get("username_exists"))
}


pub async fn update_last_login_at(db_pool: &PgPool, user_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE app_user
            SET last_login_at = NOW()
            WHERE user_id = $1
            "#,
            &[&user_id],
        )
        .await?;

    Ok(result)
}


pub async fn update_user_password(db_pool: &PgPool, user_id: i64, new_password_hash: &str) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE app_user
            SET password_hash = $1, updated_at = NOW()
            WHERE user_id = $2
            "#,
            &[&new_password_hash, &user_id],
        )
        .await?;

    Ok(result)
}


pub async fn update_user_username(db_pool: &PgPool, user_id: i64, new_username: &str) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE app_user
            SET user_name = $1, updated_at = NOW()
            WHERE user_id = $2
            "#,
            &[&new_username, &user_id],
        )
        .await?;

    Ok(result)
}


pub async fn deactivate_user(db_pool: &PgPool, user_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE app_user
            SET is_active = FALSE, updated_at = NOW()
            WHERE user_id = $1
            "#,
            &[&user_id],
        )
        .await?;

    Ok(result)
}


pub async fn update_user_display_name(db_pool: &PgPool, user_id: i64, new_display_name: &str) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE app_user
            SET display_name = $1, updated_at = NOW()
            WHERE user_id = $2
            "#,
            &[&new_display_name, &user_id],
        )
        .await?;

    Ok(result)
}


pub async fn update_user_primary_email(db_pool: &PgPool, user_id: i64, new_primary_email: &str) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE app_user
            SET primary_email = $1, is_email_verified = FALSE, updated_at = NOW()
            WHERE user_id = $2
            "#,
            &[&new_primary_email, &user_id],
        )
        .await?;

    Ok(result)
}


pub async fn list_all_users(db_pool: &PgPool, organization_id: uuid::Uuid, query: &ListUsersReqDto) -> PgResult<Vec<UserProfileListing>> {
    let client = db_pool.get().await?;

    let rows = client.query(
        r#"
        SELECT
            user_id,
            user_name,
            display_name,
            user_details,
            primary_email,
            is_email_verified,
            is_active,
            role_id,
            has_ip_restriction,
            has_geo_restriction,
            allowed_ips,
            allowed_geo_locations,
            has_totp_2fa,
            has_email_2fa,
            created_at,
            updated_at,
            last_login_at
        FROM app_user
        WHERE organization_id = $1
        AND (
            $2::TEXT IS NULL
            OR
            user_name ILIKE '%' || $2 || '%'
        )
        ORDER BY user_name ASC
        LIMIT $3
        OFFSET $4
        "#,
        &[
            &organization_id,
            &query.user_name,
            &query.limit,
            &query.offset
        ],
    )
    .await?;

    Ok(UserProfileListing::from_rows(rows))
}


pub async fn fetch_user_by_id_listing(db_pool: &PgPool, organization_id: uuid::Uuid, user_id: i64) -> PgResult<Option<UserProfileListing>> {
    let client = db_pool.get().await?;

    let row = client.query_opt(
        r#"
        SELECT
            user_id,
            user_name,
            display_name,
            user_details,
            primary_email,
            is_email_verified,
            is_active,
            role_id,
            has_ip_restriction,
            has_geo_restriction,
            allowed_ips,
            allowed_geo_locations,
            has_totp_2fa,
            has_email_2fa,
            created_at,
            updated_at,
            last_login_at
        FROM app_user
        WHERE organization_id = $1
        AND user_id = $2
        "#,
        &[&organization_id, &user_id],
    )
    .await?;

    Ok(row.map(UserProfileListing::from))
}


pub async fn create_user(db_pool: &PgPool, organization_id: uuid::Uuid, password_hash: &str, create_request: &CreateUserReqDto) -> PgResult<u64> {
    let mut client = db_pool.get().await?;

    // Start transaction
    let transaction = client.transaction().await?;

    // 1. Lock organization and retrieve user limit
    let org = transaction
        .query_opt(
            r#"
            SELECT user_limit
            FROM organization
            WHERE organization_id = $1
            FOR UPDATE
            "#,
            &[&organization_id],
        )
        .await?;

    // If the organization does not exist, rollback and return 0
    let Some(org) = org else {
        transaction.rollback().await?;
        return Err(PgError::Custom("Organization not found".into()));
    };
    let user_limit: i32 = org.get("user_limit");

    // 2. Count existing users in the organization
    let current_users: i64 = transaction
        .query_one(
            r#"
            SELECT COUNT(*)::BIGINT
            FROM app_user
            WHERE organization_id = $1
            "#,
            &[&organization_id],
        )
        .await?
        .get(0);

    // 3. Check organization user limit
    if current_users >= user_limit as i64 {
        transaction.rollback().await?;
        return Err(PgError::Custom("User limit reached".into()));
    }

    // 4. Insert only if the role belongs to this organization
    let result = transaction
        .execute(
            r#"
            INSERT INTO app_user (
                organization_id,
                user_name,
                password_hash,
                display_name,
                primary_email,
                user_details,
                ui_info,
                is_active,
                role_id,
                has_ip_restriction,
                has_geo_restriction,
                allowed_ips,
                allowed_geo_locations
            )
            SELECT
                $1, $2, $3, $4, $5, $6,
                '{}'::jsonb,
                $7, $8, $9, $10, $11, $12
            WHERE EXISTS (
                SELECT 1
                FROM user_role
                WHERE organization_id = $1
                  AND role_id = $8
            )
            "#,
            &[
                &organization_id,
                &create_request.user_name,
                &password_hash,
                &create_request.display_name,
                &create_request.primary_email,
                &create_request.user_details,
                &create_request.is_active,
                &create_request.role_id,
                &create_request.has_ip_restriction,
                &create_request.has_geo_restriction,
                &create_request.allowed_ips,
                &create_request.allowed_geo_locations,
            ],
        )
        .await?;

    // 5. Commit transaction
    transaction.commit().await?;

    Ok(result)
}
