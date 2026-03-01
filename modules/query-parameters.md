<!-- filename: modules/query-parameters.md -->

# 1. Title Page

**Module:** Query Parameters (Pagination/Projection/Escape Hatch)  
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

Provide a consistent, validated, and testable mechanism for building query parameters (pagination, field projection, and escape-hatch parameters).

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
|FR-002|Client MUST support both page-based and offset-based pagination. (RDD: FR-COMMON-PAGINATION-001)|UT-FR-002|Covered|
|FR-003|Client MUST support field projection. (RDD: FR-COMMON-PROJECTION-001)|UT-FR-003|Covered|
|FR-049|Client MUST implement endpoints listed in official docs (e.g., “Getting Started” endpoint table) but absent from the uploaded OpenAPI, using a doc-driven contract. (RDD: FR-COMMON-DOC_DRIVEN-001)|DT-MANIFEST-001, UT-FR-049|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

_None._

# 6. Module Detailed Design

## Query Parameters (Pagination/Projection/Escape Hatch)

### Purpose

Provide a consistent, validated, and testable mechanism for building query parameters (pagination, field projection, and escape-hatch parameters).

### Responsibilities (explicit; MUST/SHALL statements; testable)

- The query layer SHALL support both page-based and offset-based pagination using `_page`, `_per_page`, `_skip`, `_limit` (FR-002).
  - If `_page`/`_per_page` are set, they SHALL take precedence over `_skip`/`_limit` (FR-002).
  - Conflicting pagination inputs SHALL be rejected client-side with `InvalidPaginationParameters` and SHALL NOT issue HTTP (FR-002).
  - The query layer SHALL clamp `_limit` and `_per_page` to 1000 where documented in OpenAPI (FR-002).
  - The query layer SHALL support field projection via `_fields` (comma-separated) and `_all_fields` (boolean) (FR-003).
  - The query layer SHALL support an escape hatch for arbitrary query parameters that are passed through verbatim (FR-049, FR-005).

### In-Scope / Out-of-Scope

TBD

### Inputs/Outputs (schemas, examples)

- Inputs: typed parameter structs and/or user-provided key/value pairs.
  - Output: a stable ordered query representation (`Vec<(String, String)>`) for request building.

### Types & Definitions

#### `Pagination`

- **Kind:** DTO (Query)
- **Purpose:** User-facing pagination parameters supporting page-based and offset-based styles.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|page|Option<u32>|optional|Maps to `_page`.|
|per_page|Option<u32>|optional|Maps to `_per_page`; clamped to 1000.|
|skip|Option<u32>|optional|Maps to `_skip`.|
|limit|Option<u32>|optional|Maps to `_limit`; clamped to 1000.|

- **Serialization / Schema Notes:** Converted to query pairs via deterministic serializer.
- **Versioning / Compatibility Notes:** Additive optional fields are backward compatible.
- **Location:** src/query/pagination.rs
- **Related Requirement IDs:** FR-002, FR-003, FR-049
- **Related Test Case IDs:** UT-FR-002, UT-FR-003, DT-MANIFEST-001, UT-FR-049

#### `NormalizedPagination`

- **Kind:** Internal (Query)
- **Purpose:** Validated, normalized pagination representation ensuring a single active mode and clamped bounds.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|mode|enum { PageBased, OffsetBased }|required|Derived.|
|page|Option<u32>|optional|Only when PageBased.|
|per_page|Option<u32>|optional|Only when PageBased; <=1000.|
|skip|Option<u32>|optional|Only when OffsetBased.|
|limit|Option<u32>|optional|Only when OffsetBased; <=1000.|

- **Serialization / Schema Notes:** Converted to query pairs deterministically.
- **Versioning / Compatibility Notes:** Internal.
- **Location:** src/query/pagination.rs
- **Related Requirement IDs:** FR-002, FR-003, FR-049
- **Related Test Case IDs:** UT-FR-002, UT-FR-003, DT-MANIFEST-001, UT-FR-049

#### `Projection`

- **Kind:** DTO (Query)
- **Purpose:** Field projection parameters controlling `_fields` and `_all_fields`.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|fields|Option<Vec<String>>|optional|Serialized as comma-separated stable order.|
|all_fields|bool|required|Serialize `_all_fields=true` when true.|

- **Serialization / Schema Notes:** Deterministic; stable ordering for test determinism.
- **Versioning / Compatibility Notes:** Backward compatible.
- **Location:** src/query/projection.rs
- **Related Requirement IDs:** FR-002, FR-003, FR-049
- **Related Test Case IDs:** UT-FR-002, UT-FR-003, DT-MANIFEST-001, UT-FR-049

#### `ExtraQueryParams`

- **Kind:** DTO (Query)
- **Purpose:** Escape hatch for arbitrary query parameters passed through verbatim.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|params|BTreeMap<String, String>|required|Deterministic ordering; values not pre-validated.|

- **Serialization / Schema Notes:** Deterministic due to BTreeMap ordering.
- **Versioning / Compatibility Notes:** Backward compatible.
- **Location:** src/query/extra.rs
- **Related Requirement IDs:** FR-002, FR-003, FR-049
- **Related Test Case IDs:** UT-FR-002, UT-FR-003, DT-MANIFEST-001, UT-FR-049

#### `ToQueryPairs`

- **Kind:** Trait
- **Purpose:** Trait implemented by query parameter structures to produce stable query pairs.
- **Fields / Properties**
_TBD: Field list not specified by upstream MDDD/OpenAPI; treat as an implementation detail._
- **Serialization / Schema Notes:** N/A
- **Versioning / Compatibility Notes:** Public trait changes are breaking; keep minimal.
- **Location:** src/query/mod.rs
- **Related Requirement IDs:** FR-002, FR-003, FR-049
- **Related Test Case IDs:** UT-FR-002, UT-FR-003, DT-MANIFEST-001, UT-FR-049


### Public Interfaces

- `pub struct Pagination { pub page: Option<u32>, pub per_page: Option<u32>, pub skip: Option<u32>, pub limit: Option<u32> }`
  - `impl Pagination { pub fn validate_and_normalize(&self) -> Result<NormalizedPagination, MpApiError>; }`
  - `pub struct Projection { pub fields: Option<Vec<String>>, pub all_fields: bool }`
  - `pub struct ExtraQueryParams(pub std::collections::BTreeMap<String, String>);`
  - `pub trait ToQueryPairs { fn to_query_pairs(&self) -> Result<Vec<(String,String)>, MpApiError>; }`

### Internal Design

- Normalization algorithm:
    - detect conflict: any combination where (`page` is set AND `skip` is set) OR (`per_page` is set AND `limit` is set) => error
    - effective pagination:
      - if `page`/`per_page` set => serialize only `_page`/`_per_page` (ignore skip/limit)
      - else => serialize `_skip`/`_limit` when present
    - clamp: `_limit` and `_per_page` = min(value, 1000)

### Source Files & Responsibilities

#### `src/query/mod.rs`

- **Responsibility:** Query module root; exports pagination, projection, and extra query support.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `ToQueryPairs`
- **Related requirement IDs:** FR-002, FR-003, FR-049
- **Related test case IDs:** UT-FR-002, UT-FR-003, DT-MANIFEST-001, UT-FR-049

#### `src/query/pagination.rs`

- **Responsibility:** Pagination types, validation, normalization, and serialization.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `Pagination`, `NormalizedPagination`
- **Related requirement IDs:** FR-002, FR-003, FR-049
- **Related test case IDs:** UT-FR-002, UT-FR-003, DT-MANIFEST-001, UT-FR-049

#### `src/query/projection.rs`

- **Responsibility:** Field projection types and serialization (_fields/_all_fields).
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `Projection`
- **Related requirement IDs:** FR-002, FR-003, FR-049
- **Related test case IDs:** UT-FR-002, UT-FR-003, DT-MANIFEST-001, UT-FR-049

#### `src/query/extra.rs`

- **Responsibility:** Escape-hatch query parameter map with deterministic ordering.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `ExtraQueryParams`
- **Related requirement IDs:** FR-002, FR-003, FR-049
- **Related test case IDs:** UT-FR-002, UT-FR-003, DT-MANIFEST-001, UT-FR-049


### Data Model

- N/A

### Business Rules & Validation (mapped to requirement IDs)

- Projection serialization:
    - `_fields` joins with commas in stable order
    - `_all_fields=true` when requested; if true and `_fields` set, both are allowed; client passes through.

### Error Handling

- `InvalidPaginationParameters` for conflicts.
  - No client-side validation for escape-hatch parameter names/values; server 422 maps to `ValidationError` (FR-005).

### Logging & Metrics

- Query parameters are loggable except sensitive values; escape hatch keys are logged but values are truncated.

### Security

- Escape hatch affects only query string; it cannot override headers.

### Performance/Scalability Notes

- Use `BTreeMap` to keep deterministic ordering for testability.

### Dependencies

- `serde_urlencoded` or equivalent query serializer.

### Test Design

- UT-FR-002: pagination serialization, clamping, conflict errors.
  - UT-FR-003: projection serialization and payload-size fixture comparison (as specified in the RDD acceptance).

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|FR-002|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|UT-FR-002|Covered|
|FR-003|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|UT-FR-003|Covered|
|FR-049|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|DT-MANIFEST-001, UT-FR-049|Covered|
|FR-050|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|DT-FR-MOL-ASSOC-GET|Covered|
|FR-051|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|DT-FR-MOL-BONDING-GET|Covered|
|FR-052|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|DT-FR-MOL-CORE-GET|Covered|
|FR-053|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|DT-FR-MOL-ORBITALS-GET|Covered|
|FR-054|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|DT-FR-MOL-PARTIAL_CHARGES-GET|Covered|
|FR-055|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|DT-FR-MOL-PARTIAL_SPINS-GET|Covered|
|FR-056|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|DT-FR-MOL-REDOX-GET|Covered|
|FR-057|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|DT-FR-MOL-TASKS-GET|Covered|
|FR-058|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|DT-FR-MOL-THERMO-GET|Covered|
|FR-059|§6 "Query Parameters (Pagination/Projection/Escape Hatch)"|`src/query/mod.rs`, `src/query/pagination.rs`, `src/query/projection.rs`, `src/query/extra.rs`|DT-FR-MOL-VIBRATIONS-GET|Covered|


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
