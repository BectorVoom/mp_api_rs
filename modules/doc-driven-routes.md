<!-- filename: modules/doc-driven-routes.md -->

# 1. Title Page

**Module:** Doc-Driven Routes  
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

Implement endpoints listed in official documentation (Getting Started) that are missing from the uploaded OpenAPI, using a doc-driven contract and returning raw JSON.

## 4.2 Scope / Out of Scope

- In-scope: endpoints explicitly listed under RDD §6.2.
  - Out-of-scope: any other undocumented endpoints; those require OpenAPI addition or explicit RDD update.

## 4.3 Definitions & Acronyms (module-scoped)

- This module follows the shared glossary defined in the Index (modules/README.md §4.3).
- Module-specific terms are defined inline where first introduced.

# 5. Requirements Coverage (module-scoped)

## 5.1 Covered Requirements (IDs + brief statement)

Primary (ownership) coverage:

|Requirement ID|Brief|Test Case IDs|Coverage Status|
|---|---|---|---|
|FR-049..FR-059|Range of related requirements; see §7 for full list.|See §7|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

_None._

# 6. Module Detailed Design

## Doc-Driven Routes

### Purpose

Implement endpoints listed in official documentation (Getting Started) that are missing from the uploaded OpenAPI, using a doc-driven contract and returning raw JSON.

### Responsibilities (explicit; MUST/SHALL statements; testable)

- The client SHALL implement each doc-driven endpoint exactly as specified by path in the RDD (§6.2) and apply common query parameters + escape hatch (FR-049).
  - The client SHALL not pre-validate endpoint-specific parameters for doc-driven endpoints; it SHALL pass through query parameters and surface server 422 as `ValidationError` (FR-049, FR-005).
  - Response parsing SHALL accept either:
    1) standard envelope `{data, errors, meta}` or
    2) top-level JSON array/object when the server does not wrap responses (FR-049).
  - Doc-driven endpoints SHALL return raw JSON mode (`serde_json::Value`) for `data` (FR-049, DR-002).

### In-Scope / Out-of-Scope

- In-scope: endpoints explicitly listed under RDD §6.2.
  - Out-of-scope: any other undocumented endpoints; those require OpenAPI addition or explicit RDD update.

### Inputs/Outputs (schemas, examples)

- Inputs: `DocQuery` consisting of pagination/projection plus extra query pairs.
  - Outputs:
    - preferred: `Response<serde_json::Value>`
    - fallback: `serde_json::Value` (top-level)

### Types & Definitions

#### `DocDrivenRoot`

- **Kind:** Public API (Routes root)
- **Purpose:** Root accessor for doc-driven endpoints absent from OpenAPI, returning raw JSON.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|molecules|DocDrivenMolecules|required|Doc-driven molecules endpoints.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Additive endpoints are backward compatible.
- **Location:** src/routes/doc_driven/mod.rs
- **Related Requirement IDs:** FR-049..FR-059
- **Related Test Case IDs:** TBD

#### `DocQuery`

- **Kind:** DTO (Doc-driven query)
- **Purpose:** Query container for doc-driven endpoints (pagination, projection, extra).
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|pagination|Option<Pagination>|optional|Common pagination.|
|projection|Option<Projection>|optional|Common projection.|
|extra|ExtraQueryParams|required|Escape hatch.|

- **Serialization / Schema Notes:** Converted to query pairs via Query module.
- **Versioning / Compatibility Notes:** Backward compatible.
- **Location:** src/routes/doc_driven/mod.rs
- **Related Requirement IDs:** FR-049..FR-059
- **Related Test Case IDs:** TBD

#### `DocResponse`

- **Kind:** DTO (Doc-driven response)
- **Purpose:** Doc-driven response in raw JSON mode, optionally enveloped.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|enveloped|Option<Response<serde_json::Value>>|optional|Preferred when envelope exists.|
|raw|serde_json::Value|required|Fallback for top-level JSON.|

- **Serialization / Schema Notes:** Serde JSON.
- **Versioning / Compatibility Notes:** Forward compatible.
- **Location:** src/routes/doc_driven/mod.rs
- **Related Requirement IDs:** FR-049..FR-059
- **Related Test Case IDs:** TBD


### Public Interfaces

- `pub struct DocDrivenRoot { /* fields */ }`
  - Provide one async method per endpoint listed below; naming is flexible but must be documented.

### Internal Design

- Hard-code the HTTP paths listed in the RDD for doc-driven endpoints.
  - Use the same Transport as OpenAPI routes.
  - Parse response as envelope-first, fallback-to-raw.

### Source Files & Responsibilities

#### `src/routes/doc_driven/mod.rs`

- **Responsibility:** Doc-driven route root and shared request helpers.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `DocDrivenRoot`, `DocQuery`, `DocResponse`
- **Related requirement IDs:** FR-049..FR-059
- **Related test case IDs:** TBD

#### `src/routes/doc_driven/molecules.rs`

- **Responsibility:** Doc-driven molecules endpoints missing from OpenAPI (raw JSON mode).
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `DocDrivenRoot`, `DocQuery`, `DocResponse`
- **Related requirement IDs:** FR-049..FR-059
- **Related test case IDs:** TBD


### Data Model

- `DocResponse` types are raw JSON values plus optional envelope metadata.

### Business Rules & Validation (mapped to requirement IDs)

- Escape hatch query parameters are passed through verbatim.

### Error Handling

- HTTP 422 => `ValidationError`
  - HTTP 429/5xx => `RetryableHttpError` (subject to retry policy)
  - Other non-2xx => `HttpError`

### Logging & Metrics

- Same as Transport.

### Security

- Same as Transport.

### Performance/Scalability Notes

- Minimal parsing overhead due to raw JSON mode.

### Dependencies

- Transport, Query, Data, Errors.

### Test Design

- DT-MANIFEST-001: asserts doc-driven endpoint list matches RDD and implemented methods exist.
  - DT-<FR-MOL-...>: validates URL/query formation and deserialization into JSON.

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|DR-001|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-DR-001|Covered|
|DR-002|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-DR-002|Covered|
|DR-003|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-DR-003|Covered|
|FR-002|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-002|Covered|
|FR-003|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-003|Covered|
|FR-004|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-004|Covered|
|FR-005|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-005|Covered|
|FR-049|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|DT-MANIFEST-001, UT-FR-049|Covered|
|FR-050|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|DT-FR-MOL-ASSOC-GET|Covered|
|FR-051|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|DT-FR-MOL-BONDING-GET|Covered|
|FR-052|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|DT-FR-MOL-CORE-GET|Covered|
|FR-053|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|DT-FR-MOL-ORBITALS-GET|Covered|
|FR-054|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|DT-FR-MOL-PARTIAL_CHARGES-GET|Covered|
|FR-055|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|DT-FR-MOL-PARTIAL_SPINS-GET|Covered|
|FR-056|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|DT-FR-MOL-REDOX-GET|Covered|
|FR-057|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|DT-FR-MOL-TASKS-GET|Covered|
|FR-058|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|DT-FR-MOL-THERMO-GET|Covered|
|FR-059|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|DT-FR-MOL-VIBRATIONS-GET|Covered|
|FR-060|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-060|Covered|
|FR-061|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-061|Covered|
|FR-062|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-062|Covered|
|FR-063|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-063|Covered|
|FR-064|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-064|Covered|
|FR-065|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-065|Covered|
|FR-066|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-066|Covered|
|FR-067|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-067|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|
|FR-068|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|UT-FR-068|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|
|FR-071|§6 "Doc-Driven Routes"|`src/routes/doc_driven/mod.rs`, `src/routes/doc_driven/molecules.rs`|PT-MANIFEST-001, PT-PY-ALL-001|Covered|


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
