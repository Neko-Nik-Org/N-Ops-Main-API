# N-Ops Main API

**Open-source infrastructure control plane by Neko Nik, in partnership with YSPL.**

> One platform. Complete infrastructure control.

-   Main API repository: https://github.com/Neko-Nik-Org/N-Ops-Main-API
-   Project: N-Ops
-   Component: Main API / control-plane backend

## Project status

N-Ops is an evolving project. This README describes the product
direction and intended capabilities; it does not claim that every
feature below is already implemented or production-ready. Check the
current source, release notes, and API documentation for implementation
status and supported configuration.

## What is N-Ops?

N-Ops is an open-source platform for centralized infrastructure
management, Linux server administration, secrets and configuration
management, Docker orchestration, monitoring, auditability, and
automation.

It is designed for developers, system administrators, and infrastructure
teams who want to manage servers, environments, secrets, containers,
files, logs, and remote operations through one control plane.

The project is intended to support self-hosted deployments and a managed
SaaS offering with enterprise support and SLAs. It is designed for small
environments through larger server fleets, with extensibility for
integrations and custom monitoring extensions.

## Planned capabilities

The following describes the product scope, not a guarantee that each
feature is available in the current build.

### Organizations, users, authentication, and RBAC

-   Organization and user management.
-   Fine-grained role-based access control (RBAC).
-   Multi-factor authentication options intended to include email, TOTP,
    physical security keys, FIDO2, and WebAuthn.
-   Approval workflows for sensitive operations.

### Secrets and configuration management

Resources are organized as:

`Organization → Project → Environment → Service → Variables / Files`

-   Centralized encrypted secrets vault per organization.
-   Project and environment separation, including development, staging,
    and production.
-   Environment-variable and configuration-file management.
-   RBAC-controlled secret reveal permissions.
-   Secret version history, rollback, generation, and rotation
    workflows.
-   Secret access audit logs.
-   Bulk variable editing and `.env` import/export.
-   API access, CLI workflows, and cURL examples.
-   Secure agent-side secret retrieval.
-   Client libraries planned for Rust, Python, Go, and JavaScript.
-   Docker Compose secrets support.
-   Ordinary environment variables and write-only/non-revealable
    secrets.
-   Read-only API keys scoped to a project, environment, service, or
    selected variables.

**Cryptography note:** Argon2id is intended for deriving or protecting
key-encryption keys from user-provided material; it is not an encryption
algorithm. A production design should use authenticated encryption,
envelope encryption, and documented key rotation. Secrets must never be
written to logs in plaintext.

### Linux server inventory and agents

-   Register and manage Linux servers through N-Ops agents.
-   Organize servers with names, labels, tags, and groups such as
    `test`, `staging`, and `production`.
-   Collect operating system, kernel, CPU architecture, and hostname
    metadata.
-   Track agent connectivity and last-seen status.
-   Agent reconnection, recovery, and controlled update workflows.

### Docker, Podman, and Compose

-   Discover installed container runtimes.
-   List running and stopped containers (similar to `docker ps -a`).
-   Discover Docker Compose projects and Compose files.
-   View and manage Compose definitions.
-   Start, stop, restart, and pull containers.
-   Inspect images, networks, and volumes.
-   View container resource usage.
-   Manage service-level environment variables and Compose secrets.
-   Deployment rollback workflows.

### SSH and remote execution

-   Remote command execution through the control plane and/or agents.
-   Server grouping and bulk targeting.
-   Execution history, live output, and error reporting.
-   User-name allowlists integrated with authorization policy.
-   Explicit confirmation for dangerous operations.
-   Approval workflows for sensitive commands and changes.
-   Auditable records of who initiated an operation, its target, and its
    outcome.

Remote execution is security-sensitive. Production deployments should
use least-privilege identities, scoped authorization, secure transport,
and explicit policy controls.

### Monitoring, metrics, and extensions

Planned built-in host metrics:

-   CPU, load average, memory, and swap.
-   Disk capacity, filesystem usage, and disk I/O.
-   Network traffic and interface statistics.
-   Uptime and process information.
-   Agent health, connectivity, and last-seen time.

Custom metrics and extension-provided checks are also part of the
product direction. For example, a Postfix extension could report mail
queue depth and trigger an alert when the queue exceeds a configured
threshold.

Additional planned capabilities:

-   Configurable health-check intervals.
-   Custom alert conditions and graphs.
-   Custom metric schemas.
-   Offline-server detection and email alerts.
-   Email as the initial notification channel.
-   Agent reconnection and recovery workflows.

#### Metrics storage direction

The current design direction is to store time-series metrics in
**Parquet** files, including built-in and custom metrics.

-   Collect metrics at a configurable interval (for example, every 10
    seconds).
-   Flush buffered records based on time, size, or another configured
    policy.
-   Rotate files when they reach a configured maximum row count.
-   Initial proposed ceiling: **1,000,000 records per file** (10 lakh).
-   Store schema information so custom metric fields can be represented
    without requiring a dedicated application struct for every
    extension.

The Parquet schema, partitioning, compression, retention policy, and
query/index layer remain implementation decisions. The design should
account for schema evolution, timestamps, server and metric identity,
concurrent writers, and efficient time-range queries.

### Audit logs

-   Search and inspect user and system activity.
-   Record security-sensitive actions and remote operations.
-   Redact secrets and sensitive values.
-   Allow authorized users to download logs from the UI.
-   Do not send audit-log exports by email.

### Remote file management

Planned remote file operations include browsing directories and files,
creating/editing/deleting files, path-level authorization, audit
records, and safeguards for sensitive paths and destructive actions.

### Secure notes

A simple secure-notes capability is planned for operational notes and
related sensitive information, protected by appropriate access controls.

### PostgreSQL operations

PostgreSQL connectivity and query execution are under consideration, not
a confirmed feature contract. If implemented, it should use explicitly
configured connections, least-privilege database accounts, query
auditing, and safeguards for destructive statements.

### Durable jobs and orchestration

N-Ops is intended to provide durable job queues for remote operations
and rollouts:

-   Per-device FIFO execution queues.
-   Job priorities and manual queue reordering.
-   Configurable concurrency limits.
-   Automatic retry with exponential backoff and jitter.
-   Persistence of jobs while a device is offline.
-   Job acknowledgements and execution-state tracking.
-   Live execution output and error reporting.
-   Duplicate-execution protection and idempotency controls.
-   Cancellation, pause, resume, and retry/continue workflows.
-   Job timeouts and permanent-failure/dead-letter states.
-   Per-device visibility into running jobs.
-   Bulk targeting.
-   Staged and canary rollouts with rollback support.
-   Ansible execution/integration.

## Main application navigation

The planned command-center navigation includes:

-   **Servers** --- inventory, agents, groups, and remote operations.
-   **Secret Keys** --- projects, environments, services, secrets, and
    configuration.
-   **Monitoring** --- health, metrics, extensions, and alerts.
-   **Logs** --- searchable audit and execution logs.
-   **Backups** --- backup-related workflows.
-   **Admin** --- organization, users, policies, and settings.
-   **Approvals** --- administrative approval queue for sensitive
    operations.

## Architecture direction

N-Ops is designed around a central control plane and server-side agents:

1.  Users and automation clients interact with the N-Ops API.
2.  The API applies organization, project, resource, and user
    authorization policies.
3.  The control plane stores configuration, job state, audit records,
    and metadata.
4.  Agents receive authorized work and report status, output, inventory,
    and metrics.
5.  Monitoring extensions contribute custom metrics and health checks.
6.  Durable job state allows work to be tracked across retries and
    temporary device disconnections.

This is a conceptual overview, not a guarantee of the current
implementation or wire protocol. Refer to the source and API
documentation for actual service boundaries and contracts.

## Deployment

N-Ops is intended to support self-hosted and managed deployments. Use
the current release's deployment files and documentation for actual
commands, dependencies, ports, volumes, environment variables, and
supported database versions.

### Docker Hub

The project targets Docker Hub distribution. The exact Docker Hub
namespace, image name, published tags, ports, and required environment
variables must be confirmed from the official image listing and release
documentation. Do not assume an image is published simply because Docker
deployment is planned.

The official image name and tag are published at https://hub.docker.com/r/neko7nik/n-ops-main-api
The pull command will follow this pattern:

``` bash
docker pull neko7nik/n-ops-main-api:<TAG>
```

For production deployments:

-   Pin a specific image tag rather than relying on `latest`.
-   Persist application data using documented volumes.
-   Keep secrets out of source control and image layers.
-   Restrict API and agent network access.
-   Back up persistent data and test restoration.
-   Follow the release's upgrade and rollback instructions.

### From source

Source installation instructions will be documented once the runtime,
dependencies, configuration, and supported startup process are
established. Do not rely on guessed commands or environment-variable
names.

## Pricing direction

The proposed managed SaaS pricing is usage-based:

  Plan   Proposed included usage
  ------ ---------------------------------------------------------------
  Free   One user, up to 5 secret-store projects, and up to 10 servers
  Paid   Custom pricing; first 5 projects and 10 servers included

Organization-level limits may also be configurable for users, projects,
services, environment variables/files, and server inventory. These are
proposed prices, not a statement that billing is currently available.
Final pricing, billing intervals, taxes, and plan limits will be
published with the managed service.

## Security

Infrastructure control planes handle highly sensitive credentials and
privileged operations. Security is a core design requirement.

The intended security model includes organization- and resource-scoped
RBAC, MFA, encrypted secret storage and controlled key management,
least-privilege agent/API credentials, scoped read-only tokens, secret
redaction, audit trails, and explicit confirmation/approval for
dangerous operations.

Until a security review and production-readiness statement are
published, evaluate the current implementation carefully before using it
to manage production credentials or infrastructure.

**Never submit real production secrets, private keys, or credentials in
GitHub issues or public discussions.**

## Open source and community

N-Ops is intended to be fully open source, self-hostable, and
community-driven, with development by Neko Nik in partnership with YSPL.

-   **Main API source:** https://github.com/Neko-Nik-Org/N-Ops-Main-API
-   **Organization:** https://github.com/Neko-Nik-Org
-   **Official Website:** https://nops.app

Contributions, bug reports, feature proposals, documentation
improvements, and integrations are welcome. Before opening an issue,
check existing issues and include a clear description, reproduction
steps, relevant logs with secrets removed, and the version or commit you
are using.

## Roadmap

These areas represent product direction, not release commitments:

-   Organization management, RBAC, and MFA.
-   Secrets and configuration management.
-   Linux agent enrollment and server inventory.
-   Docker/Podman and Compose operations.
-   Remote execution, approvals, and audit logs.
-   Host and custom-extension monitoring.
-   Parquet-based metrics storage and retention.
-   Remote file management and secure notes.
-   Durable jobs, staged rollouts, and Ansible integration.
-   Backup workflows and managed SaaS billing.

Track implementation status through repository issues, milestones, and
release notes.

## License

This project is released under the MIT license. See [LICENSE](LICENSE). You are free to use, modify, and distribute this template for any purpose.

------------------------------------------------------------------------

```{=html}
<p align="center">
```
`<strong>`{=html}N-Ops`</strong>`{=html}`<br>`{=html} One platform.
Complete infrastructure control.
```{=html}
</p>
```
