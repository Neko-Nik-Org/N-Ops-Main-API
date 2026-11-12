# N-Ops

Neko Nik Keys Suite Command Center Secrets and Configuration Management Jobs and orchestration

- Org + Users with full RBAC
	- 2FA - email,totp,physical-key,fido,WebAuthn

- Secrets, environment variables and configuration with tag separation and project separation
	- Centralized secrets vault per org
	- Environment-variable management
        - project (limited) -> service (unlimited) -> envs/files [tagged] (limited)
        - ENV's / Files will have tagging support, so that no need to have explicit environment references
	- Configuration file management
	- Fully Encrypted secret storage with AES-256-GCM (Authenticated Encryption with Associated Data)
	- Secret reveal permissions for users [RBAC]
	- Secret versioning + history + Rollback + generate new
	- Secret reveal access audit logs
	- Bulk variable editing + import .env + export
	- API access to secrets + CLI tool / CURL cmd + Docker integration
	- Agent-side secure secret retrieval
	- Programming language libs too (Rust, Python, Go, JS)
	- Support Docker Compose secrets
	- support both conventional environment variables and proper secrets (non viewable too)
	- Simple API key based RO permission for specific project or environment or service or envs

- Linux servers, agents and inventory, lables/names, tags/groups like Test / Prod server etc
	- From the Agent get and store OS, kernel, architecture and hostname information

- Docker Compose and container management
	- List installed Docker instances + docker/podman ps -a like that too
	- Detect Docker Compose projects + Discover Compose files + CRUD on that
	- View container resource usage
	- Start / stop / restart / pull containers
	- Inspect images, networks and volumes
	- Manage environment variables per service
	- Support Docker Compose secrets
	- Support deployment rollback

- SSH and remote command execution full support from UI with Audit logs
	- Server grouping and bulk targeting
	- Agent-side execution history
	- User name based access like from allowed user names list only they can access [RBAC]
	- Explicit confirmation for dangerous operations
	- Approval workflows for sensitive changes
    - https://xtermjs.org - Pseudo terminal in the browser (PTY)

- Health, system metrics and monitoring + extentions support (like postfix mailq)
	- Call health check every x seconds last-seen status + custom Alerts
	- Custom extention alerts also like mailq >100 means like that too
	- Basic CPU, RAM, Swap, Disk, IO, Network, Load, uptime, proc info etc metrics graphs
	- Support to add new graphs too for custom extentions
	- Alerts by E-Mail only
	- Offline detection and then alert
	- Auto Agent reconnect and recovery
	- Automatic agent update mechanism

- Search, view and audit logs of each user action
	- Download logs from UI only (no export by email)
	- Sensitive-data redaction

- Remote file manager and editor and creator from UI any file in that system
    - Transfer files from local to remote and vice versa
    - Transfer files from one remote system to another

- Simple secure notes
    - First release will have simple note creation and management
    - Sharing internally within the organization (Read-only)
    - Sharing externally via link with optional expiration and password protection
    - Non-encrypted storage with password (on set - encrypt the note with the password)
        - If password is not provided, store as plain text (Means mostly not so important)

- Connect to PGSQL and execute stuff ?

- Link to GitHub account (for integration with repositories and actions)

- Jobs - Durable task queues and rollouts with pause/stop/kill support
	- Per-device FIFO execution queues
	- Job priorities and change of order manually from UI
	- Concurrent execution limits
	- Automatic retry with exponential backoff and jitter
	- Offline device job persistence
	- Job acknowledgements + status and Execution output live feed + error reportingz
	- Duplicate execution protection
	- Job cancellation + Pause and resume + Continue failed jobs
	- Job timeout handling
	- Permanent failure / dead-letter state
	- Per-device execution status like jobs are running on this server indications
	- Bulk job targeting
	- Staged and canary rollouts + Rollback support
	- Run Ansible


---


Similar projects

- https://opsfabric.io/

- https://infisical.com/

- https://envs.ai/welcome

- https://dockhand.pro/

- https://render.com/

- https://apps.umbrel.com/app/termix


---


Pricing plan

- Unlimited users

- Secret store projects $5 per project

- Server inventory $3 per server

- Free basic tire = Single user + 5 Projects + 10 Servers

- If paid then Unlimited users and any 5+ projects charges $5 per and any 10+ servers changes $3 per

- Can limit the following per org
	- Number of users
	- Secret store projects
	- Secret store services 
	- Secret store env var limit / file limit
	- Server list/add limit

---

Side panel will have

- Admin
    - Organization (RU)
    - Users (CRUD)
    - API Keys (CRUD) (Use same roles no need to make new)
    - Roles (CRUD)
    - Tags (CRUD)
        - View and manage organization tags
    - Audit Logs (Read-only)
        - All audit logs (viewable and filterable)
        - Export/Download audit logs
        - Filter audit logs by date, user, or action type
        - Only for 90 days
        - Can auto export audit logs periodically (Use the Workflow system!)

- Servers (Single view - Admin kind)
    - CRUD - Servers by tag
    - Have server type and Join ID (Unique identifier for joining the server to the system)
    - Allow remote command execution
        - Allow SSH access
        - Linux User names and their corresponding tag ? (but then what about server tag ?)
        - How to give SSH access to normal users based on their access level
            - Lets say i want to give access to user A with only "user_a" user name then how ?
        - Require Admin approval for sensitive commands (Bool)
            - Can list sensitive commands requiring admin approval
    - Allow file transfer
        - Require Admin approval for File Read/Create/Edit/Delete (Bool's for each)
        - Allow file download
        - Allow file upload
    - Allow monitoring
    - Has container management capabilities (Bool)
        - Type of container management (e.g., Docker, Podman)
        - Container management actions allowed (e.g., start, stop, restart, remove)
            - Specify which container management actions are allowed (e.g., create, start, stop, restart, remove)
            - Require Admin approval for container management actions (Bool)

- Tools (Select one server or multiple servers)
    - List available servers for remote command execution and Select one server or multiple servers
    - How to do this UI ? I dont know

    - Remote command execution
        - Select one server for SSH
        - Select multiple servers for parallel remote command execution
        - Save bundle of commands for reuse ? (Like a template or script?)
    - Docker containers management (On selected servers)
        - Can handle based on the selection
    - Normal system services management (htop kind of view?)
        - Can handle based on the selection
    - File management (local -> remote and vice versa + remote -> remote + remote editing + copy from Secrets config files)
        - Can handle based on the selection

- Service Management
    - Postgres management (CRUD + Connect and execute simple queries)
    - Add more services later in the future (like RabbitMQ, Redis, etc.)

- Vault - (Secrets & config) (should have versioning)
    - Keys Store
    - Secure Notes
    - Config files (Simple file storage)

- Approvals
    - Create an approval by any user under one tag and optional server or servers
    - Approvals can be pending, approved, or rejected
    - Approvals should have an expiration time or validity period
    - Users can Chat there regarding the approval status and discussions (like a chat thread)
    - Users can attach files or references to the approval for additional context
    - Users can mention other users to notify them about the approval status or discussions (like @username)
    - On approval status change (e.g., approved, rejected)
        - Notify relevant users
        - Update the chat thread with the status change
        - Log the status change for auditing purposes
        - Trigger any dependent workflows or actions based on the status change
        - Dont allow further actions on the approval once it has reached a final status (approved or rejected)
    - Only Admins with appropriate approval permissions can modify the approval once it has been created
    - No one can modify or delete an approval once it has reached a final status (approved or rejected)
    - Auto delete of approvals will occur after their expiration time or validity period has passed
        - After 90 days from the approval's creation date, it should be automatically deleted

- Metrics / Monitoring (Infrastructure & Custom)
    - Send a JSON payload for custom metrics to the monitoring endpoint
    - CRUD infrastructure and custom monitoring metrics
        - View and filter metrics
        - Export/Download metrics
        - Show graphs, visualizations and trends

- Workflows (Automation)
    - Create and manage automation workflows (using reactflow.dev)
    - This is the Flow-based automation system for orchestrating tasks and workflows
    - At any point, workflows can use the available components like Vault, Metrics, Remote Execution, and self-defined components to build complex automation logic

    - This can do something like
        1. Every 10 seconds -> Check server health -> Trigger alert if unhealthy
        2. Every 1 minute -> Check Mail Queue -> Parse output in integer -> If threshold exceeded -> Trigger alert
        3. If Metrics have met certain conditions -> Parse some values -> Check on server -> Trigger alert with relevant details

    - Workflows components (building blocks for automation)
        - Alert
            - Single/Multiple connections accepted
                - Should have Subject and Body fields + Additional Files are optional
            - Send Email notifications with some body, subject and recipients and attachments if needed
            - Make API calls, with customizable request body, headers and method
            - Send User based notifications (App notifications), select self org user names and define the message content
            - Nothing will connect after this. This is the end step.
        - If condition
            - Can read JSON input and extract keys and values for condition evaluation
            - If input meets certain conditions
                - Boolean conditions (true/false)
                - Comparison conditions (>, <, >=, <=, ==, !=)
                - Logical conditions (AND, OR, NOT)
                - Nested conditions (combination of boolean, comparison, and logical conditions)
                - String conditions (e.g., contains, starts with, ends with, matches regex)
                - Numeric conditions (e.g., within range, greater than, less than)
                - Date/Time conditions (e.g., before, after, within a specific range)
                - Presence conditions (e.g., exists, does not exist)
                - Custom conditions (e.g., user-defined logic or scripts)
                - Combination of multiple conditions using logical operators (AND, OR, NOT)
            - Give Boolean output based on the evaluation of the conditions
        - Remote Execution on server(s)
            - Given input should be JSON - Key-Value or other structured format
                - Can use that in %KEY for command execution
            - Execute commands on remote server/servers
            - Capture and return the output of executed commands
            - Can fail with an error or exception, which should be captured and handled appropriately
            - Will wait for Admin approval if required before executing commands (Or admin can add an exception to bypass approval)
            - Can set the return type (e.g., bool, int, JSON, raw text) on successful execution (Auto parse)
            - Can use the available secrets from Vault
            - If multiple servers are specified, execute commands by each server individually or in parallel or canary deployment strategies
            - Flag to need output or not (e.g., whether to capture and return the output of executed commands)
        - Run Ansible playbooks on remote server/servers
            - Specify the playbook file and server(s) to run it on
            - Specify any extra variables or options required for the playbook execution
            - Can fail with an error or exception, which can be captured and handled appropriately
            - Can use the available secrets from Vault
            - No output will be returned because Ansible playbooks typically handle their own output internally
            - Can not connect from this next step
            - Only Error connect is possible
        - AI
            - Perform AI-based tasks or predictions
            - Specify the AI model and input data
            - Capture and return the AI model's output
            - Can fail with an error or exception, which should be captured and handled appropriately
            - Can NOT use the available secrets from Vault
            - Output will always be string (If JSON then it should be parsed using the parser step)
        - API Calls
            - Make API calls with customizable request body, headers, and method
            - Capture and return the response of API calls
            - Can fail with an error or exception, which should be captured and handled appropriately
            - Will wait for Admin approval if required before making API calls (Or admin can add an exception to bypass approval)
            - Can set the return type (e.g., bool, int, JSON, raw text) on successful execution (Auto parse)
            - Can use the available secrets from Vault
        - Parser
            - Given an input of raw text, JSON, or other structured data
            - Output the required fields or transformed data based on the parsing logic
            - Like input is string "{'key1': 'value1'}" and user wants to convert it to JSON object
            - Like input is string "  321 " and user wants to convert it to an integer etc.
            - Can handle parsing errors and exceptions appropriately
            - Always return a consistent output format

- Service Logs (all custom, user-defined) [If possible]
    - View and filter service logs
    - Export/Download service logs
    - Add new service logs


---


# N-Ops

**Open-source Infrastructure Control Plane by Neko Nik partner with YSPL**

N-Ops is an open-source platform for centralized infrastructure management, server administration, secrets management, configuration deployment, Docker orchestration and automation.

Manage Linux servers, environments, secrets, containers, files, logs, monitoring and remote operations from one unified interface.

Built for developers, system administrators and infrastructure teams who want complete control over their infrastructure without relying on expensive proprietary platforms.

**One platform. Complete infrastructure control.**

- Fully open source
- Self-hostable and Managed Saas with Entp. SLA's&#x20;
- Free and community-driven backed by huge org YSPL
- No artificial feature restrictions, simple pricing for managed service
- Designed for automation and small to large-scale infrastructure
- Extensible with plugins and integrations for various tools and services


---

How to store the metrics from the infrastructure and applications?
Save in parquet file and allow every 10 seconds or any logic but max of 10L records in one file 
Metrics and also custom metrics too with custom schema its all possible no need to build struct i think
