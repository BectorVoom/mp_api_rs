<!-- filename: modules/openapi-routes.md -->

# 1. Title Page

**Module:** OpenAPI Routes  
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

Provide typed, async methods for every OpenAPI operation enumerated in Appendix A-OpenAPI, with consistent application of cross-cutting behavior.

## 4.2 Scope / Out of Scope

- In-scope: all OpenAPI-defined endpoints (RDD §6.1 and Appendix A-OpenAPI).
  - Out-of-scope: endpoints absent from OpenAPI (handled by Doc-Driven Routes).

## 4.3 Definitions & Acronyms (module-scoped)

- This module follows the shared glossary defined in the Index (modules/README.md §4.3).
- Module-specific terms are defined inline where first introduced.

# 5. Requirements Coverage (module-scoped)

## 5.1 Covered Requirements (IDs + brief statement)

Primary (ownership) coverage:

|Requirement ID|Brief|Test Case IDs|Coverage Status|
|---|---|---|---|
|FR-007|For every OpenAPI operation enumerated in Appendix A-OpenAPI, the Rust client MUST provide an async method that issues the corresponding HTTP request and returns a parsed response envelope or a typed error. (RDD: FR-COMMON-OPENAPI_COVERAGE-001)|CT-MANIFEST-001, UT-INVENTORY-001|Covered|
|FR-008..FR-048|Range of related requirements; see §7 for full list.|See §7|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

_None._

# 6. Module Detailed Design

## OpenAPI Routes

### Purpose

Provide typed, async methods for every OpenAPI operation enumerated in Appendix A-OpenAPI, with consistent application of cross-cutting behavior.

### Responsibilities (explicit; MUST/SHALL statements; testable)

- For every OpenAPI operation in Appendix A-OpenAPI, the client SHALL provide an async method issuing the corresponding HTTP request and returning a parsed response envelope or typed error (FR-007).
  - Methods SHALL use HTTP method + path from OpenAPI and serialize parameters per OpenAPI definitions (FR-007).
  - Methods SHALL apply common behavior: auth header injection, pagination, field projection, error mapping (FR-001, FR-002, FR-003, FR-006).

### In-Scope / Out-of-Scope

- In-scope: all OpenAPI-defined endpoints (RDD §6.1 and Appendix A-OpenAPI).
  - Out-of-scope: endpoints absent from OpenAPI (handled by Doc-Driven Routes).

### Inputs/Outputs (schemas, examples)

- Inputs: per-operation parameter structs (generated), optional request body structs for POST.
  - Outputs: `Response<TDoc>` where `TDoc` is the per-operation document type.

### Types & Definitions

#### `OpenApiRoot`

- **Kind:** Public API (Routes root)
- **Purpose:** Root accessor for OpenAPI-generated route group clients.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|materials|MaterialsRoot|required|Materials operations.|
|molecules|MoleculesRoot|required|Molecules operations.|
|defects|DefectsRoot|required|Defects operations.|
|doi|DoiRoot|required|DOI operations.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Additive route groups are backward compatible.
- **Location:** src/routes/openapi/mod.rs
- **Related Requirement IDs:** FR-007, FR-008..FR-048
- **Related Test Case IDs:** CT-MANIFEST-001, UT-INVENTORY-001

#### `MaterialsRoot`

- **Kind:** Public API (Routes client)
- **Purpose:** Typed async methods for Materials OpenAPI operations.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|transport|Transport|required|Shared transport handle.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Generated API; changes follow OpenAPI evolution.
- **Location:** src/routes/openapi/materials.rs (generated or semi-generated)
- **Related Requirement IDs:** FR-007, FR-008..FR-048
- **Related Test Case IDs:** CT-MANIFEST-001, UT-INVENTORY-001

#### `MoleculesRoot`

- **Kind:** Public API (Routes client)
- **Purpose:** Typed async methods for Molecules OpenAPI operations.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|transport|Transport|required|Shared transport handle.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Generated API.
- **Location:** src/routes/openapi/molecules.rs (generated or semi-generated)
- **Related Requirement IDs:** FR-007, FR-008..FR-048
- **Related Test Case IDs:** CT-MANIFEST-001, UT-INVENTORY-001

#### `DefectsRoot`

- **Kind:** Public API (Routes client)
- **Purpose:** Typed async methods for Defects OpenAPI operations.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|transport|Transport|required|Shared transport handle.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Generated API.
- **Location:** src/routes/openapi/defects.rs (generated or semi-generated)
- **Related Requirement IDs:** FR-007, FR-008..FR-048
- **Related Test Case IDs:** CT-MANIFEST-001, UT-INVENTORY-001

#### `DoiRoot`

- **Kind:** Public API (Routes client)
- **Purpose:** Typed async methods for DOI OpenAPI operations.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|transport|Transport|required|Shared transport handle.|

- **Serialization / Schema Notes:** Not serialized.
- **Versioning / Compatibility Notes:** Generated API.
- **Location:** src/routes/openapi/doi.rs (generated or semi-generated)
- **Related Requirement IDs:** FR-007, FR-008..FR-048
- **Related Test Case IDs:** CT-MANIFEST-001, UT-INVENTORY-001

#### `inventory::Operation`

- **Kind:** Inventory entry (DTO)
- **Purpose:** Inventory entry describing one OpenAPI operation for coverage checks.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|method|http::Method|required|HTTP method.|
|path|&'static str|required|OpenAPI path template.|
|operation_id|&'static str|required|OpenAPI operationId.|
|tag|Option<&'static str>|optional|Route group tag (if available).|

- **Serialization / Schema Notes:** Not serialized; compiled into inventory list.
- **Versioning / Compatibility Notes:** Internal.
- **Location:** src/routes/openapi/inventory.rs (generated)
- **Related Requirement IDs:** FR-007, FR-008..FR-048
- **Related Test Case IDs:** CT-MANIFEST-001, UT-INVENTORY-001


### Public Interfaces

- Namespace shape (example; naming may differ, coverage is mandatory):
    - `routes::openapi::OpenApiRoot`
      - `.materials() -> MaterialsRoot`
      - `.molecules() -> MoleculesRoot`
      - `.defects() -> DefectsRoot`
      - `.doi() -> DoiRoot`

### Internal Design

- OpenAPI-driven code generation:
    - `spec/openapi.json` is authoritative for paths, params, and schemas.
    - Generate:
      - parameter structs with `serde` serialization aligned to OpenAPI
      - response document structs
      - route methods that create `RequestSpec` and call Transport

### Source Files & Responsibilities

#### `src/routes/mod.rs`

- **Responsibility:** Routes module root; exposes openapi and doc_driven submodules.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `OpenApiRoot`, `MaterialsRoot`, `MoleculesRoot`
- **Related requirement IDs:** FR-007, FR-008..FR-048
- **Related test case IDs:** CT-MANIFEST-001, UT-INVENTORY-001, TBD

#### `src/routes/openapi/mod.rs`

- **Responsibility:** OpenAPI route root and group clients; generated or semi-generated.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `OpenApiRoot`
- **Related requirement IDs:** FR-007, FR-008..FR-048
- **Related test case IDs:** CT-MANIFEST-001, UT-INVENTORY-001, TBD

#### `src/routes/openapi/inventory.rs`

- **Responsibility:** Generated inventory of OpenAPI operations (method/path/operationId) used for coverage checks.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `inventory::Operation`
- **Related requirement IDs:** FR-007, FR-008..FR-048
- **Related test case IDs:** CT-MANIFEST-001, UT-INVENTORY-001, TBD

#### `src/routes/openapi/materials.rs`

- **Responsibility:** Generated materials route group client and per-operation methods.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `MaterialsRoot`
- **Related requirement IDs:** FR-007, FR-008..FR-048
- **Related test case IDs:** CT-MANIFEST-001, UT-INVENTORY-001, TBD

#### `src/routes/openapi/molecules.rs`

- **Responsibility:** Generated molecules route group client and per-operation methods.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `MoleculesRoot`
- **Related requirement IDs:** FR-007, FR-008..FR-048
- **Related test case IDs:** CT-MANIFEST-001, UT-INVENTORY-001, TBD

#### `src/routes/openapi/defects.rs`

- **Responsibility:** Generated defects route group client and per-operation methods.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `DefectsRoot`
- **Related requirement IDs:** FR-007, FR-008..FR-048
- **Related test case IDs:** CT-MANIFEST-001, UT-INVENTORY-001, TBD

#### `src/routes/openapi/doi.rs`

- **Responsibility:** Generated DOI route group client and per-operation methods.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `DoiRoot`
- **Related requirement IDs:** FR-007, FR-008..FR-048
- **Related test case IDs:** CT-MANIFEST-001, UT-INVENTORY-001, TBD


### Data Model

- Generated Rust structs corresponding to OpenAPI schemas (stored under `src/data/models/`).

### Business Rules & Validation (mapped to requirement IDs)

- Common validation:
    - pagination conflicts rejected locally (FR-002)
    - server-side validation errors mapped from 422 (FR-005)

### Error Handling

- Use common Errors module mapping (FR-006).

### Logging & Metrics

- Route methods rely on Transport for request spans and metrics.

### Security

- Inherit Transport security rules (HTTPS, redaction).

### Performance/Scalability Notes

- Generated code should avoid per-call allocation where possible; reuse reqwest client.

### Dependencies

- Transport, Query, Data, Errors, generated code.

### Test Design

- CT-MANIFEST-001: asserts 1:1 coverage between Appendix A inventory, implemented methods, and contract tests (FR-007).
  - CT-<FR-...>: per-operation contract tests validate request formation and envelope parsing.

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|DR-001|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-DR-001|Covered|
|DR-002|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-DR-002|Covered|
|DR-003|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-DR-003|Covered|
|FR-002|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-002|Covered|
|FR-003|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-003|Covered|
|FR-004|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-004|Covered|
|FR-005|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-005|Covered|
|FR-007|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-MANIFEST-001, UT-INVENTORY-001|Covered|
|FR-008|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-DEF-TASKS-GET|Covered|
|FR-009|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-DOI-ROOT-GET|Covered|
|FR-010|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-ABSORPTION-GET|Covered|
|FR-011|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-ALLOYS-GET|Covered|
|FR-012|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-BONDS-GET|Covered|
|FR-013|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-CHEMENV-GET|Covered|
|FR-014|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-CONVERSION_ELECTRODES-GET|Covered|
|FR-015|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-CORE-GET|Covered|
|FR-016|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-CORE_BLESSED_TASKS-GET|Covered|
|FR-017|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-CORE_FIND_STRUCTURE-POST|Covered|
|FR-018|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET|Covered|
|FR-019|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-DIELECTRIC-GET|Covered|
|FR-020|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-ELASTICITY-GET|Covered|
|FR-021|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-ELECTRONIC_STRUCTURE-GET|Covered|
|FR-022|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET|Covered|
|FR-023|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET|Covered|
|FR-024|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-EOS-GET|Covered|
|FR-025|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-FERMI-GET|Covered|
|FR-026|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-GRAIN_BOUNDARIES-GET|Covered|
|FR-027|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-INSERTION_ELECTRODES-GET|Covered|
|FR-028|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-MAGNETISM-GET|Covered|
|FR-029|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-OXIDATION_STATES-GET|Covered|
|FR-030|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-PHONON-GET|Covered|
|FR-031|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-PIEZOELECTRIC-GET|Covered|
|FR-032|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-PROVENANCE-GET|Covered|
|FR-033|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-ROBOCRYS-GET|Covered|
|FR-034|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-ROBOCRYS_TEXT_SEARCH-GET|Covered|
|FR-035|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-SIMILARITY-GET|Covered|
|FR-036|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-SIMILARITY_MATCH-GET|Covered|
|FR-037|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-SUBSTRATES-GET|Covered|
|FR-038|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-SUMMARY-GET|Covered|
|FR-039|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-SURFACE_PROPERTIES-GET|Covered|
|FR-040|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-SYNTHESIS-GET|Covered|
|FR-041|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-TASKS-GET|Covered|
|FR-042|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-TASKS_DEPRECATION-GET|Covered|
|FR-043|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-TASKS_ENTRIES-GET|Covered|
|FR-044|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-TASKS_TRAJECTORY-GET|Covered|
|FR-045|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-THERMO-GET|Covered|
|FR-046|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MAT-XAS-GET|Covered|
|FR-047|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MOL-JCESR-GET|Covered|
|FR-048|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|CT-FR-MOL-SUMMARY-GET|Covered|
|FR-060|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-060|Covered|
|FR-061|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-061|Covered|
|FR-062|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-062|Covered|
|FR-063|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-063|Covered|
|FR-064|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-064|Covered|
|FR-065|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-065|Covered|
|FR-066|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-066|Covered|
|FR-067|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-067|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|
|FR-068|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|UT-FR-068|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|
|FR-071|§6 "OpenAPI Routes"|`src/routes/mod.rs`, `src/routes/openapi/mod.rs`, `src/routes/openapi/inventory.rs`, `src/routes/openapi/materials.rs`, `src/routes/openapi/molecules.rs`, `src/routes/openapi/defects.rs`, `src/routes/openapi/doi.rs`|PT-MANIFEST-001, PT-PY-ALL-001|Covered|


# 8. Open Questions (module-scoped)

- Code generation strategy details (tooling, checked-in vs build-time) must be finalized (see Index §12).
- Contract-test fixture strategy for schema-conformant deserialization must be finalized (see Index §12).

# 9. Final Self-Check (module-scoped)

- English-only content (code identifiers/proper nouns allowed): **Yes**
- Table of Contents present: **Yes**
- Covered requirements listed (primary + full appendix): **Yes**
- Responsibility contract uses SHALL/MUST language: **Yes**
- Types & Definitions includes field-level details (or explicit TBD where upstream spec is incomplete): **Yes**
- Source Files & Responsibilities enumerated for the module directory: **Yes**
- Traceability appendix includes requirement-to-test mapping: **Yes**
