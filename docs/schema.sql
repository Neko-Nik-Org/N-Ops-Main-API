-- SQL schema for N-Ops


-- Organization to manage different organizations within N-Ops
CREATE TABLE organization (
    organization_id UUID PRIMARY KEY,
    organization_name VARCHAR(250) UNIQUE NOT NULL, -- Name of the organization (Unique across all organizations)

    -- Metadata about the organization (not indexed)
    organization_info JSONB NOT NULL,
    -- billing_info JSONB NOT NULL,  -- Billing and GST, SLA and other financial details
    encryption_key TEXT NOT NULL,  -- AES-256-GCM encryption key (Encrypted with Master Key from environment)

    -- Quota for the organization
    allocated_ai_credit BIGINT NOT NULL,  -- Total AI credits allocated to the organization
    utilized_ai_credit BIGINT NOT NULL,  -- AI credits utilized by the organization

    allocated_storage_bytes BIGINT NOT NULL,  -- Total storage allocated to the organization
    utilized_storage_bytes BIGINT NOT NULL,  -- Storage utilized by the organization

    is_active BOOLEAN DEFAULT TRUE NOT NULL,

    -- Limits for the organization
    user_limit INT NOT NULL,     -- Total number of users allowed in the organization
    server_limit INT NOT NULL,   -- Total number of servers allowed in the organization
    workflow_limit INT NOT NULL,   -- Total number of workflows allowed in the organization

    -- Limits for secret keys/stores within the organization (SS - Secret Store)
    ss_project_limit INT NOT NULL,   -- Total number of projects allowed in the organization
    ss_keys_limit INT NOT NULL,      -- Total number of secret keys allowed per project
    ss_notes_limit INT NOT NULL,   -- Total number of notes allowed per project

    -- Additional limits can be added here as needed

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);


-- Tag for categorizing resources within an organization
CREATE TABLE resource_tag (
    tag_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES organization(organization_id) ON DELETE CASCADE,

    tag_name VARCHAR(50) NOT NULL,
    tag_description TEXT NOT NULL,

    UNIQUE (tag_name, organization_id)  -- Unique tag name per organization
);


-- Roles within an organization
CREATE TABLE user_role (
    role_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES organization(organization_id) ON DELETE CASCADE,

    role_name VARCHAR(100) NOT NULL,
    role_description TEXT NOT NULL,

    -- Each role will have Tags associated with it for scoping users, Ref: 'role_tag' table
    -- if not, user accociated with this role will have unrestricted access to all tags
    permissions TEXT[] NOT NULL,    -- e.g., ['user:create', 'user:delete', 'server:read']

    -- Is Admin role (Roles with elevated privileges)
    can_approve_approvals BOOLEAN DEFAULT FALSE NOT NULL,  -- Can the role approve approvals
    can_bypass_approvals BOOLEAN DEFAULT FALSE NOT NULL,  -- Can the role bypass all approvals (Else needs approval)

    -- Linux Users list - Allow only specific Linux users for this role
    allowed_linux_users TEXT[] NOT NULL,  -- List of allowed Linux users for this role

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    UNIQUE (role_name, organization_id)  -- Unique role name per organization
);


-- Role-Tag Many-to-Many relationship (User Role restricted by tags)
CREATE TABLE role_tag (
    role_id BIGINT NOT NULL REFERENCES user_role(role_id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES resource_tag(tag_id) ON DELETE CASCADE,

    PRIMARY KEY (role_id, tag_id)  -- Composite primary key to ensure uniqueness
);


-- Application users within an organization
CREATE TABLE app_user (
    user_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES organization(organization_id) ON DELETE CASCADE,

    user_name VARCHAR(100) NOT NULL UNIQUE,  -- e.g., 'john_doe'
    primary_email VARCHAR(254) UNIQUE NOT NULL,
    display_name VARCHAR(100) NOT NULL,

    password_hash TEXT NOT NULL,    -- Bcrypt hash of the user's password
    user_details JSONB NOT NULL,  -- Just for storing extra info (not indexed)
    ui_info JSONB NOT NULL,  -- e.g., theme, state, etc. (not indexed)

    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    is_email_verified BOOLEAN DEFAULT FALSE NOT NULL,  -- Is email verified

    -- Role assigned to the user within the organization
    role_id BIGINT NOT NULL REFERENCES user_role(role_id) ON DELETE CASCADE,

    -- Geographic or IP-based restrictions for this user (Allowed if either IP or Geo matches)
    has_ip_restriction BOOLEAN DEFAULT FALSE NOT NULL,  -- Is IP restriction enabled for this user
    has_geo_restriction BOOLEAN DEFAULT FALSE NOT NULL,  -- Is geographic restriction enabled for this user
    allowed_ips TEXT[] NOT NULL,  -- List of allowed IP addresses for this user
    allowed_geo_locations TEXT[] NOT NULL,  -- List of allowed geographic locations for this user

    has_totp_2fa BOOLEAN DEFAULT FALSE NOT NULL,  -- Is TOTP 2FA enabled for this user
    has_email_2fa BOOLEAN DEFAULT FALSE NOT NULL,  -- Is Email 2FA enabled for this user
    -- Other 2FA methods (Can use same table structure to manage WebAuthn/FIDO2)
    -- WebAuthn -> Touch ID, Face ID, Windows Hello, Android/iOS Biometrics
    -- FIDO2 -> Security keys like YubiKey, etc.

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    last_login_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,

    UNIQUE (display_name, organization_id)  -- Unique display name per organization
);


-- IP based Geographic Locations (IP range to Country mapping)
CREATE TABLE ip_geographic_locations (
    network CIDR PRIMARY KEY,
    country_iso_code TEXT NOT NULL,
    country_name TEXT NOT NULL
);


-- API keys for an organization
CREATE TABLE api_key (
    api_key_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- Lookup ID for the API key
    organization_id UUID NOT NULL REFERENCES organization(organization_id) ON DELETE CASCADE,

    key_name VARCHAR(100) NOT NULL,
    key_description TEXT NOT NULL,  -- Reason or description for the API key

    -- Example: 'nk-{api_key_id}_base64({actual_api_key_value})' so that its easy to do lookups and verify the key
    key_value TEXT UNIQUE NOT NULL,  -- HMAC-SHA-256 of the API key value

    -- Only has 'Keys vault' & 'Config files' resource permissions (No other resource management allowed programmatically)
    resource_permissions TEXT[] NOT NULL,  -- Resource-specific permissions for the API key

    -- Associated resource tag for the API key (Only one tag can be associated per API key)
    resource_tag BIGINT NOT NULL REFERENCES resource_tag(tag_id) ON DELETE CASCADE,

    is_active BOOLEAN DEFAULT TRUE NOT NULL,

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expires_at TIMESTAMPTZ,  -- Expiration date for the API key (optional)

    UNIQUE (key_name, organization_id)  -- Unique key name per organization
);


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
