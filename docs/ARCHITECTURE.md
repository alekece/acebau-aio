# Architecture

## 1. Purpose

This document defines the technical architecture for the application.

The goal is to build a modular internal business-management platform that can support different professional activity from the same codebase.

The initial primary use case is 3D printing and freelance activity management, but the architecture must remain reusable for other professional activity profiles.

The architecture favors:

- modularity;
- strong compile-time guarantees;
- explicit module dependencies;
- a single repository;
- a single Rust workspace;
- a single deployable backend API;
- a single frontend application;
- PostgreSQL as the main database;
- deployment both locally and in the cloud;
- maintainability over unnecessary abstraction;
- modern Rust tooling without introducing distributed-system complexity.

The application is intentionally designed as a **modular monolith**, not as a microservice architecture.

---

## 2. High-Level Architecture

The system consists of:

- a SvelteKit frontend;
- a Rust backend API;
- PostgreSQL;
- a Rust CLI for administration and maintenance;
- optional first-party business modules selected through Cargo features.

```text
Browser
   │
   ▼
SvelteKit frontend
   │
   │ GraphQL / HTTP
   ▼
Rust API
Axum + async-graphql
   │
   ▼
PostgreSQL
```

The application must support both:

- local/private deployment;
- cloud deployment.

The same application architecture should be usable in both environments.

---

## 3. Technology Stack

### Frontend

- SvelteKit
- TypeScript

SvelteKit is preferred over a Rust frontend for now because the application contains a large amount of:

- forms;
- tables;
- dashboards;
- charts;
- filters;
- complex interactive business interfaces.

A Rust frontend may be reconsidered later if the ecosystem becomes sufficiently mature for this type of application.

### Backend

- Rust
- Tokio
- Axum
- async-graphql

Axum is the default HTTP framework.

The backend exposes:

- one unified GraphQL API for application/business data;
- standard HTTP endpoints where GraphQL does not provide value.

Examples of non-GraphQL endpoints include:

```text
GET /health
GET /ready
GET /metrics
GET /files/...
POST /files/...
```

### Database

- PostgreSQL
- SQLx

PostgreSQL is the default and authoritative relational database.

SQLx is used directly by modules through the shared database abstraction.

No ORM is required.

---

## 4. Repository Structure

All Rust code must live under `crates/`.

The repository should approximately follow this structure:

```text
.
├── Cargo.toml
├── ARCHITECTURE.md
├── AGENTS.md
├── README.md
│
├── crates/
│   ├── api/
│   ├── auth/
│   ├── cli/
│   ├── config/
│   ├── core/
│   ├── database/
│   ├── unit/
│   ├── unit-derive/
│   │
│   └── modules/
│       ├── activity/
│       ├── finance/
│       ├── product/
│       ├── variant/
│       ├── part/
│       ├── material/
│       ├── inventory/
│       ├── machine/
│       ├── production/
│       ├── order/
│       ├── reseller/
│       ├── analytics/
│
└── web/
    └── SvelteKit application
```

The exact module list may evolve according to the product specification.

---

## 5. Architectural Style

The backend is a **modular monolith**.

A module generally corresponds to a user-facing business capability or page.

Examples:

- product;
- variant;
- inventory;
- machine;
- production;
- finance;
Modules are intentionally granular.

A module may expose more than one route or view when they belong to the same logical capability, but the default module granularity should remain close to a user-facing feature.

Each module must live in its own crate.

---

## 6. Module System

Modules are first-party crates stored inside the repository.

There is no third-party plugin system.

There is no dynamic module loading.

Modules are enabled at **compile time** through Cargo features.

Example:

```toml
[features]
activity = ["dep:activity"]

finance = [
    "dep:finance",
    "activity",
]

product = ["dep:product"]

variant = [
    "dep:variant",
    "product",
]

order = [
    "dep:order",
    "product",
]

inventory = ["dep:inventory"]

machine = ["dep:machine"]

production = [
    "dep:production",
    "variant",
    "inventory",
    "machine",
]
```

### Module dependencies

Cargo features are the source of truth for module dependencies.

If a module requires another module, that dependency must be represented directly in `Cargo.toml`.

Example:

```toml
order = [
    "dep:order",
    "product",
]
```

This guarantees that enabling `order` automatically enables `product`.

The application should not implement a second custom module dependency system unless a future requirement makes it necessary.

---

## 7. Mandatory Platform Crates

Not everything is an optional module.

The following crates are part of the platform itself and are always available:

```text
api
auth
cli
config
core
database
unit
unit-derive
```

Business functionality lives under:

```text
crates/modules/
```

`activity` and `finance` are modules and are therefore optional.

Authentication is mandatory.

---

## 8. Core Crate

`core` contains stable domain primitives and concepts shared by more than one module.

It must not become a general-purpose dumping ground.

Examples of acceptable shared concepts include:

- identifiers;
- money types;
- timestamps;
- shared statuses;
- addresses;
- contact-like value objects;
- pagination primitives;
- common domain errors;
- small shared traits.

A business entity whose lifecycle belongs clearly to one module should remain in that module.

Examples:

- `Product` belongs to `product`;
- `Invoice` belongs to `finance`;
- `Activity` belongs to `activity`;
- `Machine` belongs to `machine`.

### Dependency rule

`core` must remain low in the dependency graph.

It must not depend on:

- `api`;
- `auth`;
- `database`;
- business modules.

---

## 8. Unit Crates

Reusable typed units must live in dedicated crates:

```text
crates/unit/
crates/unit-derive/
```

The existing `unit-derive` implementation should be reused.

The unit abstraction is especially important for 3D-printing-related modules.

Possible unit types include:

- mass;
- length;
- duration;
- volume;
- energy;
- filament weight;
- print time;
- dimensions.

Business modules should use typed units instead of loosely passing raw numeric values where practical.

---

## 8. API Architecture

The primary application API is GraphQL.

Implementation:

- Axum;
- async-graphql.

The application exposes a **single unified GraphQL schema**.

Each enabled module contributes its own:

- GraphQL queries;
- GraphQL mutations;
- GraphQL types;
- resolvers.

The frontend sees one GraphQL graph rather than separate APIs per module.

Example:

```graphql
type Query {
  product(...): ProductConnection!
  product(id: ID!): Product

  order(...): OrderConnection!
  order(id: ID!): Order

  invoices(...): InvoiceConnection!
  activity: [Activity!]!
}
```

Cross-module relationships may appear naturally in the unified schema.

Example:

```graphql
query {
  order(id: "...") {
    number

    activity {
      name
    }

    items {
      quantity

      variant {
        sku

        product {
          name
        }
      }
    }
  }
}
```

GraphQL is the transport layer, not the domain architecture.

Business logic must not live only inside resolvers.

Resolvers should remain thin and delegate to application/domain logic.

---

## 8. GraphQL Composition

Schema composition happens inside the single Rust backend process.

GraphQL Federation is not required.

The application currently has:

```text
one repository
one Rust API process
one PostgreSQL database
one deployment unit
```

Federation would introduce unnecessary complexity.

Only compiled modules contribute to the final schema.

If a feature is not enabled, its GraphQL surface is not compiled into that build.

---

## 8. HTTP Endpoints Outside GraphQL

GraphQL should not be forced onto technical or binary-oriented use cases.

Standard HTTP endpoints should be used when they are a better fit.

Typical examples:

```text
GET  /health
GET  /ready
GET  /metrics

GET  /files/{...}
POST /files/{...}
```

Authentication endpoints may also use conventional HTTP if this results in a simpler and safer implementation.

---

## 8. Database Architecture

The `database` crate owns shared database infrastructure.

It may provide:

- PostgreSQL pool creation;
- SQLx configuration;
- transaction abstractions;
- shared SQLx helpers;
- migration execution infrastructure;
- error conversion;
- test database helpers.

It must not own all application persistence logic.

### Module-owned persistence

Each module owns its own SQLx persistence code and database migrations while using the abstractions provided by `database`.

The architecture intentionally does **not** prescribe a mandatory internal folder/layer structure for modules at this stage. The concrete separation of domain, application, persistence and API concerns should be decided during implementation when the actual module requirements are known.

The dependency direction is:

```text
module
  │
  ▼
database
  │
  ▼
SQLx
  │
  ▼
PostgreSQL
```

The `database` crate must not depend on business modules.

---

## 8. Migrations

Each module owns the migrations for its own database objects.

When an application build contains a module, its migrations may be executed automatically during server startup.

Important rule:

**Normal application startup must never perform destructive rollback or cleanup.**

If a module has been enabled previously and later disappears from a new build:

- its tables remain in PostgreSQL;
- its data remains intact;
- normal startup does not delete anything.

This allows the module to be re-enabled later without losing historical data.

Destructive operations belong exclusively to explicit CLI commands.

---

## 8. CLI

The CLI is a Rust crate:

```text
crates/cli/
```

It should compile with the same feature set as the API where practical so it has access to the administrative capabilities of the modules included in that build.

The exact CLI commands and internal organization are intentionally not prescribed yet. They should be introduced as concrete administrative needs appear.

The CLI is expected to cover platform and database administration such as:

- database setup and migration operations;
- explicit destructive database maintenance;
- initial user bootstrap;
- other maintenance operations required by enabled modules.

Destructive commands must always require clear explicit intent and must never be triggered implicitly by normal API startup.

---

## 8. Authentication

Authentication is required for production usage, but it is exposed as a Cargo feature named:

```text
auth
```

Development builds may omit the `auth` feature to make local iteration easier.

Release builds must refuse to compile when `auth` is not enabled.

Conceptually:

```rust
#[cfg(all(not(debug_assertions), not(feature = "auth")))]
compile_error!("release builds require the `auth` feature");
```

When `auth` is enabled, the default architecture should use:

- local user account authentication;
- secure password hashing;
- Argon2id or an equivalent modern password hashing scheme;
- server-side sessions;
- secure HttpOnly cookies.

The purpose of authentication is to safely expose the personal tool over the internet, including access from mobile devices outside the local network.

Authentication must not introduce an unnecessary permission/role system.

---

## 8. Access Model and Connection Logging

The application is a personal tool.

There is no role, capability or permission model.

Once authenticated, the user may access all capabilities compiled into the application.

The authentication layer should record security-relevant access information so connections can be reviewed later.

At minimum, consider recording:

- successful logins;
- failed login attempts;
- logout/session termination;
- timestamp;
- source IP when available and appropriate;
- user agent/device information where useful.

The exact storage and UI for connection history may be decided during implementation, but authentication activity must be inspectable somewhere in the application or through an administrative interface.

---

## 8. Activities

`activity` is an optional module.

It represents business activity inside a single application installation.

Example:

```text
3D printing
Freelance
Future activity
```

Activities are not tenants.

The architecture is not multi-tenant by default.

One installation may contain multiple activity.

Other modules may optionally depend on the `activity` module.

For example, `finance` may depend on `activity`.

Financial records can then be filtered and aggregated by activity.

The model should remain generic and must not hardcode values such as:

```text
3D printing
Freelance
other professional use cases
```

Activity types should only be introduced if they provide real business value.

---

## 8. Finance and Activities

The finance module is optional and should generally depend on activity.

Examples of activity-aware financial data include:

- invoices;
- expenses;
- revenues;
- payments;
- taxes;
- financial analytics.

Finance views should support:

- all activity combined;
- filtering by one activity;
- filtering by several activity where useful.

Activities are business dimensions, not feature toggles.

Modules determine which capabilities exist.

Activities classify business data.

---

## 8. Module Data Relationships

Modules may reference each other.

Modularity does not imply database isolation.

When a real relational dependency exists, PostgreSQL foreign keys should be used normally.

Examples:

```text
order -> product
order -> variant
production -> variant
production -> machine
production -> inventory
finance -> activity
```

Compile-time feature dependencies must mirror required application relationships.

Example:

```toml
production = [
    "dep:production",
    "variant",
    "inventory",
    "machine",
]
```

If `production` requires `variant`, a build containing production automatically contains variant.

---

## 8. Frontend Modularity

The frontend is a single SvelteKit application.

SvelteKit does not have a Cargo-style native feature system. Frontend modularity should therefore not introduce an independent module dependency model.

The Rust/Cargo feature selection remains the authoritative source of which business capabilities are part of a build.

The frontend build should receive the selected module set through generated or build-time configuration.

SvelteKit supports build-time environment variables through `$env/static/*`; these values are statically injected during the build and can participate in dead-code elimination.

The exact synchronization mechanism may be implemented through:

- generated TypeScript configuration;
- build-time environment variables;
- another simple build step derived from the Rust feature profile.

There must not be two independently maintained sources of truth for module availability.

Frontend module availability should determine:

- navigation entries;
- available pages;
- dashboard widgets;
- actions shown by the UI.

The implementation should favor simple compile/build-time composition over a custom frontend plugin framework.

---

## 8. Dashboard Composition

The dashboard must be able to differ depending on available capabilities and business profile.

Examples:

A 3D-printing-oriented installation may expose:

- production backlog;
- stock alerts;
- machine utilization;
- sales;
- margins;
- defect rates.

Dashboard composition must therefore remain modular.

Dashboard widgets should be associated with the capabilities/modules that own them.

---

## 8. Deployment

The application must remain deployment-agnostic.

It should support:

- local development;
- deployment on a private/home server;
- VPS deployment;
- cloud deployment.

The architecture should favor containerized deployment.

Typical runtime:

```text
SvelteKit
Rust API
PostgreSQL
```

Docker Compose may be used for local/private deployments.

Example conceptual layout:

```text
docker compose up

├── web
├── api
└── postgres
```

The same container images should be reusable in cloud environments where practical.

The project must not depend on serverless infrastructure.

Serverless may be used in the future for specific ancillary tasks, but it is not an architectural requirement.

---

## 8. Configuration

Runtime configuration should configure behavior, not decide which business modules are compiled.

Cargo features decide:

> Which capabilities exist in this build?

Runtime configuration decides:

> How should those capabilities behave?

Rust configuration and command-line parsing should use the existing `clap-config-fallback` crate.

This allows configuration values to be supplied through configuration files while retaining Clap command-line capabilities and fallback behavior.

Configuration may include:

- PostgreSQL connection;
- bind address;
- public URLs;
- storage backend;
- session settings;
- logging;
- mail configuration;
- module-specific settings;
- external integration credentials.

Avoid maintaining both runtime module toggles and compile-time module features for the same purpose.

---

## 8. Suggested Dependency Direction

The architecture should generally preserve this dependency direction:

```text
core
unit
database
config
   ▲
   │
auth
   ▲
   │
modules/*
   ▲
   │
api
cli
```

This is illustrative rather than an absolute compile graph.

The important rules are:

- `core` must remain independent of business modules;
- `database` must remain independent of business modules;
- modules may depend on shared infrastructure;
- modules may depend on other modules when the dependency is explicit;
- `api` composes enabled modules;
- `cli` composes administrative functionality for enabled modules.

Circular crate dependencies must be avoided.

---

## 8. Module Internal Structure

Each business module lives in its own crate under:

```text
crates/modules/<module>/
```

A module owns the code and migrations required to implement its capability.

The architecture intentionally does not prescribe a mandatory internal directory structure for modules at this stage.

Separation of concerns remains required, but the concrete organization should be chosen based on the needs of each module instead of forcing folders such as `domain/`, `application/`, `graphql/` or `persistence/` before they provide value.

---

## 8. Repository and Persistence Abstractions

Persistence should expose a reusable generic repository abstraction by default.

The preferred baseline is conceptually:

```rust
Repository<T>::create(...)
Repository<T>::read(...)
Repository<T>::update(...)
Repository<T>::delete(...)
```

The exact Rust signatures should use the type system appropriately rather than forcing every entity into identical inputs or outputs.

When an entity requires persistence operations beyond generic CRUD, introduce an entity-specific repository trait.

Example:

```rust
trait ProductRepository {
    // Product-specific persistence operations.
}
```

The specific repository may extend or compose the generic repository capabilities and add only the operations required by that domain.

This approach provides generic CRUD capabilities out of the box while keeping specialized persistence behavior explicit.

SQLx remains the persistence implementation underneath these abstractions.

---

## 8. API / Domain Separation

GraphQL is an interface layer.

Resolvers should generally perform:

1. request/extractor handling;
2. authorization;
3. input conversion;
4. delegation to application logic;
5. response conversion.

Resolvers must not become the sole location of domain rules.

This keeps the system reusable from:

- GraphQL;
- CLI commands;
- scheduled jobs;
- future HTTP endpoints;
- future integrations.

---

## 8. Testing Strategy

Each crate should own relevant tests.

Expected layers include:

- unit tests for domain logic;
- SQLx/database integration tests;
- GraphQL/API integration tests;
- frontend component tests where valuable;
- end-to-end tests for critical workflows.

Representative module dependency combinations should be validated in CI.

At minimum, CI should compile and test:

```text
default/core build
representative 3D-printing module combination
all-features
```

where technically meaningful.

---

## 8. Design Principles

The following rules should guide implementation.

### Prefer the Rust type system

Use Rust's type system to make invalid states difficult or impossible to represent.

Prefer:

- strong types;
- newtypes;
- enums;
- traits;
- generics;
- typestate or other established Rust patterns when they provide real value.

Avoid representing domain concepts as loosely typed strings, numbers or generic maps when a precise Rust type can express the constraint.

### Prefer established Rust design patterns

Favor idiomatic Rust architecture and patterns over patterns copied mechanically from other ecosystems.

Abstractions should make ownership, dependencies and invariants clearer.

### Separation of concerns by default

Keep concerns separated when they have different responsibilities or reasons to change.

When callers need several concerns together, expose a simpler higher-level interface that composes them rather than collapsing the underlying boundaries.

### Prefer compile-time correctness

Use Rust types, Cargo dependencies and SQL constraints instead of duplicating guarantees in runtime configuration.

### Prefer explicit dependencies

If one module requires another, declare it in Cargo.

### Avoid unnecessary distributed architecture

This is a modular monolith.

Do not introduce microservices, queues, federation or service discovery unless a future requirement clearly justifies them.

### Preserve module ownership

A module owns its business capability and its related persistence/migrations.

The exact internal layering is intentionally left to implementation.

### Keep infrastructure reusable

Shared technical concerns belong in dedicated crates such as:

- `api`;
- `database`;
- `auth`;
- `config`;
- `unit`.

### Keep core useful but controlled

Shared domain concepts may live in `core` when they are genuinely used by multiple modules.

### Avoid destructive implicit behavior

Normal API startup must never silently remove module data.

### Prefer simple external interfaces

Complex internal concerns may remain separated while a higher-level facade or service exposes a simple interface to callers when appropriate.

### Avoid architecture for architecture's sake

Use abstractions when they simplify the system or improve correctness, testing, reuse or maintainability.

Do not introduce layers solely to follow a pattern.

---

## 8. Current Architecture Decisions

The current agreed baseline is:

```text
Repository             Monorepo
Backend language       Rust
Async runtime          Tokio
HTTP framework         Axum
Primary API            GraphQL
GraphQL library        async-graphql
Database               PostgreSQL
Database access        SQLx
Frontend               SvelteKit + TypeScript
Architecture           Modular monolith
Module isolation       One crate per module
Module selection       `MODULES` build variable → Cargo features + SvelteKit build config
Authentication         `auth` Cargo feature; mandatory in release
Development auth       Omit `auth` feature in debug builds
Deployment             Containers / deployment-agnostic
CLI                    Rust crate in workspace
Units                   unit + unit-derive crates
```

This document is the baseline architecture.

Implementation details may evolve, but changes that materially alter these architectural decisions should update this document.
