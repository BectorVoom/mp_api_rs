<!-- filename: modules/configuration.md -->

# 1. Title Page

**Module:** Configuration (Config & Builder)  
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

Centralize configuration source resolution, defaults, and validation.

## 4.2 Scope / Out of Scope

- In-scope: API key resolution, base URL validation, settings normalization.
  - Out-of-scope: storing secrets beyond process memory.

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
|NFR-001|Client MUST provide a configurable rate limiter and MUST default to **25 requests per second** (aggregate) unless explicitly overridden. (RDD: NFR-PERF-RATE_LIMIT-001)|UT-NFR-001|Covered|
|NFR-004|Client MUST use HTTPS by default and MUST reject non-HTTPS base URLs unless the caller explicitly opts in (for testing). (RDD: NFR-SEC-TLS-001)|UT-NFR-004|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

- Default values: timeout=30s, concurrency=16, user_agent="mp-api-rs/<crate_version>" per upstream assumptions.
- Environment variables supported: MP_API_KEY and PMG_MAPI_KEY (FR-069).

# 6. Module Detailed Design

## Configuration (Config & Builder)

### Purpose

Centralize configuration source resolution, defaults, and validation.

### Responsibilities (explicit; MUST/SHALL statements; testable)

- The config layer SHALL resolve settings from builder inputs and environment variables with explicit precedence (FR-069).
  - The config layer SHALL support all settings enumerated in OPS-SETTINGS-001, including default `base_url=https://api.materialsproject.org` and `allow_insecure_http` gating (FR-070, NFR-004).
  - The config layer SHALL provide deterministic defaults for:
    - `qps_limit = 25` (NFR-001)
    - `timeout = 30s` (selected default; see §11 Assumptions)
    - `concurrency = 16` (selected default; see §11 Assumptions)
    - `user_agent = "mp-api-rs/<crate_version>"` where `<crate_version>` is the Cargo package version (selected default; see §11 Assumptions)

### In-Scope / Out-of-Scope

- In-scope: API key resolution, base URL validation, settings normalization.
  - Out-of-scope: storing secrets beyond process memory.

### Inputs/Outputs (schemas, examples)

- Inputs:
    - explicit builder settings
    - env vars: `MP_API_KEY`, `PMG_MAPI_KEY` (FR-069)
  - Output: immutable `Config` struct.

### Types & Definitions

#### `Config`

- **Kind:** Config
- **Purpose:** Resolved, validated configuration used by all requests.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|api_key|String|required|Non-empty; sourced from builder or env.|
|base_url|url::Url|required|HTTPS by default; http allowed only if allow_insecure_http=true.|
|timeout|Option<std::time::Duration>|optional|Applies to full request; None means reqwest default.|
|concurrency|usize|required|>= 1; caps in-flight requests per client instance.|
|qps_limit|u32|required|>= 1; default 25.|
|user_agent|String|required|Default mp-api-rs/<crate_version>.|
|allow_insecure_http|bool|required|Default false; enables http:// base_url for local testing only.|
|retry|RetryConfig|required|Retry policy configuration.|

- **Serialization / Schema Notes:** Not serialized; constructed from env vars and builder settings.
- **Versioning / Compatibility Notes:** Semver: adding optional config fields is backward compatible; changing defaults must be documented and tested.
- **Location:** src/config.rs
- **Related Requirement IDs:** FR-069, FR-070, NFR-001, NFR-004
- **Related Test Case IDs:** UT-FR-069, UT-FR-070, UT-NFR-001, UT-NFR-004

#### `BuilderSettings`

- **Kind:** Config (Builder-only)
- **Purpose:** Intermediate builder settings before env-resolution and validation.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|api_key|Option<String>|optional|If None, resolve from env.|
|base_url|Option<url::Url>|optional|If None, default is applied.|
|timeout|Option<std::time::Duration>|optional|If None, default 30s (per upstream assumptions).|
|concurrency|Option<usize>|optional|If None, default 16 (per upstream assumptions).|
|qps_limit|Option<u32>|optional|If None, default 25.|
|user_agent|Option<String>|optional|If None, default is applied.|
|allow_insecure_http|Option<bool>|optional|If None, default false.|
|retry|Option<RetryConfig>|optional|If None, default policy is applied.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Internal to builder API.
- **Location:** src/config.rs
- **Related Requirement IDs:** FR-069, FR-070, NFR-001, NFR-004
- **Related Test Case IDs:** UT-FR-069, UT-FR-070, UT-NFR-001, UT-NFR-004


### Public Interfaces

- `pub struct Config { api_key: String, base_url: url::Url, timeout: Option<Duration>, concurrency: usize, qps_limit: u32, user_agent: String, allow_insecure_http: bool, retry: RetryConfig }`
  - `impl Config { pub fn from_builder_and_env(builder: BuilderSettings) -> Result<Self, MpApiError>; }`

### Internal Design

- Parsing/validation rules:
    - base_url:
      - default to `https://api.materialsproject.org` (FR-070, NFR-004)
      - reject `http://` unless `allow_insecure_http=true` (NFR-004)
    - api_key:
      - resolved via precedence: builder > MP_API_KEY > PMG_MAPI_KEY (FR-069)
      - missing produces `MissingApiKey` (FR-001)

### Source Files & Responsibilities

#### `src/config.rs`

- **Responsibility:** Configuration resolution, defaults, validation, and environment-variable integration.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `Config`, `BuilderSettings`
- **Related requirement IDs:** FR-069, FR-070, NFR-001, NFR-004
- **Related test case IDs:** UT-FR-069, UT-FR-070, UT-NFR-001, UT-NFR-004


### Data Model

- N/A

### Business Rules & Validation (mapped to requirement IDs)

- All validation errors are raised during `build()` and SHALL prevent any HTTP.

### Error Handling

- `MissingApiKey`
  - `InvalidBaseUrl` / `InsecureBaseUrlNotAllowed`

### Logging & Metrics

- Log only non-sensitive configuration (never api_key).

### Security

- Treat api_key as secret; never expose via `Debug` / display.

### Performance/Scalability Notes

- Config is immutable; cheap clones via `Arc<Config>` if required.

### Dependencies

- `std::env`, `url` crate.

### Test Design

- UT-FR-069: precedence builder > MP_API_KEY > PMG_MAPI_KEY.
  - UT-FR-070: each supported setting affects behavior deterministically (headers/URL/limits/TLS gate).
  - UT-NFR-004: TLS enforcement.
  - UT-NFR-001: qps default 25.

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|FR-001|§6 "Configuration (Config & Builder)"|`src/config.rs`|UT-FR-001|Covered|
|FR-069|§6 "Configuration (Config & Builder)"|`src/config.rs`|UT-FR-069|Covered|
|FR-070|§6 "Configuration (Config & Builder)"|`src/config.rs`|UT-FR-070|Covered|
|NFR-001|§6 "Configuration (Config & Builder)"|`src/config.rs`|UT-NFR-001|Covered|
|NFR-002|§6 "Configuration (Config & Builder)"|`src/config.rs`|UT-NFR-002|Covered|
|NFR-004|§6 "Configuration (Config & Builder)"|`src/config.rs`|UT-NFR-004|Covered|
|NFR-008|§6 "Configuration (Config & Builder)"|`src/config.rs`|CI-DEPS-001|Covered|


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
