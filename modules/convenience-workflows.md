<!-- filename: modules/convenience-workflows.md -->

# 1. Title Page

**Module:** Convenience Workflows  
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

Provide ergonomic, examples-driven helper methods that compose underlying route calls and field projection to match common user workflows from official documentation.

## 4.2 Scope / Out of Scope

- In-scope: convenience requirements listed in RDD §6.3.
  - Out-of-scope: higher-level multi-call workflows not explicitly required.

## 4.3 Definitions & Acronyms (module-scoped)

- This module follows the shared glossary defined in the Index (modules/README.md §4.3).
- Module-specific terms are defined inline where first introduced.

# 5. Requirements Coverage (module-scoped)

## 5.1 Covered Requirements (IDs + brief statement)

Primary (ownership) coverage:

|Requirement ID|Brief|Test Case IDs|Coverage Status|
|---|---|---|---|
|FR-060..FR-068|Range of related requirements; see §7 for full list.|See §7|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

_None._

# 6. Module Detailed Design

## Convenience Workflows

### Purpose

Provide ergonomic, examples-driven helper methods that compose underlying route calls and field projection to match common user workflows from official documentation.

### Responsibilities (explicit; MUST/SHALL statements; testable)

- The client SHALL provide the convenience methods enumerated in RDD §6.3 and implement them as thin wrappers over underlying endpoints (FR-060).
  - Each convenience method SHALL make exactly one underlying HTTP request to the specified endpoint with the required projection parameters, and SHALL parse the response into the specified sub-structure (RDD acceptance statements for each FR-MPR-*).

### In-Scope / Out-of-Scope

- In-scope: convenience requirements listed in RDD §6.3.
  - Out-of-scope: higher-level multi-call workflows not explicitly required.

### Inputs/Outputs (schemas, examples)

- Inputs: identifiers such as `material_id` or `task_id`.
  - Outputs: extracted artifacts (structure dict, task_id list, bandstructure, DOS, phonon variants, charge density).

### Types & Definitions

#### `ConvenienceRoot`

- **Kind:** Public API (Convenience)
- **Purpose:** Examples-driven helper methods composing a single underlying route call and projection.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|inner|Arc<InnerClient>|required|Access to route roots and transport.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Additive methods are backward compatible.
- **Location:** src/convenience/mod.rs
- **Related Requirement IDs:** FR-060..FR-068
- **Related Test Case IDs:** TBD


### Public Interfaces

- Implement one async method per requirement listed below; exact Rust type names may differ but semantics are normative.

### Internal Design

- Compose a single OpenAPI route call plus projection:
    - `GET /materials/summary/` for structure/task_ids/bandstructure/dos.
    - `GET /materials/phonon/` for phonon variants.
  - Charge density feature detection:
    - If the relevant route is missing from OpenAPI, return `UnsupportedBySpecification("charge_density")` without HTTP.
    - If present, call the OpenAPI route and return `Response<serde_json::Value>` unless the schema is explicitly defined in OpenAPI, in which case return `Response<T>` for that schema.

### Source Files & Responsibilities

#### `src/convenience/mod.rs`

- **Responsibility:** Example-driven convenience wrappers composing one underlying route call + projection.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `ConvenienceRoot`
- **Related requirement IDs:** FR-060..FR-068
- **Related test case IDs:** TBD


### Data Model

- Output artifact types are derived from the underlying response document model; when schema is unknown, use raw JSON.

### Business Rules & Validation (mapped to requirement IDs)

- Exactly one underlying request per convenience method (enforced in tests).

### Error Handling

- Propagate underlying route errors unchanged.
  - Charge density missing => `UnsupportedBySpecification("charge_density")`.

### Logging & Metrics

- Same as Transport; convenience methods may add span fields (e.g., material_id) but MUST not log secrets.

### Security

- Same as Transport.

### Performance/Scalability Notes

- Single-call wrappers; no additional network overhead.

### Dependencies

- OpenAPI Routes, Data, Errors.

### Test Design

- UT-<FR-MPR-...>: verify exactly one underlying request with expected path and projection; parse into expected sub-structure.

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|FR-060|§6 "Convenience Workflows"|`src/convenience/mod.rs`|UT-FR-060|Covered|
|FR-061|§6 "Convenience Workflows"|`src/convenience/mod.rs`|UT-FR-061|Covered|
|FR-062|§6 "Convenience Workflows"|`src/convenience/mod.rs`|UT-FR-062|Covered|
|FR-063|§6 "Convenience Workflows"|`src/convenience/mod.rs`|UT-FR-063|Covered|
|FR-064|§6 "Convenience Workflows"|`src/convenience/mod.rs`|UT-FR-064|Covered|
|FR-065|§6 "Convenience Workflows"|`src/convenience/mod.rs`|UT-FR-065|Covered|
|FR-066|§6 "Convenience Workflows"|`src/convenience/mod.rs`|UT-FR-066|Covered|
|FR-067|§6 "Convenience Workflows"|`src/convenience/mod.rs`|UT-FR-067|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|
|FR-068|§6 "Convenience Workflows"|`src/convenience/mod.rs`|UT-FR-068|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|
|FR-071|§6 "Convenience Workflows"|`src/convenience/mod.rs`|PT-MANIFEST-001, PT-PY-ALL-001|Covered|


# 8. Open Questions (module-scoped)

- Charge density endpoint discovery: exact OpenAPI path/schema names are unknown; behavior MUST fall back to `UnsupportedBySpecification("charge_density")` without HTTP if not in spec (FR-067/FR-068).

# 9. Final Self-Check (module-scoped)

- English-only content (code identifiers/proper nouns allowed): **Yes**
- Table of Contents present: **Yes**
- Covered requirements listed (primary + full appendix): **Yes**
- Responsibility contract uses SHALL/MUST language: **Yes**
- Types & Definitions includes field-level details (or explicit TBD where upstream spec is incomplete): **Yes**
- Source Files & Responsibilities enumerated for the module directory: **Yes**
- Traceability appendix includes requirement-to-test mapping: **Yes**
