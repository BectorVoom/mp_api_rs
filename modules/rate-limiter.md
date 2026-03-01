<!-- filename: modules/rate-limiter.md -->

# 1. Title Page

**Module:** Rate Limiter  
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

Enforce a per-client aggregate requests-per-second (QPS) limit to prevent server overload and comply with published guidance.

## 4.2 Scope / Out of Scope

- In-scope: QPS limiting for all outbound HTTP calls made by this client instance.
  - Out-of-scope: global process-wide or cross-process rate limiting; server-side quota management.

## 4.3 Definitions & Acronyms (module-scoped)

- This module follows the shared glossary defined in the Index (modules/README.md §4.3).
- Module-specific terms are defined inline where first introduced.

# 5. Requirements Coverage (module-scoped)

## 5.1 Covered Requirements (IDs + brief statement)

Primary (ownership) coverage:

|Requirement ID|Brief|Test Case IDs|Coverage Status|
|---|---|---|---|
|NFR-001|Client MUST provide a configurable rate limiter and MUST default to **25 requests per second** (aggregate) unless explicitly overridden. (RDD: NFR-PERF-RATE_LIMIT-001)|UT-NFR-001|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

_None._

# 6. Module Detailed Design

## Rate Limiter

### Purpose

Enforce a per-client aggregate requests-per-second (QPS) limit to prevent server overload and comply with published guidance.

### Responsibilities (explicit; MUST/SHALL statements; testable)

- The client SHALL provide a configurable rate limiter and SHALL default to 25 req/s unless overridden (NFR-001).
  - The rate limiter SHALL apply across all requests issued by a single client instance (FR-070).

### In-Scope / Out-of-Scope

- In-scope: QPS limiting for all outbound HTTP calls made by this client instance.
  - Out-of-scope: global process-wide or cross-process rate limiting; server-side quota management.

### Inputs/Outputs (schemas, examples)

- Input: an await point `acquire()` before request execution.
  - Output: a permit (implicit) allowing the request to proceed.

### Types & Definitions

#### `RateLimiter`

- **Kind:** Middleware
- **Purpose:** Async token-bucket rate limiter enforcing aggregate QPS per client instance.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|qps_limit|u32|required|>=1; default 25.|
|capacity|u32|required|Equals qps_limit.|
|state|Mutex/Atomic|required|Tracks tokens and last refill instant (implementation choice).|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Internal.
- **Location:** src/middleware/rate_limit.rs
- **Related Requirement IDs:** NFR-001
- **Related Test Case IDs:** UT-NFR-001


### Public Interfaces

- `pub struct RateLimiter { /* token bucket */ }`
  - `impl RateLimiter { pub async fn acquire(&self) -> (); }`

### Internal Design

- Token-bucket algorithm (deterministic, testable):
    - capacity = qps_limit
    - tokens are refilled based on elapsed time since last refill
    - `acquire()` waits until at least one token is available, then consumes one
  - Alternative: use a dedicated crate (e.g., `governor`) if acceptable.

### Source Files & Responsibilities

#### `src/middleware/mod.rs`

- **Responsibility:** Middleware module root; exports rate limit and retry components.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `RateLimiter`
- **Related requirement IDs:** NFR-001
- **Related test case IDs:** UT-NFR-001

#### `src/middleware/rate_limit.rs`

- **Responsibility:** Token-bucket rate limiter enforcing per-client QPS.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `RateLimiter`
- **Related requirement IDs:** NFR-001
- **Related test case IDs:** UT-NFR-001


### Data Model

- N/A

### Business Rules & Validation (mapped to requirement IDs)

- Default qps_limit=25 (NFR-001).
  - qps_limit MUST be >= 1; qps_limit=0 is rejected at build time with `MpApiError::ConfigurationError("qps_limit must be >= 1")`.

### Error Handling

- No runtime errors during `acquire()`; invalid settings are rejected during config validation.

### Logging & Metrics

- Optional metrics: queue wait time and effective throughput (exported via hooks; see §9.1).

### Security

- N/A

### Performance/Scalability Notes

- Must be low overhead; prefer lock-free or minimal-lock implementation.
  - Works with concurrency semaphore to bound both burst and sustained load.

### Dependencies

- `tokio::time` (and optionally `governor`).

### Test Design

- UT-NFR-001: default 25 req/s.
  - UT-NFR-001: overriding qps_limit changes pacing deterministically (use mocked time).

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|FR-069|§6 "Rate Limiter"|`src/middleware/mod.rs`, `src/middleware/rate_limit.rs`|UT-FR-069|Covered|
|FR-070|§6 "Rate Limiter"|`src/middleware/mod.rs`, `src/middleware/rate_limit.rs`|UT-FR-070|Covered|
|NFR-001|§6 "Rate Limiter"|`src/middleware/mod.rs`, `src/middleware/rate_limit.rs`|UT-NFR-001|Covered|


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
