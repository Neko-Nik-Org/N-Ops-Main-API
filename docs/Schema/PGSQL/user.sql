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
