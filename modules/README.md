<!-- filename: modules/README.md -->

# 1. Title Page

**System Name:** Materials Project Rust Async Client Library  
**Document:** Per-Module Module Detailed Design Documents (Index)  
**Version:** 1.0  
**Date:** 2026-03-01  
**Author(s):** <TBD>

[Download this document](<DOWNLOAD_LINK>)

# 2. Revision History

| Version | Date | Author | Notes |
|---|---|---|---|
| 1.0 | 2026-03-01 | <TBD> | Per-module decomposition generated from MDDD v1.5 (2026-03-01). |

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
  - [6.5 MDDD Section Digest & Coverage Map](#65-mddd-section-digest--coverage-map)
- [7. Architecture Overview](#7-architecture-overview)
  - [7.1 Component Diagram (textual)](#71-component-diagram-textual)
  - [7.2 Data Flow Overview](#72-data-flow-overview)
  - [7.3 High-Level Pipeline](#73-high-level-pipeline)
  - [7.4 Source Tree](#74-source-tree)
  - [7.5 Key Design Decisions & Trade-offs](#75-key-design-decisions--trade-offs)
- [8. Module Document Index](#8-module-document-index)
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

Design and specify an implementable, testable, async-first Rust client library for the Materials Project API, covering the full union of endpoints present in:
- official public documentation (“Getting Started” and “Examples”), and
- the provided OpenAPI specification snapshot.

This index decomposes the upstream MDDD (v1.5, 2026-03-01) into independently implementable modules with explicit responsibility contracts and test traceability.

## 4.2 Scope / Out of Scope

**In Scope**
- All route groups present in OpenAPI (materials, molecules, defects, DOI, and any additional groups present in the OpenAPI snapshot).
- Async-only API surface (Tokio runtime).
- Capability parity with official documentation; no requirement to match Python client API shape.

**Out of Scope**
- Exact reproduction of Python/pymatgen runtime objects.
- Blocking/synchronous client.

## 4.3 Definitions & Acronyms

This design adopts the upstream MDDD terminology, including: OpenAPI, route group, response envelope, doc-driven contract, raw JSON mode, contract test, inventory item, RTM, escape hatch, allow_insecure_http, timeout, concurrency, qps_limit, user_agent, correlation ID, full-jitter, token bucket, idempotent method, manifest test, crate baseline, parity test, and canonical JSON equivalence.

# 5. Design Philosophy

The per-module design applies the project’s architectural principles explicitly:

- **Modularity & Separation of Concerns:** modules are split into config, query, transport, middleware, data/envelope/models, routes (OpenAPI + doc-driven), convenience workflows, and tests. Every module has a SHALL/MUST responsibility contract.
- **Scalability:** stateless request execution with per-client concurrency and QPS limiting.
- **Maintainability:** OpenAPI-driven generation and contract tests reduce drift; deterministic serialization and error mapping stabilize behavior.
- **Security & Compliance:** HTTPS-by-default, explicit opt-in for insecure base URLs, and mandatory secret redaction in logs.
- **Performance & Reliability:** bounded retries with exponential backoff and jitter; shared underlying HTTP client to avoid unnecessary allocations.

# 6. Requirements Extraction Summary

## 6.1 Functional Requirements (FR-###)

|Req ID|RDD ID|Summary|
|---|---|---|
|FR-001|FR-COMMON-AUTH-001|Client MUST authenticate using API key header.|
|FR-002|FR-COMMON-PAGINATION-001|Client MUST support both page-based and offset-based pagination.|
|FR-003|FR-COMMON-PROJECTION-001|Client MUST support field projection.|
|FR-004|FR-COMMON-ENVELOPE-001|Client MUST parse the standard `{data, errors, meta}` response envelope.|
|FR-005|FR-COMMON-VALIDATION-001|Client MUST parse 422 validation errors into a typed error structure.|
|FR-006|FR-COMMON-ERROR_MODEL-001|Client MUST expose a typed error model that distinguishes configuration errors, request construction errors, transport errors, HTTP status errors, validation errors, and deserialization errors.|
|FR-007|FR-COMMON-OPENAPI_COVERAGE-001|For every OpenAPI operation enumerated in Appendix A-OpenAPI, the Rust client MUST provide an async method that issues the corresponding HTTP request and returns a parsed response envelope or a typed error.|
|FR-008|FR-DEF-TASKS-GET|Get DefectTaskDoc documents|
|FR-009|FR-DOI-ROOT-GET|Get DOIDoc documents|
|FR-010|FR-MAT-ABSORPTION-GET|Get AbsorptionDoc documents|
|FR-011|FR-MAT-ALLOYS-GET|Get AlloyPairDoc documents|
|FR-012|FR-MAT-BONDS-GET|Get BondingDoc documents|
|FR-013|FR-MAT-CHEMENV-GET|Get ChemEnvDoc documents|
|FR-014|FR-MAT-CONVERSION_ELECTRODES-GET|Get ConversionElectrodeDoc documents|
|FR-015|FR-MAT-CORE-GET|Get MaterialsDoc documents|
|FR-016|FR-MAT-CORE_BLESSED_TASKS-GET|Get MaterialsDoc documents|
|FR-017|FR-MAT-CORE_FIND_STRUCTURE-POST|Post FindStructure documents|
|FR-018|FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET|Get FormulaAutocomplete documents|
|FR-019|FR-MAT-DIELECTRIC-GET|Get DielectricDoc documents|
|FR-020|FR-MAT-ELASTICITY-GET|Get ElasticityDoc documents|
|FR-021|FR-MAT-ELECTRONIC_STRUCTURE-GET|Get ElectronicStructureDoc documents|
|FR-022|FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET|Get ElectronicStructureDoc documents|
|FR-023|FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET|Get ElectronicStructureDoc documents|
|FR-024|FR-MAT-EOS-GET|Get EOSDoc documents|
|FR-025|FR-MAT-FERMI-GET|Get FermiDoc documents|
|FR-026|FR-MAT-GRAIN_BOUNDARIES-GET|Get GrainBoundaryDoc documents|
|FR-027|FR-MAT-INSERTION_ELECTRODES-GET|Get InsertionElectrodeDoc documents|
|FR-028|FR-MAT-MAGNETISM-GET|Get MagnetismDoc documents|
|FR-029|FR-MAT-OXIDATION_STATES-GET|Get OxidationStateDoc documents|
|FR-030|FR-MAT-PHONON-GET|Get PhononBSDOSDoc documents|
|FR-031|FR-MAT-PIEZOELECTRIC-GET|Get PiezoelectricDoc documents|
|FR-032|FR-MAT-PROVENANCE-GET|Get ProvenanceDoc documents|
|FR-033|FR-MAT-ROBOCRYS-GET|Get RobocrystallogapherDoc documents|
|FR-034|FR-MAT-ROBOCRYS_TEXT_SEARCH-GET|Get RobocrystallogapherDoc documents|
|FR-035|FR-MAT-SIMILARITY-GET|Get SimilarityDoc documents|
|FR-036|FR-MAT-SIMILARITY_MATCH-GET|Get SimilarityDoc documents|
|FR-037|FR-MAT-SUBSTRATES-GET|Get SubstratesDoc documents|
|FR-038|FR-MAT-SUMMARY-GET|Get SummaryDoc documents|
|FR-039|FR-MAT-SURFACE_PROPERTIES-GET|Get SurfacePropDoc documents|
|FR-040|FR-MAT-SYNTHESIS-GET|Get SynthesisSearchResultModel documents|
|FR-041|FR-MAT-TASKS-GET|Get TaskDoc documents|
|FR-042|FR-MAT-TASKS_DEPRECATION-GET|Get DeprecationDoc documents|
|FR-043|FR-MAT-TASKS_ENTRIES-GET|Get EntryDoc documents|
|FR-044|FR-MAT-TASKS_TRAJECTORY-GET|Get TrajectoryDoc documents|
|FR-045|FR-MAT-THERMO-GET|Get ThermoDoc documents|
|FR-046|FR-MAT-XAS-GET|Get XASDoc documents|
|FR-047|FR-MOL-JCESR-GET|Get MoleculesDoc documents|
|FR-048|FR-MOL-SUMMARY-GET|Get MoleculeSummaryDoc documents|
|FR-049|FR-COMMON-DOC_DRIVEN-001|Client MUST implement endpoints listed in official docs (e.g., “Getting Started” endpoint table) but absent from the uploaded OpenAPI, using a doc-driven contract.|
|FR-050|FR-MOL-ASSOC-GET|Client MUST support searching associated molecule documents via `/molecules/assoc`.|
|FR-051|FR-MOL-BONDING-GET|Client MUST support searching molecule bonding documents via `/molecules/bonding`.|
|FR-052|FR-MOL-CORE-GET|Client MUST support searching molecule core documents via `/molecules/core`.|
|FR-053|FR-MOL-ORBITALS-GET|Client MUST support searching orbital documents via `/molecules/orbitals`.|
|FR-054|FR-MOL-PARTIAL_CHARGES-GET|Client MUST support searching partial charge documents via `/molecules/partial_charges`.|
|FR-055|FR-MOL-PARTIAL_SPINS-GET|Client MUST support searching partial spin documents via `/molecules/partial_spins`.|
|FR-056|FR-MOL-REDOX-GET|Client MUST support searching redox documents via `/molecules/redox`.|
|FR-057|FR-MOL-TASKS-GET|Client MUST support searching molecule task documents via `/molecules/tasks`.|
|FR-058|FR-MOL-THERMO-GET|Client MUST support searching molecule thermochemistry documents via `/molecules/thermo`.|
|FR-059|FR-MOL-VIBRATIONS-GET|Client MUST support searching vibration documents via `/molecules/vibrations`.|
|FR-060|FR-COMMON-CONVENIENCE-001|The Rust client MUST provide convenience wrappers matching the official Examples workflows (idiomatic Rust names allowed).|
|FR-061|FR-MPR-GET_STRUCTURE_BY_MATERIAL_ID|Get a structure for a material_id (wraps `GET /materials/summary/` with projected field `structure`).|
|FR-062|FR-MPR-GET_TASK_IDS_ASSOCIATED_WITH_MATERIAL_ID|Get task IDs associated with a material_id (wraps `GET /materials/summary/` with projected field `task_ids`).|
|FR-063|FR-MPR-GET_BANDSTRUCTURE_BY_MATERIAL_ID|Get electronic bandstructure for a material_id (wraps `GET /materials/summary/` with projected field `bandstructure`).|
|FR-064|FR-MPR-GET_DOS_BY_MATERIAL_ID|Get electronic DOS for a material_id (wraps `GET /materials/summary/` with projected field `dos`).|
|FR-065|FR-MPR-GET_PHONON_BANDSTRUCTURE_BY_MATERIAL_ID|Get phonon bandstructure for a material_id (wraps phonon endpoint).|
|FR-066|FR-MPR-GET_PHONON_DOS_BY_MATERIAL_ID|Get phonon DOS for a material_id (wraps phonon endpoint).|
|FR-067|FR-MPR-GET_CHARGE_DENSITY_FROM_MATERIAL_ID|Get charge density by material_id (Examples-driven).|
|FR-068|FR-MPR-GET_CHARGE_DENSITY_FROM_TASK_ID|Get charge density by task_id (Examples-driven).|
|FR-069|OPS-CONFIG-SOURCES-001|Client MUST support configuration from (1) explicit constructor/builder inputs and (2) environment variables, and MUST define precedence.|
|FR-070|OPS-SETTINGS-001|Client MUST support setting: `api_key`, `base_url`, `timeout`, `concurrency`, `qps_limit`, `user_agent`, and `allow_insecure_http` (see §9 security).|
|FR-071|USER-PYTHON-PARITY-COVERAGE-001|The Rust client MUST implement the full API surface of the Python `mp-api` client (behavioral parity; API shape may differ).|


## 6.2 Non-Functional Requirements (NFR-###)

|Req ID|RDD ID|Summary|
|---|---|---|
|NFR-001|NFR-PERF-RATE_LIMIT-001|Client MUST provide a configurable rate limiter and MUST default to **25 requests per second** (aggregate) unless explicitly overridden.|
|NFR-002|NFR-REL-RETRY-001|Client MUST implement retries with exponential backoff for transient failures, using a configurable policy with explicit defaults (max_retries=3, initial_backoff=200ms, max_backoff=2s, jitter=full-jitter).|
|NFR-003|NFR-OBS-LOGGING-001|Client MUST support structured logs including request path, status code, latency, retry count, and correlation ID, and MUST redact API keys and other secrets.|
|NFR-004|NFR-SEC-TLS-001|Client MUST use HTTPS by default and MUST reject non-HTTPS base URLs unless the caller explicitly opts in (for testing).|
|NFR-005|NFR-TEST-UNIT-001|Unit tests SHALL cover query serialization and error mapping.|
|NFR-006|NFR-TEST-CONTRACT-001|Contract tests SHALL validate request/response shapes against OpenAPI for every inventory operation in Appendix A-OpenAPI.|
|NFR-007|NFR-TEST-INTEGRATION-001|Integration smoke tests SHALL be skipped unless MP_API_KEY or PMG_MAPI_KEY is set; when set, they SHALL run as smoke tests.|
|NFR-008|NFR-BUILD-DEPS-001|The crate SHALL pin a reproducible crate baseline (versions + required features) for core runtime dependencies.|
|NFR-009|NFR-TEST-PYTHON-PARITY-001|When enabled, Python parity tests SHALL assert (a) full Python `mp-api` API surface coverage and (b) input/output equivalence between Rust and Python for every mapped API.|


## 6.3 Integration Requirements (IR-###)

The upstream MDDD defines no standalone IR-prefixed requirements; integration obligations are captured within FR/NFR/DR.

## 6.4 Data Requirements (DR-###)

|Req ID|RDD ID|Summary|
|---|---|---|
|DR-001|DR-ENVELOPE-001|Response envelope fields and schemas (data/errors/meta; Error{code,message}; Meta fields).|
|DR-002|DR-TYPING-001|Typed and raw JSON modes for response data.|
|DR-003|DR-SERDE-001|Serde policy for nullable and unknown fields.|


## 6.5 MDDD Section Digest & Coverage Map

### MDDD Read-Through Confirmation (Headings)

- # 1. Title Page
- # 2. Revision History
- # 3. Table of Contents
- # 4. Overview
- ## 4.1 Purpose
- ## 4.2 Scope / Out of Scope
- ## 4.3 Definitions & Acronyms
- # 5. Design Philosophy
- # 6. Requirements Extraction Summary
- ## 6.1 Functional Requirements (FR-###)
- ## 6.2 Non-Functional Requirements (NFR-###)
- ## 6.3 Integration Requirements (IR-###)
- ## 6.4 Data Requirements (DR-###)
- ## 6.5 RDD Section Digest & Coverage Map
- ### RDD Read-Through Confirmation (all RDD top-level sections found)
- ### Digest & Mapping (section-by-section)
- ## 6.6 Acceptance Criteria Catalog (Normalized REQ-IDs)
- ### 6.6.1 Functional Requirements
- ### 6.6.2 Operations & Configuration Requirements
- ### 6.6.3 Non-Functional Requirements
- ### 6.6.4 Data Requirements
- # 7. Architecture Overview
- ## 7.1 Component Diagram (textual)
- ## 7.2 Data Flow Overview
- ## 7.3 High-Level Pipeline
- ## 7.4 Source Tree
- ## 7.5 Key Design Decisions & Trade-offs (with requirement IDs)
- # 8. Module Detailed Design
- ## Client Facade (MpClient)
- ## Configuration (Config & Builder)
- ## Errors (MpApiError)
- ## Envelope & Models
- ## Query Parameters (Pagination/Projection/Escape Hatch)
- ## HTTP Transport
- ## Rate Limiter
- ## Retry Policy
- ## OpenAPI Routes
- ## Doc-Driven Routes
- ## Convenience Workflows
- ## Testing Harness
- # 9. Cross-cutting Concerns
- ## 9.1 Observability (logs/metrics/traces)
- ## 9.2 Security & Privacy
- ## 9.3 Performance & Capacity
- ## 9.4 Deployment & Configuration
- ## 9.5 Backward Compatibility / Migration / Rollback
- # 10. Requirements Traceability Matrix
- # 11. Assumptions
- # 12. Open Questions
- # 13. Final Self-Check

### Module List (as stated in upstream MDDD §8)

- Client Facade (MpClient)
- Configuration (Config & Builder)
- Errors (MpApiError)
- Envelope & Models
- Query Parameters (Pagination/Projection/Escape Hatch)
- HTTP Transport
- Rate Limiter
- Retry Policy
- OpenAPI Routes
- Doc-Driven Routes
- Convenience Workflows
- Testing Harness

### Digest & Mapping (section-by-section)

|MDDD Section|Digest|Incorporated In|
|---|---|---|
|1. Title Page|**System Name:** Materials Project Rust Async Client Library **Document:** Module Detailed Design Document **Version:** 1.5 **Date:** 2026-03-01 **Last Updated (JST):** 2026-03-01 11:30 **Author(s):** mp-api-rs design wo…|modules/README.md: §1 Title Page|
|2. Revision History|See section content.|modules/README.md: §2 Revision History|
|3. Table of Contents|[1. Title Page](#1-title-page) [2. Revision History](#2-revision-history) [3. Table of Contents](#3-table-of-contents) [4. Overview](#4-overview) [4.1 Purpose](#41-purpose) [4.2 Scope / Out of Scope](#42-scope--out-of-sc…|modules/README.md: §3 Table of Contents|
|4. Overview|See section content.|modules/README.md: §4 Overview|
|4.1 Purpose|Design and specify an implementable, testable, async-first Rust client library for the Materials Project API, covering the full union of (a) endpoints in official Getting Started documentation and (b) endpoints in the up…|modules/README.md: §4 Overview|
|4.2 Scope / Out of Scope|**In Scope** All routes (materials, molecules, defects, DOI, and any additional groups present in OpenAPI). Functional coverage parity with the Python `mp-api` client: every API exposed by Python `mp-api` MUST have a Rus…|modules/README.md: §4 Overview|
|4.3 Definitions & Acronyms|(Directly adopted from RDD §3) **OpenAPI**: uploaded `openapi.json` (derived from [api.materialsproject.org/docs](https://api.materialsproject.org/docs)) **Route group**: logical grouping such as `materials.summary`, `ma…|modules/README.md: §4 Overview|
|5. Design Philosophy|This design follows the architectural principles mandated by the prompt and aligns them to RDD requirements. 1. **Modularity & Separation of Concerns** Decompose into explicit modules: config, auth, query building, trans…|modules/README.md: §5 Design Philosophy|
|6. Requirements Extraction Summary|This section enumerates **all normative requirements** from RDD §6, §9, §10 and additionally extracts explicit data/test requirements from RDD §7 and §11.|modules/README.md: §6 Requirements Extraction Summary|
|6.1 Functional Requirements (FR-###)|Each row maps an internal requirement ID (FR-###) to the RDD requirement ID and summary.|modules/README.md: TBD|
|6.2 Non-Functional Requirements (NFR-###)|See section content.|modules/README.md: TBD|
|6.3 Integration Requirements (IR-###)|The RDD does not define standalone IR-prefixed requirements. Interface/integration obligations are captured within FR (e.g., auth header, base URL) and OPS/NFR requirements. **Result:** No independent IR-### requirements…|modules/README.md: TBD|
|6.4 Data Requirements (DR-###)|See section content.|modules/README.md: TBD|
|6.5 RDD Section Digest & Coverage Map|See section content.|modules/README.md: TBD|
|RDD Read-Through Confirmation (all RDD top-level sections found)|## 1) Background & Objectives ## 2) Scope & Assumptions ## 3) Terminology ## 4) Use Cases (All adopted) ## 5) System Boundary & Context ## 6) Functional Requirements ## 7) Data Requirements ## 8) Rust Client Design Requi…|modules/README.md: TBD|
|Digest & Mapping (section-by-section)|See section content.|modules/README.md: TBD|
|6.6 Acceptance Criteria Catalog (Normalized REQ-IDs)|This catalog adds stable requirement IDs (`REQ-*`) and testable acceptance criteria for every extracted FR/NFR/DR row. Each `REQ-*` maps to exactly one source row unless explicitly noted. Rationale/justification is via t…|modules/README.md: TBD|
|6.6.1 Functional Requirements|**REQ-F-001** (Source: `FR-001` / RDD ID: `FR-COMMON-AUTH-001`) Requirement: Client MUST authenticate using API key header. Acceptance Criteria: AC-1: Given a built `MpClient` with an API key, when any HTTP request is ex…|modules/README.md: TBD|
|6.6.2 Operations & Configuration Requirements|**REQ-OPS-001** (Source: `FR-069` / RDD ID: `OPS-CONFIG-SOURCES-001`) Requirement: Client MUST support configuration from (1) explicit constructor/builder inputs and (2) environment variables, and MUST define precedence.…|modules/README.md: TBD|
|6.6.3 Non-Functional Requirements|**REQ-NF-001** (Source: `NFR-001` / RDD ID: `NFR-PERF-RATE_LIMIT-001`) Requirement: Client MUST provide a configurable rate limiter and MUST default to **25 requests per second** (aggregate) unless explicitly overridden.…|modules/README.md: TBD|
|6.6.4 Data Requirements|**REQ-DATA-001** (Source: `DR-001` / RDD ID: `DR-ENVELOPE-001`) Requirement: Response envelope fields and schemas (data/errors/meta; Error{code,message}; Meta fields). Acceptance Criteria: AC-1: Response envelope struct …|modules/README.md: TBD|
|7. Architecture Overview|See section content.|modules/README.md: §7 Architecture Overview|
|7.1 Component Diagram (textual)|See section content.|modules/README.md: TBD|
|7.2 Data Flow Overview|1. Caller constructs `MpClient` via builder/environment config. 2. Caller invokes an OpenAPI route method (typed) or a doc-driven route method (raw JSON). 3. Route method: builds query pairs (pagination/projection/escape…|modules/README.md: TBD|
|7.3 High-Level Pipeline|End-to-end processing pipeline (I/O and requirement linkage): 1. **Config resolution** Input: builder settings + env vars → Output: validated `Config` Requirements: FR-069, FR-070, FR-001, NFR-004 2. **Route selection** …|modules/README.md: TBD|
|7.4 Source Tree|Proposed repository layout (modules mapped to directories):|modules/README.md: TBD|
|7.5 Key Design Decisions & Trade-offs (with requirement IDs)|1. **OpenAPI-driven code generation** (vs. hand-written route/modeled types) Rationale: prevents drift and supports 1:1 coverage + contract tests. Requirements: FR-007, NFR-006. 2. **Doc-driven endpoints return raw JSON*…|modules/README.md: TBD|
|8. Module Detailed Design|See section content.|modules/README.md: §8 Module Document Index + module files §6|
|Client Facade (MpClient)|Purpose Provide the primary async Rust entrypoint for the Materials Project API, exposing nested route clients and convenience workflows while centralizing shared behavior (auth, config, transport, and middleware wiring)…|modules/client-facade.md: §6 "Client Facade (MpClient)"|
|Configuration (Config & Builder)|Purpose Centralize configuration source resolution, defaults, and validation. Files (responsibility & description) `src/config.rs`: Configuration resolution, defaults, validation, and environment-variable integration. Ty…|modules/configuration.md: §6 "Configuration (Config & Builder)"|
|Errors (MpApiError)|Purpose Provide a single stable error type and deterministic error mapping across configuration, request building, transport, HTTP status handling, and deserialization. Files (responsibility & description) `src/error.rs`…|modules/errors.md: §6 "Errors (MpApiError)"|
|Envelope & Models|Purpose Define core data types used by all endpoints: the response envelope, error/meta structures, and model generation policy. Files (responsibility & description) `src/data/mod.rs`: Data-layer module root; re-exports …|modules/envelope-models.md: §6 "Envelope & Models"|
|Query Parameters (Pagination/Projection/Escape Hatch)|Purpose Provide a consistent, validated, and testable mechanism for building query parameters (pagination, field projection, and escape-hatch parameters). Files (responsibility & description) `src/query/mod.rs`: Query mo…|modules/query-parameters.md: §6 "Query Parameters (Pagination/Projection/Escape Hatch)"|
|HTTP Transport|Purpose Execute HTTP requests with consistent behavior: base URL resolution, headers, timeouts, concurrency caps, QPS limiting, retries, response capture, and observability. Files (responsibility & description) `src/tran…|modules/http-transport.md: §6 "HTTP Transport"|
|Rate Limiter|Purpose Enforce a per-client aggregate requests-per-second (QPS) limit to prevent server overload and comply with published guidance. Files (responsibility & description) `src/middleware/mod.rs`: Middleware module root; …|modules/rate-limiter.md: §6 "Rate Limiter"|
|Retry Policy|Purpose Improve reliability by retrying transient failures with exponential backoff while avoiding unsafe retries for non-transient errors. Files (responsibility & description) `src/middleware/mod.rs`: Middleware module …|modules/retry-policy.md: §6 "Retry Policy"|
|OpenAPI Routes|Purpose Provide typed, async methods for every OpenAPI operation enumerated in Appendix A-OpenAPI, with consistent application of cross-cutting behavior. Files (responsibility & description) `src/routes/mod.rs`: Routes m…|modules/openapi-routes.md: §6 "OpenAPI Routes"|
|Doc-Driven Routes|Purpose Implement endpoints listed in official documentation (Getting Started) that are missing from the uploaded OpenAPI, using a doc-driven contract and returning raw JSON. Files (responsibility & description) `src/rou…|modules/doc-driven-routes.md: §6 "Doc-Driven Routes"|
|Convenience Workflows|Purpose Provide ergonomic, examples-driven helper methods that compose underlying route calls and field projection to match common user workflows from official documentation. Files (responsibility & description) `src/con…|modules/convenience-workflows.md: §6 "Convenience Workflows"|
|Testing Harness|Purpose Provide deterministic automated verification that the client satisfies coverage and contract requirements: per-operation contract tests, inventory cross-checks, and opt-in integration smoke tests. Files (responsi…|modules/testing-harness.md: §6 "Testing Harness"|
|9. Cross-cutting Concerns|See section content.|modules/README.md: §9 Cross-cutting Concerns + relevant module docs §6/§9|
|9.1 Observability (logs/metrics/traces)|Use `tracing` spans for each request; include: correlation_id method, path status latency_ms retry_count Redaction is mandatory for secrets (API keys). Requirement: NFR-003|modules/README.md: TBD|
|9.2 Security & Privacy|HTTPS enforced by default; `allow_insecure_http` is explicitly opt-in and intended only for local testing harnesses. Requirement: NFR-004 API keys are treated as secrets: never printed in logs excluded from `Debug` outpu…|modules/README.md: TBD|
|9.3 Performance & Capacity|Default QPS limit of 25 requests/sec; configurable. Requirement: NFR-001 Concurrency cap enforced via semaphore; default `concurrency=16` (see §11 Assumptions). Retry policy uses exponential backoff with jitter, bounded …|modules/README.md: TBD|
|9.4 Deployment & Configuration|Configuration sources and precedence are explicitly defined by OPS-CONFIG-SOURCES-001. All settings enumerated in OPS-SETTINGS-001 are supported; settings are applied at build time and stored in immutable `Config`.|modules/README.md: TBD|
|9.5 Backward Compatibility / Migration / Rollback|Model structs allow unknown fields to tolerate server-side schema expansion (DR-003). Typed mode may introduce compile-time breaking changes when OpenAPI changes; mitigate by: versioning the crate according to Semantic V…|modules/README.md: TBD|
|10. Requirements Traceability Matrix|See section content.|modules/README.md: §10 Requirements Traceability Matrix|
|11. Assumptions|(Assumptions/proposals are directly reflected from RDD §2 unless noted.) 1. The uploaded OpenAPI file is authoritative for endpoint paths/params and response envelopes. 2. `allow_insecure_http` is supported (default `fal…|modules/README.md: §11 Assumptions|
|12. Open Questions|1. **OpenAPI code generation approach:** Which generator strategy will be used (build-time Rust codegen vs. checked-in generated code vs. handwritten wrappers)? The design assumes code generation but does not mandate a s…|modules/README.md: §12 Open Questions|
|13. Final Self-Check|English-only document content (code identifiers/proper nouns allowed): **Yes** Single Markdown file output: **Yes** Table of Contents present: **Yes** Design Philosophy present: **Yes** High-Level Pipeline present: **Yes…|modules/README.md: §13 Final Self-Check|


# 7. Architecture Overview

## 7.1 Component Diagram (textual)

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

## 7.5 Key Design Decisions & Trade-offs

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

# 8. Module Document Index

|Module name|File name|Short description|Primary requirement IDs covered|
|---|---|---|---|
|Client Facade (MpClient)|modules/client-facade.md|Public crate entrypoint and builder; wires config, transport, middleware, and route roots.|FR-069, FR-070, FR-001, NFR-003, NFR-004|
|Configuration (Config & Builder)|modules/configuration.md|Settings resolution (builder + env), defaults, and validation.|FR-069, FR-070, NFR-001, NFR-004|
|Errors (MpApiError)|modules/errors.md|Typed error taxonomy and deterministic mapping (config/request/transport/http/validation/deserialization).|FR-005, FR-006|
|Envelope & Models|modules/envelope-models.md|Response envelope and OpenAPI-derived model types; raw JSON compatibility policy.|FR-004, DR-001, DR-002, DR-003|
|Query Parameters (Pagination/Projection/Escape Hatch)|modules/query-parameters.md|Deterministic query building with pagination, projection, and escape-hatch parameters.|FR-002, FR-003, FR-049|
|HTTP Transport|modules/http-transport.md|Reqwest-backed HTTP execution with timeouts, concurrency, rate limiting, retries, and observability.|FR-001, FR-070, NFR-003, NFR-004|
|Rate Limiter|modules/rate-limiter.md|Per-client token-bucket QPS limiter with deterministic behavior.|NFR-001|
|Retry Policy|modules/retry-policy.md|Retry classifier + exponential backoff with full jitter for transient failures.|NFR-002|
|OpenAPI Routes|modules/openapi-routes.md|Typed route clients generated from OpenAPI with 1:1 operation coverage.|FR-007, FR-008..FR-048|
|Doc-Driven Routes|modules/doc-driven-routes.md|Endpoints absent from OpenAPI, implemented via doc-driven contract and raw JSON.|FR-049..FR-059|
|Convenience Workflows|modules/convenience-workflows.md|Examples-driven thin wrappers for common workflows using projection and underlying routes.|FR-060..FR-068|
|Testing Harness|modules/testing-harness.md|Unit + contract + integration + Python parity tests, plus coverage and dependency baseline checks.|NFR-005..NFR-009, FR-071|


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

|Requirement ID|Requirement Text|Module File(s)|Design Section|Test Case IDs|Coverage Status|
|---|---|---|---|---|---|
|FR-001|Client MUST authenticate using API key header. (RDD: FR-COMMON-AUTH-001)|modules/client-facade.md, modules/configuration.md, modules/http-transport.md, modules/errors.md|client-facade.md §6 "Client Facade (MpClient)"; configuration.md §6 "Configuration (Config & Builder)"; http-transport.md §6 "HTTP Transport"; errors.md §6 "Errors (MpApiError)"|UT-FR-001|Covered|
|FR-002|Client MUST support both page-based and offset-based pagination. (RDD: FR-COMMON-PAGINATION-001)|modules/query-parameters.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/errors.md|query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; errors.md §6 "Errors (MpApiError)"|UT-FR-002|Covered|
|FR-003|Client MUST support field projection. (RDD: FR-COMMON-PROJECTION-001)|modules/query-parameters.md, modules/openapi-routes.md, modules/doc-driven-routes.md|query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"|UT-FR-003|Covered|
|FR-004|Client MUST parse the standard `{data, errors, meta}` response envelope. (RDD: FR-COMMON-ENVELOPE-001)|modules/envelope-models.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/errors.md|envelope-models.md §6 "Envelope & Models"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; errors.md §6 "Errors (MpApiError)"|UT-FR-004|Covered|
|FR-005|Client MUST parse 422 validation errors into a typed error structure. (RDD: FR-COMMON-VALIDATION-001)|modules/errors.md, modules/openapi-routes.md, modules/doc-driven-routes.md|errors.md §6 "Errors (MpApiError)"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"|UT-FR-005|Covered|
|FR-006|Client MUST expose a typed error model that distinguishes configuration errors, request construction errors, transport errors, HTTP status errors, validation errors, and deserialization errors. (RDD: FR-COMMON-ERROR_MODEL-001)|modules/errors.md, modules/retry-policy.md, modules/http-transport.md, modules/client-facade.md|errors.md §6 "Errors (MpApiError)"; retry-policy.md §6 "Retry Policy"; http-transport.md §6 "HTTP Transport"; client-facade.md §6 "Client Facade (MpClient)"|UT-FR-006|Covered|
|FR-007|For every OpenAPI operation enumerated in Appendix A-OpenAPI, the Rust client MUST provide an async method that issues the corresponding HTTP request and returns a parsed response envelope or a typed error. (RDD: FR-COMMON-OPENAPI_COVERAGE-001)|modules/openapi-routes.md, modules/testing-harness.md, modules/client-facade.md|openapi-routes.md §6 "OpenAPI Routes"; testing-harness.md §6 "Testing Harness"; client-facade.md §6 "Client Facade (MpClient)"|CT-MANIFEST-001, UT-INVENTORY-001|Covered|
|FR-008|Get DefectTaskDoc documents (RDD: FR-DEF-TASKS-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-DEF-TASKS-GET|Covered|
|FR-009|Get DOIDoc documents (RDD: FR-DOI-ROOT-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-DOI-ROOT-GET|Covered|
|FR-010|Get AbsorptionDoc documents (RDD: FR-MAT-ABSORPTION-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-ABSORPTION-GET|Covered|
|FR-011|Get AlloyPairDoc documents (RDD: FR-MAT-ALLOYS-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-ALLOYS-GET|Covered|
|FR-012|Get BondingDoc documents (RDD: FR-MAT-BONDS-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-BONDS-GET|Covered|
|FR-013|Get ChemEnvDoc documents (RDD: FR-MAT-CHEMENV-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-CHEMENV-GET|Covered|
|FR-014|Get ConversionElectrodeDoc documents (RDD: FR-MAT-CONVERSION_ELECTRODES-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-CONVERSION_ELECTRODES-GET|Covered|
|FR-015|Get MaterialsDoc documents (RDD: FR-MAT-CORE-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-CORE-GET|Covered|
|FR-016|Get MaterialsDoc documents (RDD: FR-MAT-CORE_BLESSED_TASKS-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-CORE_BLESSED_TASKS-GET|Covered|
|FR-017|Post FindStructure documents (RDD: FR-MAT-CORE_FIND_STRUCTURE-POST)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-CORE_FIND_STRUCTURE-POST|Covered|
|FR-018|Get FormulaAutocomplete documents (RDD: FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET|Covered|
|FR-019|Get DielectricDoc documents (RDD: FR-MAT-DIELECTRIC-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-DIELECTRIC-GET|Covered|
|FR-020|Get ElasticityDoc documents (RDD: FR-MAT-ELASTICITY-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-ELASTICITY-GET|Covered|
|FR-021|Get ElectronicStructureDoc documents (RDD: FR-MAT-ELECTRONIC_STRUCTURE-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-ELECTRONIC_STRUCTURE-GET|Covered|
|FR-022|Get ElectronicStructureDoc documents (RDD: FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET|Covered|
|FR-023|Get ElectronicStructureDoc documents (RDD: FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET|Covered|
|FR-024|Get EOSDoc documents (RDD: FR-MAT-EOS-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-EOS-GET|Covered|
|FR-025|Get FermiDoc documents (RDD: FR-MAT-FERMI-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-FERMI-GET|Covered|
|FR-026|Get GrainBoundaryDoc documents (RDD: FR-MAT-GRAIN_BOUNDARIES-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-GRAIN_BOUNDARIES-GET|Covered|
|FR-027|Get InsertionElectrodeDoc documents (RDD: FR-MAT-INSERTION_ELECTRODES-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-INSERTION_ELECTRODES-GET|Covered|
|FR-028|Get MagnetismDoc documents (RDD: FR-MAT-MAGNETISM-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-MAGNETISM-GET|Covered|
|FR-029|Get OxidationStateDoc documents (RDD: FR-MAT-OXIDATION_STATES-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-OXIDATION_STATES-GET|Covered|
|FR-030|Get PhononBSDOSDoc documents (RDD: FR-MAT-PHONON-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-PHONON-GET|Covered|
|FR-031|Get PiezoelectricDoc documents (RDD: FR-MAT-PIEZOELECTRIC-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-PIEZOELECTRIC-GET|Covered|
|FR-032|Get ProvenanceDoc documents (RDD: FR-MAT-PROVENANCE-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-PROVENANCE-GET|Covered|
|FR-033|Get RobocrystallogapherDoc documents (RDD: FR-MAT-ROBOCRYS-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-ROBOCRYS-GET|Covered|
|FR-034|Get RobocrystallogapherDoc documents (RDD: FR-MAT-ROBOCRYS_TEXT_SEARCH-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-ROBOCRYS_TEXT_SEARCH-GET|Covered|
|FR-035|Get SimilarityDoc documents (RDD: FR-MAT-SIMILARITY-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-SIMILARITY-GET|Covered|
|FR-036|Get SimilarityDoc documents (RDD: FR-MAT-SIMILARITY_MATCH-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-SIMILARITY_MATCH-GET|Covered|
|FR-037|Get SubstratesDoc documents (RDD: FR-MAT-SUBSTRATES-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-SUBSTRATES-GET|Covered|
|FR-038|Get SummaryDoc documents (RDD: FR-MAT-SUMMARY-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-SUMMARY-GET|Covered|
|FR-039|Get SurfacePropDoc documents (RDD: FR-MAT-SURFACE_PROPERTIES-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-SURFACE_PROPERTIES-GET|Covered|
|FR-040|Get SynthesisSearchResultModel documents (RDD: FR-MAT-SYNTHESIS-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-SYNTHESIS-GET|Covered|
|FR-041|Get TaskDoc documents (RDD: FR-MAT-TASKS-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-TASKS-GET|Covered|
|FR-042|Get DeprecationDoc documents (RDD: FR-MAT-TASKS_DEPRECATION-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-TASKS_DEPRECATION-GET|Covered|
|FR-043|Get EntryDoc documents (RDD: FR-MAT-TASKS_ENTRIES-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-TASKS_ENTRIES-GET|Covered|
|FR-044|Get TrajectoryDoc documents (RDD: FR-MAT-TASKS_TRAJECTORY-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-TASKS_TRAJECTORY-GET|Covered|
|FR-045|Get ThermoDoc documents (RDD: FR-MAT-THERMO-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-THERMO-GET|Covered|
|FR-046|Get XASDoc documents (RDD: FR-MAT-XAS-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MAT-XAS-GET|Covered|
|FR-047|Get MoleculesDoc documents (RDD: FR-MOL-JCESR-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MOL-JCESR-GET|Covered|
|FR-048|Get MoleculeSummaryDoc documents (RDD: FR-MOL-SUMMARY-GET)|modules/openapi-routes.md, modules/envelope-models.md, modules/errors.md|openapi-routes.md §6 "OpenAPI Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|CT-FR-MOL-SUMMARY-GET|Covered|
|FR-049|Client MUST implement endpoints listed in official docs (e.g., “Getting Started” endpoint table) but absent from the uploaded OpenAPI, using a doc-driven contract. (RDD: FR-COMMON-DOC_DRIVEN-001)|modules/doc-driven-routes.md, modules/query-parameters.md, modules/envelope-models.md, modules/errors.md|doc-driven-routes.md §6 "Doc-Driven Routes"; query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|DT-MANIFEST-001, UT-FR-049|Covered|
|FR-050|Client MUST support searching associated molecule documents via `/molecules/assoc`. (RDD: FR-MOL-ASSOC-GET)|modules/doc-driven-routes.md, modules/query-parameters.md, modules/envelope-models.md, modules/errors.md|doc-driven-routes.md §6 "Doc-Driven Routes"; query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|DT-FR-MOL-ASSOC-GET|Covered|
|FR-051|Client MUST support searching molecule bonding documents via `/molecules/bonding`. (RDD: FR-MOL-BONDING-GET)|modules/doc-driven-routes.md, modules/query-parameters.md, modules/envelope-models.md, modules/errors.md|doc-driven-routes.md §6 "Doc-Driven Routes"; query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|DT-FR-MOL-BONDING-GET|Covered|
|FR-052|Client MUST support searching molecule core documents via `/molecules/core`. (RDD: FR-MOL-CORE-GET)|modules/doc-driven-routes.md, modules/query-parameters.md, modules/envelope-models.md, modules/errors.md|doc-driven-routes.md §6 "Doc-Driven Routes"; query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|DT-FR-MOL-CORE-GET|Covered|
|FR-053|Client MUST support searching orbital documents via `/molecules/orbitals`. (RDD: FR-MOL-ORBITALS-GET)|modules/doc-driven-routes.md, modules/query-parameters.md, modules/envelope-models.md, modules/errors.md|doc-driven-routes.md §6 "Doc-Driven Routes"; query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|DT-FR-MOL-ORBITALS-GET|Covered|
|FR-054|Client MUST support searching partial charge documents via `/molecules/partial_charges`. (RDD: FR-MOL-PARTIAL_CHARGES-GET)|modules/doc-driven-routes.md, modules/query-parameters.md, modules/envelope-models.md, modules/errors.md|doc-driven-routes.md §6 "Doc-Driven Routes"; query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|DT-FR-MOL-PARTIAL_CHARGES-GET|Covered|
|FR-055|Client MUST support searching partial spin documents via `/molecules/partial_spins`. (RDD: FR-MOL-PARTIAL_SPINS-GET)|modules/doc-driven-routes.md, modules/query-parameters.md, modules/envelope-models.md, modules/errors.md|doc-driven-routes.md §6 "Doc-Driven Routes"; query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|DT-FR-MOL-PARTIAL_SPINS-GET|Covered|
|FR-056|Client MUST support searching redox documents via `/molecules/redox`. (RDD: FR-MOL-REDOX-GET)|modules/doc-driven-routes.md, modules/query-parameters.md, modules/envelope-models.md, modules/errors.md|doc-driven-routes.md §6 "Doc-Driven Routes"; query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|DT-FR-MOL-REDOX-GET|Covered|
|FR-057|Client MUST support searching molecule task documents via `/molecules/tasks`. (RDD: FR-MOL-TASKS-GET)|modules/doc-driven-routes.md, modules/query-parameters.md, modules/envelope-models.md, modules/errors.md|doc-driven-routes.md §6 "Doc-Driven Routes"; query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|DT-FR-MOL-TASKS-GET|Covered|
|FR-058|Client MUST support searching molecule thermochemistry documents via `/molecules/thermo`. (RDD: FR-MOL-THERMO-GET)|modules/doc-driven-routes.md, modules/query-parameters.md, modules/envelope-models.md, modules/errors.md|doc-driven-routes.md §6 "Doc-Driven Routes"; query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|DT-FR-MOL-THERMO-GET|Covered|
|FR-059|Client MUST support searching vibration documents via `/molecules/vibrations`. (RDD: FR-MOL-VIBRATIONS-GET)|modules/doc-driven-routes.md, modules/query-parameters.md, modules/envelope-models.md, modules/errors.md|doc-driven-routes.md §6 "Doc-Driven Routes"; query-parameters.md §6 "Query Parameters (Pagination/Projection/Escape Hatch)"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|DT-FR-MOL-VIBRATIONS-GET|Covered|
|FR-060|The Rust client MUST provide convenience wrappers matching the official Examples workflows (idiomatic Rust names allowed). (RDD: FR-COMMON-CONVENIENCE-001)|modules/convenience-workflows.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/envelope-models.md, modules/errors.md|convenience-workflows.md §6 "Convenience Workflows"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|UT-FR-060|Covered|
|FR-061|Get a structure for a material_id (wraps `GET /materials/summary/` with projected field `structure`). (RDD: FR-MPR-GET_STRUCTURE_BY_MATERIAL_ID)|modules/convenience-workflows.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/envelope-models.md, modules/errors.md|convenience-workflows.md §6 "Convenience Workflows"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|UT-FR-061|Covered|
|FR-062|Get task IDs associated with a material_id (wraps `GET /materials/summary/` with projected field `task_ids`). (RDD: FR-MPR-GET_TASK_IDS_ASSOCIATED_WITH_MATERIAL_ID)|modules/convenience-workflows.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/envelope-models.md, modules/errors.md|convenience-workflows.md §6 "Convenience Workflows"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|UT-FR-062|Covered|
|FR-063|Get electronic bandstructure for a material_id (wraps `GET /materials/summary/` with projected field `bandstructure`). (RDD: FR-MPR-GET_BANDSTRUCTURE_BY_MATERIAL_ID)|modules/convenience-workflows.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/envelope-models.md, modules/errors.md|convenience-workflows.md §6 "Convenience Workflows"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|UT-FR-063|Covered|
|FR-064|Get electronic DOS for a material_id (wraps `GET /materials/summary/` with projected field `dos`). (RDD: FR-MPR-GET_DOS_BY_MATERIAL_ID)|modules/convenience-workflows.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/envelope-models.md, modules/errors.md|convenience-workflows.md §6 "Convenience Workflows"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|UT-FR-064|Covered|
|FR-065|Get phonon bandstructure for a material_id (wraps phonon endpoint). (RDD: FR-MPR-GET_PHONON_BANDSTRUCTURE_BY_MATERIAL_ID)|modules/convenience-workflows.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/envelope-models.md, modules/errors.md|convenience-workflows.md §6 "Convenience Workflows"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|UT-FR-065|Covered|
|FR-066|Get phonon DOS for a material_id (wraps phonon endpoint). (RDD: FR-MPR-GET_PHONON_DOS_BY_MATERIAL_ID)|modules/convenience-workflows.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/envelope-models.md, modules/errors.md|convenience-workflows.md §6 "Convenience Workflows"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|UT-FR-066|Covered|
|FR-067|Get charge density by material_id (Examples-driven). (RDD: FR-MPR-GET_CHARGE_DENSITY_FROM_MATERIAL_ID)|modules/convenience-workflows.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/envelope-models.md, modules/errors.md|convenience-workflows.md §6 "Convenience Workflows"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|UT-FR-067|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|
|FR-068|Get charge density by task_id (Examples-driven). (RDD: FR-MPR-GET_CHARGE_DENSITY_FROM_TASK_ID)|modules/convenience-workflows.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/envelope-models.md, modules/errors.md|convenience-workflows.md §6 "Convenience Workflows"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; envelope-models.md §6 "Envelope & Models"; errors.md §6 "Errors (MpApiError)"|UT-FR-068|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|
|FR-069|Client MUST support configuration from (1) explicit constructor/builder inputs and (2) environment variables, and MUST define precedence. (RDD: OPS-CONFIG-SOURCES-001)|modules/configuration.md, modules/http-transport.md, modules/rate-limiter.md, modules/retry-policy.md, modules/client-facade.md|configuration.md §6 "Configuration (Config & Builder)"; http-transport.md §6 "HTTP Transport"; rate-limiter.md §6 "Rate Limiter"; retry-policy.md §6 "Retry Policy"; client-facade.md §6 "Client Facade (MpClient)"|UT-FR-069|Covered|
|FR-070|Client MUST support setting: `api_key`, `base_url`, `timeout`, `concurrency`, `qps_limit`, `user_agent`, and `allow_insecure_http` (see §9 security). (RDD: OPS-SETTINGS-001)|modules/configuration.md, modules/http-transport.md, modules/rate-limiter.md, modules/retry-policy.md, modules/client-facade.md|configuration.md §6 "Configuration (Config & Builder)"; http-transport.md §6 "HTTP Transport"; rate-limiter.md §6 "Rate Limiter"; retry-policy.md §6 "Retry Policy"; client-facade.md §6 "Client Facade (MpClient)"|UT-FR-070|Covered|
|FR-071|The Rust client MUST implement the full API surface of the Python `mp-api` client (behavioral parity; API shape may differ). (RDD: USER-PYTHON-PARITY-COVERAGE-001)|modules/testing-harness.md, modules/openapi-routes.md, modules/doc-driven-routes.md, modules/convenience-workflows.md|testing-harness.md §6 "Testing Harness"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"; convenience-workflows.md §6 "Convenience Workflows"|PT-MANIFEST-001, PT-PY-ALL-001|Covered|
|NFR-001|Client MUST provide a configurable rate limiter and MUST default to **25 requests per second** (aggregate) unless explicitly overridden. (RDD: NFR-PERF-RATE_LIMIT-001)|modules/rate-limiter.md, modules/configuration.md, modules/http-transport.md, modules/client-facade.md|rate-limiter.md §6 "Rate Limiter"; configuration.md §6 "Configuration (Config & Builder)"; http-transport.md §6 "HTTP Transport"; client-facade.md §6 "Client Facade (MpClient)"|UT-NFR-001|Covered|
|NFR-002|Client MUST implement retries with exponential backoff for transient failures, using a configurable policy with explicit defaults (max_retries=3, initial_backoff=200ms, max_backoff=2s, jitter=full-jitter). (RDD: NFR-REL-RETRY-001)|modules/retry-policy.md, modules/configuration.md, modules/http-transport.md, modules/errors.md|retry-policy.md §6 "Retry Policy"; configuration.md §6 "Configuration (Config & Builder)"; http-transport.md §6 "HTTP Transport"; errors.md §6 "Errors (MpApiError)"|UT-NFR-002|Covered|
|NFR-003|Client MUST support structured logs including request path, status code, latency, retry count, and correlation ID, and MUST redact API keys and other secrets. (RDD: NFR-OBS-LOGGING-001)|modules/http-transport.md, modules/client-facade.md, modules/errors.md|http-transport.md §6 "HTTP Transport"; client-facade.md §6 "Client Facade (MpClient)"; errors.md §6 "Errors (MpApiError)"|UT-NFR-003|Covered|
|NFR-004|Client MUST use HTTPS by default and MUST reject non-HTTPS base URLs unless the caller explicitly opts in (for testing). (RDD: NFR-SEC-TLS-001)|modules/configuration.md, modules/http-transport.md, modules/errors.md|configuration.md §6 "Configuration (Config & Builder)"; http-transport.md §6 "HTTP Transport"; errors.md §6 "Errors (MpApiError)"|UT-NFR-004|Covered|
|NFR-005|Unit tests SHALL cover query serialization and error mapping. (RDD: NFR-TEST-UNIT-001)|modules/testing-harness.md|testing-harness.md §6 "Testing Harness"|CI-PIPELINE-001|Covered|
|NFR-006|Contract tests SHALL validate request/response shapes against OpenAPI for every inventory operation in Appendix A-OpenAPI. (RDD: NFR-TEST-CONTRACT-001)|modules/testing-harness.md|testing-harness.md §6 "Testing Harness"|CI-PIPELINE-001|Covered|
|NFR-007|Integration smoke tests SHALL be skipped unless MP_API_KEY or PMG_MAPI_KEY is set; when set, they SHALL run as smoke tests. (RDD: NFR-TEST-INTEGRATION-001)|modules/testing-harness.md|testing-harness.md §6 "Testing Harness"|CI-PIPELINE-001|Covered|
|NFR-008|The crate SHALL pin a reproducible crate baseline (versions + required features) for core runtime dependencies. (RDD: NFR-BUILD-DEPS-001)|modules/testing-harness.md, modules/configuration.md, modules/http-transport.md|testing-harness.md §6 "Testing Harness"; configuration.md §6 "Configuration (Config & Builder)"; http-transport.md §6 "HTTP Transport"|CI-DEPS-001|Covered|
|NFR-009|When enabled, Python parity tests SHALL assert (a) full Python `mp-api` API surface coverage and (b) input/output equivalence between Rust and Python for every mapped API. (RDD: NFR-TEST-PYTHON-PARITY-001)|modules/testing-harness.md|testing-harness.md §6 "Testing Harness"|PT-MANIFEST-001, PT-PY-ALL-001|Covered|
|DR-001|Response envelope fields and schemas (data/errors/meta; Error{code,message}; Meta fields). (RDD: DR-ENVELOPE-001)|modules/envelope-models.md, modules/openapi-routes.md, modules/doc-driven-routes.md|envelope-models.md §6 "Envelope & Models"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"|UT-DR-001|Covered|
|DR-002|Typed and raw JSON modes for response data. (RDD: DR-TYPING-001)|modules/envelope-models.md, modules/openapi-routes.md, modules/doc-driven-routes.md|envelope-models.md §6 "Envelope & Models"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"|UT-DR-002|Covered|
|DR-003|Serde policy for nullable and unknown fields. (RDD: DR-SERDE-001)|modules/envelope-models.md, modules/openapi-routes.md, modules/doc-driven-routes.md|envelope-models.md §6 "Envelope & Models"; openapi-routes.md §6 "OpenAPI Routes"; doc-driven-routes.md §6 "Doc-Driven Routes"|UT-DR-003|Covered|


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

Additional decomposition assumption for this per-module deliverable:
- The provided upstream MDDD v1.0 duplicates are treated as superseded by MDDD v1.5; any deltas are resolved in favor of the latest statements.

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
- Index file + per-module Markdown files created: **Yes**
- Table of Contents present (index + all modules): **Yes**
- Design Philosophy present (index): **Yes**
- High-Level Pipeline present (index): **Yes**
- Source Tree present (index): **Yes**
- Upstream MDDD headings listed and mapped in §6.5: **Yes**
- All requirement IDs (FR/NFR/DR) mapped to modules and tests in §10: **Yes**
- Any missing/ambiguous items recorded in §11–§12 with TBD markers: **Yes**
- Download link is Markdown link format in every file: **Yes**
