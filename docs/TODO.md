Neko Nik Keys Suite Command Center Secrets and Configuration Management Jobs and orchestration

# N-Ops

Neko Nik Keys Suite Command Center Secrets and Configuration Management Jobs and orchestration

- Org + Users with full RBAC
	- 2FA - email,totp,physical-key,fido,WebAuthn

- Secrets, environment variables and configuration with staging seperation and project seperation
	- Centralized secrets vault per org
	- Environment-variable management by (project -> environment -> service -> envs/files)
	- Configuration file management
	- Fully Encrypted secret storage with Argon2-secured keys
	- Secret reveal permissions for users [RBAC]
	- Secret versioning + history + Rollback + generate new
	- Secret access audit logs
	- Bulk variable editing + import .env + export
	- API access to secrets + CLI tool / CURL cmd
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
- Servers
- Secreat Keys
- Monitoring
- Logs
- Backups
- Admin
- Approvals (Admin)


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
