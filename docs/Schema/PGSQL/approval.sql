-- TODO: This is not finalized, may need adjustments based on approval workflow requirements

-- Not yet done

-- Approval workflow schema for managing approval requests, associated admins, and chat discussions


CREATE TYPE approval_status_enum AS ENUM (
    'PENDING',
    'APPROVED',
    'REJECTED'
);


CREATE TYPE approval_type_enum AS ENUM (
    'ALL_APPROVALS', -- All admins that are requested for approval should approve
    'ANY_APPROVAL', -- Any one of the requested admins can approve
    'HALF_APPROVAL' -- Half of the requested admins need to approve (If 3 then 2 need to approve; If 4 then 2 need to approve)
);


CREATE TABLE approval_main (
    approval_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    requested_by_user_id BIGINT NOT NULL REFERENCES app_user(user_id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organization(organization_id) ON DELETE CASCADE,

    title VARCHAR(100) NOT NULL,

    approval_type approval_type_enum NOT NULL,  -- Type of approval required (e.g., all, any, half)

    linux_commands TEXT[],  -- List of Linux commands related to the approval request
    approved_servers TEXT[],  -- List of server ID's that have been approved

    approved_till TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expired_at TIMESTAMPTZ NOT NULL,    -- When will the approval request expire

    -- Associated resource tag for the secret key (Only one tag can be associated per secret key)
    resource_tag BIGINT NOT NULL REFERENCES resource_tag(tag_id) ON DELETE CASCADE
);


CREATE TABLE approval_chat (
    chat_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    approval_id BIGINT NOT NULL REFERENCES approval_main(approval_id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES app_user(user_id) ON DELETE CASCADE,

    message TEXT NOT NULL,
    details JSONB,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


CREATE TABLE approval_admin (
    approval_id BIGINT NOT NULL REFERENCES approval_main(approval_id) ON DELETE CASCADE,
    admin_user_id BIGINT NOT NULL REFERENCES app_user(user_id) ON DELETE CASCADE,

    approval_status approval_status_enum NOT NULL,
    justification TEXT,  -- Reason or justification for the admin's approval status

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (approval_id, admin_user_id)
);
