<!-- filename: requirements_updated.md -->

# 1. Title Page

**System Name:** Materials Project Rust Async Client Library  
**Document:** Module Detailed Design Document  
**Version:** 1.5  
**Date:** 2026-03-01  
**Last Updated (JST):** 2026-03-01 11:30  
**Author(s):** mp-api-rs design working group (auto-generated draft)

[Download this document](sandbox:/mnt/data/requirements_updated.md)

# 2. Revision History

| Version | Date | Author | Notes |
|---|---|---|---|
| 1.0 | 2026-02-15 | mp-api-rs design working group (auto-generated) | Initial draft generated from RDD v1.6 |
| 1.1 | 2026-02-28 | mp-api-rs design working group (review patch) | Resolved TBDs, added acceptance criteria, clarified ambiguous terms. |
| 1.2 | 2026-02-28 | mp-api-rs design working group (generated) | Generated complete MDD from provided RDD; fixed acceptance criteria, defaults, and link formatting. |
| 1.4 | 2026-03-01 | mp-api-rs design working group (review patch) | Added dependency baseline and Python mp-api full-surface parity requirement; added normalized REQ-IDs + acceptance criteria catalog; updated Testing Harness for parity and baseline checks. |
| 1.5 | 2026-03-01 | mp-api-rs design working group (review patch) | Clarified Python `mp-api` parity pass criteria (full surface + identical inputs + canonical output equivalence); removed duplicated NFR rows. |


# 3. Table of Contents

- [1. Title Page](#1-title-page)
- [2. Revision History](#2-revision-history)
- [3. Table of Contents](#3-table-of-contents)
- [4. Overview](#4-overview)
  - [4.1 Purpose](#41-purpose)
  - [4.2 Scope / Out of Scope](#42-scope--out-of-scope)
  - [4.3 Definitions & Acronyms](#43-definitions--acronyms)
- [5. Design Philosophy](#5-design-philosophy)
- [6. Requirements Extraction Summary](#6-requirements-extraction-summary)
  - [6.1 Functional Requirements (FR-###)](#61-functional-requirements-fr-)
  - [6.2 Non-Functional Requirements (NFR-###)](#62-non-functional-requirements-nfr-)
  - [6.3 Integration Requirements (IR-###)](#63-integration-requirements-ir-)
  - [6.4 Data Requirements (DR-###)](#64-data-requirements-dr-)
  - [6.5 RDD Section Digest & Coverage Map](#65-rdd-section-digest--coverage-map)
  - [6.6 Acceptance Criteria Catalog (Normalized REQ-IDs)](#66-acceptance-criteria-catalog-normalized-req-ids)
- [7. Architecture Overview](#7-architecture-overview)
  - [7.1 Component Diagram (textual)](#71-component-diagram-textual)
  - [7.2 Data Flow Overview](#72-data-flow-overview)
  - [7.3 High-Level Pipeline](#73-high-level-pipeline)
  - [7.4 Source Tree](#74-source-tree)
  - [7.5 Key Design Decisions & Trade-offs](#75-key-design-decisions--trade-offs)
- [8. Module Detailed Design](#8-module-detailed-design)
- [9. Cross-cutting Concerns](#9-cross-cutting-concerns)
  - [9.1 Observability (logs/metrics/traces)](#91-observability-logsmetricstraces)
  - [9.2 Security & Privacy](#92-security--privacy)
  - [9.3 Performance & Capacity](#93-performance--capacity)
  - [9.4 Deployment & Configuration](#94-deployment--configuration)
  - [9.5 Backward Compatibility / Migration / Rollback](#95-backward-compatibility--migration--rollback)
- [10. Requirements Traceability Matrix](#10-requirements-traceability-matrix)
- [11. Assumptions](#11-assumptions)
- [12. Open Questions](#12-open-questions)
- [13. Final Self-Check](#13-final-self-check)

# 4. Overview

## 4.1 Purpose

Design and specify an implementable, testable, async-first Rust client library for the Materials Project API, covering the full union of (a) endpoints in official Getting Started documentation and (b) endpoints in the uploaded OpenAPI specification.

Primary sources (as cited by the RDD):
- [materialsproject.github.io/api](https://materialsproject.github.io/api/)
- [materialsproject.github.io/api/py-modindex.html](https://materialsproject.github.io/api/py-modindex.html)
- [docs.materialsproject.org/downloading-data/using-the-api](https://docs.materialsproject.org/downloading-data/using-the-api)
- [docs.materialsproject.org/downloading-data/using-the-api/getting-started](https://docs.materialsproject.org/downloading-data/using-the-api/getting-started)
- [docs.materialsproject.org/downloading-data/using-the-api/examples](https://docs.materialsproject.org/downloading-data/using-the-api/examples)
- [api.materialsproject.org/docs](https://api.materialsproject.org/docs)

## 4.2 Scope / Out of Scope

**In Scope**
- All routes (materials, molecules, defects, DOI, and any additional groups present in OpenAPI).
- Functional coverage parity with the Python `mp-api` client: every API exposed by Python `mp-api` MUST have a Rust equivalent within this crate (API shape may differ; behavioral parity is normative).
- Async-only API surface (Tokio runtime).
- Capability parity with official documentation; no requirement to match Python client shape.

**Out of Scope**
- Exact reproduction of Python/pymatgen runtime objects.
- Blocking/synchronous client.

## 4.3 Definitions & Acronyms

(Directly adopted from RDD §3)

- **OpenAPI**: uploaded `openapi.json` (derived from [api.materialsproject.org/docs](https://api.materialsproject.org/docs))
- **Route group**: logical grouping such as `materials.summary`, `materials.tasks`, and other groups present in OpenAPI.
- **Response envelope**: OpenAPI responses use a `{data, errors, meta}` wrapper (see §7).
- **Doc-driven contract**: endpoint listed in official docs but missing from uploaded OpenAPI; client implements HTTP path and parses responses in Raw JSON mode; tests are limited to URL/query formation and JSON deserialization.
- **Raw JSON mode**: response `data` is exposed as `serde_json::Value` to remain forward-compatible when schema is unknown or evolves.
- **Contract test**: validates request parameters and response parsing against OpenAPI schema (where available) or against documented path + JSON deserialization rule.
- **Inventory item**: a single HTTP operation (method + path) enumerated in Appendix A and/or Getting Started tables.
- **RTM**: Requirements Traceability Matrix mapping each inventory item to exactly one requirement ID.
- **Escape hatch (query params)**: API surface allowing callers to supply additional query key/value pairs passed through verbatim.
- **allow_insecure_http**: configuration flag (default `false`) that permits an `http://` base URL for local test harnesses; when `false`, reject non-HTTPS base URLs as a configuration error.
- **timeout**: per-request client timeout applied to the full HTTP operation (connect + request + response body read).
- **concurrency**: max number of in-flight HTTP requests per client instance.
- **qps_limit**: client-side rate limit in requests per second applied across all requests issued by a client instance.
- **user_agent**: HTTP `User-Agent` header sent on each request.
- **correlation ID**: client-generated unique identifier attached to each request for log correlation.

- **full-jitter**: retry backoff strategy where the sleep duration is uniformly randomized in `[0, backoff_cap]` for the attempt.
- **token bucket**: rate limiting algorithm that refills tokens over time; each request consumes one token.
- **idempotent HTTP method**: a method where repeated identical requests have no additional side effects (used to restrict default retries).
- **manifest test**: a test that asserts inventory parity (e.g., OpenAPI operations ↔ implemented methods ↔ contract tests).
- **crate baseline**: The minimum Cargo dependency set (crate names, versions, and required feature flags) required for reproducible builds across environments.
- **parity test**: A cross-language test that invokes the Python `mp-api` API and the mapped Rust API with identical input values and asserts canonical JSON equivalence of their outputs; the parity suite passes iff all Python `mp-api` APIs are mapped/implemented in Rust and all parity vectors pass.
- **canonical JSON equivalence**: The parity comparison rule: parse both outputs into JSON values, canonicalize object key order and apply stable array ordering when a stable key exists (e.g., `material_id`, `task_id`, `molecule_id`), then require deep equality.

# 5. Design Philosophy

This design follows the architectural principles mandated by the prompt and aligns them to RDD requirements.

1. **Modularity & Separation of Concerns**  
   - Decompose into explicit modules: config, auth, query building, transport, middleware, data parsing, routes, and tests.  
   - Each module has a responsibility contract using SHALL/MUST statements.  
   - Motivation: reduces coupling while enabling independent testability (supports FR-007 and the RDD test requirements).

2. **Scalability**  
   - Stateless request execution with client-side concurrency and QPS limiting (FR-070, NFR-001).  
   - Designed for high-volume download workflows (RDD §4) without server overload.

3. **Maintainability**  
   - Treat OpenAPI as authoritative and generate route + model code to prevent drift (RDD §2 assumption; FR-007).  
   - Deterministic query serialization and stable error mapping provide a consistent developer experience.

4. **Security & Compliance**  
   - Enforce HTTPS by default; allow `allow_insecure_http` only when explicitly set (NFR-004).  
   - Mandatory redaction of API keys in logs (NFR-003).

5. **Performance & Reliability**  
   - Retry policy with exponential backoff for transient failures (NFR-002).  
   - Avoid unnecessary allocations and reuse the underlying HTTP client.

Key guiding constraint: **Do not invent facts**. Wherever OpenAPI details are missing (notably doc-driven endpoints), the design explicitly returns raw JSON and limits validation to what is defined in the RDD.

# 6. Requirements Extraction Summary

This section enumerates **all normative requirements** from RDD §6, §9, §10 and additionally extracts explicit data/test requirements from RDD §7 and §11.

## 6.1 Functional Requirements (FR-###)

Each row maps an internal requirement ID (FR-###) to the RDD requirement ID and summary.

| Req ID | RDD ID | Summary |
|---|---|---|
| FR-001 | FR-COMMON-AUTH-001 | Client MUST authenticate using API key header. |
| FR-002 | FR-COMMON-PAGINATION-001 | Client MUST support both page-based and offset-based pagination. |
| FR-003 | FR-COMMON-PROJECTION-001 | Client MUST support field projection. |
| FR-004 | FR-COMMON-ENVELOPE-001 | Client MUST parse the standard `{data, errors, meta}` response envelope. |
| FR-005 | FR-COMMON-VALIDATION-001 | Client MUST parse 422 validation errors into a typed error structure. |
| FR-006 | FR-COMMON-ERROR_MODEL-001 | Client MUST expose a typed error model that distinguishes configuration errors, request construction errors, transport errors, HTTP status errors, validation errors, and deserialization errors. |
| FR-007 | FR-COMMON-OPENAPI_COVERAGE-001 | For every OpenAPI operation enumerated in Appendix A-OpenAPI, the Rust client MUST provide an async method that issues the corresponding HTTP request and returns a parsed response envelope or a typed error. |
| FR-008 | FR-DEF-TASKS-GET | Get DefectTaskDoc documents |
| FR-009 | FR-DOI-ROOT-GET | Get DOIDoc documents |
| FR-010 | FR-MAT-ABSORPTION-GET | Get AbsorptionDoc documents |
| FR-011 | FR-MAT-ALLOYS-GET | Get AlloyPairDoc documents |
| FR-012 | FR-MAT-BONDS-GET | Get BondingDoc documents |
| FR-013 | FR-MAT-CHEMENV-GET | Get ChemEnvDoc documents |
| FR-014 | FR-MAT-CONVERSION_ELECTRODES-GET | Get ConversionElectrodeDoc documents |
| FR-015 | FR-MAT-CORE-GET | Get MaterialsDoc documents |
| FR-016 | FR-MAT-CORE_BLESSED_TASKS-GET | Get MaterialsDoc documents |
| FR-017 | FR-MAT-CORE_FIND_STRUCTURE-POST | Post FindStructure documents |
| FR-018 | FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET | Get FormulaAutocomplete documents |
| FR-019 | FR-MAT-DIELECTRIC-GET | Get DielectricDoc documents |
| FR-020 | FR-MAT-ELASTICITY-GET | Get ElasticityDoc documents |
| FR-021 | FR-MAT-ELECTRONIC_STRUCTURE-GET | Get ElectronicStructureDoc documents |
| FR-022 | FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET | Get ElectronicStructureDoc documents |
| FR-023 | FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET | Get ElectronicStructureDoc documents |
| FR-024 | FR-MAT-EOS-GET | Get EOSDoc documents |
| FR-025 | FR-MAT-FERMI-GET | Get FermiDoc documents |
| FR-026 | FR-MAT-GRAIN_BOUNDARIES-GET | Get GrainBoundaryDoc documents |
| FR-027 | FR-MAT-INSERTION_ELECTRODES-GET | Get InsertionElectrodeDoc documents |
| FR-028 | FR-MAT-MAGNETISM-GET | Get MagnetismDoc documents |
| FR-029 | FR-MAT-OXIDATION_STATES-GET | Get OxidationStateDoc documents |
| FR-030 | FR-MAT-PHONON-GET | Get PhononBSDOSDoc documents |
| FR-031 | FR-MAT-PIEZOELECTRIC-GET | Get PiezoelectricDoc documents |
| FR-032 | FR-MAT-PROVENANCE-GET | Get ProvenanceDoc documents |
| FR-033 | FR-MAT-ROBOCRYS-GET | Get RobocrystallogapherDoc documents |
| FR-034 | FR-MAT-ROBOCRYS_TEXT_SEARCH-GET | Get RobocrystallogapherDoc documents |
| FR-035 | FR-MAT-SIMILARITY-GET | Get SimilarityDoc documents |
| FR-036 | FR-MAT-SIMILARITY_MATCH-GET | Get SimilarityDoc documents |
| FR-037 | FR-MAT-SUBSTRATES-GET | Get SubstratesDoc documents |
| FR-038 | FR-MAT-SUMMARY-GET | Get SummaryDoc documents |
| FR-039 | FR-MAT-SURFACE_PROPERTIES-GET | Get SurfacePropDoc documents |
| FR-040 | FR-MAT-SYNTHESIS-GET | Get SynthesisSearchResultModel documents |
| FR-041 | FR-MAT-TASKS-GET | Get TaskDoc documents |
| FR-042 | FR-MAT-TASKS_DEPRECATION-GET | Get DeprecationDoc documents |
| FR-043 | FR-MAT-TASKS_ENTRIES-GET | Get EntryDoc documents |
| FR-044 | FR-MAT-TASKS_TRAJECTORY-GET | Get TrajectoryDoc documents |
| FR-045 | FR-MAT-THERMO-GET | Get ThermoDoc documents |
| FR-046 | FR-MAT-XAS-GET | Get XASDoc documents |
| FR-047 | FR-MOL-JCESR-GET | Get MoleculesDoc documents |
| FR-048 | FR-MOL-SUMMARY-GET | Get MoleculeSummaryDoc documents |
| FR-049 | FR-COMMON-DOC_DRIVEN-001 | Client MUST implement endpoints listed in official docs (e.g., “Getting Started” endpoint table) but absent from the uploaded OpenAPI, using a doc-driven contract. |
| FR-050 | FR-MOL-ASSOC-GET | Client MUST support searching associated molecule documents via `/molecules/assoc`. |
| FR-051 | FR-MOL-BONDING-GET | Client MUST support searching molecule bonding documents via `/molecules/bonding`. |
| FR-052 | FR-MOL-CORE-GET | Client MUST support searching molecule core documents via `/molecules/core`. |
| FR-053 | FR-MOL-ORBITALS-GET | Client MUST support searching orbital documents via `/molecules/orbitals`. |
| FR-054 | FR-MOL-PARTIAL_CHARGES-GET | Client MUST support searching partial charge documents via `/molecules/partial_charges`. |
| FR-055 | FR-MOL-PARTIAL_SPINS-GET | Client MUST support searching partial spin documents via `/molecules/partial_spins`. |
| FR-056 | FR-MOL-REDOX-GET | Client MUST support searching redox documents via `/molecules/redox`. |
| FR-057 | FR-MOL-TASKS-GET | Client MUST support searching molecule task documents via `/molecules/tasks`. |
| FR-058 | FR-MOL-THERMO-GET | Client MUST support searching molecule thermochemistry documents via `/molecules/thermo`. |
| FR-059 | FR-MOL-VIBRATIONS-GET | Client MUST support searching vibration documents via `/molecules/vibrations`. |
| FR-060 | FR-COMMON-CONVENIENCE-001 | The Rust client MUST provide convenience wrappers matching the official Examples workflows (idiomatic Rust names allowed). |
| FR-061 | FR-MPR-GET_STRUCTURE_BY_MATERIAL_ID | Get a structure for a material_id (wraps `GET /materials/summary/` with projected field `structure`). |
| FR-062 | FR-MPR-GET_TASK_IDS_ASSOCIATED_WITH_MATERIAL_ID | Get task IDs associated with a material_id (wraps `GET /materials/summary/` with projected field `task_ids`). |
| FR-063 | FR-MPR-GET_BANDSTRUCTURE_BY_MATERIAL_ID | Get electronic bandstructure for a material_id (wraps `GET /materials/summary/` with projected field `bandstructure`). |
| FR-064 | FR-MPR-GET_DOS_BY_MATERIAL_ID | Get electronic DOS for a material_id (wraps `GET /materials/summary/` with projected field `dos`). |
| FR-065 | FR-MPR-GET_PHONON_BANDSTRUCTURE_BY_MATERIAL_ID | Get phonon bandstructure for a material_id (wraps phonon endpoint). |
| FR-066 | FR-MPR-GET_PHONON_DOS_BY_MATERIAL_ID | Get phonon DOS for a material_id (wraps phonon endpoint). |
| FR-067 | FR-MPR-GET_CHARGE_DENSITY_FROM_MATERIAL_ID | Get charge density by material_id (Examples-driven). |
| FR-068 | FR-MPR-GET_CHARGE_DENSITY_FROM_TASK_ID | Get charge density by task_id (Examples-driven). |
| FR-069 | OPS-CONFIG-SOURCES-001 | Client MUST support configuration from (1) explicit constructor/builder inputs and (2) environment variables, and MUST define precedence. |
| FR-070 | OPS-SETTINGS-001 | Client MUST support setting: `api_key`, `base_url`, `timeout`, `concurrency`, `qps_limit`, `user_agent`, and `allow_insecure_http` (see §9 security). |
| FR-071 | USER-PYTHON-PARITY-COVERAGE-001 | The Rust client MUST implement the full API surface of the Python `mp-api` client (behavioral parity; API shape may differ). |

## 6.2 Non-Functional Requirements (NFR-###)

| Req ID | RDD ID | Summary |
|---|---|---|
| NFR-001 | NFR-PERF-RATE_LIMIT-001 | Client MUST provide a configurable rate limiter and MUST default to **25 requests per second** (aggregate) unless explicitly overridden. |
| NFR-002 | NFR-REL-RETRY-001 | Client MUST implement retries with exponential backoff for transient failures, using a configurable policy with explicit defaults (max_retries=3, initial_backoff=200ms, max_backoff=2s, jitter=full-jitter). |
| NFR-003 | NFR-OBS-LOGGING-001 | Client MUST support structured logs including request path, status code, latency, retry count, and correlation ID, and MUST redact API keys and other secrets. |
| NFR-004 | NFR-SEC-TLS-001 | Client MUST use HTTPS by default and MUST reject non-HTTPS base URLs unless the caller explicitly opts in (for testing). |
| NFR-005 | NFR-TEST-UNIT-001 | Unit tests SHALL cover query serialization and error mapping. |
| NFR-006 | NFR-TEST-CONTRACT-001 | Contract tests SHALL validate request/response shapes against OpenAPI for every inventory operation in Appendix A-OpenAPI. |
| NFR-007 | NFR-TEST-INTEGRATION-001 | Integration smoke tests SHALL be skipped unless MP_API_KEY or PMG_MAPI_KEY is set; when set, they SHALL run as smoke tests. |
| NFR-008 | NFR-BUILD-DEPS-001: The crate SHALL pin a reproducible crate baseline (versions + required features) for core runtime dependencies. | Testing Harness, Configuration (Config & Builder), HTTP Transport | §11 Assumptions; §8 Testing Harness | CI-DEPS-001 | Covered |
| NFR-009 | NFR-TEST-PYTHON-PARITY-001: When enabled, Python parity tests SHALL assert (a) full Python `mp-api` API surface coverage and (b) input/output equivalence between Rust and Python for every mapped API. | Testing Harness | §8 Testing Harness | PT-MANIFEST-001, PT-PY-ALL-001 | Covered |

## 6.3 Integration Requirements (IR-###)

The RDD does not define standalone IR-prefixed requirements. Interface/integration obligations are captured within FR (e.g., auth header, base URL) and OPS/NFR requirements.  
**Result:** No independent IR-### requirements extracted.

## 6.4 Data Requirements (DR-###)

| Req ID | RDD ID | Summary |
|---|---|---|
| DR-001 | DR-ENVELOPE-001 | Response envelope fields and schemas (data/errors/meta; Error{code,message}; Meta fields). |
| DR-002 | DR-TYPING-001 | Typed and raw JSON modes for response data. |
| DR-003 | DR-SERDE-001 | Serde policy for nullable and unknown fields. |

## 6.5 RDD Section Digest & Coverage Map

### RDD Read-Through Confirmation (all RDD top-level sections found)

- ## 1) Background & Objectives
- ## 2) Scope & Assumptions
- ## 3) Terminology
- ## 4) Use Cases (All adopted)
- ## 5) System Boundary & Context
- ## 6) Functional Requirements
- ## 7) Data Requirements
- ## 8) Rust Client Design Requirements (Public API)
- ## 9) Non-Functional Requirements
- ## 10) Operations & Configuration
- ## 11) Test Requirements
- ## 12) Constraints & Risks
- ## 13) Open Items / Missing Contracts
- ## 14) Appendix A — Population Inventory
- ## 15) Appendix B — RTM (Inventory → Requirement)

### Digest & Mapping (section-by-section)

| RDD Section | Digest | Covered In (Design Doc) |
|---|---|---|
| 1) Background & Objectives | Defines goal: async-first Rust client covering all routes/use cases using public docs + OpenAPI; success criteria for endpoint coverage and CI contract tests. | §4 Overview; §7 Architecture Overview; §8 OpenAPI Routes; §8 Testing Harness; §10 RTM; §13 Self-Check |
| 2) Scope & Assumptions | In-scope/out-of-scope and explicit assumptions/proposals (env var support, OpenAPI authoritative, allow_insecure_http, structured logging). | §4.2 Scope; §5 Design Philosophy; §8 Configuration; §8 HTTP Transport; §9 Cross-cutting Security/Observability; §11 Assumptions |
| 3) Terminology | Defines key terms: route group, response envelope, doc-driven contract, raw JSON mode, contract test, inventory item, RTM, escape hatch, config settings. | §4.3 Definitions; §8 Query Parameters; §8 Doc-Driven Routes; §8 Testing Harness |
| 4) Use Cases (All adopted) | Enumerates supported usage patterns: searches, ID-based retrieval, high-volume downloads (projection/pagination/rate limit), specialized workflows, convenience example workflows. | §7 High-Level Pipeline; §8 OpenAPI Routes; §8 Convenience Workflows; §9 Performance & Capacity |
| 5) System Boundary & Context | Specifies external interface expectations: API key header `X-API-KEY` and configurable base URL. | §8 Auth/Transport; §8 Configuration; §9 Security & Privacy |
| 6) Functional Requirements | Normative functional requirements: common behaviors; OpenAPI operation coverage; doc-driven endpoints; convenience methods; per-endpoint specs. | §6.1 FR table; §7 Architecture; §8 Modules (all); §10 RTM |
| 7) Data Requirements | Defines response envelope schema and Rust typing policy (typed + raw, forward compatibility). | §6.4 DR; §8 Envelope & Models; §9 Backward Compatibility |
| 8) Rust Client Design Requirements (Public API) | Informative guidance on public API shape (nested route clients) and capability checklist. | §8 Client Facade; §7 Component Diagram; §5 Design Philosophy |
| 9) Non-Functional Requirements | Rate limiting default 25 qps, retry policy, structured logging + redaction, TLS enforcement. | §6.2 NFR; §8 Rate Limiter; §8 Retry Policy; §8 HTTP Transport; §9 Cross-cutting |
| 10) Operations & Configuration | Config source precedence; supported settings and their behavioral effects. | §8 Configuration; §8 HTTP Transport; §9 Deployment & Configuration; §10 RTM |
| 11) Test Requirements | Unit tests, OpenAPI contract tests for each operation, and opt-in integration smoke tests gated by env vars. | §8 Testing Harness; §10 RTM; §13 Self-Check |
| 12) Constraints & Risks | Notes OpenAPI gaps (molecules endpoints) and incomplete non-422 error descriptions; mandates surfacing unknown statuses as HttpError. | §8 Doc-Driven Routes; §8 Errors; §9 Reliability; §11 Assumptions; §12 Open Questions |
| 13) Open Items / Missing Contracts | States no open items; doc-driven contract handles endpoints absent from OpenAPI; charge density returns UnsupportedBySpecification if not in spec. | §8 Doc-Driven Routes; §8 Convenience Workflows; §11 Assumptions |
| 14) Appendix A — Population Inventory | Endpoint inventory derived from uploaded OpenAPI including method/path/tag/operationId and schemas. | §8 OpenAPI Routes (codegen + manifest); §10 RTM; §8 Testing Harness (1:1 coverage) |
| 15) Appendix B — RTM (Inventory → Requirement) | Maps each inventory row (method+path) to exactly one requirement ID; target is 0 unmapped rows. | §10 RTM; §8 Testing Harness (inventory cross-check) |

## 6.6 Acceptance Criteria Catalog (Normalized REQ-IDs)

This catalog adds stable requirement IDs (`REQ-*`) and testable acceptance criteria for every extracted FR/NFR/DR row. Each `REQ-*` maps to exactly one source row unless explicitly noted. Rationale/justification is via the cited RDD ID (source-of-truth) and the design rationale in §5.

### 6.6.1 Functional Requirements

- **REQ-F-001** (Source: `FR-001` / RDD ID: `FR-COMMON-AUTH-001`)
  - Requirement: Client MUST authenticate using API key header.
  - Acceptance Criteria:
    - AC-1: Given a built `MpClient` with an API key, when any HTTP request is executed, then the request includes header `X-API-KEY: <api_key>` exactly once.
    - AC-2: Given logging enabled, when requests/errors are logged, then the API key value is not present in any emitted log fields or messages (redaction enforced).

- **REQ-F-002** (Source: `FR-002` / RDD ID: `FR-COMMON-PAGINATION-001`)
  - Requirement: Client MUST support both page-based and offset-based pagination.
  - Acceptance Criteria:
    - AC-1: Given `_page`/`_per_page` are set, when building a request, then query parameters include `_page` and `_per_page` and do not include `_skip`/`_limit`.
    - AC-2: Given `_skip`/`_limit` are set (and `_page`/`_per_page` are unset), when building a request, then query parameters include `_skip` and `_limit`.
    - AC-3: Given both page-based and offset-based inputs are provided, when building a request, then the client returns `InvalidPaginationParameters` and issues no HTTP request.

- **REQ-F-003** (Source: `FR-003` / RDD ID: `FR-COMMON-PROJECTION-001`)
  - Requirement: Client MUST support field projection.
  - Acceptance Criteria:
    - AC-1: Given a projection set via `_fields`, when building a request, then the client serializes `_fields` as a comma-separated list.
    - AC-2: Given `_all_fields=true`, when building a request, then the client serializes `_all_fields=true` and does not require `_fields`.

- **REQ-F-004** (Source: `FR-004` / RDD ID: `FR-COMMON-ENVELOPE-001`)
  - Requirement: Client MUST parse the standard `{data, errors, meta}` response envelope.
  - Acceptance Criteria:
    - AC-1: Given a 2xx response with JSON envelope `{data, errors, meta}`, when parsing, then the client deserializes into `Response<T>` (typed) or `Response<serde_json::Value>` (raw) as configured.
    - AC-2: Given envelope fields are missing or malformed, when parsing, then the client returns `DeserializeError`.

- **REQ-F-005** (Source: `FR-005` / RDD ID: `FR-COMMON-VALIDATION-001`)
  - Requirement: Client MUST parse 422 validation errors into a typed error structure.
  - Acceptance Criteria:
    - AC-1: Given an HTTP 422 response with body matching OpenAPI `HTTPValidationError`, when parsing, then the client returns `MpApiError::ValidationError` containing the parsed details.
    - AC-2: Given a non-422 status, the client does not return `ValidationError`.

- **REQ-F-006** (Source: `FR-006` / RDD ID: `FR-COMMON-ERROR_MODEL-001`)
  - Requirement: Client MUST expose a typed error model that distinguishes configuration errors, request construction errors, transport errors, HTTP status errors, validation errors, and deserialization errors.
  - Acceptance Criteria:
    - AC-1: The public error type includes distinct variants for: configuration errors, request construction errors, transport errors, HTTP status errors, validation errors, and deserialization errors.
    - AC-2: Error variants include status code and a truncated response body (≤ 8192 bytes) for non-2xx HTTP errors.
    - AC-3: Unit tests cover representative mappings for 422, 429, 5xx, transport timeout, and deserialization failure.

- **REQ-F-007** (Source: `FR-007` / RDD ID: `FR-COMMON-OPENAPI_COVERAGE-001`)
  - Requirement: For every OpenAPI operation enumerated in Appendix A-OpenAPI, the Rust client MUST provide an async method that issues the corresponding HTTP request and returns a parsed response envelope or a typed error.
  - Acceptance Criteria:
    - AC-1: For every OpenAPI operation enumerated in Appendix A-OpenAPI (and/or the generated `inventory.rs`), there exists an async client method that issues the corresponding HTTP request.
    - AC-2: A manifest contract test asserts 1:1 coverage between the OpenAPI inventory, implemented methods, and contract tests.

- **REQ-F-008** (Source: `FR-008` / RDD ID: `FR-DEF-TASKS-GET`)
  - Requirement: Get DefectTaskDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-DEF-TASKS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-008` or `CT-FR-DEF-TASKS-GET`) validating request serialization and envelope parsing.

- **REQ-F-009** (Source: `FR-009` / RDD ID: `FR-DOI-ROOT-GET`)
  - Requirement: Get DOIDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-DOI-ROOT-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-009` or `CT-FR-DOI-ROOT-GET`) validating request serialization and envelope parsing.

- **REQ-F-010** (Source: `FR-010` / RDD ID: `FR-MAT-ABSORPTION-GET`)
  - Requirement: Get AbsorptionDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-ABSORPTION-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-010` or `CT-FR-MAT-ABSORPTION-GET`) validating request serialization and envelope parsing.

- **REQ-F-011** (Source: `FR-011` / RDD ID: `FR-MAT-ALLOYS-GET`)
  - Requirement: Get AlloyPairDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-ALLOYS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-011` or `CT-FR-MAT-ALLOYS-GET`) validating request serialization and envelope parsing.

- **REQ-F-012** (Source: `FR-012` / RDD ID: `FR-MAT-BONDS-GET`)
  - Requirement: Get BondingDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-BONDS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-012` or `CT-FR-MAT-BONDS-GET`) validating request serialization and envelope parsing.

- **REQ-F-013** (Source: `FR-013` / RDD ID: `FR-MAT-CHEMENV-GET`)
  - Requirement: Get ChemEnvDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-CHEMENV-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-013` or `CT-FR-MAT-CHEMENV-GET`) validating request serialization and envelope parsing.

- **REQ-F-014** (Source: `FR-014` / RDD ID: `FR-MAT-CONVERSION_ELECTRODES-GET`)
  - Requirement: Get ConversionElectrodeDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-CONVERSION_ELECTRODES-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-014` or `CT-FR-MAT-CONVERSION_ELECTRODES-GET`) validating request serialization and envelope parsing.

- **REQ-F-015** (Source: `FR-015` / RDD ID: `FR-MAT-CORE-GET`)
  - Requirement: Get MaterialsDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-CORE-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-015` or `CT-FR-MAT-CORE-GET`) validating request serialization and envelope parsing.

- **REQ-F-016** (Source: `FR-016` / RDD ID: `FR-MAT-CORE_BLESSED_TASKS-GET`)
  - Requirement: Get MaterialsDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-CORE_BLESSED_TASKS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-016` or `CT-FR-MAT-CORE_BLESSED_TASKS-GET`) validating request serialization and envelope parsing.

- **REQ-F-017** (Source: `FR-017` / RDD ID: `FR-MAT-CORE_FIND_STRUCTURE-POST`)
  - Requirement: Post FindStructure documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-CORE_FIND_STRUCTURE-POST` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-017` or `CT-FR-MAT-CORE_FIND_STRUCTURE-POST`) validating request serialization and envelope parsing.

- **REQ-F-018** (Source: `FR-018` / RDD ID: `FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET`)
  - Requirement: Get FormulaAutocomplete documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-018` or `CT-FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET`) validating request serialization and envelope parsing.

- **REQ-F-019** (Source: `FR-019` / RDD ID: `FR-MAT-DIELECTRIC-GET`)
  - Requirement: Get DielectricDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-DIELECTRIC-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-019` or `CT-FR-MAT-DIELECTRIC-GET`) validating request serialization and envelope parsing.

- **REQ-F-020** (Source: `FR-020` / RDD ID: `FR-MAT-ELASTICITY-GET`)
  - Requirement: Get ElasticityDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-ELASTICITY-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-020` or `CT-FR-MAT-ELASTICITY-GET`) validating request serialization and envelope parsing.

- **REQ-F-021** (Source: `FR-021` / RDD ID: `FR-MAT-ELECTRONIC_STRUCTURE-GET`)
  - Requirement: Get ElectronicStructureDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-ELECTRONIC_STRUCTURE-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-021` or `CT-FR-MAT-ELECTRONIC_STRUCTURE-GET`) validating request serialization and envelope parsing.

- **REQ-F-022** (Source: `FR-022` / RDD ID: `FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET`)
  - Requirement: Get ElectronicStructureDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-022` or `CT-FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET`) validating request serialization and envelope parsing.

- **REQ-F-023** (Source: `FR-023` / RDD ID: `FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET`)
  - Requirement: Get ElectronicStructureDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-023` or `CT-FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET`) validating request serialization and envelope parsing.

- **REQ-F-024** (Source: `FR-024` / RDD ID: `FR-MAT-EOS-GET`)
  - Requirement: Get EOSDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-EOS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-024` or `CT-FR-MAT-EOS-GET`) validating request serialization and envelope parsing.

- **REQ-F-025** (Source: `FR-025` / RDD ID: `FR-MAT-FERMI-GET`)
  - Requirement: Get FermiDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-FERMI-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-025` or `CT-FR-MAT-FERMI-GET`) validating request serialization and envelope parsing.

- **REQ-F-026** (Source: `FR-026` / RDD ID: `FR-MAT-GRAIN_BOUNDARIES-GET`)
  - Requirement: Get GrainBoundaryDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-GRAIN_BOUNDARIES-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-026` or `CT-FR-MAT-GRAIN_BOUNDARIES-GET`) validating request serialization and envelope parsing.

- **REQ-F-027** (Source: `FR-027` / RDD ID: `FR-MAT-INSERTION_ELECTRODES-GET`)
  - Requirement: Get InsertionElectrodeDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-INSERTION_ELECTRODES-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-027` or `CT-FR-MAT-INSERTION_ELECTRODES-GET`) validating request serialization and envelope parsing.

- **REQ-F-028** (Source: `FR-028` / RDD ID: `FR-MAT-MAGNETISM-GET`)
  - Requirement: Get MagnetismDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-MAGNETISM-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-028` or `CT-FR-MAT-MAGNETISM-GET`) validating request serialization and envelope parsing.

- **REQ-F-029** (Source: `FR-029` / RDD ID: `FR-MAT-OXIDATION_STATES-GET`)
  - Requirement: Get OxidationStateDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-OXIDATION_STATES-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-029` or `CT-FR-MAT-OXIDATION_STATES-GET`) validating request serialization and envelope parsing.

- **REQ-F-030** (Source: `FR-030` / RDD ID: `FR-MAT-PHONON-GET`)
  - Requirement: Get PhononBSDOSDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-PHONON-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-030` or `CT-FR-MAT-PHONON-GET`) validating request serialization and envelope parsing.

- **REQ-F-031** (Source: `FR-031` / RDD ID: `FR-MAT-PIEZOELECTRIC-GET`)
  - Requirement: Get PiezoelectricDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-PIEZOELECTRIC-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-031` or `CT-FR-MAT-PIEZOELECTRIC-GET`) validating request serialization and envelope parsing.

- **REQ-F-032** (Source: `FR-032` / RDD ID: `FR-MAT-PROVENANCE-GET`)
  - Requirement: Get ProvenanceDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-PROVENANCE-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-032` or `CT-FR-MAT-PROVENANCE-GET`) validating request serialization and envelope parsing.

- **REQ-F-033** (Source: `FR-033` / RDD ID: `FR-MAT-ROBOCRYS-GET`)
  - Requirement: Get RobocrystallogapherDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-ROBOCRYS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-033` or `CT-FR-MAT-ROBOCRYS-GET`) validating request serialization and envelope parsing.

- **REQ-F-034** (Source: `FR-034` / RDD ID: `FR-MAT-ROBOCRYS_TEXT_SEARCH-GET`)
  - Requirement: Get RobocrystallogapherDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-ROBOCRYS_TEXT_SEARCH-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-034` or `CT-FR-MAT-ROBOCRYS_TEXT_SEARCH-GET`) validating request serialization and envelope parsing.

- **REQ-F-035** (Source: `FR-035` / RDD ID: `FR-MAT-SIMILARITY-GET`)
  - Requirement: Get SimilarityDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-SIMILARITY-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-035` or `CT-FR-MAT-SIMILARITY-GET`) validating request serialization and envelope parsing.

- **REQ-F-036** (Source: `FR-036` / RDD ID: `FR-MAT-SIMILARITY_MATCH-GET`)
  - Requirement: Get SimilarityDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-SIMILARITY_MATCH-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-036` or `CT-FR-MAT-SIMILARITY_MATCH-GET`) validating request serialization and envelope parsing.

- **REQ-F-037** (Source: `FR-037` / RDD ID: `FR-MAT-SUBSTRATES-GET`)
  - Requirement: Get SubstratesDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-SUBSTRATES-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-037` or `CT-FR-MAT-SUBSTRATES-GET`) validating request serialization and envelope parsing.

- **REQ-F-038** (Source: `FR-038` / RDD ID: `FR-MAT-SUMMARY-GET`)
  - Requirement: Get SummaryDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-SUMMARY-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-038` or `CT-FR-MAT-SUMMARY-GET`) validating request serialization and envelope parsing.

- **REQ-F-039** (Source: `FR-039` / RDD ID: `FR-MAT-SURFACE_PROPERTIES-GET`)
  - Requirement: Get SurfacePropDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-SURFACE_PROPERTIES-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-039` or `CT-FR-MAT-SURFACE_PROPERTIES-GET`) validating request serialization and envelope parsing.

- **REQ-F-040** (Source: `FR-040` / RDD ID: `FR-MAT-SYNTHESIS-GET`)
  - Requirement: Get SynthesisSearchResultModel documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-SYNTHESIS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-040` or `CT-FR-MAT-SYNTHESIS-GET`) validating request serialization and envelope parsing.

- **REQ-F-041** (Source: `FR-041` / RDD ID: `FR-MAT-TASKS-GET`)
  - Requirement: Get TaskDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-TASKS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-041` or `CT-FR-MAT-TASKS-GET`) validating request serialization and envelope parsing.

- **REQ-F-042** (Source: `FR-042` / RDD ID: `FR-MAT-TASKS_DEPRECATION-GET`)
  - Requirement: Get DeprecationDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-TASKS_DEPRECATION-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-042` or `CT-FR-MAT-TASKS_DEPRECATION-GET`) validating request serialization and envelope parsing.

- **REQ-F-043** (Source: `FR-043` / RDD ID: `FR-MAT-TASKS_ENTRIES-GET`)
  - Requirement: Get EntryDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-TASKS_ENTRIES-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-043` or `CT-FR-MAT-TASKS_ENTRIES-GET`) validating request serialization and envelope parsing.

- **REQ-F-044** (Source: `FR-044` / RDD ID: `FR-MAT-TASKS_TRAJECTORY-GET`)
  - Requirement: Get TrajectoryDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-TASKS_TRAJECTORY-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-044` or `CT-FR-MAT-TASKS_TRAJECTORY-GET`) validating request serialization and envelope parsing.

- **REQ-F-045** (Source: `FR-045` / RDD ID: `FR-MAT-THERMO-GET`)
  - Requirement: Get ThermoDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-THERMO-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-045` or `CT-FR-MAT-THERMO-GET`) validating request serialization and envelope parsing.

- **REQ-F-046** (Source: `FR-046` / RDD ID: `FR-MAT-XAS-GET`)
  - Requirement: Get XASDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MAT-XAS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-046` or `CT-FR-MAT-XAS-GET`) validating request serialization and envelope parsing.

- **REQ-F-047** (Source: `FR-047` / RDD ID: `FR-MOL-JCESR-GET`)
  - Requirement: Get MoleculesDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-JCESR-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-047` or `CT-FR-MOL-JCESR-GET`) validating request serialization and envelope parsing.

- **REQ-F-048** (Source: `FR-048` / RDD ID: `FR-MOL-SUMMARY-GET`)
  - Requirement: Get MoleculeSummaryDoc documents
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-SUMMARY-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-048` or `CT-FR-MOL-SUMMARY-GET`) validating request serialization and envelope parsing.

- **REQ-F-049** (Source: `FR-049` / RDD ID: `FR-COMMON-DOC_DRIVEN-001`)
  - Requirement: Client MUST implement endpoints listed in official docs (e.g., “Getting Started” endpoint table) but absent from the uploaded OpenAPI, using a doc-driven contract.
  - Acceptance Criteria:
    - AC-1: The client provides one async method per doc-driven endpoint listed under §8 Doc-Driven Routes inventory, with HTTP path exactly matching the RDD-listed path.
    - AC-2: Doc-driven methods return raw JSON mode (`serde_json::Value`) in `data` and accept the escape-hatch query parameter map.
    - AC-3: Response parsing is envelope-first with fallback to top-level JSON array/object when the server does not wrap responses.
    - AC-4: A manifest test asserts the doc-driven endpoint list matches the RDD and that implemented methods exist.

- **REQ-F-050** (Source: `FR-050` / RDD ID: `FR-MOL-ASSOC-GET`)
  - Requirement: Client MUST support searching associated molecule documents via `/molecules/assoc`.
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-ASSOC-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-050` or `CT-FR-MOL-ASSOC-GET`) validating request serialization and envelope parsing.

- **REQ-F-051** (Source: `FR-051` / RDD ID: `FR-MOL-BONDING-GET`)
  - Requirement: Client MUST support searching molecule bonding documents via `/molecules/bonding`.
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-BONDING-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-051` or `CT-FR-MOL-BONDING-GET`) validating request serialization and envelope parsing.

- **REQ-F-052** (Source: `FR-052` / RDD ID: `FR-MOL-CORE-GET`)
  - Requirement: Client MUST support searching molecule core documents via `/molecules/core`.
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-CORE-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-052` or `CT-FR-MOL-CORE-GET`) validating request serialization and envelope parsing.

- **REQ-F-053** (Source: `FR-053` / RDD ID: `FR-MOL-ORBITALS-GET`)
  - Requirement: Client MUST support searching orbital documents via `/molecules/orbitals`.
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-ORBITALS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-053` or `CT-FR-MOL-ORBITALS-GET`) validating request serialization and envelope parsing.

- **REQ-F-054** (Source: `FR-054` / RDD ID: `FR-MOL-PARTIAL_CHARGES-GET`)
  - Requirement: Client MUST support searching partial charge documents via `/molecules/partial_charges`.
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-PARTIAL_CHARGES-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-054` or `CT-FR-MOL-PARTIAL_CHARGES-GET`) validating request serialization and envelope parsing.

- **REQ-F-055** (Source: `FR-055` / RDD ID: `FR-MOL-PARTIAL_SPINS-GET`)
  - Requirement: Client MUST support searching partial spin documents via `/molecules/partial_spins`.
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-PARTIAL_SPINS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-055` or `CT-FR-MOL-PARTIAL_SPINS-GET`) validating request serialization and envelope parsing.

- **REQ-F-056** (Source: `FR-056` / RDD ID: `FR-MOL-REDOX-GET`)
  - Requirement: Client MUST support searching redox documents via `/molecules/redox`.
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-REDOX-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-056` or `CT-FR-MOL-REDOX-GET`) validating request serialization and envelope parsing.

- **REQ-F-057** (Source: `FR-057` / RDD ID: `FR-MOL-TASKS-GET`)
  - Requirement: Client MUST support searching molecule task documents via `/molecules/tasks`.
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-TASKS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-057` or `CT-FR-MOL-TASKS-GET`) validating request serialization and envelope parsing.

- **REQ-F-058** (Source: `FR-058` / RDD ID: `FR-MOL-THERMO-GET`)
  - Requirement: Client MUST support searching molecule thermochemistry documents via `/molecules/thermo`.
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-THERMO-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-058` or `CT-FR-MOL-THERMO-GET`) validating request serialization and envelope parsing.

- **REQ-F-059** (Source: `FR-059` / RDD ID: `FR-MOL-VIBRATIONS-GET`)
  - Requirement: Client MUST support searching vibration documents via `/molecules/vibrations`.
  - Acceptance Criteria:
    - AC-1: The client exposes an async method implementing OpenAPI operation `FR-MOL-VIBRATIONS-GET` (generated route), issuing the correct HTTP method + path defined in `spec/openapi.json`.
    - AC-2: The method accepts a generated parameter struct that covers all required OpenAPI parameters; optional parameters are optional.
    - AC-3: The method returns `Response<TDoc>` where `TDoc` matches the OpenAPI response schema for the operation (or `Response<serde_json::Value>` when raw JSON mode is selected).
    - AC-4: A per-operation contract test exists (e.g., `CT-FR-059` or `CT-FR-MOL-VIBRATIONS-GET`) validating request serialization and envelope parsing.

- **REQ-F-060** (Source: `FR-060` / RDD ID: `FR-COMMON-CONVENIENCE-001`)
  - Requirement: The Rust client MUST provide convenience wrappers matching the official Examples workflows (idiomatic Rust names allowed).
  - Acceptance Criteria:
    - AC-1: The client exposes convenience wrappers that cover each official Examples workflow listed in §8 Convenience Workflows inventory.
    - AC-2: Each wrapper issues exactly one underlying OpenAPI request (unless explicitly documented otherwise) and returns a domain-relevant extracted artifact.

- **REQ-F-061** (Source: `FR-061` / RDD ID: `FR-MPR-GET_STRUCTURE_BY_MATERIAL_ID`)
  - Requirement: Get a structure for a material_id (wraps `GET /materials/summary/` with projected field `structure`).
  - Acceptance Criteria:
    - AC-1: Given an input `material_id`, when calling the convenience method, then the client issues exactly one OpenAPI request with the required projection for the target field.
    - AC-2: The method returns the extracted field value (or a typed error) and does not return the full summary document unless explicitly documented.

- **REQ-F-062** (Source: `FR-062` / RDD ID: `FR-MPR-GET_TASK_IDS_ASSOCIATED_WITH_MATERIAL_ID`)
  - Requirement: Get task IDs associated with a material_id (wraps `GET /materials/summary/` with projected field `task_ids`).
  - Acceptance Criteria:
    - AC-1: Given an input `material_id`, when calling the convenience method, then the client issues exactly one OpenAPI request with the required projection for the target field.
    - AC-2: The method returns the extracted field value (or a typed error) and does not return the full summary document unless explicitly documented.

- **REQ-F-063** (Source: `FR-063` / RDD ID: `FR-MPR-GET_BANDSTRUCTURE_BY_MATERIAL_ID`)
  - Requirement: Get electronic bandstructure for a material_id (wraps `GET /materials/summary/` with projected field `bandstructure`).
  - Acceptance Criteria:
    - AC-1: Given an input `material_id`, when calling the convenience method, then the client issues exactly one OpenAPI request with the required projection for the target field.
    - AC-2: The method returns the extracted field value (or a typed error) and does not return the full summary document unless explicitly documented.

- **REQ-F-064** (Source: `FR-064` / RDD ID: `FR-MPR-GET_DOS_BY_MATERIAL_ID`)
  - Requirement: Get electronic DOS for a material_id (wraps `GET /materials/summary/` with projected field `dos`).
  - Acceptance Criteria:
    - AC-1: Given an input `material_id`, when calling the convenience method, then the client issues exactly one OpenAPI request with the required projection for the target field.
    - AC-2: The method returns the extracted field value (or a typed error) and does not return the full summary document unless explicitly documented.

- **REQ-F-065** (Source: `FR-065` / RDD ID: `FR-MPR-GET_PHONON_BANDSTRUCTURE_BY_MATERIAL_ID`)
  - Requirement: Get phonon bandstructure for a material_id (wraps phonon endpoint).
  - Acceptance Criteria:
    - AC-1: Given an input `material_id`, when calling the convenience method, then the client issues exactly one OpenAPI request with the required projection for the target field.
    - AC-2: The method returns the extracted field value (or a typed error) and does not return the full summary document unless explicitly documented.

- **REQ-F-066** (Source: `FR-066` / RDD ID: `FR-MPR-GET_PHONON_DOS_BY_MATERIAL_ID`)
  - Requirement: Get phonon DOS for a material_id (wraps phonon endpoint).
  - Acceptance Criteria:
    - AC-1: Given an input `material_id`, when calling the convenience method, then the client issues exactly one OpenAPI request with the required projection for the target field.
    - AC-2: The method returns the extracted field value (or a typed error) and does not return the full summary document unless explicitly documented.

- **REQ-F-067** (Source: `FR-067` / RDD ID: `FR-MPR-GET_CHARGE_DENSITY_FROM_MATERIAL_ID`)
  - Requirement: Get charge density by material_id (Examples-driven).
  - Acceptance Criteria:
    - AC-1: Given an input `material_id`/`task_id`, when calling the charge density convenience method, then if a charge density route is not present in OpenAPI, the client returns `UnsupportedBySpecification("charge_density")` without HTTP.
    - AC-2: If a charge density route exists in OpenAPI, the client calls it and returns parsed `Response<serde_json::Value>` unless a typed schema is available.

- **REQ-F-068** (Source: `FR-068` / RDD ID: `FR-MPR-GET_CHARGE_DENSITY_FROM_TASK_ID`)
  - Requirement: Get charge density by task_id (Examples-driven).
  - Acceptance Criteria:
    - AC-1: Given an input `material_id`/`task_id`, when calling the charge density convenience method, then if a charge density route is not present in OpenAPI, the client returns `UnsupportedBySpecification("charge_density")` without HTTP.
    - AC-2: If a charge density route exists in OpenAPI, the client calls it and returns parsed `Response<serde_json::Value>` unless a typed schema is available.

### 6.6.2 Operations & Configuration Requirements

- **REQ-OPS-001** (Source: `FR-069` / RDD ID: `OPS-CONFIG-SOURCES-001`)
  - Requirement: Client MUST support configuration from (1) explicit constructor/builder inputs and (2) environment variables, and MUST define precedence.
  - Acceptance Criteria:
    - AC-1: Given `api_key` is provided via builder, when `MP_API_KEY` and `PMG_MAPI_KEY` are also set, then the resolved API key equals the builder-provided value.
    - AC-2: Given builder `api_key` is not provided and `MP_API_KEY` is set, then the resolved API key equals `MP_API_KEY`.
    - AC-3: Given builder `api_key` is not provided and `MP_API_KEY` is not set but `PMG_MAPI_KEY` is set, then the resolved API key equals `PMG_MAPI_KEY`.
    - AC-4: Given none of the above are set, then `MpClientBuilder::build()` returns `MpApiError::MissingApiKey` and no HTTP request is attempted.

- **REQ-OPS-002** (Source: `FR-070` / RDD ID: `OPS-SETTINGS-001`)
  - Requirement: Client MUST support setting: `api_key`, `base_url`, `timeout`, `concurrency`, `qps_limit`, `user_agent`, and `allow_insecure_http`.
  - Acceptance Criteria:
    - AC-1: Each setting is configurable via builder, and affects request execution as follows:
      - `api_key`: populates `X-API-KEY` header on every request.
      - `base_url`: prefixes all request paths; invalid URL yields configuration error at build time.
      - `timeout`: bounds the end-to-end request (connect + response body read).
      - `concurrency`: caps in-flight requests per client instance.
      - `qps_limit`: enforces per-client aggregate request pacing.
      - `user_agent`: populates `User-Agent` header on every request.
      - `allow_insecure_http`: when `false`, rejects `http://` base URLs; when `true`, permits them for local testing harnesses.
    - AC-2: A unit test suite verifies each setting’s effect (headers/URL/limits/TLS gate) without requiring live network calls (mock transport).

- **REQ-F-071** (Source: `FR-071` / RDD ID: `USER-PYTHON-PARITY-COVERAGE-001`)
  - Requirement: The Rust client MUST implement the full API surface of the Python `mp-api` client (behavioral parity; API shape may differ).
  - Acceptance Criteria:
    - AC-1: PT-MANIFEST-001 enumerates the Python `mp-api` API surface and fails if any Python API lacks a mapped Rust equivalent.
    - AC-2: A mapping table (Python API identifier → Rust method identifier) is version-controlled under `tests/python/parity_manifest.json`.
    - AC-3: PT-PY-ALL-001 executes one or more parity vectors for every Python `mp-api` API in the manifest, invoking Python and Rust with identical input values, and compares canonical JSON outputs; it fails on any missing mapping/implementation or any mismatch.
    - AC-4: Pass criteria: all Python `mp-api` APIs are implemented in Rust and input/output values are equivalent for every parity vector.

### 6.6.3 Non-Functional Requirements

- **REQ-NF-001** (Source: `NFR-001` / RDD ID: `NFR-PERF-RATE_LIMIT-001`)
  - Requirement: Client MUST provide a configurable rate limiter and MUST default to **25 requests per second** (aggregate) unless explicitly overridden.
  - Acceptance Criteria:
    - AC-1: Default `qps_limit` is 25 requests/second per client instance when not explicitly configured.
    - AC-2: Rate limiting is enforced for all requests (OpenAPI + doc-driven) issued by a client instance.

- **REQ-NF-002** (Source: `NFR-002` / RDD ID: `NFR-REL-RETRY-001`)
  - Requirement: Client MUST implement retries with exponential backoff for transient failures, using a configurable policy with explicit defaults (max_retries=3, initial_backoff=200ms, max_backoff=2s, jitter=full-jitter).
  - Acceptance Criteria:
    - AC-1: Retry policy uses exponential backoff with jitter and has explicit defaults: max_retries=3, initial_backoff=200ms, max_backoff=2s.
    - AC-2: Retries occur only for retryable statuses (429, 5xx) and retryable transport failures; default request-method eligibility is GET/HEAD/OPTIONS only unless configured otherwise.
    - AC-3: Retry count is included in structured logs.

- **REQ-NF-003** (Source: `NFR-003` / RDD ID: `NFR-OBS-LOGGING-001`)
  - Requirement: Client MUST support structured logs including request path, status code, latency, retry count, and correlation ID, and MUST redact API keys and other secrets.
  - Acceptance Criteria:
    - AC-1: Logs are structured and include: request path, status code, latency, retry count, correlation ID.
    - AC-2: API keys and other secrets are redacted in all logs and error renderings.

- **REQ-NF-004** (Source: `NFR-004` / RDD ID: `NFR-SEC-TLS-001`)
  - Requirement: Client MUST use HTTPS by default and MUST reject non-HTTPS base URLs unless the caller explicitly opts in (for testing).
  - Acceptance Criteria:
    - AC-1: Base URL must be HTTPS by default; `http://` base URLs are rejected unless `allow_insecure_http=true`.
    - AC-2: When `allow_insecure_http=true`, `http://` base URLs are accepted only for local testing harness usage.

- **REQ-NF-005** (Source: `NFR-005` / RDD ID: `NFR-TEST-UNIT-001`)
  - Requirement: Unit tests SHALL cover query serialization and error mapping.
  - Acceptance Criteria:
    - AC-1: Unit tests cover query serialization (pagination/projection/escape hatch) and error mapping (422, non-2xx, transport errors).

- **REQ-NF-006** (Source: `NFR-006` / RDD ID: `NFR-TEST-CONTRACT-001`)
  - Requirement: Contract tests SHALL validate request/response shapes against OpenAPI for every inventory operation in Appendix A-OpenAPI.
  - Acceptance Criteria:
    - AC-1: For every OpenAPI operation in Appendix A inventory, there exists at least one contract test validating request/response shapes against OpenAPI.
    - AC-2: A CI step runs contract tests and fails on uncovered operations.

- **REQ-NF-007** (Source: `NFR-007` / RDD ID: `NFR-TEST-INTEGRATION-001`)
  - Requirement: Integration smoke tests SHALL be skipped unless MP_API_KEY or PMG_MAPI_KEY is set; when set, they SHALL run as smoke tests.
  - Acceptance Criteria:
    - AC-1: Integration smoke tests are skipped unless `MP_API_KEY` or `PMG_MAPI_KEY` is set; when set, they execute at least one authenticated query-based request and validate envelope parsing.
    - AC-2: Integration tests do not print or store API keys; failures redact secrets.

- **REQ-NF-008** (Source: `NFR-008` / RDD ID: `NFR-BUILD-DEPS-001`)
  - Requirement: The crate SHALL pin a reproducible crate baseline (versions + required features) for core runtime dependencies.
  - Acceptance Criteria:
    - AC-1: `Cargo.toml` specifies at minimum the dependency entries listed in §11 Assumptions (crate baseline) with the same versions and required features.
    - AC-2: CI-DEPS-001 fails if the baseline dependency set deviates unless the document and revision history are updated in the same change.

- **REQ-NF-009** (Source: `NFR-009` / RDD ID: `NFR-TEST-PYTHON-PARITY-001`)
  - Requirement: When enabled, Python parity tests SHALL assert (a) full Python `mp-api` API surface coverage and (b) input/output equivalence between Rust and Python for every mapped API.
  - Acceptance Criteria:
    - AC-1: Parity tests are skipped unless `MP_API_PY_PARITY=1`, an API key env var is set, and Python `mp-api` is importable.
    - AC-2: PT-MANIFEST-001 fails if any Python `mp-api` API lacks a mapped Rust equivalent.
    - AC-3: PT-PY-ALL-001 executes one or more parity vectors for every Python `mp-api` API in the manifest, invoking Python and Rust with identical input values, and compares canonical JSON outputs; it fails on any missing mapping/implementation or any mismatch.
    - AC-4: Pass criteria: all Python `mp-api` APIs are implemented in Rust and input/output values are equivalent for every parity vector.

### 6.6.4 Data Requirements

- **REQ-DATA-001** (Source: `DR-001` / RDD ID: `DR-ENVELOPE-001`)
  - Requirement: Response envelope fields and schemas (data/errors/meta; Error{code,message}; Meta fields).
  - Acceptance Criteria:
    - AC-1: Response envelope struct includes `data`, `errors`, `meta` fields and can deserialize the canonical wrapper from OpenAPI.
    - AC-2: `Error` includes required `code` and `message` fields; `Meta` includes the documented fields and allows additional unknown fields.

- **REQ-DATA-002** (Source: `DR-002` / RDD ID: `DR-TYPING-001`)
  - Requirement: Typed and raw JSON modes for response data.
  - Acceptance Criteria:
    - AC-1: Typed mode returns `Response<T>` where `T` is a generated OpenAPI model; raw JSON mode returns `Response<serde_json::Value>`.
    - AC-2: Doc-driven endpoints return raw JSON mode for `data`.

- **REQ-DATA-003** (Source: `DR-003` / RDD ID: `DR-SERDE-001`)
  - Requirement: Serde policy for nullable and unknown fields.
  - Acceptance Criteria:
    - AC-1: Generated and handwritten models use `Option<T>` for nullable fields.
    - AC-2: Unknown fields are tolerated (no deserialization failure) for forward compatibility.
# 7. Architecture Overview

## 7.1 Component Diagram (textual)

```
+-------------------+          +---------------------+
|  MpClient Facade   |          | Convenience Root    |
|  (builder + roots) |----------| (example workflows) |
+---------+---------+          +----------+----------+
          |                               |
          v                               v
+-------------------+          +---------------------+
| Route Roots        |          | Doc-Driven Routes   |
| - OpenAPI Routes   |          | (raw JSON endpoints)|
| - Doc-Driven Routes|          +----------+----------+
+---------+---------+                     |
          |                               |
          v                               v
+-------------------+          +---------------------+
| Query Builder      |          | Envelope & Models   |
| (pagination, proj, |          | (Response<T>, Meta) |
|  escape hatch)     |          +----------+----------+
+---------+---------+                     |
          |                               |
          v                               v
+----------------------------------------------------+
| HTTP Transport (reqwest)                           |
|  + Concurrency Semaphore                           |
|  + Rate Limiter (token bucket)                     |
|  + Retry Policy (exp backoff)                      |
|  + Structured Logging / Correlation ID / Redaction |
+-------------------------+--------------------------+
                          |
                          v
                +-------------------+
                | MP API Server     |
                +-------------------+
```

## 7.2 Data Flow Overview

1. Caller constructs `MpClient` via builder/environment config.
2. Caller invokes an OpenAPI route method (typed) or a doc-driven route method (raw JSON).
3. Route method:
   - builds query pairs (pagination/projection/escape hatch)
   - constructs `RequestSpec` and delegates to Transport
4. Transport applies middleware (rate limit, concurrency, retry) and issues HTTP request.
5. Response is mapped to `MpApiError` on non-2xx or parsed into `Response<T>` / raw JSON on success.
6. Convenience workflows compose a single underlying route call with a required projection and return a sub-field.

## 7.3 High-Level Pipeline

End-to-end processing pipeline (I/O and requirement linkage):

1. **Config resolution**  
   Input: builder settings + env vars → Output: validated `Config`  
   Requirements: FR-069, FR-070, FR-001, NFR-004

2. **Route selection**  
   Input: route group + method call → Output: `RequestSpec`  
   Requirements: FR-007, FR-049

3. **Query construction**  
   Input: pagination/projection/extra → Output: query string pairs  
   Requirements: FR-002, FR-003

4. **Request execution**  
   Input: `RequestSpec` → Output: `RawResponse`  
   Requirements: NFR-001, FR-070, NFR-002, NFR-003

5. **Error mapping**  
   Input: status + body → Output: typed error or continue  
   Requirements: FR-006, FR-005

6. **Response parsing**  
   Input: body → Output: `Response<T>` or raw JSON  
   Requirements: FR-004, DR-001..003

7. **Convenience extraction (optional)**  
Input: `Response<T>` → Output: extracted artifact (structure, task_ids, bandstructure, dos, phonon_bandstructure, phonon_dos, charge_density).
   Requirements: FR-060

## 7.4 Source Tree

Proposed repository layout (modules mapped to directories):

```
mp-api-rs/
  Cargo.toml
  build.rs
  spec/
    openapi.json
  src/
    lib.rs
    client.rs
    config.rs
    error.rs
    data/
      mod.rs
      envelope.rs
      models/               # generated from OpenAPI
        mod.rs
        ...
    query/
      mod.rs
      pagination.rs
      projection.rs
      extra.rs
    transport/
      mod.rs
      reqwest_transport.rs
    middleware/
      mod.rs
      rate_limit.rs
      retry.rs
    routes/
      mod.rs
      openapi/
        mod.rs
        inventory.rs         # generated: operation list + mapping
        materials.rs         # generated or semi-generated
        molecules.rs
        defects.rs
        doi.rs
      doc_driven/
        mod.rs
        molecules.rs
    convenience/
      mod.rs
  tests/
    contract_manifest.rs
    contract_openapi_materials.rs
    contract_openapi_molecules.rs
    contract_openapi_defects.rs
    contract_openapi_doi.rs
    doc_driven_molecules.rs
    integration_smoke.rs
    fixtures/
      ...
```

## 7.5 Key Design Decisions & Trade-offs (with requirement IDs)

1. **OpenAPI-driven code generation** (vs. hand-written route/modeled types)  
   - Rationale: prevents drift and supports 1:1 coverage + contract tests.  
   - Requirements: FR-007, NFR-006.

2. **Doc-driven endpoints return raw JSON** (vs. guessed typed models)  
   - Rationale: RDD prohibits inventing schema; raw JSON ensures forward compatibility.  
   - Requirements: FR-049, DR-002.

3. **Central middleware stack at Transport layer**  
   - Rationale: uniform rate limiting, retry, logging across all endpoints.  
   - Requirements: NFR-001, NFR-002, NFR-003.

4. **Fail-fast client-side validation for pagination conflicts**  
   - Rationale: deterministic errors, avoids unnecessary network calls.  
   - Requirements: FR-002.

# 8. Module Detailed Design

## Client Facade (MpClient)

* Purpose  
  Provide the primary async Rust entrypoint for the Materials Project API, exposing nested route clients and convenience workflows while centralizing shared behavior (auth, config, transport, and middleware wiring).

* Files (responsibility & description)
  - `src/lib.rs`: Crate entrypoint; re-exports the public API surface and module roots.
  - `src/client.rs`: Defines MpClient and MpClientBuilder; wires Config, Transport, middleware, and route roots.

* Type Inventory (definitions & descriptions)
  - `MpClient`: Primary async client handle; internally holds Arc<InnerClient>.
  - `MpClientBuilder`: Resolves configuration from builder + environment and constructs MpClient with validation.
  - `InnerClient`: Internal shared state (Config, Transport, middleware); not part of the public API.
  - `routes::openapi::OpenApiRoot`: Entry point for OpenAPI route groups.
  - `routes::doc_driven::DocDrivenRoot`: Entry point for doc-driven endpoints.
  - `convenience::ConvenienceRoot`: Entry point for example-driven helper workflows.


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - The client SHALL be async-first and require a Tokio runtime (Scope: RDD §2).
  - The client SHALL expose nested route clients for OpenAPI endpoints and doc-driven endpoints (guidance: RDD §8; functional coverage: FR-007 / FR-049).
  - The client SHALL enforce configuration validation at build time (missing API key, invalid base URL, invalid pagination presets) and SHALL fail fast without issuing HTTP when invalid (FR-001, FR-002, FR-070).
  - The client SHALL share a single underlying HTTP transport + middleware stack across all route groups to ensure consistent rate limiting, concurrency, retries, and logging (FR-070, NFR-001, NFR-002, NFR-003).

* In-Scope / Out-of-Scope
  - In-scope: all OpenAPI routes (Appendix A) and doc-driven routes (RDD §6.2), plus example-driven convenience workflows (RDD §6.3).
  - Out-of-scope: sync/blocking client; exact Python object parity (RDD §2).

* Inputs/Outputs (schemas, examples)
  - Inputs: `MpClientBuilder` settings (`api_key`, `base_url`, `timeout`, `concurrency`, `qps_limit`, `user_agent`, `allow_insecure_http`).
  - Outputs: strongly typed responses (`Response<T>`) where OpenAPI schemas exist, or raw JSON responses (`serde_json::Value`) for doc-driven endpoints (RDD §7.2).

* Public Interfaces
  - Type aliases and core API surface (illustrative; implementers may adjust naming without changing semantics):
    - `pub type Client = MpClient;`
    - `pub struct MpClientBuilder { /* fields */ }`
    - `impl MpClient { pub fn builder() -> MpClientBuilder; }`
    - `impl MpClientBuilder {
         pub fn api_key(self, key: impl Into<String>) -> Self;
         pub fn base_url(self, url: impl Into<String>) -> Self;
         pub fn timeout(self, d: std::time::Duration) -> Self;
         pub fn concurrency(self, n: usize) -> Self;
         pub fn qps_limit(self, qps: u32) -> Self;
         pub fn user_agent(self, ua: impl Into<String>) -> Self;
         pub fn allow_insecure_http(self, allow: bool) -> Self;
         pub fn build(self) -> Result<MpClient, MpApiError>;
       }`
    - Route accessors:
      - `pub fn openapi(&self) -> routes::openapi::OpenApiRoot;`
      - `pub fn doc(&self) -> routes::doc_driven::DocDrivenRoot;`
      - `pub fn convenience(&self) -> convenience::ConvenienceRoot;`

* Internal Design
  - `MpClient` holds `Arc<InnerClient>` where `InnerClient` contains:
    - immutable `Config`
    - `Transport` (reqwest client wrapper)
    - `Middleware` instances (rate limiter, concurrency semaphore, retry policy)
    - optional `SpecRegistry` (OpenAPI-derived index used for coverage checks and feature detection)
  - Route roots are thin handles cloning `Arc<InnerClient>` and providing ergonomic group namespaces.

* Data Model
  - N/A (library; no persistent storage). Data types are Rust structs generated from OpenAPI and/or raw JSON.

* Business Rules & Validation (mapped to requirement IDs)
  - Build-time config precedence and validation: FR-069, FR-070, NFR-004.
  - Missing API key is a client-side error, no HTTP: FR-001.

* Error Handling
  - Builder returns `MpApiError::MissingApiKey` when no API key is resolved (FR-001).
  - Builder returns `MpApiError::ConfigurationError` (or equivalent) for invalid `base_url` per TLS rule (NFR-004).

* Logging & Metrics
  - Client does not mandate a logging backend; it emits `tracing` spans/events with required fields and redaction policy (NFR-003).

* Security
  - API keys MUST never be logged; redaction is enforced in transport logging (see HTTP Transport module).

* Performance/Scalability Notes
  - Stateless request execution; horizontal scaling is achieved by running multiple client instances.
  - Concurrency and rate limiting are enforced per client instance (FR-070, NFR-001).

* Dependencies
  - `config`, `transport`, `middleware`, `routes`, `convenience`, `error`, `data`.

* Test Design
  - UT-FR-001: missing API key yields `MissingApiKey` and no HTTP.
  - UT-FR-069/UT-FR-070: config precedence and supported settings are deterministic.

## Configuration (Config & Builder)

* Purpose  
  Centralize configuration source resolution, defaults, and validation.

* Files (responsibility & description)
  - `src/config.rs`: Configuration resolution, defaults, validation, and environment-variable integration.

* Type Inventory (definitions & descriptions)
  - `Config`: Immutable resolved client configuration (api_key, base_url, timeout, concurrency, qps_limit, user_agent, allow_insecure_http, retry).
  - `BuilderSettings`: Intermediate builder settings before validation (internal).


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - The config layer SHALL resolve settings from builder inputs and environment variables with explicit precedence (FR-069).
  - The config layer SHALL support all settings enumerated in OPS-SETTINGS-001, including default `base_url=https://api.materialsproject.org` and `allow_insecure_http` gating (FR-070, NFR-004).
  - The config layer SHALL provide deterministic defaults for:
    - `qps_limit = 25` (NFR-001)
    - `timeout = 30s` (selected default; see §11 Assumptions)
    - `concurrency = 16` (selected default; see §11 Assumptions)
    - `user_agent = "mp-api-rs/<crate_version>"` where `<crate_version>` is the Cargo package version (selected default; see §11 Assumptions)

* In-Scope / Out-of-Scope
  - In-scope: API key resolution, base URL validation, settings normalization.
  - Out-of-scope: storing secrets beyond process memory.

* Inputs/Outputs
  - Inputs:
    - explicit builder settings
    - env vars: `MP_API_KEY`, `PMG_MAPI_KEY` (FR-069)
  - Output: immutable `Config` struct.

* Public Interfaces
  - `pub struct Config { api_key: String, base_url: url::Url, timeout: Option<Duration>, concurrency: usize, qps_limit: u32, user_agent: String, allow_insecure_http: bool, retry: RetryConfig }`
  - `impl Config { pub fn from_builder_and_env(builder: BuilderSettings) -> Result<Self, MpApiError>; }`

* Internal Design
  - Parsing/validation rules:
    - base_url:
      - default to `https://api.materialsproject.org` (FR-070, NFR-004)
      - reject `http://` unless `allow_insecure_http=true` (NFR-004)
    - api_key:
      - resolved via precedence: builder > MP_API_KEY > PMG_MAPI_KEY (FR-069)
      - missing produces `MissingApiKey` (FR-001)

* Data Model
  - N/A

* Business Rules & Validation
  - All validation errors are raised during `build()` and SHALL prevent any HTTP.

* Error Handling
  - `MissingApiKey`
  - `InvalidBaseUrl` / `InsecureBaseUrlNotAllowed`

* Logging & Metrics
  - Log only non-sensitive configuration (never api_key).

* Security
  - Treat api_key as secret; never expose via `Debug` / display.

* Performance/Scalability Notes
  - Config is immutable; cheap clones via `Arc<Config>` if required.

* Dependencies
  - `std::env`, `url` crate.

* Test Design
  - UT-FR-069: precedence builder > MP_API_KEY > PMG_MAPI_KEY.
  - UT-FR-070: each supported setting affects behavior deterministically (headers/URL/limits/TLS gate).
  - UT-NFR-004: TLS enforcement.
  - UT-NFR-001: qps default 25.

## Errors (MpApiError)

* Purpose  
  Provide a single stable error type and deterministic error mapping across configuration, request building, transport, HTTP status handling, and deserialization.

* Files (responsibility & description)
  - `src/error.rs`: Defines MpApiError and all typed error payloads (validation, HTTP status, config).

* Type Inventory (definitions & descriptions)
  - `MpApiError`: Top-level error enum distinguishing configuration, request, transport, HTTP, validation, and deserialization failures.
  - `HTTPValidationError`: Typed representation of OpenAPI 422 validation error payload.
  - `ValidationErrorItem`: One validation error entry (loc/msg/type).


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - The client SHALL expose a typed error model with semantically distinct variants including at minimum the cases enumerated by the RDD (FR-006).
  - The client SHALL detect client-side errors (missing API key, invalid pagination combinations) before sending HTTP (FR-001, FR-002).
  - The client SHALL map HTTP 422 to `ValidationError(HTTPValidationError)` and SHALL parse the OpenAPI `HTTPValidationError` schema (FR-005, FR-006).
  - The client SHALL classify HTTP 429 and HTTP 5xx as retryable errors (`RetryableHttpError`) and all other non-2xx as `HttpError` (FR-006, NFR-002).
  - The client SHALL surface an explicit `UnsupportedBySpecification(feature)` when a documented workflow cannot be implemented due to missing OpenAPI contract (FR-006; used by charge density convenience methods).

* Inputs/Outputs
  - Inputs: config validation failures; request validation failures; HTTP status + body; transport exceptions; JSON parse exceptions.
  - Output: `MpApiError`.

* Public Interfaces
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

* Internal Design
  - Central mapping function: `fn map_http(status: StatusCode, body: bytes::Bytes) -> Result<T, MpApiError>`
  - Redaction helper for log-safe error rendering.

* Data Model
  - N/A

* Business Rules & Validation
  - Pagination conflicts are caught in Query module and returned as `InvalidPaginationParameters` (no HTTP) (FR-002).

* Error Handling
  - See public interface.

* Logging & Metrics
  - Error logs MUST include correlation ID and context, but MUST redact secrets (NFR-003).

* Security
  - Ensure that serialized `MpApiError` never includes API key; error messages should omit headers by default.

* Performance/Scalability Notes
  - Avoid cloning large bodies; store truncated body in errors (truncate to 8192 bytes / 8 KiB; see §11 Assumptions).

* Dependencies
  - `serde`, `serde_json`, `reqwest` (or `http` types), `bytes`.

* Test Design
  - UT-FR-006: invalid pagination combinations return `InvalidPaginationParameters` without HTTP.
  - UT-FR-006: 422 fixture parses into `ValidationError`.
  - UT-FR-006: 429 and 500 fixtures map to `RetryableHttpError`.

## Envelope & Models

* Purpose  
  Define core data types used by all endpoints: the response envelope, error/meta structures, and model generation policy.

* Files (responsibility & description)
  - `src/data/mod.rs`: Data-layer module root; re-exports envelope and generated models.
  - `src/data/envelope.rs`: Defines Response<T>, ApiErrorItem, Meta, and envelope parsing helpers.
  - `src/data/models/mod.rs`: OpenAPI-generated model module root (generated files per schema).

* Type Inventory (definitions & descriptions)
  - `Response<T>`: Canonical response envelope wrapper with data/errors/meta.
  - `ApiErrorItem`: One API-reported error object (code/message).
  - `Meta`: Response metadata (api_version/time_stamp/total_doc/facet + forward-compatible extras).
  - `DefectTaskDoc`: Defects task document (response item for /defects/tasks).
  - `DOIDoc`: DOI document (response item for /doi).
  - `AbsorptionDoc`: Materials absorption document.
  - `AlloyPairDoc`: Alloy pairing document.
  - `BondingDoc`: Bonding document.
  - `ChemEnvDoc`: Chemical environment document.
  - `ConversionElectrodeDoc`: Conversion electrode document.
  - `MaterialsDoc`: Core materials document (materials/core and related endpoints).
  - `FormulaAutocomplete`: Formula autocomplete response document.
  - `DielectricDoc`: Dielectric document.
  - `ElasticityDoc`: Elasticity document.
  - `ElectronicStructureDoc`: Electronic structure document (bandstructure/DOS).
  - `EOSDoc`: Equation-of-state document.
  - `FermiDoc`: Fermi surface document.
  - `GrainBoundaryDoc`: Grain boundary document.
  - `InsertionElectrodeDoc`: Insertion electrode document.
  - `MagnetismDoc`: Magnetism document.
  - `OxidationStateDoc`: Oxidation state document.
  - `PhononBSDOSDoc`: Phonon bandstructure/DOS document.
  - `PiezoelectricDoc`: Piezoelectric document.
  - `ProvenanceDoc`: Provenance document.
  - `RobocrystallogapherDoc`: Robocrystallographer document.
  - `SimilarityDoc`: Similarity document.
  - `SubstratesDoc`: Substrates document.
  - `SummaryDoc`: Materials summary document.
  - `SurfacePropDoc`: Surface properties document.
  - `SynthesisSearchResultModel`: Synthesis search result model.
  - `TaskDoc`: Task document.
  - `DeprecationDoc`: Deprecation document.
  - `EntryDoc`: Entry document.
  - `TrajectoryDoc`: Trajectory document.
  - `ThermoDoc`: Thermochemistry document.
  - `XASDoc`: X-ray absorption spectroscopy document.
  - `MoleculesDoc`: Molecules document (e.g., /molecules/jcesr).
  - `MoleculeSummaryDoc`: Molecule summary document.


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - The client SHALL parse the standard response envelope `{data, errors, meta}` into `Response<T>` (FR-004, DR-001).
  - The data layer SHALL provide both typed and raw JSON modes (DR-002).
  - Model structs SHALL support forward compatibility by allowing unknown fields and using `Option<T>` for nullable fields (DR-003).

* Inputs/Outputs
  - Inputs: HTTP response JSON bytes.
  - Outputs:
    - `Response<T>` for typed operations
    - `Response<serde_json::Value>` or `serde_json::Value` for raw mode (doc-driven endpoints)

* Public Interfaces
  - `pub struct Response<T> { pub data: Vec<T>, pub errors: Vec<ApiErrorItem>, pub meta: Meta }`
    - Note: `data` is modeled as `Vec<T>` consistent with the RDD envelope example (RDD §7.1).
  - `pub struct ApiErrorItem { pub code: i32, pub message: String }`
  - `pub struct Meta { pub api_version: Option<String>, pub time_stamp: Option<String>, pub total_doc: Option<i64>, pub facet: Option<serde_json::Value> }`

* Internal Design
  - Deserialization strategy:
    1. Attempt to deserialize into `Response<T>` (typed mode).
    2. In doc-driven raw mode, attempt:
       - `Response<serde_json::Value>` first (preferred)
       - fallback to top-level JSON (array/object) if no envelope present (per FR-049).

* Data Model
  - N/A (in-memory only).

* Business Rules & Validation
  - Schema mismatch yields `DeserializeError` (FR-004).

* Error Handling
  - Use `MpApiError::DeserializeError` for JSON parse failures.
  - Use `MpApiError::ValidationError` for HTTP 422 (handled in Errors module, but deserializes using `HTTPValidationError` model).

* Logging & Metrics
  - Deserialization failures are logged with correlation ID; payloads are truncated and sanitized.

* Security
  - Ensure that any logged payloads do not include secrets (headers are redacted at transport level).

* Performance/Scalability Notes
  - Prefer reading response to bytes then deserializing; v1.1 does not include streaming APIs. For large payloads, callers can page results and/or use raw JSON mode; streaming support may be added in a future major version (see §12).

* Dependencies
  - `serde`, `serde_json`.

* Test Design
  - UT-FR-004: envelope parsing happy path and error path.
  - UT-DR-001/002/003: envelope/meta/error type policy tests.

## Query Parameters (Pagination/Projection/Escape Hatch)

* Purpose  
  Provide a consistent, validated, and testable mechanism for building query parameters (pagination, field projection, and escape-hatch parameters).

* Files (responsibility & description)
  - `src/query/mod.rs`: Query module root; exports pagination, projection, and extra query support.
  - `src/query/pagination.rs`: Pagination types, validation, normalization, and serialization.
  - `src/query/projection.rs`: Field projection types and serialization (_fields/_all_fields).
  - `src/query/extra.rs`: Escape-hatch query parameter map with deterministic ordering.

* Type Inventory (definitions & descriptions)
  - `Pagination`: User-provided pagination settings (page/per_page/skip/limit).
  - `NormalizedPagination`: Validated, conflict-free pagination representation used for serialization (internal).
  - `Projection`: Field projection settings for _fields and _all_fields.
  - `ExtraQueryParams`: Deterministically ordered map of extra query parameters passed through verbatim.
  - `ToQueryPairs`: Trait to serialize params into ordered query pairs for request building.


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - The query layer SHALL support both page-based and offset-based pagination using `_page`, `_per_page`, `_skip`, `_limit` (FR-002).
  - The query layer SHALL serialize either page-based or offset-based pagination, and SHALL reject any request that mixes page-based and offset-based pagination inputs (FR-002).
  - Conflicting pagination inputs SHALL be rejected client-side with `InvalidPaginationParameters` and SHALL NOT issue HTTP (FR-002).
  - The query layer SHALL clamp `_limit` and `_per_page` to 1000 where documented in OpenAPI (FR-002).
  - The query layer SHALL support field projection via `_fields` (comma-separated) and `_all_fields` (boolean) (FR-003).
  - The query layer SHALL support an escape hatch for additional non-reserved query parameters that are passed through verbatim; reserved typed-query keys (`_page`, `_per_page`, `_skip`, `_limit`, `_fields`, `_all_fields`) SHALL be rejected from the escape hatch (FR-049, FR-005).

* Inputs/Outputs
  - Inputs: typed parameter structs and/or user-provided key/value pairs.
  - Output: a stable ordered query representation (`Vec<(String, String)>`) for request building.

* Public Interfaces
  - `pub struct Pagination { pub page: Option<u32>, pub per_page: Option<u32>, pub skip: Option<u32>, pub limit: Option<u32> }`
  - `pub struct Projection { pub fields: Option<Vec<String>>, pub all_fields: bool }`
  - `pub struct ExtraQueryParams(pub std::collections::BTreeMap<String, String>);`
  - `pub trait ToQueryPairs { fn to_query_pairs(&self) -> Result<Vec<(String,String)>, MpApiError>; }`

* Internal Design
  - Normalization algorithm:
    - detect conflict: any combination where any page-based field (`page` or `per_page`) is set and any offset-based field (`skip` or `limit`) is set => error
    - effective pagination:
      - if `page`/`per_page` set (and `skip`/`limit` are unset) => serialize only `_page`/`_per_page`
      - else => serialize `_skip`/`_limit` when present
    - clamp: `_limit` and `_per_page` = min(value, 1000)

* Data Model
  - N/A

* Business Rules & Validation
  - Projection serialization:
    - `_fields` joins with commas in stable order
    - `_all_fields=true` when requested; if true and `_fields` set, both are allowed; client passes through.

* Error Handling
  - `InvalidPaginationParameters` for conflicts.
  - No client-side validation for escape-hatch parameter names/values; server 422 maps to `ValidationError` (FR-005).

* Logging & Metrics
  - Query parameters are loggable except sensitive values; escape hatch keys are logged but values are truncated.

* Security
  - Escape hatch affects only query string; it cannot override headers.

* Performance/Scalability Notes
  - Use `BTreeMap` to keep deterministic ordering for testability.

* Dependencies
  - `serde_urlencoded` or equivalent query serializer.

* Test Design
  - UT-FR-002: pagination serialization, clamping, conflict errors.
  - UT-FR-003: projection serialization and payload-size fixture comparison (as specified in the RDD acceptance).

## HTTP Transport

* Purpose  
  Execute HTTP requests with consistent behavior: base URL resolution, headers, timeouts, concurrency caps, QPS limiting, retries, response capture, and observability.

* Files (responsibility & description)
  - `src/transport/mod.rs`: Transport module root; defines RequestSpec/RawResponse and Transport trait/handle.
  - `src/transport/reqwest_transport.rs`: Reqwest-backed transport implementation and request execution pipeline.

* Type Inventory (definitions & descriptions)
  - `RequestSpec`: Internal request description (method/path/query/body) passed to the transport.
  - `RawResponse`: Raw HTTP response payload (status/body) captured by transport.
  - `Transport`: Transport handle exposing async execute() with middleware integration.


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - The transport SHALL prefix all paths with `base_url` and SHALL enforce HTTPS unless `allow_insecure_http=true` (FR-070, NFR-004).
  - The transport SHALL set `X-API-KEY` header on every request (FR-001).
  - The transport SHALL set `User-Agent` header on every request when configured (FR-070).
  - The transport SHALL apply per-request timeout to the full HTTP operation (connect + request + body read) (FR-070).
  - The transport SHALL cap concurrent in-flight requests per client instance (FR-070).
  - The transport SHALL enforce an aggregate per-client QPS rate limiter (NFR-001).
  - The transport SHALL emit structured logs with required fields and redaction (NFR-003).

* Inputs/Outputs
  - Inputs: `RequestSpec` = { method, path, query_pairs, body_json? } plus internal correlation ID.
  - Outputs: `RawResponse` = { status, headers, body_bytes }.

* Public Interfaces
  - `pub struct RequestSpec { pub method: http::Method, pub path: String, pub query: Vec<(String,String)>, pub body: Option<serde_json::Value> }`
  - `pub struct RawResponse { pub status: u16, pub body: bytes::Bytes }`
  - `impl Transport { pub async fn execute(&self, req: RequestSpec) -> Result<RawResponse, MpApiError>; }`

* Internal Design
  - Underlying HTTP client: `reqwest::Client` configured once.
  - URL build: `base_url.join(&path)`; query pairs appended via deterministic serializer.
  - Concurrency: `tokio::sync::Semaphore` acquire permit around request execution.
  - Rate limiting: await token from Rate Limiter module before executing.
  - Retries: delegated to Retry module; transport provides a closure `attempt_execute_once`.
  - Observability:
    - generate `correlation_id` (UUID v4)
    - start `tracing` span with: method, path, status, latency_ms, retry_count, correlation_id
    - redact headers: `X-API-KEY` and any configured sensitive keys

* Data Model
  - N/A

* Business Rules & Validation
  - Reject non-HTTPS base URL at build time unless allow_insecure_http.

* Error Handling
  - Network failures => `TransportError`
  - Timeout => `TransportError` (distinguishable via error string or dedicated variant)
  - Non-2xx => mapped by Errors module

* Logging & Metrics
  - Required fields: request path, status code, latency, retry count, correlation ID (NFR-003).
  - Optional metrics integration: expose hooks/callbacks rather than forcing a metrics crate (library-friendly).

* Security
  - Redaction MUST be applied to secrets in logs.
  - TLS enforced by default.

* Performance/Scalability Notes
  - Minimal allocations: reuse reqwest client; avoid cloning body where possible.
  - Allow caller to configure concurrency and QPS.

* Dependencies
  - `reqwest`, `tokio`, `tracing`, `uuid`, `bytes`, `http`.

* Test Design
  - UT-FR-070: verify headers, URL prefixing, timeout, concurrency semaphore behavior, HTTPS gate.
  - UT-NFR-003: verify structured logs contain required fields and redaction behavior.

## Rate Limiter

* Purpose  
  Enforce a per-client aggregate requests-per-second (QPS) limit to prevent server overload and comply with published guidance.

* Files (responsibility & description)
  - `src/middleware/mod.rs`: Middleware module root; exports rate limit and retry components.
  - `src/middleware/rate_limit.rs`: Token-bucket rate limiter enforcing per-client QPS.

* Type Inventory (definitions & descriptions)
  - `RateLimiter`: Async token-bucket rate limiter enforcing aggregate QPS per client instance.


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - The client SHALL provide a configurable rate limiter and SHALL default to 25 req/s unless overridden (NFR-001).
  - The rate limiter SHALL apply across all requests issued by a single client instance (FR-070).

* In-Scope / Out-of-Scope
  - In-scope: QPS limiting for all outbound HTTP calls made by this client instance.
  - Out-of-scope: global process-wide or cross-process rate limiting; server-side quota management.

* Inputs/Outputs (schemas, examples)
  - Input: an await point `acquire()` before request execution.
  - Output: a permit (implicit) allowing the request to proceed.

* Public Interfaces
  - `pub struct RateLimiter { /* token bucket */ }`
  - `impl RateLimiter { pub async fn acquire(&self) -> (); }`

* Internal Design
  - Token-bucket algorithm (deterministic, testable):
    - capacity = qps_limit
    - tokens are refilled based on elapsed time since last refill
    - `acquire()` waits until at least one token is available, then consumes one
  - Alternative: use a dedicated crate (e.g., `governor`) if acceptable.

* Data Model
  - N/A

* Business Rules & Validation (mapped to requirement IDs)
  - Default qps_limit=25 (NFR-001).
  - qps_limit MUST be >= 1; qps_limit=0 is rejected at build time with `MpApiError::ConfigurationError("qps_limit must be >= 1")`.

* Error Handling
  - No runtime errors during `acquire()`; invalid settings are rejected during config validation.

* Logging & Metrics
  - Optional metrics: queue wait time and effective throughput (exported via hooks; see §9.1).

* Security
  - N/A

* Performance/Scalability Notes
  - Must be low overhead; prefer lock-free or minimal-lock implementation.
  - Works with concurrency semaphore to bound both burst and sustained load.

* Dependencies
  - `tokio::time` (and optionally `governor`).

* Test Design
  - UT-NFR-001: default 25 req/s.
  - UT-NFR-001: overriding qps_limit changes pacing deterministically (use mocked time).

## Retry Policy

* Purpose  
  Improve reliability by retrying transient failures with exponential backoff while avoiding unsafe retries for non-transient errors.

* Files (responsibility & description)
  - `src/middleware/mod.rs`: Middleware module root; exports rate limit and retry components.
  - `src/middleware/retry.rs`: Retry policy implementation (classification + backoff + jitter).

* Type Inventory (definitions & descriptions)
  - `RetryConfig`: Retry configuration (max_attempts/base_backoff/max_backoff/jitter/retry_statuses/retry_methods).
  - `retry()`: Async retry executor applying classification and backoff around a request attempt closure.


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - The client SHALL implement retries with exponential backoff for transient failures with configurable policy and explicit defaults (max_retries=3, initial_backoff=200ms, max_backoff=2s, jitter=full-jitter) (NFR-002).
  - The client SHALL retry on HTTP 429 and HTTP 5xx, and on selected transport failures (timeouts, connection resets) (NFR-002).
  - The client SHALL NOT retry on HTTP 4xx other than 429, including 422 validation errors (NFR-002, FR-005).
  - Default policy MUST be: max_retries=3 (max_attempts=4), initial_backoff=200ms, max_backoff=2s, jitter=full-jitter, retryable_statuses={429, 5xx}, and retryable_methods={GET, HEAD, OPTIONS} unless explicitly configured otherwise (NFR-002).

* In-Scope / Out-of-Scope
  - In-scope: transient error retries for idempotent/safe operations. Default retry policy retries only for GET/HEAD/OPTIONS requests; POST/PUT/PATCH/DELETE are not retried unless explicitly enabled by caller configuration.
  - Out-of-scope: distributed tracing across retries (beyond correlation ID), request deduplication tokens.

* Inputs/Outputs (schemas, examples)
  - Input: `RetryConfig` and a closure performing one request attempt.
  - Output: final success response or last encountered error.

* Public Interfaces
  - `pub struct RetryConfig { pub max_attempts: u32, pub base_backoff: Duration, pub max_backoff: Duration, pub retry_statuses: Vec<u16> }`
  - `pub async fn retry<F, Fut, T>(cfg: &RetryConfig, f: F) -> Result<T, MpApiError>
       where F: FnMut(u32) -> Fut, Fut: Future<Output=Result<T, MpApiError>>;`

* Internal Design
  - Backoff schedule:
    - attempt 1: no delay
    - attempt n>1: delay = min(max_backoff, base_backoff * 2^(n-2)) + jitter
  - Retry classification:
    - retryable HTTP statuses: 429 + 5xx
    - retryable transport failures: request timeouts, connect timeouts, DNS resolution failures, connection resets, and TLS handshake failures (mapped from reqwest error categories); all others are non-retryable by default.

* Data Model
  - N/A

* Business Rules & Validation (mapped to requirement IDs)
  - Retry budget and backoff settings are configurable via OPS-SETTINGS-001, with defaults per NFR-REL-RETRY-001.

* Error Handling
  - When retries are exhausted, return the last error.
  - Retry logic MUST preserve the original error classification (RetryableHttpError vs TransportError vs HttpError).

* Logging & Metrics
  - Emit retry_count and backoff_ms fields on the request span; do not log request bodies.

* Security
  - Do not log sensitive headers or bodies during retries.

* Performance/Scalability Notes
  - Backoff mitigates thundering herd; coordinates with QPS limiter.

* Dependencies
  - `tokio::time`, optional RNG crate (`rand`).

* Test Design
  - UT-NFR-002: retry vs no-retry behavior by status and transport error type; backoff bounded.

## OpenAPI Routes

* Purpose  
  Provide typed, async methods for every OpenAPI operation enumerated in Appendix A-OpenAPI, with consistent application of cross-cutting behavior.

* Files (responsibility & description)
  - `src/routes/mod.rs`: Routes module root; exposes openapi and doc_driven submodules.
  - `src/routes/openapi/mod.rs`: OpenAPI route root and group clients; generated or semi-generated.
  - `src/routes/openapi/inventory.rs`: Generated inventory of OpenAPI operations (method/path/operationId) used for coverage checks.
  - `src/routes/openapi/materials.rs`: Generated materials route group client and per-operation methods.
  - `src/routes/openapi/molecules.rs`: Generated molecules route group client and per-operation methods.
  - `src/routes/openapi/defects.rs`: Generated defects route group client and per-operation methods.
  - `src/routes/openapi/doi.rs`: Generated DOI route group client and per-operation methods.

* Type Inventory (definitions & descriptions)
  - `OpenApiRoot`: Root accessor for OpenAPI-generated route group clients.
  - `MaterialsRoot`: Materials route group client.
  - `MoleculesRoot`: Molecules route group client.
  - `DefectsRoot`: Defects route group client.
  - `DoiRoot`: DOI route group client.
  - `inventory::Operation`: Inventory entry describing one OpenAPI operation (method/path/operationId).


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - For every OpenAPI operation in Appendix A-OpenAPI, the client SHALL provide an async method issuing the corresponding HTTP request and returning a parsed response envelope or typed error (FR-007).
  - Methods SHALL use HTTP method + path from OpenAPI and serialize parameters per OpenAPI definitions (FR-007).
  - Methods SHALL apply common behavior: auth header injection, pagination, field projection, error mapping (FR-001, FR-002, FR-003, FR-006).

* In-Scope / Out-of-Scope
  - In-scope: all OpenAPI-defined endpoints (RDD §6.1 and Appendix A-OpenAPI).
  - Out-of-scope: endpoints absent from OpenAPI (handled by Doc-Driven Routes).

* Inputs/Outputs (schemas, examples)
  - Inputs: per-operation parameter structs (generated), optional request body structs for POST.
  - Outputs: `Response<TDoc>` where `TDoc` is the per-operation document type.

* Public Interfaces
  - Namespace shape (example; naming may differ, coverage is mandatory):
    - `routes::openapi::OpenApiRoot`
      - `.materials() -> MaterialsRoot`
      - `.molecules() -> MoleculesRoot`
      - `.defects() -> DefectsRoot`
      - `.doi() -> DoiRoot`

* Internal Design
  - OpenAPI-driven code generation:
    - `spec/openapi.json` is authoritative for paths, params, and schemas.
    - Generate:
      - parameter structs with `serde` serialization aligned to OpenAPI
      - response document structs
      - route methods that create `RequestSpec` and call Transport

* Data Model
  - Generated Rust structs corresponding to OpenAPI schemas (stored under `src/data/models/`).

* Business Rules & Validation (mapped to requirement IDs)
  - Common validation:
    - pagination conflicts rejected locally (FR-002)
    - server-side validation errors mapped from 422 (FR-005)

* Error Handling
  - Use common Errors module mapping (FR-006).

* Logging & Metrics
  - Route methods rely on Transport for request spans and metrics.

* Security
  - Inherit Transport security rules (HTTPS, redaction).

* Performance/Scalability Notes
  - Generated code should avoid per-call allocation where possible; reuse reqwest client.

* Dependencies
  - Transport, Query, Data, Errors, generated code.

* OpenAPI Endpoint Coverage Inventory (RDD §6.1)
- `FR-DEF-TASKS-GET`
- `FR-DOI-ROOT-GET`
- `FR-MAT-ABSORPTION-GET`
- `FR-MAT-ALLOYS-GET`
- `FR-MAT-BONDS-GET`
- `FR-MAT-CHEMENV-GET`
- `FR-MAT-CONVERSION_ELECTRODES-GET`
- `FR-MAT-CORE-GET`
- `FR-MAT-CORE_BLESSED_TASKS-GET`
- `FR-MAT-CORE_FIND_STRUCTURE-POST`
- `FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET`
- `FR-MAT-DIELECTRIC-GET`
- `FR-MAT-ELASTICITY-GET`
- `FR-MAT-ELECTRONIC_STRUCTURE-GET`
- `FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET`
- `FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET`
- `FR-MAT-EOS-GET`
- `FR-MAT-FERMI-GET`
- `FR-MAT-GRAIN_BOUNDARIES-GET`
- `FR-MAT-INSERTION_ELECTRODES-GET`
- `FR-MAT-MAGNETISM-GET`
- `FR-MAT-OXIDATION_STATES-GET`
- `FR-MAT-PHONON-GET`
- `FR-MAT-PIEZOELECTRIC-GET`
- `FR-MAT-PROVENANCE-GET`
- `FR-MAT-ROBOCRYS-GET`
- `FR-MAT-ROBOCRYS_TEXT_SEARCH-GET`
- `FR-MAT-SIMILARITY-GET`
- `FR-MAT-SIMILARITY_MATCH-GET`
- `FR-MAT-SUBSTRATES-GET`
- `FR-MAT-SUMMARY-GET`
- `FR-MAT-SURFACE_PROPERTIES-GET`
- `FR-MAT-SYNTHESIS-GET`
- `FR-MAT-TASKS-GET`
- `FR-MAT-TASKS_DEPRECATION-GET`
- `FR-MAT-TASKS_ENTRIES-GET`
- `FR-MAT-TASKS_TRAJECTORY-GET`
- `FR-MAT-THERMO-GET`
- `FR-MAT-XAS-GET`
- `FR-MOL-JCESR-GET`
- `FR-MOL-SUMMARY-GET`

* Test Design
  - CT-MANIFEST-001: asserts 1:1 coverage between Appendix A inventory, implemented methods, and contract tests (FR-007).
  - CT-<FR-...>: per-operation contract tests validate request formation and envelope parsing.

## Doc-Driven Routes

* Purpose  
  Implement endpoints listed in official documentation (Getting Started) that are missing from the uploaded OpenAPI, using a doc-driven contract and returning raw JSON.

* Files (responsibility & description)
  - `src/routes/doc_driven/mod.rs`: Doc-driven route root and shared request helpers.
  - `src/routes/doc_driven/molecules.rs`: Doc-driven molecules endpoints missing from OpenAPI (raw JSON mode).

* Type Inventory (definitions & descriptions)
  - `DocDrivenRoot`: Root accessor for doc-driven endpoint clients.
  - `DocQuery`: Query wrapper for pagination/projection/extra parameters for doc-driven requests.
  - `DocResponse`: Response wrapper supporting envelope-first and raw JSON fallback.


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - The client SHALL implement each doc-driven endpoint exactly as specified by path in the RDD (§6.2) and apply common query parameters + escape hatch (FR-049).
  - The client SHALL not pre-validate endpoint-specific parameters for doc-driven endpoints; it SHALL pass through query parameters and surface server 422 as `ValidationError` (FR-049, FR-005).
  - Response parsing SHALL accept either:
    1) standard envelope `{data, errors, meta}` or
    2) top-level JSON array/object when the server does not wrap responses (FR-049).
  - Doc-driven endpoints SHALL return raw JSON mode (`serde_json::Value`) for `data` (FR-049, DR-002).

* In-Scope / Out-of-Scope
  - In-scope: endpoints explicitly listed under RDD §6.2.
  - Out-of-scope: any other undocumented endpoints; those require OpenAPI addition or explicit RDD update.

* Inputs/Outputs (schemas, examples)
  - Inputs: `DocQuery` consisting of pagination/projection plus extra query pairs.
  - Outputs:
    - preferred: `Response<serde_json::Value>`
    - fallback: `serde_json::Value` (top-level)

* Public Interfaces
  - `pub struct DocDrivenRoot { /* fields */ }`
  - Provide one async method per endpoint listed below; naming is flexible but must be documented.

* Internal Design
  - Hard-code the HTTP paths listed in the RDD for doc-driven endpoints.
  - Use the same Transport as OpenAPI routes.
  - Parse response as envelope-first, fallback-to-raw.

* Data Model
  - `DocResponse` types are raw JSON values plus optional envelope metadata.

* Business Rules & Validation (mapped to requirement IDs)
  - Escape hatch query parameters are passed through verbatim.

* Error Handling
  - HTTP 422 => `ValidationError`
  - HTTP 429/5xx => `RetryableHttpError` (subject to retry policy)
  - Other non-2xx => `HttpError`

* Logging & Metrics
  - Same as Transport.

* Security
  - Same as Transport.

* Performance/Scalability Notes
  - Minimal parsing overhead due to raw JSON mode.

* Dependencies
  - Transport, Query, Data, Errors.

* Doc-Driven Endpoint Inventory (RDD §6.2)
- `FR-MOL-ASSOC-GET`
- `FR-MOL-BONDING-GET`
- `FR-MOL-CORE-GET`
- `FR-MOL-ORBITALS-GET`
- `FR-MOL-PARTIAL_CHARGES-GET`
- `FR-MOL-PARTIAL_SPINS-GET`
- `FR-MOL-REDOX-GET`
- `FR-MOL-TASKS-GET`
- `FR-MOL-THERMO-GET`
- `FR-MOL-VIBRATIONS-GET`

* Test Design
  - DT-MANIFEST-001: asserts doc-driven endpoint list matches RDD and implemented methods exist.
  - DT-<FR-MOL-...>: validates URL/query formation and deserialization into JSON.

## Convenience Workflows

* Purpose  
  Provide ergonomic, examples-driven helper methods that compose underlying route calls and field projection to match common user workflows from official documentation.

* Files (responsibility & description)
  - `src/convenience/mod.rs`: Example-driven convenience wrappers composing one underlying route call + projection.

* Type Inventory (definitions & descriptions)
  - `ConvenienceRoot`: Root accessor for convenience workflow methods.


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - The client SHALL provide the convenience methods enumerated in RDD §6.3 and implement them as thin wrappers over underlying endpoints (FR-060).
  - Each convenience method SHALL make exactly one underlying HTTP request to the specified endpoint with the required projection parameters, and SHALL parse the response into the specified sub-structure (RDD acceptance statements for each FR-MPR-*).

* In-Scope / Out-of-Scope
  - In-scope: convenience requirements listed in RDD §6.3.
  - Out-of-scope: higher-level multi-call workflows not explicitly required.

* Inputs/Outputs (schemas, examples)
  - Inputs: identifiers such as `material_id` or `task_id`.
  - Outputs: extracted artifacts (structure dict, task_id list, bandstructure, DOS, phonon variants, charge density).

* Public Interfaces
  - Implement one async method per requirement listed below; exact Rust type names may differ but semantics are normative.

* Internal Design
  - Compose a single OpenAPI route call plus projection:
    - `GET /materials/summary/` for structure/task_ids/bandstructure/dos.
    - `GET /materials/phonon/` for phonon variants.
  - Charge density feature detection:
    - If the relevant route is missing from OpenAPI, return `UnsupportedBySpecification("charge_density")` without HTTP.
    - If present, call the OpenAPI route and return `Response<serde_json::Value>` unless the schema is explicitly defined in OpenAPI, in which case return `Response<T>` for that schema.

* Data Model
  - Output artifact types are derived from the underlying response document model; when schema is unknown, use raw JSON.

* Business Rules & Validation (mapped to requirement IDs)
  - Exactly one underlying request per convenience method (enforced in tests).

* Error Handling
  - Propagate underlying route errors unchanged.
  - Charge density missing => `UnsupportedBySpecification("charge_density")`.

* Logging & Metrics
  - Same as Transport; convenience methods may add span fields (e.g., material_id) but MUST not log secrets.

* Security
  - Same as Transport.

* Performance/Scalability Notes
  - Single-call wrappers; no additional network overhead.

* Dependencies
  - OpenAPI Routes, Data, Errors.

* Convenience Inventory (RDD §6.3)
- `FR-MPR-GET_STRUCTURE_BY_MATERIAL_ID`
- `FR-MPR-GET_TASK_IDS_ASSOCIATED_WITH_MATERIAL_ID`
- `FR-MPR-GET_BANDSTRUCTURE_BY_MATERIAL_ID`
- `FR-MPR-GET_DOS_BY_MATERIAL_ID`
- `FR-MPR-GET_PHONON_BANDSTRUCTURE_BY_MATERIAL_ID`
- `FR-MPR-GET_PHONON_DOS_BY_MATERIAL_ID`
- `FR-MPR-GET_CHARGE_DENSITY_FROM_MATERIAL_ID`
- `FR-MPR-GET_CHARGE_DENSITY_FROM_TASK_ID`

* Test Design
  - UT-<FR-MPR-...>: verify exactly one underlying request with expected path and projection; parse into expected sub-structure.

## Testing Harness

* Purpose  
  Provide deterministic automated verification that the client satisfies coverage and contract requirements: per-operation contract tests, inventory cross-checks, and opt-in integration smoke tests.

* Files (responsibility & description)
  - `tests/contract_manifest.rs`: Manifest tests asserting inventory parity (OpenAPI ↔ methods ↔ tests).
  - `tests/contract_openapi_materials.rs`: Contract tests for OpenAPI materials operations.
  - `tests/contract_openapi_molecules.rs`: Contract tests for OpenAPI molecules operations.
  - `tests/contract_openapi_defects.rs`: Contract tests for OpenAPI defects operations.
  - `tests/contract_openapi_doi.rs`: Contract tests for OpenAPI DOI operations.
  - `tests/doc_driven_molecules.rs`: Doc-driven endpoint tests (URL/query formation + JSON deserialization).
  - `tests/integration_smoke.rs`: Opt-in live smoke tests gated by MP_API_KEY/PMG_MAPI_KEY.
  - `tests/parity_python_mp_api.rs`: Optional parity tests comparing Rust client calls with the Python `mp-api` client across the full mapped API surface.
  - `tests/python/parity_mp_api.py`: Python helper used by parity tests to enumerate Python `mp-api` APIs and produce canonical JSON outputs.
  - `tests/python/parity_manifest.json`: Version-controlled mapping of Python `mp-api` API identifiers to Rust method identifiers.
  - `tests/fixtures/`: JSON fixtures used by contract tests (minimal schema-conformant payloads).

* Type Inventory (definitions & descriptions)
  - `CT-MANIFEST-001`: Manifest test ID asserting 1:1 coverage for OpenAPI operations.
  - `IT-SMOKE-001`: Integration smoke test ID (gated).
  - `CI-DEPS-001`: CI check ID asserting dependency baseline compliance (Cargo.toml vs §11 Assumptions).
  - `PT-MANIFEST-001`: Parity manifest test ID asserting full Python `mp-api` API surface coverage via mapping.
  - `PT-PY-ALL-001`: Python parity suite test ID executing parity vectors for every mapped Python API.


* Responsibilities (explicit; MUST/SHALL statements; testable)
  - Unit tests SHALL cover query serialization and error mapping (RDD §11.1; NFR-005).
  - Contract tests SHALL validate request/response shapes against OpenAPI for every Appendix A operation (RDD §11.2; NFR-006).
  - Integration smoke tests SHALL be skipped unless `MP_API_KEY` or `PMG_MAPI_KEY` is set; when set, they SHALL run as smoke tests (RDD §11.3; NFR-007).
  - A manifest test SHALL assert 1:1 coverage between Appendix A inventory, implemented methods, and contract tests (FR-007).
  - A CI check SHALL verify the crate baseline dependency set (NFR-008).
  - When enabled, Python parity tests SHALL assert full Python `mp-api` API surface coverage and input/output equivalence (NFR-009, FR-071).

* In-Scope / Out-of-Scope
  - In-scope: CI tests verifying coverage and contract fidelity (unit tests + contract tests always run; integration smoke tests are conditional; see §8 Testing Harness).
  - Out-of-scope: performance benchmarking suites.

* Inputs/Outputs (schemas, examples)
  - Inputs: `spec/openapi.json`; optional API key env vars for integration.
  - Outputs: CI pass/fail, test reports.

* Public Interfaces
  - Tests are invoked via `cargo test`; the harness provides:
    - `CT-MANIFEST-001`
    - `CT-<FR-...>` per OpenAPI endpoint
    - `DT-<FR-...>` per doc-driven endpoint
    - `IT-SMOKE-001` (conditional)

* Internal Design
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

* Data Model
  - Fixtures stored under `tests/fixtures/`.

* Business Rules & Validation (mapped to requirement IDs)
  - Integration tests do not run without API key env vars.

* Error Handling
  - Tests must not print API keys; all failures redact secrets.

* Logging & Metrics
  - Use `tracing_test` or equivalent to capture logs and assert required fields for logging requirement.

* Security
  - CI secrets handled by CI environment; tests must not print or store API keys.

* Performance/Scalability Notes
  - Integration tests are skipped by default to keep CI fast when secrets are absent.

* Dependencies
  - `wiremock`/`httpmock`, `openapiv3`, `tokio`.

* Test Design
  - CI-PIPELINE-001: CI runs unit + contract tests always; integration tests are conditional.

# 9. Cross-cutting Concerns

## 9.1 Observability (logs/metrics/traces)

- Use `tracing` spans for each request; include:
  - correlation_id
  - method, path
  - status
  - latency_ms
  - retry_count
- Redaction is mandatory for secrets (API keys).  
  Requirement: NFR-003

## 9.2 Security & Privacy

- HTTPS enforced by default; `allow_insecure_http` is explicitly opt-in and intended only for local testing harnesses.  
  Requirement: NFR-004
- API keys are treated as secrets:
  - never printed in logs
  - excluded from `Debug` output
  - stored only in process memory  
  Requirement: FR-001

## 9.3 Performance & Capacity

- Default QPS limit of 25 requests/sec; configurable.  
  Requirement: NFR-001
- Concurrency cap enforced via semaphore; default `concurrency=16` (see §11 Assumptions).
- Retry policy uses exponential backoff with jitter, bounded by max_backoff.  
  Requirement: NFR-002

## 9.4 Deployment & Configuration

- Configuration sources and precedence are explicitly defined by OPS-CONFIG-SOURCES-001.
- All settings enumerated in OPS-SETTINGS-001 are supported; settings are applied at build time and stored in immutable `Config`.

## 9.5 Backward Compatibility / Migration / Rollback

- Model structs allow unknown fields to tolerate server-side schema expansion (DR-003).
- Typed mode may introduce compile-time breaking changes when OpenAPI changes; mitigate by:
  - versioning the crate according to Semantic Versioning (MAJOR for breaking public API changes, MINOR for backwards-compatible additions, PATCH for backwards-compatible fixes)
  - providing raw JSON mode for forward compatibility (DR-002).
- Rollback strategy: pin crate version; configuration remains external.

# 10. Requirements Traceability Matrix

| Requirement ID | Requirement Text | Module(s) | Design Section | Test Case IDs | Coverage Status |
|---|---|---|---|---|---|
| FR-001 | Client MUST authenticate using API key header. (RDD: FR-COMMON-AUTH-001) | Client Facade (MpClient), Configuration (Config & Builder), HTTP Transport, Errors (MpApiError) | §8 Client Facade (MpClient) | UT-FR-001 | Covered |
| FR-002 | Client MUST support both page-based and offset-based pagination. (RDD: FR-COMMON-PAGINATION-001) | Query Parameters (Pagination/Projection/Escape Hatch), OpenAPI Routes, Doc-Driven Routes, Errors (MpApiError) | §8 Query Parameters (Pagination/Projection/Escape Hatch) | UT-FR-002 | Covered |
| FR-003 | Client MUST support field projection. (RDD: FR-COMMON-PROJECTION-001) | Query Parameters (Pagination/Projection/Escape Hatch), OpenAPI Routes, Doc-Driven Routes | §8 Query Parameters (Pagination/Projection/Escape Hatch) | UT-FR-003 | Covered |
| FR-004 | Client MUST parse the standard `{data, errors, meta}` response envelope. (RDD: FR-COMMON-ENVELOPE-001) | Envelope & Models, OpenAPI Routes, Doc-Driven Routes, Errors (MpApiError) | §8 Envelope & Models | UT-FR-004 | Covered |
| FR-005 | Client MUST parse 422 validation errors into a typed error structure. (RDD: FR-COMMON-VALIDATION-001) | Errors (MpApiError), OpenAPI Routes, Doc-Driven Routes | §8 Errors (MpApiError) | UT-FR-005 | Covered |
| FR-006 | Client MUST expose a typed error model that distinguishes configuration errors, request construction errors, transport errors, HTTP status errors, validation errors, and deserialization errors. (RDD: FR-COMMON-ERROR_MODEL-001) | Errors (MpApiError), Retry Policy, HTTP Transport, Client Facade (MpClient) | §8 Errors (MpApiError) | UT-FR-006 | Covered |
| FR-007 | For every OpenAPI operation enumerated in Appendix A-OpenAPI, the Rust client MUST provide an async method that issues the corresponding HTTP request and returns a parsed response envelope or a typed error. (RDD: FR-COMMON-OPENAPI_COVERAGE-001) | OpenAPI Routes, Testing Harness, Client Facade (MpClient) | §8 OpenAPI Routes | CT-MANIFEST-001, UT-INVENTORY-001 | Covered |
| FR-008 | FR-DEF-TASKS-GET — GET `/defects/tasks/`: Get DefectTaskDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-DEF-TASKS-GET | Covered |
| FR-009 | FR-DOI-ROOT-GET — GET `/doi/`: Get DOIDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-DOI-ROOT-GET | Covered |
| FR-010 | FR-MAT-ABSORPTION-GET — GET `/materials/absorption/`: Get AbsorptionDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-ABSORPTION-GET | Covered |
| FR-011 | FR-MAT-ALLOYS-GET — GET `/materials/alloys/`: Get AlloyPairDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-ALLOYS-GET | Covered |
| FR-012 | FR-MAT-BONDS-GET — GET `/materials/bonds/`: Get BondingDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-BONDS-GET | Covered |
| FR-013 | FR-MAT-CHEMENV-GET — GET `/materials/chemenv/`: Get ChemEnvDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-CHEMENV-GET | Covered |
| FR-014 | FR-MAT-CONVERSION_ELECTRODES-GET — GET `/materials/conversion_electrodes/`: Get ConversionElectrodeDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-CONVERSION_ELECTRODES-GET | Covered |
| FR-015 | FR-MAT-CORE-GET — GET `/materials/core/`: Get MaterialsDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-CORE-GET | Covered |
| FR-016 | FR-MAT-CORE_BLESSED_TASKS-GET — GET `/materials/core/blessed_tasks/`: Get MaterialsDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-CORE_BLESSED_TASKS-GET | Covered |
| FR-017 | FR-MAT-CORE_FIND_STRUCTURE-POST — POST `/materials/core/find_structure/`: Post FindStructure documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-CORE_FIND_STRUCTURE-POST | Covered |
| FR-018 | FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET — GET `/materials/core/formula_autocomplete/`: Get FormulaAutocomplete documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET | Covered |
| FR-019 | FR-MAT-DIELECTRIC-GET — GET `/materials/dielectric/`: Get DielectricDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-DIELECTRIC-GET | Covered |
| FR-020 | FR-MAT-ELASTICITY-GET — GET `/materials/elasticity/`: Get ElasticityDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-ELASTICITY-GET | Covered |
| FR-021 | FR-MAT-ELECTRONIC_STRUCTURE-GET — GET `/materials/electronic_structure/`: Get ElectronicStructureDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-ELECTRONIC_STRUCTURE-GET | Covered |
| FR-022 | FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET — GET `/materials/electronic_structure/bandstructure/`: Get ElectronicStructureDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET | Covered |
| FR-023 | FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET — GET `/materials/electronic_structure/dos/`: Get ElectronicStructureDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET | Covered |
| FR-024 | FR-MAT-EOS-GET — GET `/materials/eos/`: Get EOSDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-EOS-GET | Covered |
| FR-025 | FR-MAT-FERMI-GET — GET `/materials/fermi/`: Get FermiDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-FERMI-GET | Covered |
| FR-026 | FR-MAT-GRAIN_BOUNDARIES-GET — GET `/materials/grain_boundaries/`: Get GrainBoundaryDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-GRAIN_BOUNDARIES-GET | Covered |
| FR-027 | FR-MAT-INSERTION_ELECTRODES-GET — GET `/materials/insertion_electrodes/`: Get InsertionElectrodeDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-INSERTION_ELECTRODES-GET | Covered |
| FR-028 | FR-MAT-MAGNETISM-GET — GET `/materials/magnetism/`: Get MagnetismDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-MAGNETISM-GET | Covered |
| FR-029 | FR-MAT-OXIDATION_STATES-GET — GET `/materials/oxidation_states/`: Get OxidationStateDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-OXIDATION_STATES-GET | Covered |
| FR-030 | FR-MAT-PHONON-GET — GET `/materials/phonon/`: Get PhononBSDOSDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-PHONON-GET | Covered |
| FR-031 | FR-MAT-PIEZOELECTRIC-GET — GET `/materials/piezoelectric/`: Get PiezoelectricDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-PIEZOELECTRIC-GET | Covered |
| FR-032 | FR-MAT-PROVENANCE-GET — GET `/materials/provenance/`: Get ProvenanceDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-PROVENANCE-GET | Covered |
| FR-033 | FR-MAT-ROBOCRYS-GET — GET `/materials/robocrys/`: Get RobocrystallogapherDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-ROBOCRYS-GET | Covered |
| FR-034 | FR-MAT-ROBOCRYS_TEXT_SEARCH-GET — GET `/materials/robocrys/text_search/`: Get RobocrystallogapherDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-ROBOCRYS_TEXT_SEARCH-GET | Covered |
| FR-035 | FR-MAT-SIMILARITY-GET — GET `/materials/similarity/`: Get SimilarityDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-SIMILARITY-GET | Covered |
| FR-036 | FR-MAT-SIMILARITY_MATCH-GET — GET `/materials/similarity/match/`: Get SimilarityDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-SIMILARITY_MATCH-GET | Covered |
| FR-037 | FR-MAT-SUBSTRATES-GET — GET `/materials/substrates/`: Get SubstratesDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-SUBSTRATES-GET | Covered |
| FR-038 | FR-MAT-SUMMARY-GET — GET `/materials/summary/`: Get SummaryDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-SUMMARY-GET | Covered |
| FR-039 | FR-MAT-SURFACE_PROPERTIES-GET — GET `/materials/surface_properties/`: Get SurfacePropDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-SURFACE_PROPERTIES-GET | Covered |
| FR-040 | FR-MAT-SYNTHESIS-GET — GET `/materials/synthesis/`: Get SynthesisSearchResultModel documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-SYNTHESIS-GET | Covered |
| FR-041 | FR-MAT-TASKS-GET — GET `/materials/tasks/`: Get TaskDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-TASKS-GET | Covered |
| FR-042 | FR-MAT-TASKS_DEPRECATION-GET — GET `/materials/tasks/deprecation/`: Get DeprecationDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-TASKS_DEPRECATION-GET | Covered |
| FR-043 | FR-MAT-TASKS_ENTRIES-GET — GET `/materials/tasks/entries/`: Get EntryDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-TASKS_ENTRIES-GET | Covered |
| FR-044 | FR-MAT-TASKS_TRAJECTORY-GET — GET `/materials/tasks/trajectory/`: Get TrajectoryDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-TASKS_TRAJECTORY-GET | Covered |
| FR-045 | FR-MAT-THERMO-GET — GET `/materials/thermo/`: Get ThermoDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-THERMO-GET | Covered |
| FR-046 | FR-MAT-XAS-GET — GET `/materials/xas/`: Get XASDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MAT-XAS-GET | Covered |
| FR-047 | FR-MOL-JCESR-GET — GET `/molecules/jcesr/`: Get MoleculesDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MOL-JCESR-GET | Covered |
| FR-048 | FR-MOL-SUMMARY-GET — GET `/molecules/summary/`: Get MoleculeSummaryDoc documents | OpenAPI Routes, Envelope & Models, Errors (MpApiError) | §8 OpenAPI Routes | CT-FR-MOL-SUMMARY-GET | Covered |
| FR-049 | Client MUST implement endpoints listed in official docs (e.g., “Getting Started” endpoint table) but absent from the uploaded OpenAPI, using a doc-driven contract. (RDD: FR-COMMON-DOC_DRIVEN-001) | Doc-Driven Routes, Query Parameters (Pagination/Projection/Escape Hatch), Envelope & Models, Errors (MpApiError) | §8 Doc-Driven Routes | DT-MANIFEST-001, UT-FR-049 | Covered |
| FR-050 | FR-MOL-ASSOC-GET — GET `/molecules/assoc`: Client MUST support searching associated molecule documents via `/molecules/assoc`. | Doc-Driven Routes, Query Parameters (Pagination/Projection/Escape Hatch), Envelope & Models, Errors (MpApiError) | §8 Doc-Driven Routes | DT-FR-MOL-ASSOC-GET | Covered |
| FR-051 | FR-MOL-BONDING-GET — GET `/molecules/bonding`: Client MUST support searching molecule bonding documents via `/molecules/bonding`. | Doc-Driven Routes, Query Parameters (Pagination/Projection/Escape Hatch), Envelope & Models, Errors (MpApiError) | §8 Doc-Driven Routes | DT-FR-MOL-BONDING-GET | Covered |
| FR-052 | FR-MOL-CORE-GET — GET `/molecules/core`: Client MUST support searching molecule core documents via `/molecules/core`. | Doc-Driven Routes, Query Parameters (Pagination/Projection/Escape Hatch), Envelope & Models, Errors (MpApiError) | §8 Doc-Driven Routes | DT-FR-MOL-CORE-GET | Covered |
| FR-053 | FR-MOL-ORBITALS-GET — GET `/molecules/orbitals`: Client MUST support searching orbital documents via `/molecules/orbitals`. | Doc-Driven Routes, Query Parameters (Pagination/Projection/Escape Hatch), Envelope & Models, Errors (MpApiError) | §8 Doc-Driven Routes | DT-FR-MOL-ORBITALS-GET | Covered |
| FR-054 | FR-MOL-PARTIAL_CHARGES-GET — GET `/molecules/partial_charges`: Client MUST support searching partial charge documents via `/molecules/partial_charges`. | Doc-Driven Routes, Query Parameters (Pagination/Projection/Escape Hatch), Envelope & Models, Errors (MpApiError) | §8 Doc-Driven Routes | DT-FR-MOL-PARTIAL_CHARGES-GET | Covered |
| FR-055 | FR-MOL-PARTIAL_SPINS-GET — GET `/molecules/partial_spins`: Client MUST support searching partial spin documents via `/molecules/partial_spins`. | Doc-Driven Routes, Query Parameters (Pagination/Projection/Escape Hatch), Envelope & Models, Errors (MpApiError) | §8 Doc-Driven Routes | DT-FR-MOL-PARTIAL_SPINS-GET | Covered |
| FR-056 | FR-MOL-REDOX-GET — GET `/molecules/redox`: Client MUST support searching redox documents via `/molecules/redox`. | Doc-Driven Routes, Query Parameters (Pagination/Projection/Escape Hatch), Envelope & Models, Errors (MpApiError) | §8 Doc-Driven Routes | DT-FR-MOL-REDOX-GET | Covered |
| FR-057 | FR-MOL-TASKS-GET — GET `/molecules/tasks`: Client MUST support searching molecule task documents via `/molecules/tasks`. | Doc-Driven Routes, Query Parameters (Pagination/Projection/Escape Hatch), Envelope & Models, Errors (MpApiError) | §8 Doc-Driven Routes | DT-FR-MOL-TASKS-GET | Covered |
| FR-058 | FR-MOL-THERMO-GET — GET `/molecules/thermo`: Client MUST support searching molecule thermochemistry documents via `/molecules/thermo`. | Doc-Driven Routes, Query Parameters (Pagination/Projection/Escape Hatch), Envelope & Models, Errors (MpApiError) | §8 Doc-Driven Routes | DT-FR-MOL-THERMO-GET | Covered |
| FR-059 | FR-MOL-VIBRATIONS-GET — GET `/molecules/vibrations`: Client MUST support searching vibration documents via `/molecules/vibrations`. | Doc-Driven Routes, Query Parameters (Pagination/Projection/Escape Hatch), Envelope & Models, Errors (MpApiError) | §8 Doc-Driven Routes | DT-FR-MOL-VIBRATIONS-GET | Covered |
| FR-060 | The Rust client MUST provide convenience wrappers matching the official Examples workflows (idiomatic Rust names allowed). (RDD: FR-COMMON-CONVENIENCE-001) | Convenience Workflows, OpenAPI Routes, Doc-Driven Routes, Envelope & Models, Errors (MpApiError) | §8 Convenience Workflows | UT-FR-060 | Covered |
| FR-061 | Get a structure for a material_id (wraps `GET /materials/summary/` with projected field `structure`). (RDD: FR-MPR-GET_STRUCTURE_BY_MATERIAL_ID) | Convenience Workflows, OpenAPI Routes, Doc-Driven Routes, Envelope & Models, Errors (MpApiError) | §8 Convenience Workflows | UT-FR-061 | Covered |
| FR-062 | Get task IDs associated with a material_id (wraps `GET /materials/summary/` with projected field `task_ids`). (RDD: FR-MPR-GET_TASK_IDS_ASSOCIATED_WITH_MATERIAL_ID) | Convenience Workflows, OpenAPI Routes, Doc-Driven Routes, Envelope & Models, Errors (MpApiError) | §8 Convenience Workflows | UT-FR-062 | Covered |
| FR-063 | Get electronic bandstructure for a material_id (wraps `GET /materials/summary/` with projected field `bandstructure`). (RDD: FR-MPR-GET_BANDSTRUCTURE_BY_MATERIAL_ID) | Convenience Workflows, OpenAPI Routes, Doc-Driven Routes, Envelope & Models, Errors (MpApiError) | §8 Convenience Workflows | UT-FR-063 | Covered |
| FR-064 | Get electronic DOS for a material_id (wraps `GET /materials/summary/` with projected field `dos`). (RDD: FR-MPR-GET_DOS_BY_MATERIAL_ID) | Convenience Workflows, OpenAPI Routes, Doc-Driven Routes, Envelope & Models, Errors (MpApiError) | §8 Convenience Workflows | UT-FR-064 | Covered |
| FR-065 | Get phonon bandstructure for a material_id (wraps phonon endpoint). (RDD: FR-MPR-GET_PHONON_BANDSTRUCTURE_BY_MATERIAL_ID) | Convenience Workflows, OpenAPI Routes, Doc-Driven Routes, Envelope & Models, Errors (MpApiError) | §8 Convenience Workflows | UT-FR-065 | Covered |
| FR-066 | Get phonon DOS for a material_id (wraps phonon endpoint). (RDD: FR-MPR-GET_PHONON_DOS_BY_MATERIAL_ID) | Convenience Workflows, OpenAPI Routes, Doc-Driven Routes, Envelope & Models, Errors (MpApiError) | §8 Convenience Workflows | UT-FR-066 | Covered |
| FR-067 | Get charge density by material_id (Examples-driven). (RDD: FR-MPR-GET_CHARGE_DENSITY_FROM_MATERIAL_ID) | Convenience Workflows, OpenAPI Routes, Doc-Driven Routes, Envelope & Models, Errors (MpApiError) | §8 Convenience Workflows | UT-FR-067 | Covered (conditional: UnsupportedBySpecification if no OpenAPI route) |
| FR-068 | Get charge density by task_id (Examples-driven). (RDD: FR-MPR-GET_CHARGE_DENSITY_FROM_TASK_ID) | Convenience Workflows, OpenAPI Routes, Doc-Driven Routes, Envelope & Models, Errors (MpApiError) | §8 Convenience Workflows | UT-FR-068 | Covered (conditional: UnsupportedBySpecification if no OpenAPI route) |
| FR-069 | Client MUST support configuration from (1) explicit constructor/builder inputs and (2) environment variables, and MUST define precedence. (RDD: OPS-CONFIG-SOURCES-001) | Configuration (Config & Builder), HTTP Transport, Rate Limiter, Retry Policy, Client Facade (MpClient) | §8 Configuration (Config & Builder) | UT-FR-069 | Covered |
| FR-070 | Client MUST support setting: `api_key`, `base_url`, `timeout`, `concurrency`, `qps_limit`, `user_agent`, and `allow_insecure_http` (see §9 security). (RDD: OPS-SETTINGS-001) | Configuration (Config & Builder), HTTP Transport, Rate Limiter, Retry Policy, Client Facade (MpClient) | §8 Configuration (Config & Builder) | UT-FR-070 | Covered |
| FR-071 | The Rust client MUST implement the full API surface of the Python `mp-api` client (behavioral parity; API shape may differ). (RDD: USER-PYTHON-PARITY-COVERAGE-001) | Testing Harness, OpenAPI Routes, Doc-Driven Routes, Convenience Workflows | §8 Testing Harness | PT-MANIFEST-001, PT-PY-ALL-001 | Covered |
| NFR-001 | NFR-PERF-RATE_LIMIT-001 — Default QPS Limit: Client MUST provide a configurable rate limiter and MUST default to **25 requests per second** (aggregate) unless explicitly overridden. | Rate Limiter, Configuration (Config & Builder), HTTP Transport, Client Facade (MpClient) | §8 Rate Limiter | UT-NFR-001 | Covered |
| NFR-002 | NFR-REL-RETRY-001 — Retry Policy: Client MUST implement retries with exponential backoff for transient failures, using a configurable policy with explicit defaults (max_retries=3, initial_backoff=200ms, max_backoff=2s, jitter=full-jitter). | Retry Policy, Configuration (Config & Builder), HTTP Transport, Errors (MpApiError) | §8 Retry Policy | UT-NFR-002 | Covered |
| NFR-003 | NFR-OBS-LOGGING-001 — Structured Logging & Redaction: Client MUST support structured logs including request path, status code, latency, retry count, and correlation ID, and MUST redact API keys and other secrets. | HTTP Transport, Client Facade (MpClient), Errors (MpApiError) | §8 HTTP Transport | UT-NFR-003 | Covered |
| NFR-004 | NFR-SEC-TLS-001 — TLS and Host Controls: Client MUST use HTTPS by default and MUST reject non-HTTPS base URLs unless the caller explicitly opts in (for testing). | Configuration (Config & Builder), HTTP Transport, Errors (MpApiError) | §8 Configuration (Config & Builder) | UT-NFR-004 | Covered |
| NFR-005 | NFR-TEST-UNIT-001: Unit tests SHALL cover query serialization and error mapping. | Testing Harness | §8 Testing Harness | CI-PIPELINE-001 | Covered |
| NFR-006 | NFR-TEST-CONTRACT-001: Contract tests SHALL validate request/response shapes against OpenAPI for every inventory operation in Appendix A-OpenAPI. | Testing Harness | §8 Testing Harness | CI-PIPELINE-001 | Covered |
| NFR-007 | NFR-TEST-INTEGRATION-001: Integration smoke tests SHALL be skipped unless MP_API_KEY or PMG_MAPI_KEY is set; when set, they SHALL run as smoke tests. | Testing Harness | §8 Testing Harness | CI-PIPELINE-001 | Covered |
| DR-001 | DR-ENVELOPE-001: Responses SHALL follow a {data, errors, meta} envelope; Error SHALL contain required code:int and message:string; Meta SHALL include api_version, time_stamp (date-time), total_doc, facet (free-form). | Envelope & Models, OpenAPI Routes, Doc-Driven Routes | §8 Envelope & Models | UT-DR-001 | Covered |
| DR-002 | DR-TYPING-001: The client SHALL support typed mode Response<T> and raw JSON mode using serde_json::Value for response data. | Envelope & Models, OpenAPI Routes, Doc-Driven Routes | §8 Envelope & Models | UT-DR-002 | Covered |
| DR-003 | DR-SERDE-001: Generated/handwritten models SHALL use Option<T> for nullable fields and SHALL allow unknown fields for forward compatibility. | Envelope & Models, OpenAPI Routes, Doc-Driven Routes | §8 Envelope & Models | UT-DR-003 | Covered |

# 11. Assumptions

(Assumptions/proposals are directly reflected from RDD §2 unless noted.)

1. The uploaded OpenAPI file is authoritative for endpoint paths/params and response envelopes.  
2. `allow_insecure_http` is supported (default `false`) to permit non-HTTPS base URLs for local test harnesses only.  
3. Structured logging is emitted with mandatory redaction of secrets.

Implementation-detail assumptions (not specified as explicit values in the RDD; required to complete a runnable library):
4. Default values: `timeout=30s`, `concurrency=16`, and `user_agent="mp-api-rs/<crate_version>"` (where `<crate_version>` is the Cargo package version). Rationale: defaults target safe interactive use and CI determinism; callers can override.
5. Error body truncation threshold for logging/storage is 8192 bytes (8 KiB). Rationale: prevents excessive memory/log volume while preserving diagnostic context.

6. **Crate baseline (pinned versions/features)**: The implementation SHALL use at minimum the following Cargo dependencies (additional dependencies are allowed). This baseline is required to satisfy NFR-008.

```toml
[dependencies]
reqwest = { version = "0.12.24", features = ["json", "gzip", "brotli", "zstd", "rustls-tls"] }
tokio = { version = "1.48.0", features = ["macros", "rt-multi-thread", "time"] }
url = "2.5"
anyhow = "1.0.100"
utoipa = "5.4.0"
thiserror = "2.0.18"
serde_json = "1.0"
tracing = "0.1.44"
async-trait = "0.1.89"
serde = "1.0"
dotenvy = "0.15.7"
```

7. **Python parity harness dependency**: Parity tests assume that the Python `mp-api` package is available in the test environment (developer machine or CI runner) when `MP_API_PY_PARITY=1` is set.

# 12. Open Questions

1. **OpenAPI code generation approach:** Which generator strategy will be used (build-time Rust codegen vs. checked-in generated code vs. handwritten wrappers)? The design assumes code generation but does not mandate a specific tool.
   - Decision: Use checked-in generated code committed under `src/data/models/` and `src/routes/openapi/` for reproducible builds; provide an optional `cargo xtask generate` (or equivalent) to re-generate from `spec/openapi.json` and a CI check to prevent drift.
2. **Contract test fixtures:** For operations whose OpenAPI response schema requires non-empty objects, what minimal fixtures will be stored under `tests/fixtures/` to enable schema-conformant deserialization tests?
   - Decision: Store minimal, schema-conformant JSON fixtures under `tests/fixtures/` generated from OpenAPI examples when available, otherwise hand-authored minimal objects (empty arrays/objects only when allowed by schema).
3. **Charge density endpoint discovery:** If a charge density endpoint exists in OpenAPI, what are its exact path and schema name(s)? Convenience methods must either use it or return `UnsupportedBySpecification("charge_density")`.
   - Decision: Implement build-time feature detection by searching OpenAPI for operations tagged or named `charge_density`; if found, wire convenience methods to that route and return `Response<serde_json::Value>` unless a stable typed schema is present; otherwise return `UnsupportedBySpecification("charge_density")` without HTTP.
4. **Integration smoke test target identifiers:** Which stable material IDs/task IDs should be used in live smoke tests to avoid flakiness?
   - Decision: Avoid hard-coded IDs. Use query-based smoke tests that request a single well-known small result set (e.g., formula-based searches) and assert only envelope parsing and authentication behavior.
5. **Streaming downloads:** Should the client support streaming response bodies for very large payloads (beyond the envelope-bytes approach), and if so, what API shape should be used?
   - Decision: Not implemented in v1.1. Large payload handling is via pagination/projection and the existing bytes-buffered parser. Streaming may be introduced in a future major version if required by use cases.

# 13. Final Self-Check

- English-only document content (code identifiers/proper nouns allowed): **Yes**
- Single Markdown file output: **Yes**
- Table of Contents present: **Yes**
- Design Philosophy present: **Yes**
- High-Level Pipeline present: **Yes**
- Source Tree present: **Yes**
- Download link is Markdown link format in Title Page: **Yes**
- All RDD top-level headings listed in §6.5: **Yes**
- Section-by-section RDD digest & mapping provided in §6.5: **Yes**
- Requirements traceability matrix present and includes all extracted FR/NFR/DR requirements: **Yes**
- No invented facts: **Yes (no schema is guessed; doc-driven endpoints return raw JSON; no remaining unresolved placeholders)**
