-- SQL schema for N-Ops


-- Organization to manage different organizations within N-Ops
CREATE TABLE organization (
    organization_id UUID PRIMARY KEY,
    organization_name VARCHAR(250) UNIQUE NOT NULL, -- Name of the organization (Unique across all organizations)

    -- Metadata about the organization (not indexed)
    organization_info JSONB NOT NULL,
    encryption_key TEXT NOT NULL,  -- AES-256-GCM encryption key (Encrypted with Master Key from environment)

    is_active BOOLEAN DEFAULT TRUE NOT NULL,

    -- Limits for the organization
    user_limit INT,     -- Total number of users allowed in the organization
    server_limit INT,   -- Total number of servers allowed in the organization

    -- Limits for secret keys/stores within the organization (SS - Secret Store)
    ss_project_limit INT,   -- Total number of projects allowed in the organization
    ss_keys_limit INT,      -- Total number of secret keys allowed per project
    ss_file_limit INT,      -- Total number of secret files allowed per project

    -- Notes limit
    user_notes_limit INT,   -- Total number of notes allowed per user

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

    UNIQUE (display_name, organization_id)  -- Unique display name per organization
);


-- API keys for an organization
CREATE TABLE api_key (
    api_key_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- Lookup ID for the API key
    organization_id UUID NOT NULL REFERENCES organization(organization_id) ON DELETE CASCADE,

    key_name VARCHAR(100) NOT NULL,
    key_description TEXT NOT NULL,  -- Description of the API key

    -- Example: '{api_key_id}_{actual_api_key_value}' so that its easy to do lookups and verify the key
    key_value TEXT NOT NULL,  -- HMAC-SHA-256 of the API key value

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
