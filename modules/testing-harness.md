<!-- filename: modules/testing-harness.md -->

# 1. Title Page

**Module:** Testing Harness  
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

Provide deterministic automated verification that the client satisfies coverage and contract requirements: per-operation contract tests, inventory cross-checks, and opt-in integration smoke tests.

## 4.2 Scope / Out of Scope

- In-scope: CI tests verifying coverage and contract fidelity (unit tests + contract tests always run; integration smoke tests are conditional; see §8 Testing Harness).
  - Out-of-scope: performance benchmarking suites.

## 4.3 Definitions & Acronyms (module-scoped)

- This module follows the shared glossary defined in the Index (modules/README.md §4.3).
- Module-specific terms are defined inline where first introduced.

# 5. Requirements Coverage (module-scoped)

## 5.1 Covered Requirements (IDs + brief statement)

Primary (ownership) coverage:

|Requirement ID|Brief|Test Case IDs|Coverage Status|
|---|---|---|---|
|NFR-005..NFR-009|Range of related requirements; see §7 for full list.|See §7|Covered|
|FR-071|The Rust client MUST implement the full API surface of the Python `mp-api` client (behavioral parity; API shape may differ). (RDD: USER-PYTHON-PARITY-COVERAGE-001)|PT-MANIFEST-001, PT-PY-ALL-001|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

- Parity tests require Python `mp-api` available when MP_API_PY_PARITY=1 (NFR-009).
- Integration tests are opt-in and require an API key in env (NFR-007).

# 6. Module Detailed Design

## Testing Harness

### Purpose

Provide deterministic automated verification that the client satisfies coverage and contract requirements: per-operation contract tests, inventory cross-checks, and opt-in integration smoke tests.

### Responsibilities (explicit; MUST/SHALL statements; testable)

- Unit tests SHALL cover query serialization and error mapping (RDD §11.1; NFR-005).
  - Contract tests SHALL validate request/response shapes against OpenAPI for every Appendix A operation (RDD §11.2; NFR-006).
  - Integration smoke tests SHALL be skipped unless `MP_API_KEY` or `PMG_MAPI_KEY` is set; when set, they SHALL run as smoke tests (RDD §11.3; NFR-007).
  - A manifest test SHALL assert 1:1 coverage between Appendix A inventory, implemented methods, and contract tests (FR-007).
  - A CI check SHALL verify the crate baseline dependency set (NFR-008).
  - When enabled, Python parity tests SHALL assert full Python `mp-api` API surface coverage and input/output equivalence (NFR-009, FR-071).

### In-Scope / Out-of-Scope

- In-scope: CI tests verifying coverage and contract fidelity (unit tests + contract tests always run; integration smoke tests are conditional; see §8 Testing Harness).
  - Out-of-scope: performance benchmarking suites.

### Inputs/Outputs (schemas, examples)

- Inputs: `spec/openapi.json`; optional API key env vars for integration.
  - Outputs: CI pass/fail, test reports.

### Types & Definitions

#### `CT-MANIFEST-001`

- **Kind:** TBD
- **Purpose:** Manifest test ID asserting 1:1 coverage for OpenAPI operations.
- **Fields / Properties**
_TBD: Field list not specified by upstream MDDD/OpenAPI; treat as an implementation detail._
- **Serialization / Schema Notes:** TBD
- **Versioning / Compatibility Notes:** TBD
- **Location:** TBD
- **Related Requirement IDs:** TBD
- **Related Test Case IDs:** TBD

#### `IT-SMOKE-001`

- **Kind:** TBD
- **Purpose:** Integration smoke test ID (gated).
- **Fields / Properties**
_TBD: Field list not specified by upstream MDDD/OpenAPI; treat as an implementation detail._
- **Serialization / Schema Notes:** TBD
- **Versioning / Compatibility Notes:** TBD
- **Location:** TBD
- **Related Requirement IDs:** TBD
- **Related Test Case IDs:** TBD

#### `CI-DEPS-001`

- **Kind:** TBD
- **Purpose:** CI check ID asserting dependency baseline compliance (Cargo.toml vs §11 Assumptions).
- **Fields / Properties**
_TBD: Field list not specified by upstream MDDD/OpenAPI; treat as an implementation detail._
- **Serialization / Schema Notes:** TBD
- **Versioning / Compatibility Notes:** TBD
- **Location:** TBD
- **Related Requirement IDs:** TBD
- **Related Test Case IDs:** TBD

#### `PT-MANIFEST-001`

- **Kind:** TBD
- **Purpose:** Parity manifest test ID asserting full Python `mp-api` API surface coverage via mapping.
- **Fields / Properties**
_TBD: Field list not specified by upstream MDDD/OpenAPI; treat as an implementation detail._
- **Serialization / Schema Notes:** TBD
- **Versioning / Compatibility Notes:** TBD
- **Location:** TBD
- **Related Requirement IDs:** TBD
- **Related Test Case IDs:** TBD

#### `PT-PY-ALL-001`

- **Kind:** TBD
- **Purpose:** Python parity suite test ID executing parity vectors for every mapped Python API.
- **Fields / Properties**
_TBD: Field list not specified by upstream MDDD/OpenAPI; treat as an implementation detail._
- **Serialization / Schema Notes:** TBD
- **Versioning / Compatibility Notes:** TBD
- **Location:** TBD
- **Related Requirement IDs:** TBD
- **Related Test Case IDs:** TBD


### Public Interfaces

- Tests are invoked via `cargo test`; the harness provides:
    - `CT-MANIFEST-001`
    - `CT-<FR-...>` per OpenAPI endpoint
    - `DT-<FR-...>` per doc-driven endpoint
    - `IT-SMOKE-001` (conditional)

### Internal Design

- Contract manifest:
    - Parse `spec/openapi.json`
    - Compare operation list to generated `src/routes/openapi/inventory.rs`
  - Per-operation tests:
    - URL/query serialization checks against OpenAPI parameter names
    - Envelope parsing checks using minimal fixtures where feasible
  - Integration smoke tests:
    - gated by env var presence; use stable *query-based* smoke tests to avoid hard-coded IDs (e.g., `GET /materials/summary/?formula=Si&_limit=1&_fields=material_id` and `GET /molecules/summary/?_limit=1&_fields=molecule_id` when those routes exist in OpenAPI).
  - Python parity tests:
    - gated by `MP_API_PY_PARITY=1` and Python availability
    - PT-MANIFEST-001 validates full Python API coverage via `tests/python/parity_manifest.json`
    - PT-PY-ALL-001 executes one or more parity vectors for every Python `mp-api` API in the manifest, invoking Python and Rust with identical input values, and compares canonical JSON outputs; fails on any missing mapping/implementation or any mismatch

### Source Files & Responsibilities

#### `tests/contract_manifest.rs`

- **Responsibility:** Manifest tests asserting inventory parity (OpenAPI ↔ methods ↔ tests).
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `CT-MANIFEST-001`, `IT-SMOKE-001`, `CI-DEPS-001`
- **Related requirement IDs:** NFR-005..NFR-009, FR-071
- **Related test case IDs:** TBD, PT-MANIFEST-001, PT-PY-ALL-001

#### `tests/contract_openapi_materials.rs`

- **Responsibility:** Contract tests for OpenAPI materials operations.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `CT-MANIFEST-001`, `IT-SMOKE-001`, `CI-DEPS-001`
- **Related requirement IDs:** NFR-005..NFR-009, FR-071
- **Related test case IDs:** TBD, PT-MANIFEST-001, PT-PY-ALL-001

#### `tests/contract_openapi_molecules.rs`

- **Responsibility:** Contract tests for OpenAPI molecules operations.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `CT-MANIFEST-001`, `IT-SMOKE-001`, `CI-DEPS-001`
- **Related requirement IDs:** NFR-005..NFR-009, FR-071
- **Related test case IDs:** TBD, PT-MANIFEST-001, PT-PY-ALL-001

#### `tests/contract_openapi_defects.rs`

- **Responsibility:** Contract tests for OpenAPI defects operations.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `CT-MANIFEST-001`, `IT-SMOKE-001`, `CI-DEPS-001`
- **Related requirement IDs:** NFR-005..NFR-009, FR-071
- **Related test case IDs:** TBD, PT-MANIFEST-001, PT-PY-ALL-001

#### `tests/contract_openapi_doi.rs`

- **Responsibility:** Contract tests for OpenAPI DOI operations.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `CT-MANIFEST-001`, `IT-SMOKE-001`, `CI-DEPS-001`
- **Related requirement IDs:** NFR-005..NFR-009, FR-071
- **Related test case IDs:** TBD, PT-MANIFEST-001, PT-PY-ALL-001

#### `tests/doc_driven_molecules.rs`

- **Responsibility:** Doc-driven endpoint tests (URL/query formation + JSON deserialization).
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `CT-MANIFEST-001`, `IT-SMOKE-001`, `CI-DEPS-001`
- **Related requirement IDs:** NFR-005..NFR-009, FR-071
- **Related test case IDs:** TBD, PT-MANIFEST-001, PT-PY-ALL-001

#### `tests/integration_smoke.rs`

- **Responsibility:** Opt-in live smoke tests gated by MP_API_KEY/PMG_MAPI_KEY.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `CT-MANIFEST-001`, `IT-SMOKE-001`, `CI-DEPS-001`
- **Related requirement IDs:** NFR-005..NFR-009, FR-071
- **Related test case IDs:** TBD, PT-MANIFEST-001, PT-PY-ALL-001

#### `tests/parity_python_mp_api.rs`

- **Responsibility:** Optional parity tests comparing Rust client calls with the Python `mp-api` client across the full mapped API surface.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `CT-MANIFEST-001`, `IT-SMOKE-001`, `CI-DEPS-001`
- **Related requirement IDs:** NFR-005..NFR-009, FR-071
- **Related test case IDs:** TBD, PT-MANIFEST-001, PT-PY-ALL-001

#### `tests/python/parity_mp_api.py`

- **Responsibility:** Python helper used by parity tests to enumerate Python `mp-api` APIs and produce canonical JSON outputs.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `CT-MANIFEST-001`, `IT-SMOKE-001`, `CI-DEPS-001`
- **Related requirement IDs:** NFR-005..NFR-009, FR-071
- **Related test case IDs:** TBD, PT-MANIFEST-001, PT-PY-ALL-001

#### `tests/python/parity_manifest.json`

- **Responsibility:** Version-controlled mapping of Python `mp-api` API identifiers to Rust method identifiers.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `CT-MANIFEST-001`, `IT-SMOKE-001`, `CI-DEPS-001`
- **Related requirement IDs:** NFR-005..NFR-009, FR-071
- **Related test case IDs:** TBD, PT-MANIFEST-001, PT-PY-ALL-001

#### `tests/fixtures/`

- **Responsibility:** JSON fixtures used by contract tests (minimal schema-conformant payloads).
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `CT-MANIFEST-001`, `IT-SMOKE-001`, `CI-DEPS-001`
- **Related requirement IDs:** NFR-005..NFR-009, FR-071
- **Related test case IDs:** TBD, PT-MANIFEST-001, PT-PY-ALL-001


### Data Model

- Fixtures stored under `tests/fixtures/`.

### Business Rules & Validation (mapped to requirement IDs)

- Integration tests do not run without API key env vars.

### Error Handling

- Tests must not print API keys; all failures redact secrets.

### Logging & Metrics

- Use `tracing_test` or equivalent to capture logs and assert required fields for logging requirement.

### Security

- CI secrets handled by CI environment; tests must not print or store API keys.

### Performance/Scalability Notes

- Integration tests are skipped by default to keep CI fast when secrets are absent.

### Dependencies

- `wiremock`/`httpmock`, `openapiv3`, `tokio`.

### Test Design

- CI-PIPELINE-001: CI runs unit + contract tests always; integration tests are conditional.

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|FR-007|§6 "Testing Harness"|`tests/contract_manifest.rs`, `tests/contract_openapi_materials.rs`, `tests/contract_openapi_molecules.rs`, `tests/contract_openapi_defects.rs`, `tests/contract_openapi_doi.rs`, `tests/doc_driven_molecules.rs`, `tests/integration_smoke.rs`, `tests/parity_python_mp_api.rs`, `tests/python/parity_mp_api.py`, `tests/python/parity_manifest.json`, `tests/fixtures/`|CT-MANIFEST-001, UT-INVENTORY-001|Covered|
|FR-071|§6 "Testing Harness"|`tests/contract_manifest.rs`, `tests/contract_openapi_materials.rs`, `tests/contract_openapi_molecules.rs`, `tests/contract_openapi_defects.rs`, `tests/contract_openapi_doi.rs`, `tests/doc_driven_molecules.rs`, `tests/integration_smoke.rs`, `tests/parity_python_mp_api.rs`, `tests/python/parity_mp_api.py`, `tests/python/parity_manifest.json`, `tests/fixtures/`|PT-MANIFEST-001, PT-PY-ALL-001|Covered|
|NFR-005|§6 "Testing Harness"|`tests/contract_manifest.rs`, `tests/contract_openapi_materials.rs`, `tests/contract_openapi_molecules.rs`, `tests/contract_openapi_defects.rs`, `tests/contract_openapi_doi.rs`, `tests/doc_driven_molecules.rs`, `tests/integration_smoke.rs`, `tests/parity_python_mp_api.rs`, `tests/python/parity_mp_api.py`, `tests/python/parity_manifest.json`, `tests/fixtures/`|CI-PIPELINE-001|Covered|
|NFR-006|§6 "Testing Harness"|`tests/contract_manifest.rs`, `tests/contract_openapi_materials.rs`, `tests/contract_openapi_molecules.rs`, `tests/contract_openapi_defects.rs`, `tests/contract_openapi_doi.rs`, `tests/doc_driven_molecules.rs`, `tests/integration_smoke.rs`, `tests/parity_python_mp_api.rs`, `tests/python/parity_mp_api.py`, `tests/python/parity_manifest.json`, `tests/fixtures/`|CI-PIPELINE-001|Covered|
|NFR-007|§6 "Testing Harness"|`tests/contract_manifest.rs`, `tests/contract_openapi_materials.rs`, `tests/contract_openapi_molecules.rs`, `tests/contract_openapi_defects.rs`, `tests/contract_openapi_doi.rs`, `tests/doc_driven_molecules.rs`, `tests/integration_smoke.rs`, `tests/parity_python_mp_api.rs`, `tests/python/parity_mp_api.py`, `tests/python/parity_manifest.json`, `tests/fixtures/`|CI-PIPELINE-001|Covered|
|NFR-008|§6 "Testing Harness"|`tests/contract_manifest.rs`, `tests/contract_openapi_materials.rs`, `tests/contract_openapi_molecules.rs`, `tests/contract_openapi_defects.rs`, `tests/contract_openapi_doi.rs`, `tests/doc_driven_molecules.rs`, `tests/integration_smoke.rs`, `tests/parity_python_mp_api.rs`, `tests/python/parity_mp_api.py`, `tests/python/parity_manifest.json`, `tests/fixtures/`|CI-DEPS-001|Covered|
|NFR-009|§6 "Testing Harness"|`tests/contract_manifest.rs`, `tests/contract_openapi_materials.rs`, `tests/contract_openapi_molecules.rs`, `tests/contract_openapi_defects.rs`, `tests/contract_openapi_doi.rs`, `tests/doc_driven_molecules.rs`, `tests/integration_smoke.rs`, `tests/parity_python_mp_api.rs`, `tests/python/parity_mp_api.py`, `tests/python/parity_manifest.json`, `tests/fixtures/`|PT-MANIFEST-001, PT-PY-ALL-001|Covered|


# 8. Open Questions (module-scoped)

- Python parity environment contract (Python `mp-api` importability and version pinning) needs a repeatable CI strategy (NFR-009).
- Live smoke test queries must remain stable over time; ensure they are query-based (not hard-coded IDs) unless stable identifiers are agreed.

# 9. Final Self-Check (module-scoped)

- English-only content (code identifiers/proper nouns allowed): **Yes**
- Table of Contents present: **Yes**
- Covered requirements listed (primary + full appendix): **Yes**
- Responsibility contract uses SHALL/MUST language: **Yes**
- Types & Definitions includes field-level details (or explicit TBD where upstream spec is incomplete): **Yes**
- Source Files & Responsibilities enumerated for the module directory: **Yes**
- Traceability appendix includes requirement-to-test mapping: **Yes**
