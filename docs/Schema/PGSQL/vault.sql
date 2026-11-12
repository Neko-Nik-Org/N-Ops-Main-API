-- Vault Project (SS - Secret Store)
CREATE TABLE vault_project (
    project_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES organization(organization_id) ON DELETE CASCADE,

    project_name VARCHAR(100) NOT NULL,
    details JSONB NOT NULL,  -- Additional details for the vault project

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    UNIQUE (project_name, organization_id)
);


-- Vault Project Service
CREATE TABLE vault_service (
    service_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    project_id BIGINT NOT NULL REFERENCES vault_project(project_id) ON DELETE CASCADE,

    service_name VARCHAR(100) NOT NULL,
    details JSONB NOT NULL,  -- Additional details for the vault project service

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    UNIQUE (service_name, project_id)
);


-- Vault Project Service Secret Keys store (Environment variables)
CREATE TABLE vault_secret_keys (
    secret_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    service_id BIGINT NOT NULL REFERENCES vault_service(service_id) ON DELETE CASCADE,

    secret_name VARCHAR(100) NOT NULL,
    current_version BIGINT NOT NULL DEFAULT 1,  -- Current version that is in use

    -- Associated resource tag for the secret key (Only one tag can be associated per secret key)
    resource_tag BIGINT NOT NULL REFERENCES resource_tag(tag_id) ON DELETE CASCADE,

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- Unique secret name per service and resource tag combination
    UNIQUE (secret_name, service_id, resource_tag)
);


-- Vault Project Service Secret Key Versions (Can not edit once created)
CREATE TABLE vault_secret_key_versions (
    secret_id BIGINT NOT NULL REFERENCES vault_secret_keys(secret_id) ON DELETE CASCADE,
    version_number BIGINT NOT NULL,  -- Version number of the secret key

    is_encrypted BOOLEAN NOT NULL DEFAULT TRUE,  -- Indicates if the secret value is encrypted
    secret_value TEXT NOT NULL,  -- Encrypted AES-256-GCM secret value or plain text if not encrypted

    details JSONB NOT NULL,  -- Additional details like created_by, reason for change, etc.

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    PRIMARY KEY (secret_id, version_number)
);


-- Vault Secure Notes
CREATE TABLE vault_secure_notes (
    note_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    project_id BIGINT NOT NULL REFERENCES vault_project(project_id) ON DELETE CASCADE,
    created_by_user_id BIGINT NOT NULL REFERENCES app_user(user_id) ON DELETE CASCADE,

    note_title VARCHAR(100) NOT NULL,
    details JSONB NOT NULL,  -- Additional UI metadata for the secure note

    -- TODO: Is tag required for secure notes?
    is_encrypted BOOLEAN NOT NULL DEFAULT TRUE,  -- Indicates if the note content is encrypted
    note_content TEXT NOT NULL,  -- Encrypted or plain text content of the secure note

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    -- Unique note title per user and project combination
    UNIQUE (note_title, created_by_user_id, project_id)
);


-- Share Vault Secure Notes with other users (Internal sharing)
CREATE TABLE vault_secure_note_internal_shares (
    note_id BIGINT NOT NULL REFERENCES vault_secure_notes(note_id) ON DELETE CASCADE,
    shared_with_user_id BIGINT NOT NULL REFERENCES app_user(user_id) ON DELETE CASCADE,

    -- Use etcd to manage distributed locks for editing permissions
    can_edit BOOLEAN NOT NULL DEFAULT FALSE,  -- Indicates if the shared user can edit the secure note

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    PRIMARY KEY (note_id, shared_with_user_id)
);


-- Share Vault Secure Notes with public (External sharing) [by link]
CREATE TABLE vault_secure_note_external_shares (
    public_share_id VARCHAR(50) PRIMARY KEY,
    note_id BIGINT NOT NULL REFERENCES vault_secure_notes(note_id) ON DELETE CASCADE,

    password_hash TEXT NULL,    -- Hashed password required to access the public share, if any
    expires_at TIMESTAMPTZ NULL,  -- Expiration time for the public share, if any

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
