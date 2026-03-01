<!-- filename: modules/errors.md -->

# 1. Title Page

**Module:** Errors (MpApiError)  
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

Provide a single stable error type and deterministic error mapping across configuration, request building, transport, HTTP status handling, and deserialization.

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
|FR-005|Client MUST parse 422 validation errors into a typed error structure. (RDD: FR-COMMON-VALIDATION-001)|UT-FR-005|Covered|
|FR-006|Client MUST expose a typed error model that distinguishes configuration errors, request construction errors, transport errors, HTTP status errors, validation errors, and deserialization errors. (RDD: FR-COMMON-ERROR_MODEL-001)|UT-FR-006|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

- Error body truncation threshold is 8192 bytes (8 KiB) per upstream assumptions.
- Unknown JSON fields are tolerated for forward compatibility (DR-003).

# 6. Module Detailed Design

## Errors (MpApiError)

### Purpose

Provide a single stable error type and deterministic error mapping across configuration, request building, transport, HTTP status handling, and deserialization.

### Responsibilities (explicit; MUST/SHALL statements; testable)

- The client SHALL expose a typed error model with semantically distinct variants including at minimum the cases enumerated by the RDD (FR-006).
  - The client SHALL detect client-side errors (missing API key, invalid pagination combinations) before sending HTTP (FR-001, FR-002).
  - The client SHALL map HTTP 422 to `ValidationError(HTTPValidationError)` and SHALL parse the OpenAPI `HTTPValidationError` schema (FR-005, FR-006).
  - The client SHALL classify HTTP 429 and HTTP 5xx as retryable errors (`RetryableHttpError`) and all other non-2xx as `HttpError` (FR-006, NFR-002).
  - The client SHALL surface an explicit `UnsupportedBySpecification(feature)` when a documented workflow cannot be implemented due to missing OpenAPI contract (FR-006; used by charge density convenience methods).

### In-Scope / Out-of-Scope

TBD

### Inputs/Outputs (schemas, examples)

- Inputs: config validation failures; request validation failures; HTTP status + body; transport exceptions; JSON parse exceptions.
  - Output: `MpApiError`.

### Types & Definitions

#### `MpApiError`

- **Kind:** Error
- **Purpose:** Top-level error enum distinguishing configuration, request, transport, HTTP, validation, unsupported-by-spec, and deserialization failures.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|MissingApiKey|variant|n/a|No API key could be resolved.|
|InvalidPaginationParameters|variant { details: String }|n/a|Client-side pagination validation failed.|
|UnsupportedBySpecification|variant(&'static str)|n/a|Feature not implementable given available OpenAPI/doc contracts.|
|ValidationError|variant(HTTPValidationError)|n/a|HTTP 422 parsed into typed payload.|
|RetryableHttpError|variant { status: u16, body: String }|n/a|HTTP 429/5xx (subject to retry policy).|
|HttpError|variant { status: u16, body: String }|n/a|Other non-2xx HTTP.|
|TransportError|variant(String)|n/a|Network/timeouts/TLS failures.|
|DeserializeError|variant(String)|n/a|JSON parse/deserialize failure.|
|ConfigurationError|variant(String)|n/a|Invalid settings (e.g., non-HTTPS base_url).|

- **Serialization / Schema Notes:** Not serialized by default; implement Display/Error via thiserror. Body strings must be truncated/redacted before storage/logging.
- **Versioning / Compatibility Notes:** Semver: adding variants is breaking for exhaustive matches; consider non_exhaustive or helper classification APIs.
- **Location:** src/error.rs
- **Related Requirement IDs:** FR-005, FR-006
- **Related Test Case IDs:** UT-FR-005, UT-FR-006

#### `HTTPValidationError`

- **Kind:** DTO (Error payload)
- **Purpose:** Typed representation of OpenAPI 422 error payload.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|detail|Vec<ValidationErrorItem>|required|May be empty; preserves order from server.|

- **Serialization / Schema Notes:** Serde JSON; tolerant to unknown fields per DR-003.
- **Versioning / Compatibility Notes:** Forward compatible with unknown fields; additions should be optional.
- **Location:** src/error.rs
- **Related Requirement IDs:** FR-005, FR-006
- **Related Test Case IDs:** UT-FR-005, UT-FR-006

#### `ValidationErrorItem`

- **Kind:** DTO (Error item)
- **Purpose:** One validation error entry reported by the server.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|loc|Vec<serde_json::Value>|required|Location path components (mixed types).|
|msg|String|required|Human-readable message.|
|type_|String|required|Machine-oriented classification string.|

- **Serialization / Schema Notes:** Serde JSON; field name `type_` avoids Rust keyword.
- **Versioning / Compatibility Notes:** Forward compatible.
- **Location:** src/error.rs
- **Related Requirement IDs:** FR-005, FR-006
- **Related Test Case IDs:** UT-FR-005, UT-FR-006


### Public Interfaces

- `pub enum MpApiError {
       MissingApiKey,
       InvalidPaginationParameters { details: String },
       UnsupportedBySpecification(&'static str),
       ValidationError(HTTPValidationError),
       RetryableHttpError { status: u16, body: String },
       HttpError { status: u16, body: String },
       TransportError(String),
       DeserializeError(String),
       ConfigurationError(String),
     }`
  - `pub struct HTTPValidationError { detail: Vec<ValidationErrorItem> }`
  - `pub struct ValidationErrorItem { loc: Vec<serde_json::Value>, msg: String, type_: String }`

### Internal Design

- Central mapping function: `fn map_http(status: StatusCode, body: bytes::Bytes) -> Result<T, MpApiError>`
  - Redaction helper for log-safe error rendering.

### Source Files & Responsibilities

#### `src/error.rs`

- **Responsibility:** Defines MpApiError and all typed error payloads (validation, HTTP status, config).
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `MpApiError`, `HTTPValidationError`, `ValidationErrorItem`
- **Related requirement IDs:** FR-005, FR-006
- **Related test case IDs:** UT-FR-005, UT-FR-006


### Data Model

- N/A

### Business Rules & Validation (mapped to requirement IDs)

- Pagination conflicts are caught in Query module and returned as `InvalidPaginationParameters` (no HTTP) (FR-002).

### Error Handling

- See public interface.

### Logging & Metrics

- Error logs MUST include correlation ID and context, but MUST redact secrets (NFR-003).

### Security

- Ensure that serialized `MpApiError` never includes API key; error messages should omit headers by default.

### Performance/Scalability Notes

- Avoid cloning large bodies; store truncated body in errors (truncate to 8192 bytes / 8 KiB; see §11 Assumptions).

### Dependencies

- `serde`, `serde_json`, `reqwest` (or `http` types), `bytes`.

### Test Design

- UT-FR-006: invalid pagination combinations return `InvalidPaginationParameters` without HTTP.
  - UT-FR-006: 422 fixture parses into `ValidationError`.
  - UT-FR-006: 429 and 500 fixtures map to `RetryableHttpError`.

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|FR-001|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-001|Covered|
|FR-002|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-002|Covered|
|FR-004|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-004|Covered|
|FR-005|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-005|Covered|
|FR-006|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-006|Covered|
|FR-008|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-DEF-TASKS-GET|Covered|
|FR-009|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-DOI-ROOT-GET|Covered|
|FR-010|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-ABSORPTION-GET|Covered|
|FR-011|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-ALLOYS-GET|Covered|
|FR-012|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-BONDS-GET|Covered|
|FR-013|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-CHEMENV-GET|Covered|
|FR-014|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-CONVERSION_ELECTRODES-GET|Covered|
|FR-015|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-CORE-GET|Covered|
|FR-016|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-CORE_BLESSED_TASKS-GET|Covered|
|FR-017|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-CORE_FIND_STRUCTURE-POST|Covered|
|FR-018|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET|Covered|
|FR-019|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-DIELECTRIC-GET|Covered|
|FR-020|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-ELASTICITY-GET|Covered|
|FR-021|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-ELECTRONIC_STRUCTURE-GET|Covered|
|FR-022|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET|Covered|
|FR-023|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET|Covered|
|FR-024|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-EOS-GET|Covered|
|FR-025|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-FERMI-GET|Covered|
|FR-026|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-GRAIN_BOUNDARIES-GET|Covered|
|FR-027|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-INSERTION_ELECTRODES-GET|Covered|
|FR-028|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-MAGNETISM-GET|Covered|
|FR-029|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-OXIDATION_STATES-GET|Covered|
|FR-030|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-PHONON-GET|Covered|
|FR-031|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-PIEZOELECTRIC-GET|Covered|
|FR-032|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-PROVENANCE-GET|Covered|
|FR-033|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-ROBOCRYS-GET|Covered|
|FR-034|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-ROBOCRYS_TEXT_SEARCH-GET|Covered|
|FR-035|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-SIMILARITY-GET|Covered|
|FR-036|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-SIMILARITY_MATCH-GET|Covered|
|FR-037|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-SUBSTRATES-GET|Covered|
|FR-038|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-SUMMARY-GET|Covered|
|FR-039|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-SURFACE_PROPERTIES-GET|Covered|
|FR-040|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-SYNTHESIS-GET|Covered|
|FR-041|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-TASKS-GET|Covered|
|FR-042|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-TASKS_DEPRECATION-GET|Covered|
|FR-043|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-TASKS_ENTRIES-GET|Covered|
|FR-044|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-TASKS_TRAJECTORY-GET|Covered|
|FR-045|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-THERMO-GET|Covered|
|FR-046|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MAT-XAS-GET|Covered|
|FR-047|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MOL-JCESR-GET|Covered|
|FR-048|§6 "Errors (MpApiError)"|`src/error.rs`|CT-FR-MOL-SUMMARY-GET|Covered|
|FR-049|§6 "Errors (MpApiError)"|`src/error.rs`|DT-MANIFEST-001, UT-FR-049|Covered|
|FR-050|§6 "Errors (MpApiError)"|`src/error.rs`|DT-FR-MOL-ASSOC-GET|Covered|
|FR-051|§6 "Errors (MpApiError)"|`src/error.rs`|DT-FR-MOL-BONDING-GET|Covered|
|FR-052|§6 "Errors (MpApiError)"|`src/error.rs`|DT-FR-MOL-CORE-GET|Covered|
|FR-053|§6 "Errors (MpApiError)"|`src/error.rs`|DT-FR-MOL-ORBITALS-GET|Covered|
|FR-054|§6 "Errors (MpApiError)"|`src/error.rs`|DT-FR-MOL-PARTIAL_CHARGES-GET|Covered|
|FR-055|§6 "Errors (MpApiError)"|`src/error.rs`|DT-FR-MOL-PARTIAL_SPINS-GET|Covered|
|FR-056|§6 "Errors (MpApiError)"|`src/error.rs`|DT-FR-MOL-REDOX-GET|Covered|
|FR-057|§6 "Errors (MpApiError)"|`src/error.rs`|DT-FR-MOL-TASKS-GET|Covered|
|FR-058|§6 "Errors (MpApiError)"|`src/error.rs`|DT-FR-MOL-THERMO-GET|Covered|
|FR-059|§6 "Errors (MpApiError)"|`src/error.rs`|DT-FR-MOL-VIBRATIONS-GET|Covered|
|FR-060|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-060|Covered|
|FR-061|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-061|Covered|
|FR-062|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-062|Covered|
|FR-063|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-063|Covered|
|FR-064|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-064|Covered|
|FR-065|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-065|Covered|
|FR-066|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-066|Covered|
|FR-067|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-067|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|
|FR-068|§6 "Errors (MpApiError)"|`src/error.rs`|UT-FR-068|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|
|NFR-002|§6 "Errors (MpApiError)"|`src/error.rs`|UT-NFR-002|Covered|
|NFR-003|§6 "Errors (MpApiError)"|`src/error.rs`|UT-NFR-003|Covered|
|NFR-004|§6 "Errors (MpApiError)"|`src/error.rs`|UT-NFR-004|Covered|


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
