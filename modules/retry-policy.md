<!-- filename: modules/retry-policy.md -->

# 1. Title Page

**Module:** Retry Policy  
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

Improve reliability by retrying transient failures with exponential backoff while avoiding unsafe retries for non-transient errors.

## 4.2 Scope / Out of Scope

- In-scope: transient error retries for idempotent/safe operations. Default retry policy retries only for GET/HEAD/OPTIONS requests; POST/PUT/PATCH/DELETE are not retried unless explicitly enabled by caller configuration.
  - Out-of-scope: distributed tracing across retries (beyond correlation ID), request deduplication tokens.

## 4.3 Definitions & Acronyms (module-scoped)

- This module follows the shared glossary defined in the Index (modules/README.md §4.3).
- Module-specific terms are defined inline where first introduced.

# 5. Requirements Coverage (module-scoped)

## 5.1 Covered Requirements (IDs + brief statement)

Primary (ownership) coverage:

|Requirement ID|Brief|Test Case IDs|Coverage Status|
|---|---|---|---|
|NFR-002|Client MUST implement retries with exponential backoff for transient failures, using a configurable policy with explicit defaults (max_retries=3, initial_backoff=200ms, max_backoff=2s, jitter=full-jitter). (RDD: NFR-REL-RETRY-001)|UT-NFR-002|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

_None._

# 6. Module Detailed Design

## Retry Policy

### Purpose

Improve reliability by retrying transient failures with exponential backoff while avoiding unsafe retries for non-transient errors.

### Responsibilities (explicit; MUST/SHALL statements; testable)

- The client SHALL implement retries with exponential backoff for transient failures with configurable policy and explicit defaults (max_retries=3, initial_backoff=200ms, max_backoff=2s, jitter=full-jitter) (NFR-002).
  - The client SHALL retry on HTTP 429 and HTTP 5xx, and on selected transport failures (timeouts, connection resets) (NFR-002).
  - The client SHALL NOT retry on HTTP 4xx other than 429, including 422 validation errors (NFR-002, FR-005).
  - Default policy MUST be: max_retries=3 (max_attempts=4), initial_backoff=200ms, max_backoff=2s, jitter=full-jitter, retryable_statuses={429, 5xx}, and retryable_methods={GET, HEAD, OPTIONS} unless explicitly configured otherwise (NFR-002).

### In-Scope / Out-of-Scope

- In-scope: transient error retries for idempotent/safe operations. Default retry policy retries only for GET/HEAD/OPTIONS requests; POST/PUT/PATCH/DELETE are not retried unless explicitly enabled by caller configuration.
  - Out-of-scope: distributed tracing across retries (beyond correlation ID), request deduplication tokens.

### Inputs/Outputs (schemas, examples)

- Input: `RetryConfig` and a closure performing one request attempt.
  - Output: final success response or last encountered error.

### Types & Definitions

#### `RetryConfig`

- **Kind:** Config (Retry)
- **Purpose:** Configurable retry policy (classification + budget + backoff + jitter).
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|max_retries|u32|required|Default 3; attempts = max_retries + 1.|
|initial_backoff|std::time::Duration|required|Default 200ms.|
|max_backoff|std::time::Duration|required|Default 2s.|
|jitter|JitterStrategy|required|Default full-jitter.|
|retryable_statuses|Set<u16>|required|Default {429} ∪ {500..599}.|
|retryable_methods|Set<http::Method>|required|Default {GET, HEAD, OPTIONS}.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Changes require docs + tests.
- **Location:** src/middleware/retry.rs
- **Related Requirement IDs:** NFR-002
- **Related Test Case IDs:** UT-NFR-002

#### `retry()`

- **Kind:** TBD
- **Purpose:** Async retry executor applying classification and backoff around a request attempt closure.
- **Fields / Properties**
_TBD: Field list not specified by upstream MDDD/OpenAPI; treat as an implementation detail._
- **Serialization / Schema Notes:** TBD
- **Versioning / Compatibility Notes:** TBD
- **Location:** TBD
- **Related Requirement IDs:** TBD
- **Related Test Case IDs:** TBD


### Public Interfaces

- `pub struct RetryConfig { pub max_attempts: u32, pub base_backoff: Duration, pub max_backoff: Duration, pub retry_statuses: Vec<u16> }`
  - `pub async fn retry<F, Fut, T>(cfg: &RetryConfig, f: F) -> Result<T, MpApiError>
       where F: FnMut(u32) -> Fut, Fut: Future<Output=Result<T, MpApiError>>;`

### Internal Design

- Backoff schedule:
    - attempt 1: no delay
    - attempt n>1: delay = min(max_backoff, base_backoff * 2^(n-2)) + jitter
  - Retry classification:
    - retryable HTTP statuses: 429 + 5xx
    - retryable transport failures: request timeouts, connect timeouts, DNS resolution failures, connection resets, and TLS handshake failures (mapped from reqwest error categories); all others are non-retryable by default.

### Source Files & Responsibilities

#### `src/middleware/mod.rs`

- **Responsibility:** Middleware module root; exports rate limit and retry components.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `RetryConfig`, `retry()`
- **Related requirement IDs:** NFR-002
- **Related test case IDs:** UT-NFR-002

#### `src/middleware/retry.rs`

- **Responsibility:** Retry policy implementation (classification + backoff + jitter).
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `RetryConfig`
- **Related requirement IDs:** NFR-002
- **Related test case IDs:** UT-NFR-002


### Data Model

- N/A

### Business Rules & Validation (mapped to requirement IDs)

- Retry budget and backoff settings are configurable via OPS-SETTINGS-001, with defaults per NFR-REL-RETRY-001.

### Error Handling

- When retries are exhausted, return the last error.
  - Retry logic MUST preserve the original error classification (RetryableHttpError vs TransportError vs HttpError).

### Logging & Metrics

- Emit retry_count and backoff_ms fields on the request span; do not log request bodies.

### Security

- Do not log sensitive headers or bodies during retries.

### Performance/Scalability Notes

- Backoff mitigates thundering herd; coordinates with QPS limiter.

### Dependencies

- `tokio::time`, optional RNG crate (`rand`).

### Test Design

- UT-NFR-002: retry vs no-retry behavior by status and transport error type; backoff bounded.

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|FR-006|§6 "Retry Policy"|`src/middleware/mod.rs`, `src/middleware/retry.rs`|UT-FR-006|Covered|
|FR-069|§6 "Retry Policy"|`src/middleware/mod.rs`, `src/middleware/retry.rs`|UT-FR-069|Covered|
|FR-070|§6 "Retry Policy"|`src/middleware/mod.rs`, `src/middleware/retry.rs`|UT-FR-070|Covered|
|NFR-002|§6 "Retry Policy"|`src/middleware/mod.rs`, `src/middleware/retry.rs`|UT-NFR-002|Covered|


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
