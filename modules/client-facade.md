<!-- filename: modules/client-facade.md -->

# 1. Title Page

**Module:** Client Facade (MpClient)  
**Document:** Module Detailed Design Document (Per-Module)  
**Version:** 1.0  
**Date:** 2026-03-01  
**Author(s):** <TBD>

[Download this document](<DOWNLOAD_LINK>)

# 2. Revision History

| Version | Date | Author | Notes |
|---|---|---|---|
| 1.0 | 2026-03-01 | <TBD> | Initial per-module extraction from upstream MDDD v1.5 (2026-03-01). |

# 3. Table of Contents

- [1. Title Page](#1-title-page)
- [2. Revision History](#2-revision-history)
- [3. Table of Contents](#3-table-of-contents)
- [4. Module Overview](#4-module-overview)
  - [4.1 Purpose](#41-purpose)
  - [4.2 Scope / Out of Scope](#42-scope--out-of-scope)
  - [4.3 Definitions & Acronyms](#43-definitions--acronyms-module-scoped)
- [5. Requirements Coverage](#5-requirements-coverage-module-scoped)
  - [5.1 Covered Requirements](#51-covered-requirements-ids--brief-statement)
  - [5.2 Not Covered Requirements](#52-not-covered-requirements-if-any-reason)
  - [5.3 Assumptions / TBDs](#53-assumptions--tbds-module-scoped)
- [6. Module Detailed Design](#6-module-detailed-design)
- [7. Module Traceability Appendix](#7-module-traceability-appendix-module-scoped)
- [8. Open Questions](#8-open-questions-module-scoped)
- [9. Final Self-Check](#9-final-self-check-module-scoped)


# 4. Module Overview

## 4.1 Purpose

Provide the primary async Rust entrypoint for the Materials Project API, exposing nested route clients and convenience workflows while centralizing shared behavior (auth, config, transport, and middleware wiring).

## 4.2 Scope / Out of Scope

- In-scope: all OpenAPI routes (Appendix A) and doc-driven routes (RDD §6.2), plus example-driven convenience workflows (RDD §6.3).
  - Out-of-scope: sync/blocking client; exact Python object parity (RDD §2).

## 4.3 Definitions & Acronyms (module-scoped)

- This module follows the shared glossary defined in the Index (modules/README.md §4.3).
- Module-specific terms are defined inline where first introduced.

# 5. Requirements Coverage (module-scoped)

## 5.1 Covered Requirements (IDs + brief statement)

Primary (ownership) coverage:

|Requirement ID|Brief|Test Case IDs|Coverage Status|
|---|---|---|---|
|FR-069|Client MUST support configuration from (1) explicit constructor/builder inputs and (2) environment variables, and MUST define precedence. (RDD: OPS-CONFIG-SOURCES-001)|UT-FR-069|Covered|
|FR-070|Client MUST support setting: `api_key`, `base_url`, `timeout`, `concurrency`, `qps_limit`, `user_agent`, and `allow_insecure_http` (see §9 security). (RDD: OPS-SETTINGS-001)|UT-FR-070|Covered|
|FR-001|Client MUST authenticate using API key header. (RDD: FR-COMMON-AUTH-001)|UT-FR-001|Covered|
|NFR-003|Client MUST support structured logs including request path, status code, latency, retry count, and correlation ID, and MUST redact API keys and other secrets. (RDD: NFR-OBS-LOGGING-001)|UT-NFR-003|Covered|
|NFR-004|Client MUST use HTTPS by default and MUST reject non-HTTPS base URLs unless the caller explicitly opts in (for testing). (RDD: NFR-SEC-TLS-001)|UT-NFR-004|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

_None._

# 6. Module Detailed Design

## Client Facade (MpClient)

### Purpose

Provide the primary async Rust entrypoint for the Materials Project API, exposing nested route clients and convenience workflows while centralizing shared behavior (auth, config, transport, and middleware wiring).

### Responsibilities (explicit; MUST/SHALL statements; testable)

- The client SHALL be async-first and require a Tokio runtime (Scope: RDD §2).
  - The client SHALL expose nested route clients for OpenAPI endpoints and doc-driven endpoints (guidance: RDD §8; functional coverage: FR-007 / FR-049).
  - The client SHALL enforce configuration validation at build time (missing API key, invalid base URL, invalid pagination presets) and SHALL fail fast without issuing HTTP when invalid (FR-001, FR-002, FR-070).
  - The client SHALL share a single underlying HTTP transport + middleware stack across all route groups to ensure consistent rate limiting, concurrency, retries, and logging (FR-070, NFR-001, NFR-002, NFR-003).

### In-Scope / Out-of-Scope

- In-scope: all OpenAPI routes (Appendix A) and doc-driven routes (RDD §6.2), plus example-driven convenience workflows (RDD §6.3).
  - Out-of-scope: sync/blocking client; exact Python object parity (RDD §2).

### Inputs/Outputs (schemas, examples)

- Inputs: `MpClientBuilder` settings (`api_key`, `base_url`, `timeout`, `concurrency`, `qps_limit`, `user_agent`, `allow_insecure_http`).
  - Outputs: strongly typed responses (`Response<T>`) where OpenAPI schemas exist, or raw JSON responses (`serde_json::Value`) for doc-driven endpoints (RDD §7.2).

### Types & Definitions

#### `MpClient`

- **Kind:** Public API (Facade)
- **Purpose:** Async client handle providing accessors for route roots and convenience workflows; holds shared state behind an Arc for cheap cloning.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|inner|Arc<InnerClient>|required|Immutable shared state; must be initialized via builder validation.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Semver: adding new route accessors is backward compatible; changing method signatures is breaking.
- **Location:** src/client.rs
- **Related Requirement IDs:** FR-069, FR-070, FR-001, NFR-003, NFR-004
- **Related Test Case IDs:** UT-FR-069, UT-FR-070, UT-FR-001, UT-NFR-003, UT-NFR-004

#### `MpClientBuilder`

- **Kind:** Public API (Builder)
- **Purpose:** Builder used to configure and construct MpClient, resolving environment variables and applying validation.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|settings|BuilderSettings|required|Holds optional overrides prior to validation.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Semver: additive builder setters are backward compatible.
- **Location:** src/client.rs
- **Related Requirement IDs:** FR-069, FR-070, FR-001, NFR-003, NFR-004
- **Related Test Case IDs:** UT-FR-069, UT-FR-070, UT-FR-001, UT-NFR-003, UT-NFR-004

#### `InnerClient`

- **Kind:** Internal
- **Purpose:** Shared internal state for MpClient; centralizes Config, Transport, middleware handles, and route roots.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|config|Config|required|Validated resolved configuration.|
|transport|Transport|required|Reqwest transport handle.|
|openapi|OpenApiRoot|required|Root for OpenAPI routes.|
|doc_driven|DocDrivenRoot|required|Root for doc-driven routes.|
|convenience|ConvenienceRoot|required|Root for convenience workflows.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Internal; not covered by stability guarantees.
- **Location:** src/client.rs
- **Related Requirement IDs:** FR-069, FR-070, FR-001, NFR-003, NFR-004
- **Related Test Case IDs:** UT-FR-069, UT-FR-070, UT-FR-001, UT-NFR-003, UT-NFR-004

#### `routes::openapi::OpenApiRoot`

- **Kind:** TBD
- **Purpose:** Entry point for OpenAPI route groups.
- **Fields / Properties**
_TBD: Field list not specified by upstream MDDD/OpenAPI; treat as an implementation detail._
- **Serialization / Schema Notes:** TBD
- **Versioning / Compatibility Notes:** TBD
- **Location:** TBD
- **Related Requirement IDs:** TBD
- **Related Test Case IDs:** TBD

#### `routes::doc_driven::DocDrivenRoot`

- **Kind:** TBD
- **Purpose:** Entry point for doc-driven endpoints.
- **Fields / Properties**
_TBD: Field list not specified by upstream MDDD/OpenAPI; treat as an implementation detail._
- **Serialization / Schema Notes:** TBD
- **Versioning / Compatibility Notes:** TBD
- **Location:** TBD
- **Related Requirement IDs:** TBD
- **Related Test Case IDs:** TBD

#### `convenience::ConvenienceRoot`

- **Kind:** TBD
- **Purpose:** Entry point for example-driven helper workflows.
- **Fields / Properties**
_TBD: Field list not specified by upstream MDDD/OpenAPI; treat as an implementation detail._
- **Serialization / Schema Notes:** TBD
- **Versioning / Compatibility Notes:** TBD
- **Location:** TBD
- **Related Requirement IDs:** TBD
- **Related Test Case IDs:** TBD


### Public Interfaces

- Type aliases and core API surface (illustrative; implementers may adjust naming without changing semantics):
    - `pub type Client = MpClient;`
    - `pub struct MpClientBuilder { /* fields */ }`
    - `impl MpClient { pub fn builder() -> MpClientBuilder; }`
    - `impl MpClientBuilder {
         pub fn api_key(self, key: impl Into<String>) -> Self;
         pub fn base_url(self, url: impl Into<String>) -> Self;
         pub fn timeout(self, d: std::time::Duration) -> Self;
         pub fn concurrency(self, n: usize) -> Self;
         pub fn qps_limit(self, qps: u32) -> Self;
         pub fn user_agent(self, ua: impl Into<String>) -> Self;
         pub fn allow_insecure_http(self, allow: bool) -> Self;
         pub fn build(self) -> Result<MpClient, MpApiError>;
       }`
    - Route accessors:
      - `pub fn openapi(&self) -> routes::openapi::OpenApiRoot;`
      - `pub fn doc(&self) -> routes::doc_driven::DocDrivenRoot;`
      - `pub fn convenience(&self) -> convenience::ConvenienceRoot;`

### Internal Design

- `MpClient` holds `Arc<InnerClient>` where `InnerClient` contains:
    - immutable `Config`
    - `Transport` (reqwest client wrapper)
    - `Middleware` instances (rate limiter, concurrency semaphore, retry policy)
    - optional `SpecRegistry` (OpenAPI-derived index used for coverage checks and feature detection)
  - Route roots are thin handles cloning `Arc<InnerClient>` and providing ergonomic group namespaces.

### Source Files & Responsibilities

#### `src/lib.rs`

- **Responsibility:** Crate entrypoint; re-exports the public API surface and module roots.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `MpClient`, `MpClientBuilder`, `InnerClient`
- **Related requirement IDs:** FR-069, FR-070, FR-001, NFR-003, NFR-004
- **Related test case IDs:** UT-FR-069, UT-FR-070, UT-FR-001, UT-NFR-003, UT-NFR-004

#### `src/client.rs`

- **Responsibility:** Defines MpClient and MpClientBuilder; wires Config, Transport, middleware, and route roots.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `MpClient`, `MpClientBuilder`, `InnerClient`
- **Related requirement IDs:** FR-069, FR-070, FR-001, NFR-003, NFR-004
- **Related test case IDs:** UT-FR-069, UT-FR-070, UT-FR-001, UT-NFR-003, UT-NFR-004


### Data Model

- N/A (library; no persistent storage). Data types are Rust structs generated from OpenAPI and/or raw JSON.

### Business Rules & Validation (mapped to requirement IDs)

- Build-time config precedence and validation: FR-069, FR-070, NFR-004.
  - Missing API key is a client-side error, no HTTP: FR-001.

### Error Handling

- Builder returns `MpApiError::MissingApiKey` when no API key is resolved (FR-001).
  - Builder returns `MpApiError::ConfigurationError` (or equivalent) for invalid `base_url` per TLS rule (NFR-004).

### Logging & Metrics

- Client does not mandate a logging backend; it emits `tracing` spans/events with required fields and redaction policy (NFR-003).

### Security

- API keys MUST never be logged; redaction is enforced in transport logging (see HTTP Transport module).

### Performance/Scalability Notes

- Stateless request execution; horizontal scaling is achieved by running multiple client instances.
  - Concurrency and rate limiting are enforced per client instance (FR-070, NFR-001).

### Dependencies

- `config`, `transport`, `middleware`, `routes`, `convenience`, `error`, `data`.

### Test Design

- UT-FR-001: missing API key yields `MissingApiKey` and no HTTP.
  - UT-FR-069/UT-FR-070: config precedence and supported settings are deterministic.

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|FR-001|§6 "Client Facade (MpClient)"|`src/lib.rs`, `src/client.rs`|UT-FR-001|Covered|
|FR-006|§6 "Client Facade (MpClient)"|`src/lib.rs`, `src/client.rs`|UT-FR-006|Covered|
|FR-007|§6 "Client Facade (MpClient)"|`src/lib.rs`, `src/client.rs`|CT-MANIFEST-001, UT-INVENTORY-001|Covered|
|FR-069|§6 "Client Facade (MpClient)"|`src/lib.rs`, `src/client.rs`|UT-FR-069|Covered|
|FR-070|§6 "Client Facade (MpClient)"|`src/lib.rs`, `src/client.rs`|UT-FR-070|Covered|
|NFR-001|§6 "Client Facade (MpClient)"|`src/lib.rs`, `src/client.rs`|UT-NFR-001|Covered|
|NFR-003|§6 "Client Facade (MpClient)"|`src/lib.rs`, `src/client.rs`|UT-NFR-003|Covered|


# 8. Open Questions (module-scoped)

_None._

# 9. Final Self-Check (module-scoped)

- English-only content (code identifiers/proper nouns allowed): **Yes**
- Table of Contents present: **Yes**
- Covered requirements listed (primary + full appendix): **Yes**
- Responsibility contract uses SHALL/MUST language: **Yes**
- Types & Definitions includes field-level details (or explicit TBD where upstream spec is incomplete): **Yes**
- Source Files & Responsibilities enumerated for the module directory: **Yes**
- Traceability appendix includes requirement-to-test mapping: **Yes**
