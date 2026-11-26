use super::model::{VaultKeysListing, VaultKeyVersionsListing};
use super::dto::{SkNewDto, SkVersionDto, SkListVersionsDto};
use crate::errors::{PgError, PgPool, PgResult};
use uuid::Uuid;



pub async fn create_new_secret_key(db_pool: &PgPool, organization_id: Uuid, secret_key_data: &SkNewDto, encrypted_secret_value: &str) -> PgResult<u64> {
    let mut client = db_pool.get().await?;

    let transaction = client.transaction().await?;

    // 1. Lock organization and retrieve secret-key quota
    let org = transaction
        .query_opt(
            r#"
            SELECT ss_keys_limit
            FROM organization
            WHERE organization_id = $1
            FOR UPDATE
            "#,
            &[&organization_id],
        )
        .await?;

    let Some(org) = org else {
        return Err(PgError::Custom("Organization not found".into()));
    };

    let keys_limit: i32 = org.get("ss_keys_limit");

    if keys_limit < 0 {
        return Err(PgError::Custom(
            "Invalid organization secret-key quota".into(),
        ));
    }

    // 2. Validate project ownership.
    let project = transaction
        .query_opt(
            r#"
            SELECT project_id
            FROM vault_project
            WHERE project_id = $1
                AND organization_id = $2
            FOR UPDATE
            "#,
            &[
                &secret_key_data.project_id,
                &organization_id,
            ],
        )
        .await?;

    if project.is_none() {
        return Err(PgError::Custom(
            "Project not found or does not belong to this organization".into(),
        ));
    }

    // 3. Count existing secret keys in the project.
    let current_keys: i64 = transaction
        .query_one(
            r#"
            SELECT COUNT(*)::BIGINT
            FROM vault_secret_keys
            WHERE project_id = $1
            "#,
            &[&secret_key_data.project_id],
        )
        .await?
        .get(0);

    // 4. Enforce project secret-key quota.
    if current_keys >= i64::from(keys_limit) {
        return Err(PgError::Custom(
            "Project secret-key limit reached".into(),
        ));
    }

    // 5. Insert secret-key entry.
    let secret = transaction
        .query_opt(
            r#"
            INSERT INTO vault_secret_keys (
                project_id,
                secret_name,
                current_version,
                resource_tag
            )
            VALUES ($1, $2, 1, $3)
            ON CONFLICT (project_id, resource_tag, secret_name)
            DO NOTHING
            RETURNING secret_id
            "#,
            &[
                &secret_key_data.project_id,
                &secret_key_data.secret_name,
                &secret_key_data.tag_id,
            ],
        )
        .await?;

    let Some(secret) = secret else {
        return Err(PgError::Custom(
            "Secret with this name already exists for this project and tag".into(),
        ));
    };

    let secret_id: i64 = secret.get("secret_id");

    // 6. Insert initial secret-key version.
    let result = transaction
        .execute(
            r#"
            INSERT INTO vault_secret_key_versions (
                secret_id,
                version_number,
                is_hidden,
                secret_value,
                details
            )
            VALUES ($1, 1, $2, $3, $4)
            "#,
            &[
                &secret_id,
                &secret_key_data.hide_from_ui,
                &encrypted_secret_value,
                &secret_key_data.details,
            ],
        )
        .await?;

    // 7. Commit transaction.
    transaction.commit().await?;

    Ok(result)
}


pub async fn create_new_sk_version(db_pool: &PgPool, organization_id: Uuid, secret_kv_data: &SkVersionDto, encrypted_secret_value: &str) -> PgResult<u64> {
    let mut client = db_pool.get().await?;

    let transaction = client.transaction().await?;

    // 1. Validate project ownership.
    let project = transaction
        .query_opt(
            r#"
            SELECT project_id
            FROM vault_project
            WHERE project_id = $1
                AND organization_id = $2
            "#,
            &[
                &secret_kv_data.project_id,
                &organization_id,
            ],
        )
        .await?;

    if project.is_none() {
        return Err(PgError::Custom(
            "Project not found or does not belong to this organization".into(),
        ));
    }

    // 2. Validate secret key ownership and lock the row.
    let secret = transaction
        .query_opt(
            r#"
            SELECT secret_id
            FROM vault_secret_keys
            WHERE secret_id = $1
                AND project_id = $2
                AND resource_tag = $3
            FOR UPDATE
            "#,
            &[
                &secret_kv_data.secret_id,
                &secret_kv_data.project_id,
                &secret_kv_data.tag_id,
            ],
        )
        .await?;

    if secret.is_none() {
        return Err(PgError::Custom(
            "Secret key not found or does not belong to the specified project and resource tag".into(),
        ));
    }

    // 3. Fetch the latest available version.
    let latest_version = transaction
        .query_one(
            r#"
            SELECT COALESCE(MAX(version_number), 0) AS latest_version
            FROM vault_secret_key_versions
            WHERE secret_id = $1
            "#,
            &[&secret_kv_data.secret_id],
        )
        .await?;

    let latest_version: i64 = latest_version.get("latest_version");

    // 4. Calculate the next version.
    let new_version = latest_version.checked_add(1).ok_or_else(|| {
        PgError::Custom("Secret version limit reached".into())
    })?;

    // 5. Insert the new secret version.
    transaction
        .execute(
            r#"
            INSERT INTO vault_secret_key_versions (
                secret_id,
                version_number,
                is_hidden,
                secret_value,
                details
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
            &[
                &secret_kv_data.secret_id,
                &new_version,
                &secret_kv_data.hide_from_ui,
                &encrypted_secret_value,
                &secret_kv_data.details,
            ],
        )
        .await?;

    // 6. Update current version.
    let updated = transaction
        .execute(
            r#"
            UPDATE vault_secret_keys
            SET current_version = $1,
                updated_at = CURRENT_TIMESTAMP
            WHERE secret_id = $2
            "#,
            &[
                &new_version,
                &secret_kv_data.secret_id,
            ],
        )
        .await?;

    // 7. Commit transaction.
    transaction.commit().await?;

    Ok(updated)
}


pub async fn list_secret_keys(db_pool: &PgPool, organization_id: Uuid, project_id: i64, tag_id: i64, limit: i64, offset: i64) -> PgResult<(i64, Vec<VaultKeysListing>)> {
    let client = db_pool.get().await?;

    let total_count = client
        .query_one(
            r#"
            SELECT COUNT(*) AS total_count
            FROM vault_secret_keys sk
            INNER JOIN vault_project vp
                ON vp.project_id = sk.project_id
            WHERE sk.project_id = $1
                AND sk.resource_tag = $2
                AND vp.organization_id = $3
            "#,
            &[
                &project_id,
                &tag_id,
                &organization_id,
            ],
        )
        .await?;

    let total_count: i64 = total_count.get("total_count");
    if total_count == 0 {
        return Ok((0, vec![]));
    }

    // Fetch secret keys with pagination
    let rows = client
        .query(
            r#"
            SELECT
                sk.secret_id,
                sk.secret_name,
                sk.current_version,
                sk.created_at,
                sk.updated_at,
                skv.is_hidden
            FROM vault_secret_keys sk
            INNER JOIN vault_secret_key_versions skv
                ON skv.secret_id = sk.secret_id
                AND skv.version_number = sk.current_version
            WHERE sk.project_id = $1
                AND sk.resource_tag = $2
            ORDER BY sk.secret_name
            LIMIT $3
            OFFSET $4
            "#,
            &[
                &project_id,
                &tag_id,
                &limit,
                &offset,
            ],
        )
        .await?;

    Ok((total_count, VaultKeysListing::from_rows(rows)))
}


pub async fn delete_secret_key_and_versions(db_pool: &PgPool, organization_id: Uuid, project_id: i64, tag_id: i64, secret_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let deleted = client
        .execute(
            r#"
            DELETE FROM vault_secret_keys sk
            USING vault_project vp
            WHERE
                sk.secret_id = $1
                AND sk.project_id = $2
                AND sk.resource_tag = $3
                AND vp.project_id = sk.project_id
                AND vp.organization_id = $4
            "#,
            &[
                &secret_id,
                &project_id,
                &tag_id,
                &organization_id,
            ],
        )
        .await?;

    Ok(deleted)
}


pub async fn list_key_versions(db_pool: &PgPool, organization_id: Uuid, sk_query_params: &SkListVersionsDto) -> PgResult<(i64, Vec<VaultKeyVersionsListing>)> {
    let client = db_pool.get().await?;

    // 1. Count total versions.
    let total_count = client
        .query_one(
            r#"
            SELECT COUNT(*) AS total_count
            FROM vault_secret_keys sk
            INNER JOIN vault_project vp
                ON vp.project_id = sk.project_id
                AND vp.organization_id = $4
            INNER JOIN vault_secret_key_versions skv
                ON skv.secret_id = sk.secret_id
            WHERE sk.project_id = $1
                AND sk.resource_tag = $2
                AND sk.secret_id = $3
            "#,
            &[
                &sk_query_params.project_id,
                &sk_query_params.tag_id,
                &sk_query_params.secret_id,
                &organization_id,
            ],
        )
        .await?;

    let total_count: i64 = total_count.get("total_count");

    if total_count == 0 {
        return Ok((0, vec![]));
    }

    // 2. Fetch secret key versions with pagination.
    let rows = client
        .query(
            r#"
            SELECT
                skv.version_number,
                skv.is_hidden,
                skv.details,
                skv.created_at
            FROM vault_secret_keys sk
            INNER JOIN vault_project vp
                ON vp.project_id = sk.project_id
                AND vp.organization_id = $5
            INNER JOIN vault_secret_key_versions skv
                ON skv.secret_id = sk.secret_id
            WHERE sk.project_id = $1
                AND sk.resource_tag = $2
                AND sk.secret_id = $3
            ORDER BY skv.version_number DESC
            LIMIT $4
            OFFSET $6
            "#,
            &[
                &sk_query_params.project_id,
                &sk_query_params.tag_id,
                &sk_query_params.secret_id,
                &sk_query_params.limit,
                &organization_id,
                &sk_query_params.offset,
            ],
        )
        .await?;

    Ok((total_count, VaultKeyVersionsListing::from_rows(rows)))
}


pub async fn read_secret_key_by_version(db_pool: &PgPool, organization_id: Uuid, secret_id: i64, version_number: i64) -> PgResult<Option<SkVersionDto>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                sk.project_id,
                sk.secret_id,
                sk.resource_tag AS tag_id,

                skv.secret_value,
                skv.is_hidden AS hide_from_ui,
                skv.details

            FROM vault_secret_key_versions skv

            INNER JOIN vault_secret_keys sk
                ON sk.secret_id = skv.secret_id

            INNER JOIN vault_project vp
                ON vp.project_id = sk.project_id
                AND vp.organization_id = $3

            WHERE
                sk.secret_id = $1
                AND skv.version_number = $2
            "#,
            &[
                &secret_id,
                &version_number,
                &organization_id,
            ],
        )
        .await?;

    Ok(row.map(SkVersionDto::from))
}


pub async fn set_current_version(db_pool: &PgPool, secret_id: i64, version_number: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE vault_secret_keys
            SET current_version = $2, updated_at = NOW()
            WHERE secret_id = $1
            "#,
            &[
                &secret_id,
                &version_number,
            ],
        )
        .await?;

    Ok(result)
}
