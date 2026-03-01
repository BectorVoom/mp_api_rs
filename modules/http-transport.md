<!-- filename: modules/http-transport.md -->

# 1. Title Page

**Module:** HTTP Transport  
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

Execute HTTP requests with consistent behavior: base URL resolution, headers, timeouts, concurrency caps, QPS limiting, retries, response capture, and observability.

## 4.2 Scope / Out of Scope

TBD

## 4.3 Definitions & Acronyms (module-scoped)

- This module follows the shared glossary defined in the Index (modules/README.md §4.3).
- Module-specific terms are defined inline where first introduced.

# 5. Requirements Coverage (module-scoped)

## 5.1 Covered Requirements (IDs + brief statement)

Primary (ownership) coverage:

|Requirement ID|Brief|Test Case IDs|Coverage Status|
|---|---|---|---|
|FR-001|Client MUST authenticate using API key header. (RDD: FR-COMMON-AUTH-001)|UT-FR-001|Covered|
|FR-070|Client MUST support setting: `api_key`, `base_url`, `timeout`, `concurrency`, `qps_limit`, `user_agent`, and `allow_insecure_http` (see §9 security). (RDD: OPS-SETTINGS-001)|UT-FR-070|Covered|
|NFR-003|Client MUST support structured logs including request path, status code, latency, retry count, and correlation ID, and MUST redact API keys and other secrets. (RDD: NFR-OBS-LOGGING-001)|UT-NFR-003|Covered|
|NFR-004|Client MUST use HTTPS by default and MUST reject non-HTTPS base URLs unless the caller explicitly opts in (for testing). (RDD: NFR-SEC-TLS-001)|UT-NFR-004|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

- Error body truncation threshold is 8192 bytes (8 KiB) per upstream assumptions.
- Unknown JSON fields are tolerated for forward compatibility (DR-003).

# 6. Module Detailed Design

## HTTP Transport

### Purpose

Execute HTTP requests with consistent behavior: base URL resolution, headers, timeouts, concurrency caps, QPS limiting, retries, response capture, and observability.

### Responsibilities (explicit; MUST/SHALL statements; testable)

- The transport SHALL prefix all paths with `base_url` and SHALL enforce HTTPS unless `allow_insecure_http=true` (FR-070, NFR-004).
  - The transport SHALL set `X-API-KEY` header on every request (FR-001).
  - The transport SHALL set `User-Agent` header on every request when configured (FR-070).
  - The transport SHALL apply per-request timeout to the full HTTP operation (connect + request + body read) (FR-070).
  - The transport SHALL cap concurrent in-flight requests per client instance (FR-070).
  - The transport SHALL enforce an aggregate per-client QPS rate limiter (NFR-001).
  - The transport SHALL emit structured logs with required fields and redaction (NFR-003).

### In-Scope / Out-of-Scope

TBD

### Inputs/Outputs (schemas, examples)

- Inputs: `RequestSpec` = { method, path, query_pairs, body_json? } plus internal correlation ID.
  - Outputs: `RawResponse` = { status, headers, body_bytes }.

### Types & Definitions

#### `RequestSpec`

- **Kind:** Internal (Transport DTO)
- **Purpose:** Internal request description passed from routes to transport.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|method|http::Method|required|HTTP method.|
|path|String|required|Relative path (starts with "/").|
|query|Vec<(String,String)>|required|Stable ordering.|
|body|Option<serde_json::Value>|optional|For JSON body operations.|

- **Serialization / Schema Notes:** Converted to reqwest request; JSON body serialized via serde_json.
- **Versioning / Compatibility Notes:** Internal.
- **Location:** src/transport/mod.rs
- **Related Requirement IDs:** FR-001, FR-070, NFR-003, NFR-004
- **Related Test Case IDs:** UT-FR-001, UT-FR-070, UT-NFR-003, UT-NFR-004

#### `RawResponse`

- **Kind:** Internal (Transport DTO)
- **Purpose:** Raw HTTP response captured by transport for later parsing.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|status|u16|required|HTTP status code.|
|headers|http::HeaderMap|optional|Optional capture.|
|body_bytes|bytes::Bytes|required|Full body bytes.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Internal.
- **Location:** src/transport/mod.rs
- **Related Requirement IDs:** FR-001, FR-070, NFR-003, NFR-004
- **Related Test Case IDs:** UT-FR-001, UT-FR-070, UT-NFR-003, UT-NFR-004

#### `Transport`

- **Kind:** Internal/Public handle
- **Purpose:** Transport handle exposing async execution with middleware integration.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|client|reqwest::Client|required|Initialized once; reused.|
|semaphore|tokio::sync::Semaphore|required|Concurrency cap.|
|rate_limiter|RateLimiter|required|Per-client QPS limiter.|
|retry|RetryConfig|required|Retry policy configuration.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Internal.
- **Location:** src/transport/mod.rs; src/transport/reqwest_transport.rs
- **Related Requirement IDs:** FR-001, FR-070, NFR-003, NFR-004
- **Related Test Case IDs:** UT-FR-001, UT-FR-070, UT-NFR-003, UT-NFR-004


### Public Interfaces

- `pub struct RequestSpec { pub method: http::Method, pub path: String, pub query: Vec<(String,String)>, pub body: Option<serde_json::Value> }`
  - `pub struct RawResponse { pub status: u16, pub body: bytes::Bytes }`
  - `impl Transport { pub async fn execute(&self, req: RequestSpec) -> Result<RawResponse, MpApiError>; }`

### Internal Design

- Underlying HTTP client: `reqwest::Client` configured once.
  - URL build: `base_url.join(&path)`; query pairs appended via deterministic serializer.
  - Concurrency: `tokio::sync::Semaphore` acquire permit around request execution.
  - Rate limiting: await token from Rate Limiter module before executing.
  - Retries: delegated to Retry module; transport provides a closure `attempt_execute_once`.
  - Observability:
    - generate `correlation_id` (UUID v4)
    - start `tracing` span with: method, path, status, latency_ms, retry_count, correlation_id
    - redact headers: `X-API-KEY` and any configured sensitive keys

### Source Files & Responsibilities

#### `src/transport/mod.rs`

- **Responsibility:** Transport module root; defines RequestSpec/RawResponse and Transport trait/handle.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `RequestSpec`, `RawResponse`, `Transport`
- **Related requirement IDs:** FR-001, FR-070, NFR-003, NFR-004
- **Related test case IDs:** UT-FR-001, UT-FR-070, UT-NFR-003, UT-NFR-004

#### `src/transport/reqwest_transport.rs`

- **Responsibility:** Reqwest-backed transport implementation and request execution pipeline.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `RequestSpec`, `RawResponse`, `Transport`
- **Related requirement IDs:** FR-001, FR-070, NFR-003, NFR-004
- **Related test case IDs:** UT-FR-001, UT-FR-070, UT-NFR-003, UT-NFR-004


### Data Model

- N/A

### Business Rules & Validation (mapped to requirement IDs)

- Reject non-HTTPS base URL at build time unless allow_insecure_http.

### Error Handling

- Network failures => `TransportError`
  - Timeout => `TransportError` (distinguishable via error string or dedicated variant)
  - Non-2xx => mapped by Errors module

### Logging & Metrics

- Required fields: request path, status code, latency, retry count, correlation ID (NFR-003).
  - Optional metrics integration: expose hooks/callbacks rather than forcing a metrics crate (library-friendly).

### Security

- Redaction MUST be applied to secrets in logs.
  - TLS enforced by default.

### Performance/Scalability Notes

- Minimal allocations: reuse reqwest client; avoid cloning body where possible.
  - Allow caller to configure concurrency and QPS.

### Dependencies

- `reqwest`, `tokio`, `tracing`, `uuid`, `bytes`, `http`.

### Test Design

- UT-FR-070: verify headers, URL prefixing, timeout, concurrency semaphore behavior, HTTPS gate.
  - UT-NFR-003: verify structured logs contain required fields and redaction behavior.

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|FR-001|§6 "HTTP Transport"|`src/transport/mod.rs`, `src/transport/reqwest_transport.rs`|UT-FR-001|Covered|
|FR-006|§6 "HTTP Transport"|`src/transport/mod.rs`, `src/transport/reqwest_transport.rs`|UT-FR-006|Covered|
|FR-069|§6 "HTTP Transport"|`src/transport/mod.rs`, `src/transport/reqwest_transport.rs`|UT-FR-069|Covered|
|FR-070|§6 "HTTP Transport"|`src/transport/mod.rs`, `src/transport/reqwest_transport.rs`|UT-FR-070|Covered|
|NFR-001|§6 "HTTP Transport"|`src/transport/mod.rs`, `src/transport/reqwest_transport.rs`|UT-NFR-001|Covered|
|NFR-002|§6 "HTTP Transport"|`src/transport/mod.rs`, `src/transport/reqwest_transport.rs`|UT-NFR-002|Covered|
|NFR-003|§6 "HTTP Transport"|`src/transport/mod.rs`, `src/transport/reqwest_transport.rs`|UT-NFR-003|Covered|
|NFR-004|§6 "HTTP Transport"|`src/transport/mod.rs`, `src/transport/reqwest_transport.rs`|UT-NFR-004|Covered|
|NFR-008|§6 "HTTP Transport"|`src/transport/mod.rs`, `src/transport/reqwest_transport.rs`|CI-DEPS-001|Covered|


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
