<!-- filename: modules/envelope-models.md -->

# 1. Title Page

**Module:** Envelope & Models  
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

Define core data types used by all endpoints: the response envelope, error/meta structures, and model generation policy.

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
|FR-004|Client MUST parse the standard `{data, errors, meta}` response envelope. (RDD: FR-COMMON-ENVELOPE-001)|UT-FR-004|Covered|
|DR-001|Response envelope fields and schemas (data/errors/meta; Error{code,message}; Meta fields). (RDD: DR-ENVELOPE-001)|UT-DR-001|Covered|
|DR-002|Typed and raw JSON modes for response data. (RDD: DR-TYPING-001)|UT-DR-002|Covered|
|DR-003|Serde policy for nullable and unknown fields. (RDD: DR-SERDE-001)|UT-DR-003|Covered|


Full coverage list (including supporting/cross-cutting participation) is enumerated in §7.

## 5.2 Not Covered Requirements (if any; reason)

_None._

## 5.3 Assumptions / TBDs (module-scoped)

- Error body truncation threshold is 8192 bytes (8 KiB) per upstream assumptions.
- Unknown JSON fields are tolerated for forward compatibility (DR-003).

# 6. Module Detailed Design

## Envelope & Models

### Purpose

Define core data types used by all endpoints: the response envelope, error/meta structures, and model generation policy.

### Responsibilities (explicit; MUST/SHALL statements; testable)

- The client SHALL parse the standard response envelope `{data, errors, meta}` into `Response<T>` (FR-004, DR-001).
  - The data layer SHALL provide both typed and raw JSON modes (DR-002).
  - Model structs SHALL support forward compatibility by allowing unknown fields and using `Option<T>` for nullable fields (DR-003).

### In-Scope / Out-of-Scope

TBD

### Inputs/Outputs (schemas, examples)

- Inputs: HTTP response JSON bytes.
  - Outputs:
    - `Response<T>` for typed operations
    - `Response<serde_json::Value>` or `serde_json::Value` for raw mode (doc-driven endpoints)

### Types & Definitions

#### `Response<T>`

- **Kind:** Generic envelope (DTO)
- **Purpose:** Canonical `{data, errors, meta}` response envelope in typed or raw JSON mode.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|data|Vec<T>|required|May be empty; for raw JSON mode T = serde_json::Value.|
|errors|Vec<ApiErrorItem>|required|May be empty on success.|
|meta|Meta|required|Metadata with forward-compatible extras.|

- **Serialization / Schema Notes:** Serde JSON; unknown fields ignored/tolerated (DR-003).
- **Versioning / Compatibility Notes:** Envelope shape is stable; additions should be optional.
- **Location:** src/data/envelope.rs
- **Related Requirement IDs:** FR-004, DR-001, DR-002, DR-003
- **Related Test Case IDs:** UT-FR-004, UT-DR-001, UT-DR-002, UT-DR-003

#### `ApiErrorItem`

- **Kind:** DTO (Envelope error item)
- **Purpose:** One API-reported error (code/message) inside the envelope.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|code|i32|required|Required by DR-001.|
|message|String|required|Required by DR-001.|

- **Serialization / Schema Notes:** Serde JSON.
- **Versioning / Compatibility Notes:** Forward compatible via optional additive fields.
- **Location:** src/data/envelope.rs
- **Related Requirement IDs:** FR-004, DR-001, DR-002, DR-003
- **Related Test Case IDs:** UT-FR-004, UT-DR-001, UT-DR-002, UT-DR-003

#### `Meta`

- **Kind:** DTO (Envelope meta)
- **Purpose:** Response metadata fields plus forward-compatible extras.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|api_version|Option<String>|optional|Nullable/optional.|
|time_stamp|Option<String>|optional|RFC 3339 date-time string; nullable/optional.|
|total_doc|Option<i64>|optional|Nullable/optional.|
|facet|Option<serde_json::Value>|optional|Free-form.|
|extra|serde_json::Map<String, serde_json::Value>|optional|Unknown meta fields (implementation strategy).|

- **Serialization / Schema Notes:** Serde JSON; unknown fields captured or ignored (DR-003).
- **Versioning / Compatibility Notes:** Forward compatible.
- **Location:** src/data/envelope.rs
- **Related Requirement IDs:** FR-004, DR-001, DR-002, DR-003
- **Related Test Case IDs:** UT-FR-004, UT-DR-001, UT-DR-002, UT-DR-003

#### `DefectTaskDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Defects task document (response item for /defects/tasks).
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|batch_id|string \| null|optional|Identifier for this calculation; should provide rough information about the calculation origin and purpose.|
|calc_type|string \| null|optional|The functional and task type used in the calculation.|
|completed_at|string \| null|optional|Timestamp for when this task was completed|
|dir_name|string \| null|optional|The directory for this VASP task|
|icsd_id|integer \| null|optional|Inorganic Crystal Structure Database id of the structure|
|input|CalculationInput \| null|optional|VASP calculation inputs|
|last_updated|string|optional|Timestamp for the most recent calculation for this task document|
|orig_inputs|CalculationInput \| null|optional|The exact set of input parameters used to generate the current task document.|
|output|OutputDoc \| null|optional|The exact set of output parameters used to generate the current task document.|
|run_type|string \| null|optional|The functional used in the calculation.|
|structure|TypedStructureDict \| null|optional|Final output structure from the task|
|tags|array<string> \| null|optional|Metadata tagged to a given task.|
|task_id|string \| string \| null|optional|The (task) ID of this calculation, used as a universal reference across property documents.This comes in the form: mp-******.|
|task_type|string \| null|optional|The type of calculation.|
|transformations|object \| null|optional|Information on the structural transformations, parsed from a transformations.json file|
|vasp_objects|object \| null|optional|Vasp objects associated with this task|
|additional_json|object \| null|optional|Additional json loaded from the calculation directory|
|analysis|AnalysisDoc \| null|optional|Some analysis of calculation data after collection.|
|author|string \| null|optional|Author extracted from transformations|
|calcs_reversed|array<Calculation> \| null|optional|Detailed data for each VASP calculation contributing to the task document.|
|custodian|array<CustodianDoc> \| null|optional|Detailed custodian data for each VASP calculation contributing to the task document.|
|entry|object \| null|optional|The ComputedEntry from the task doc|
|included_objects|array<string> \| null|optional|List of VASP objects included with this task document|
|run_stats|object \| null|optional|Summary of runtime statistics for each calculation in this task|
|state|string \| null|optional|State of this calculation|
|task_label|string \| null|optional|A description of the task|
|defect_name|string|required||
|bulk_formula|string|required|Formula of the bulk structure.|
|defect|TypedDefectDict|required|Unit cell representation of the defect object.|
|charge_state|integer \| null|optional|Charge state of the defect.|
|supercell_matrix|array<array<integer>> \| null|optional|Supercell matrix used to construct the defect supercell.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-008, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-DEF-TASKS-GET

#### `DOIDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** DOI document (response item for /doi).
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|doi|string \| null|optional|DOI of the material.|
|bibtex|string \| null|optional|Bibtex reference of the material.|
|material_id|string \| null|optional|The Materials Project ID of the material. This comes in the form: mp-******.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-009, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-DOI-ROOT-GET

#### `AbsorptionDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Materials absorption document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|property_name|string|optional||
|task_id|string|required|Calculation id|
|energies|array<number>|required|Absorption energy in eV starting from 0|
|energy_max|number|required|Maximum energy|
|absorption_coefficient|array<number>|required|Absorption coefficient in cm^-1|
|average_imaginary_dielectric|array<number>|required|Imaginary part of the dielectric function corresponding to the energies|
|average_real_dielectric|array<number>|required|Real part of the dielectric function corresponding to the energies|
|bandgap|number \| null|optional|The electronic band gap|
|nkpoints|number \| null|optional|The number of kpoints used in the calculation|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-010, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-ABSORPTION-GET

#### `AlloyPairDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Alloy pairing document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|alloy_pair|TypedAlloyPairDict|required||
|pair_id|string|required||

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-011, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-ALLOYS-GET

#### `BondingDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Bonding document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|property_name|string|optional||
|structure_graph|TypedStructureGraphDict|required|Structure graph|
|method|string|required|Method used to compute structure graph.|
|bond_types|object|required|Dictionary of bond types to their length, e.g. a Fe-O to a list of the lengths of Fe-O bonds in Angstrom.|
|bond_length_stats|TypedBondLengthStatsDict|required|Dictionary of statistics of bonds in structure|
|coordination_envs|array<string>|required|List of co-ordination environments, e.g. ['Mo-S(6)', 'S-Mo(3)'].|
|coordination_envs_anonymous|array<string>|required|List of co-ordination environments without elements present, e.g. ['A-B(6)', 'A-B(3)'].|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-012, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-BONDS-GET

#### `ChemEnvDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Chemical environment document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|string \| null|optional|Warning|
|structure|TypedStructureDict \| null|required|The structure used in the generation of the chemical environment data|
|property_name|string|optional||
|valences|array<number>|required|List of valences for each site in this material to determine cations|
|species|array<string>|required|List of unique (cationic) species in structure.|
|chemenv_symbol|array<string>|required|List of ChemEnv symbols for unique (cationic) species in structure|
|chemenv_iupac|array<string>|required|List of symbols for unique (cationic) species in structure in IUPAC format|
|chemenv_iucr|array<string>|required|List of symbols for unique (cationic) species in structure in IUCR format|
|chemenv_name|array<string>|required|List of text description of coordination environment for unique (cationic) species in structure.|
|chemenv_name_with_alternatives|array<string>|required|List of text description of coordination environment including alternative descriptions for unique (cationic) species in structure.|
|csm|array<number \| null>|required|Saves the continous symmetry measures for unique (cationic) species in structure|
|method|string \| null|required|Method used to compute chemical environments|
|mol_from_site_environments|array<TypedMoleculeDict \| null>|required|List of Molecule Objects describing the detected environment.|
|wyckoff_positions|array<string>|required|List of Wyckoff positions for unique (cationic) species in structure.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-013, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-CHEMENV-GET

#### `ConversionElectrodeDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Conversion electrode document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|battery_type|string \| null|optional|The type of battery (insertion or conversion).|
|battery_id|string \| null|optional|The id for this battery document is the numerically smallest material_id followed by the working ion.|
|thermo_type|string \| null|optional|The functional type used to compute the thermodynamics of this electrode document.|
|battery_formula|string \| null|optional|Reduced formula with working ion range produced by combining the charge and discharge formulas.|
|working_ion|string \| null|optional|The working ion as an Element object.|
|num_steps|integer \| null|optional|The number of distinct voltage steps in from fully charge to discharge based on the stable intermediate states.|
|max_voltage_step|number \| null|optional|Maximum absolute difference in adjacent voltage steps.|
|last_updated|string|optional|Timestamp for the most recent calculation for this Material document.|
|framework|object \| null|optional|The chemical compositions of the host framework.|
|framework_formula|string \| null|optional|The id for this battery document.|
|elements|array<string> \| null|optional|The atomic species contained in this electrode (not including the working ion).|
|nelements|integer \| null|optional|The number of elements in the material (not including the working ion).|
|chemsys|string \| null|optional|The chemical system this electrode belongs to (not including the working ion).|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula (not including the working ion).|
|warnings|array<string>|optional|Any warnings related to this electrode data.|
|formula_charge|string \| null|optional|The chemical formula of the charged material.|
|formula_discharge|string \| null|optional|The chemical formula of the discharged material.|
|max_delta_volume|number \| null|optional|Volume changes in % for a particular voltage step using: max(charge, discharge) / min(charge, discharge) - 1.|
|average_voltage|number \| null|optional|The average voltage in V for a particular voltage step.|
|capacity_grav|number \| null|optional|Gravimetric capacity in mAh/g.|
|capacity_vol|number \| null|optional|Volumetric capacity in mAh/cc.|
|energy_grav|number \| null|optional|Gravimetric energy (Specific energy) in Wh/kg.|
|energy_vol|number \| null|optional|Volumetric energy (Energy Density) in Wh/l.|
|fracA_charge|number \| null|optional|Atomic fraction of the working ion in the charged state.|
|fracA_discharge|number \| null|optional|Atomic fraction of the working ion in the discharged state.|
|reaction|TypedBalancedReactionDict \| null|optional|The reaction that characterizes that particular voltage step.|
|initial_comp_formula|string \| null|optional|The starting composition for the ConversionElectrode represented as a string/formula.|
|adj_pairs|array<ConversionVoltagePairDoc> \| null|optional|Returns all of the voltage steps material pairs.|
|electrode_object|object \| null|optional|The Pymatgen conversion electrode object.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-014, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-CONVERSION_ELECTRODES-GET

#### `MaterialsDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Core materials document (materials/core and related endpoints).
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Struct array for tracking the provenance of properties|
|warnings|array<string>|optional|Any warnings related to this property.|
|structure|TypedStructureDict \| null|required|The structure of the this material.|
|initial_structures|array<TypedStructureDict>|optional|Initial structures used in the DFT optimizations corresponding to this material.|
|task_ids|array<string \| string>|optional|List of Calculations IDs used to make this Materials Document.|
|deprecated_tasks|array<string>|optional||
|calc_types|object \| null|optional|Calculation types for all the calculations that make up this material|
|created_at|string|optional|Timestamp for when this material document was first created.|
|task_types|object \| null|optional|Task types for all the calculations that make up this material|
|run_types|object \| null|optional|Run types for all the calculations that make up this material|
|entries|BlessedCalcs \| null|optional|Dictionary for tracking entries for VASP calculations|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-015, FR-016, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-CORE-GET, CT-FR-MAT-CORE_BLESSED_TASKS-GET

#### `FormulaAutocomplete`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Formula autocomplete response document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|formula_pretty|string \| null|optional|Human readable chemical formula.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-018, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET

#### `DielectricDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Dielectric document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|property_name|string|optional||
|total|array<unknown>|required|Total dielectric tensor.|
|ionic|array<unknown>|required|Ionic contribution to dielectric tensor.|
|electronic|array<unknown>|required|Electronic contribution to dielectric tensor.|
|e_total|number|required|Total electric permittivity.|
|e_ionic|number|required|Electric permittivity from atomic rearrangement.|
|e_electronic|number|required|Electric permittivity due to electrons rearrangement.|
|n|number|required|Refractive index.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-019, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-DIELECTRIC-GET

#### `ElasticityDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Elasticity document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|structure|TypedStructureDict \| null|optional|Structure to compute the elasticity|
|property_name|string|optional||
|order|integer|optional|Order of the expansion of the elastic tensor|
|elastic_tensor|ElasticTensorDoc \| null|optional|Elastic tensor|
|compliance_tensor|ComplianceTensorDoc \| null|optional|Compliance tensor|
|bulk_modulus|BulkModulus \| null|optional|Bulk modulus|
|shear_modulus|ShearModulus \| null|optional|Shear modulus|
|sound_velocity|SoundVelocity \| null|optional|Sound velocity|
|thermal_conductivity|ThermalConductivity \| null|optional|Thermal conductivity|
|youngs_modulus|number \| null|optional|Young's modulus (SI units)|
|universal_anisotropy|number \| null|optional|Universal elastic anisotropy|
|homogeneous_poisson|number \| null|optional|Homogeneous Poisson ratio|
|debye_temperature|number \| null|optional|Debye temperature (SI units)|
|fitting_data|FittingData \| null|optional|Data used to fit the elastic tensor|
|fitting_method|string \| null|optional|Method used to fit the elastic tensor|
|state|string \| null|optional|State of the fitting/analysis: `successful` or `failed`|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-020, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-ELASTICITY-GET

#### `ElectronicStructureDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Electronic structure document (bandstructure/DOS).
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|task_id|string \| string|required|The source calculation (task) ID for the electronic structure data. This has the same form as a Materials Project ID.|
|band_gap|number|required|Band gap energy in eV.|
|cbm|number \| null|optional|Conduction band minimum data.|
|vbm|number \| null|optional|Valence band maximum data.|
|efermi|number \| null|optional|Fermi energy in eV.|
|is_gap_direct|boolean|required|Whether the band gap is direct.|
|is_metal|boolean|required|Whether the material is a metal.|
|magnetic_ordering|string|required|Magnetic ordering of the calculation.|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|property_name|string|optional||
|bandstructure|BandstructureData \| null|optional|Band structure data for the material.|
|dos|DosData \| null|optional|Density of states data for the material.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-021, FR-022, FR-023, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-ELECTRONIC_STRUCTURE-GET, CT-FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET, CT-FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET

#### `EOSDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Equation-of-state document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|structure|TypedStructureDict \| null|required|The structure of the this material.|
|energies|array<number> \| null|optional|Energies in eV that the equations of state are plotted with.|
|volumes|array<number> \| null|optional|Volumes in A³ that the equations of state are plotted with.|
|eos|array<EOSFit> \| null|optional|Data for each type of equation of state.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-024, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-EOS-GET

#### `FermiDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Fermi surface document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|fermi_surfaces|array<object> \| null|optional|List of IFermi FermiSurface objects.|
|surface_types|array<string> \| null|optional|Type of each fermi surface in the fermi_surfaces list.            Is either CBM or VBM for semiconductors, or fermi_surface for metals.|
|material_id|string \| null|optional|The Materials Project ID of the material. This comes in the form: mp-******.|
|last_updated|string|optional|Timestamp for the most recent calculation for this fermi surface document.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-025, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-FERMI-GET

#### `GrainBoundaryDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Grain boundary document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|material_id|string \| null|optional|The Materials Project ID of the material. This comes in the form: mp-******.|
|sigma|integer \| null|optional|Sigma value of the boundary.|
|type|string \| null|optional|Grain boundary type.|
|rotation_axis|array<integer> \| null|optional|Rotation axis.|
|gb_plane|array<integer> \| null|optional|Grain boundary plane.|
|rotation_angle|number \| null|optional|Rotation angle in degrees.|
|gb_energy|number \| null|optional|Grain boundary energy in J/m^2.|
|initial_structure|TypedGrainBoundaryDict \| null|optional|Initial grain boundary structure.|
|final_structure|TypedGrainBoundaryDict \| null|optional|Final grain boundary structure.|
|pretty_formula|string \| null|optional|Reduced formula of the material.|
|w_sep|number \| null|optional|Work of separation in J/m^2.|
|cif|string \| null|optional|CIF file of the structure.|
|chemsys|string \| null|optional|Dash-delimited string of elements in the material.|
|last_updated|string|optional|Timestamp for the most recent calculation for this Material document.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-026, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-GRAIN_BOUNDARIES-GET

#### `InsertionElectrodeDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Insertion electrode document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|battery_type|string \| null|optional|The type of battery (insertion or conversion).|
|battery_id|string \| null|optional|The id for this battery document is the numerically smallest material_id followed by the working ion.|
|thermo_type|string \| null|optional|The functional type used to compute the thermodynamics of this electrode document.|
|battery_formula|string \| null|optional|Reduced formula with working ion range produced by combining the charge and discharge formulas.|
|working_ion|string \| null|optional|The working ion as an Element object.|
|num_steps|integer \| null|optional|The number of distinct voltage steps in from fully charge to discharge based on the stable intermediate states.|
|max_voltage_step|number \| null|optional|Maximum absolute difference in adjacent voltage steps.|
|last_updated|string|optional|Timestamp for the most recent calculation for this Material document.|
|framework|object \| null|optional|The chemical compositions of the host framework.|
|framework_formula|string \| null|optional|The id for this battery document.|
|elements|array<string> \| null|optional|The atomic species contained in this electrode (not including the working ion).|
|nelements|integer \| null|optional|The number of elements in the material (not including the working ion).|
|chemsys|string \| null|optional|The chemical system this electrode belongs to (not including the working ion).|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula (not including the working ion).|
|warnings|array<string>|optional|Any warnings related to this electrode data.|
|formula_charge|string \| null|optional|The chemical formula of the charged material.|
|formula_discharge|string \| null|optional|The chemical formula of the discharged material.|
|max_delta_volume|number \| null|optional|Volume changes in % for a particular voltage step using: max(charge, discharge) / min(charge, discharge) - 1.|
|average_voltage|number \| null|optional|The average voltage in V for a particular voltage step.|
|capacity_grav|number \| null|optional|Gravimetric capacity in mAh/g.|
|capacity_vol|number \| null|optional|Volumetric capacity in mAh/cc.|
|energy_grav|number \| null|optional|Gravimetric energy (Specific energy) in Wh/kg.|
|energy_vol|number \| null|optional|Volumetric energy (Energy Density) in Wh/l.|
|fracA_charge|number \| null|optional|Atomic fraction of the working ion in the charged state.|
|fracA_discharge|number \| null|optional|Atomic fraction of the working ion in the discharged state.|
|stability_charge|number \| null|optional|The energy above hull of the charged material in eV/atom.|
|stability_discharge|number \| null|optional|The energy above hull of the discharged material in eV/atom.|
|id_charge|string \| string \| integer \| null|optional|The Materials Project ID of the charged structure.|
|id_discharge|string \| string \| integer \| null|optional|The Materials Project ID of the discharged structure.|
|host_structure|TypedStructureDict \| null|optional|Host structure (structure without the working ion).|
|adj_pairs|array<InsertionVoltagePairDoc> \| null|optional|Returns all of the voltage steps material pairs.|
|material_ids|array<string \| string> \| null|optional|The ids of all structures that matched to the present host lattice, regardless of stability. The stable entries can be found in the adjacent pairs.|
|entries_composition_summary|EntriesCompositionSummary \| null|optional|Composition summary data for all material in entries across all voltage pairs.|
|electrode_object|object \| null|optional|The Pymatgen electrode object.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-027, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-INSERTION_ELECTRODES-GET

#### `MagnetismDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Magnetism document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|property_name|string|optional||
|ordering|string \| null|optional|Magnetic ordering.|
|is_magnetic|boolean \| null|optional|Whether the material is magnetic.|
|exchange_symmetry|integer \| null|optional|Exchange symmetry.|
|num_magnetic_sites|integer \| null|optional|The number of magnetic sites.|
|num_unique_magnetic_sites|integer \| null|optional|The number of unique magnetic sites.|
|types_of_magnetic_species|array<string> \| null|optional|Magnetic specie elements.|
|magmoms|array<number> \| null|optional|Magnetic moments for each site.|
|total_magnetization|number \| null|optional|Total magnetization in μB.|
|total_magnetization_normalized_vol|number \| null|optional|Total magnetization normalized by volume in μB/Å³.|
|total_magnetization_normalized_formula_units|number \| null|optional|Total magnetization normalized by formula unit in μB/f.u. .|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-028, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-MAGNETISM-GET

#### `OxidationStateDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Oxidation state document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|structure|TypedStructureDict|required|The structure used in the generation of the oxidation state data.|
|property_name|string|optional||
|possible_species|array<string>|required|Possible charged species in this material.|
|possible_valences|array<number>|required|List of valences for each site in this material.|
|average_oxidation_states|object|required|Average oxidation states for each unique species.|
|method|string \| null|optional|Method used to compute oxidation states.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-029, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-OXIDATION_STATES-GET

#### `PhononBSDOSDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Phonon bandstructure/DOS document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|identifier|string \| null|optional|The identifier of this phonon analysis task.|
|phonon_method|PhononMethod \| null|optional|The method used to calculate phonon properties.|
|phonon_bandstructure|PhononBS \| null|optional|Phonon band structure object.|
|phonon_dos|PhononDOS \| null|optional|Phonon density of states object.|
|epsilon_static|array<unknown> \| null|optional|The high-frequency dielectric constant.|
|epsilon_electronic|array<unknown> \| null|optional|The electronic contribution to the high-frequency dielectric constant.|
|born|array<array<unknown>> \| null|optional|Born charges, only for symmetrically inequivalent atoms|
|force_constants|array<array<array<unknown>>> \| null|optional|Force constants between every pair of atoms in the structure|
|last_updated|string|optional|Timestamp for the most recent calculation for this Material document.|
|sum_rules_breaking|SumRuleChecks \| null|optional|Deviations from sum rules.|
|structure|TypedStructureDict \| null|optional|Structure used in the calculation.|
|total_dft_energy|number \| null|optional|total DFT energy in eV/atom.|
|volume_per_formula_unit|number \| null|optional|volume per formula unit in Angstrom**3.|
|formula_units|integer \| null|optional|Formula units per cell.|
|supercell_matrix|array<unknown> \| null|optional|matrix describing the supercell.|
|primitive_matrix|array<unknown> \| null|optional|matrix describing relationship to primitive cell.|
|code|string \| null|optional|String describing the code for the computation.|
|post_process_settings|PhononComputationalSettings \| null|optional|Field including settings for the post processing code, e.g., phonopy.|
|thermal_displacement_data|ThermalDisplacementData \| null|optional|Includes all data of the computation of the thermal displacements|
|calc_meta|array<CalcMeta> \| null|optional|Metadata for individual calculations used to build this document.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, of the form mp-******.|
|task_ids|array<string> \| null|optional|A list of identifiers that were used to build this document.|
|has_imaginary_modes|boolean \| null|required||
|charge_neutral_sum_rule|array<unknown> \| null|required|Sum of Born effective charges over sites should be zero.|
|acoustic_sum_rule|array<unknown> \| null|required|Sum of q=0 atomic force constants should be zero.|
|check_sum_rule_deviations|SumRuleChecks|required|Report deviations from sum rules.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-030, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-PHONON-GET

#### `PiezoelectricDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Piezoelectric document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|property_name|string|optional||
|total|array<array<number>>|required|Total piezoelectric tensor in C/m²|
|ionic|array<array<number>>|required|Ionic contribution to piezoelectric tensor in C/m²|
|electronic|array<array<number>>|required|Electronic contribution to piezoelectric tensor in C/m²|
|e_ij_max|number|required|Piezoelectric modulus|
|max_direction|array<integer>|required|Miller direction for maximum piezo response|
|strain_for_max|array<number>|required|Normalized strain direction for maximum piezo repsonse|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-031, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-PIEZOELECTRIC-GET

#### `ProvenanceDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Provenance document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|property_name|string|optional||
|created_at|string|optional|creation date for the first structure corresponding to this material|
|references|array<string>|optional|Bibtex reference strings for this material|
|authors|array<Author>|optional|list of authors for this material|
|remarks|array<string> \| null|optional|list of remarks for the provenance of this material|
|tags|array<string> \| null|optional||
|theoretical|boolean|optional|If this material has any experimental provenance or not|
|database_IDs|object \| null|optional|Database IDs corresponding to this material|
|history|array<History> \| null|optional|list of history nodes specifying the transformations or orignation of this material for the entry closest matching the material input|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-032, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-PROVENANCE-GET

#### `RobocrystallogapherDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Robocrystallographer document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|property_name|string|optional||
|description|string|required|Description text from robocrytallographer.|
|condensed_structure|CondensedStructureData|required|Condensed structure data from robocrytallographer.|
|robocrys_version|string|required|The version of Robocrystallographer used to generate this document.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-033, FR-034, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-ROBOCRYS-GET, CT-FR-MAT-ROBOCRYS_TEXT_SEARCH-GET

#### `SimilarityDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Similarity document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| null|optional|The Materials Project ID for the material. This comes in the form: mp-******|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|property_name|string|optional||
|sim|array<SimilarityEntry> \| null|optional|List containing similar structure data for a given material.|
|feature_vector|array<number> \| null|optional|The feature / embedding vector of the structure.|
|method|string \| null|optional|The method used to score similarity.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-035, FR-036, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-SIMILARITY-GET, CT-FR-MAT-SIMILARITY_MATCH-GET

#### `SubstratesDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Substrates document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|sub_form|string \| null|optional|Reduced formula of the substrate.|
|sub_id|string \| null|optional|Materials Project ID of the substrate material. This comes in the form: mp-******.|
|film_orient|string \| null|optional|Surface orientation of the film material.|
|area|number \| null|optional|Minimum coincident interface area in Å².|
|energy|number \| null|optional|Elastic energy in meV.|
|film_id|string \| null|optional|The Materials Project ID of the film material. This comes in the form: mp-******.|
|_norients|integer \| null|optional|Number of possible surface orientations for the substrate.|
|orient|string \| null|optional|Surface orientation of the substrate material.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-037, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-SUBSTRATES-GET

#### `SummaryDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Materials summary document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|structure|TypedStructureDict|required|The lowest energy structure for this material.|
|property_name|string|optional||
|task_ids|array<string \| string>|optional|List of Calculations IDs associated with this material.|
|uncorrected_energy_per_atom|number \| null|optional|The total DFT energy of this material per atom in eV/atom.|
|energy_per_atom|number \| null|optional|The total corrected DFT energy of this material per atom in eV/atom.|
|formation_energy_per_atom|number \| null|optional|The formation energy per atom in eV/atom.|
|energy_above_hull|number \| null|optional|The energy above the hull in eV/Atom.|
|is_stable|boolean|optional|Flag for whether this material is on the hull and therefore stable.|
|equilibrium_reaction_energy_per_atom|number \| null|optional|The reaction energy of a stable entry from the neighboring equilibrium stable materials in eV. Also known as the inverse distance to hull.|
|decomposes_to|array<DecompositionProduct> \| null|optional|List of decomposition data for this material. Only valid for metastable or unstable material.|
|xas|array<XASSearchData> \| null|optional|List of xas documents.|
|grain_boundaries|array<GBSearchData> \| null|optional|List of grain boundary documents.|
|band_gap|number \| null|optional|Band gap energy in eV.|
|cbm|number \| null|optional|Conduction band minimum data.|
|vbm|number \| null|optional|Valence band maximum data.|
|efermi|number \| null|optional|Fermi energy in eV.|
|is_gap_direct|boolean \| null|optional|Whether the band gap is direct.|
|is_metal|boolean \| null|optional|Whether the material is a metal.|
|es_source_calc_id|string \| string \| null|optional|The source calculation ID for the electronic structure data.|
|bandstructure|BandstructureData \| null|optional|Band structure data for the material.|
|dos|DosData \| null|optional|Density of states data for the material.|
|dos_energy_up|number \| null|optional|Spin-up DOS band gap in eV.|
|dos_energy_down|number \| null|optional|Spin-down DOS band gap in eV.|
|is_magnetic|boolean \| null|optional|Whether the material is magnetic.|
|ordering|string \| null|optional|Type of magnetic ordering.|
|total_magnetization|number \| null|optional|Total magnetization in μB.|
|total_magnetization_normalized_vol|number \| null|optional|Total magnetization normalized by volume in μB/Å³.|
|total_magnetization_normalized_formula_units|number \| null|optional|Total magnetization normalized by formula unit in μB/f.u. .|
|num_magnetic_sites|integer \| null|optional|The number of magnetic sites.|
|num_unique_magnetic_sites|integer \| null|optional|The number of unique magnetic sites.|
|types_of_magnetic_species|array<string> \| null|optional|Magnetic specie elements.|
|bulk_modulus|object \| null|optional|Voigt, Reuss, and Voigt-Reuss-Hill averages of the bulk modulus in GPa.|
|shear_modulus|object \| null|optional|Voigt, Reuss, and Voigt-Reuss-Hill averages of the shear modulus in GPa.|
|universal_anisotropy|number \| null|optional|Elastic anisotropy.|
|homogeneous_poisson|number \| null|optional|Poisson's ratio.|
|e_total|number \| null|optional|Total dielectric constant.|
|e_ionic|number \| null|optional|Ionic contribution to dielectric constant.|
|e_electronic|number \| null|optional|Electronic contribution to dielectric constant.|
|n|number \| null|optional|Refractive index.|
|e_ij_max|number \| null|optional|Piezoelectric modulus.|
|weighted_surface_energy_EV_PER_ANG2|number \| null|optional|Weighted surface energy in eV/Å².|
|weighted_surface_energy|number \| null|optional|Weighted surface energy in J/m².|
|weighted_work_function|number \| null|optional|Weighted work function in eV.|
|surface_anisotropy|number \| null|optional|Surface energy anisotropy.|
|shape_factor|number \| null|optional|Shape factor.|
|has_reconstructed|boolean \| null|optional|Whether the material has any reconstructed surfaces.|
|possible_species|array<string> \| null|optional|Possible charged species in this material.|
|has_props|object \| null|optional|List of properties that are available for a given material.|
|theoretical|boolean|optional|Whether the material is theoretical.|
|database_IDs|object|optional|External database IDs corresponding to this material.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-038, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-SUMMARY-GET

#### `SurfacePropDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Surface properties document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|surfaces|array<SurfaceEntry> \| null|optional|List of individual surface data.|
|weighted_surface_energy_EV_PER_ANG2|number \| null|optional|Weighted surface energy in eV/Å²|
|weighted_surface_energy|number \| null|optional|Weighted surface energy in J/m²|
|surface_anisotropy|number \| null|optional|Surface energy anisotropy.|
|pretty_formula|string \| null|optional|Reduced Formula of the material.|
|shape_factor|number \| null|optional|Shape factor.|
|weighted_work_function|number \| null|optional|Weighted work function in eV.|
|has_reconstructed|boolean \| null|optional|Whether the entry has any reconstructed surfaces.|
|material_id|string \| null|optional|The Materials Project ID of the material. This comes in the form: mp-******.|
|structure|TypedStructureDict \| null|optional|The conventional crystal structure of the material.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-039, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-SURFACE_PROPERTIES-GET

#### `SynthesisSearchResultModel`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Synthesis search result model.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|doi|string|required|DOI of the journal article.|
|paragraph_string|string|optional|The paragraph from which this recipe is extracted.|
|synthesis_type|string|required|Type of the synthesis recipe.|
|reaction_string|string|required|String representation of this recipe.|
|reaction|ReactionFormula|required|The balanced reaction formula.|
|target|ExtractedMaterial|required|The target material.|
|targets_formula|array<string>|required|List of synthesized target material compositions.|
|precursors_formula|array<string>|required|List of precursor material compositions.|
|targets_formula_s|array<string>|required|List of synthesized target material compositions, as strings.|
|precursors_formula_s|array<string>|required|List of precursor material compositions, as strings.|
|precursors|array<ExtractedMaterial>|required|List of precursor materials.|
|operations|array<Operation>|required|List of operations used to synthesize this recipe.|
|search_score|number \| null|optional|Search score.|
|highlights|array<unknown> \| null|optional|Search highlights.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-040, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-SYNTHESIS-GET

#### `TaskDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Task document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|batch_id|string \| null|optional|Identifier for this calculation; should provide rough information about the calculation origin and purpose.|
|calc_type|string \| null|optional|The functional and task type used in the calculation.|
|completed_at|string \| null|optional|Timestamp for when this task was completed|
|dir_name|string \| null|optional|The directory for this VASP task|
|icsd_id|integer \| null|optional|Inorganic Crystal Structure Database id of the structure|
|input|CalculationInput \| null|optional|VASP calculation inputs|
|last_updated|string|optional|Timestamp for the most recent calculation for this task document|
|orig_inputs|CalculationInput \| null|optional|The exact set of input parameters used to generate the current task document.|
|output|OutputDoc \| null|optional|The exact set of output parameters used to generate the current task document.|
|run_type|string \| null|optional|The functional used in the calculation.|
|structure|TypedStructureDict \| null|optional|Final output structure from the task|
|tags|array<string> \| null|optional|Metadata tagged to a given task.|
|task_id|string \| string \| null|optional|The (task) ID of this calculation, used as a universal reference across property documents.This comes in the form: mp-******.|
|task_type|string \| null|optional|The type of calculation.|
|transformations|object \| null|optional|Information on the structural transformations, parsed from a transformations.json file|
|vasp_objects|object \| null|optional|Vasp objects associated with this task|
|additional_json|object \| null|optional|Additional json loaded from the calculation directory|
|analysis|AnalysisDoc \| null|optional|Some analysis of calculation data after collection.|
|author|string \| null|optional|Author extracted from transformations|
|calcs_reversed|array<Calculation> \| null|optional|Detailed data for each VASP calculation contributing to the task document.|
|custodian|array<CustodianDoc> \| null|optional|Detailed custodian data for each VASP calculation contributing to the task document.|
|entry|object \| null|optional|The ComputedEntry from the task doc|
|included_objects|array<string> \| null|optional|List of VASP objects included with this task document|
|run_stats|object \| null|optional|Summary of runtime statistics for each calculation in this task|
|state|string \| null|optional|State of this calculation|
|task_label|string \| null|optional|A description of the task|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-041, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-TASKS-GET

#### `DeprecationDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Deprecation document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|task_id|string \| null|optional|The (task) ID of this calculation, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean \| null|optional|Whether the ID corresponds to a deprecated calculation.|
|deprecation_reason|string \| null|optional|Reason for deprecation.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-042, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-TASKS_DEPRECATION-GET

#### `EntryDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Entry document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|task_id|string \| null|optional|The (task) ID of this calculation, used as a universal reference across property documents.This comes in the form: mp-******.|
|entry|object \| null|optional|Computed structure entry for the calculation associated with the task doc.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-043, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-TASKS_ENTRIES-GET

#### `TrajectoryDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Trajectory document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|task_id|string \| null|optional|The (task) ID of this calculation, used as a universal reference across property documents.This comes in the form: mp-******.|
|trajectories|array<RelaxTrajectory> \| null|optional|Trajectory data for calculations associated with a task doc.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-044, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-TASKS_TRAJECTORY-GET

#### `ThermoDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Thermochemistry document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|material_id|string \| string \| null|optional|The Materials Project ID of the material, used as a universal reference across property documents.This comes in the form: mp-******.|
|deprecated|boolean|optional|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|origins|array<PropertyOrigin> \| null|optional|Dictionary for tracking the provenance of properties.|
|warnings|array<string>|optional|Any warnings related to this property.|
|property_name|string|optional||
|thermo_type|string|required|Functional types of calculations involved in the energy mixing scheme.|
|thermo_id|string|required|Unique document ID which is composed of the Material ID and thermo data type.|
|uncorrected_energy_per_atom|number|required|The total DFT energy of this material per atom in eV/atom.|
|energy_per_atom|number|required|The total corrected DFT energy of this material per atom in eV/atom.|
|energy_uncertainy_per_atom|number \| null|optional||
|formation_energy_per_atom|number \| null|optional|The formation energy per atom in eV/atom.|
|energy_above_hull|number|required|The energy above the hull in eV/Atom.|
|is_stable|boolean|optional|Flag for whether this material is on the hull and therefore stable.|
|equilibrium_reaction_energy_per_atom|number \| null|optional|The reaction energy of a stable entry from the neighboring equilibrium stable materials in eV. Also known as the inverse distance to hull.|
|decomposes_to|array<DecompositionProduct> \| null|optional|List of decomposition data for this material. Only valid for metastable or unstable material.|
|decomposition_enthalpy|number \| null|optional|Decomposition enthalpy as defined by `get_decomp_and_phase_separation_energy` in pymatgen.|
|decomposition_enthalpy_decomposes_to|array<DecompositionProduct> \| null|optional|List of decomposition data associated with the decomposition_enthalpy quantity.|
|energy_type|string|required|The type of calculation this energy evaluation comes from.|
|entry_types|array<string>|required|List of available energy types computed for this material.|
|entries|object|required|List of all entries that are valid for this material. The keys for this dictionary are names of various calculation types.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-045, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-THERMO-GET

#### `XASDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** X-ray absorption spectroscopy document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|builder_meta|EmmetMeta \| null|optional|Builder metadata.|
|nsites|integer \| null|optional|Total number of sites in the structure.|
|elements|array<string> \| null|optional|List of elements in the material.|
|nelements|integer \| null|optional|Number of elements.|
|composition|object \| null|optional|Full composition for the material.|
|composition_reduced|object \| null|optional|Simplified representation of the composition.|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula.|
|chemsys|string \| null|optional|Dash-separated string of elements in the material.|
|volume|number \| null|optional|Total volume for this structure in Å³.|
|density|number \| null|optional|Density in g/cm³.|
|density_atomic|number \| null|optional|The atomic packing density in Å³/atom.|
|symmetry|SymmetryData \| null|optional|Symmetry data for this material.|
|spectrum_name|string|optional||
|material_id|string \| string \| null|optional|The ID of the material, used as a universal reference across proeprty documents. This comes in the form: mp-******.|
|spectrum_id|XasSpectrumID|required||
|last_updated|string|optional|Timestamp for the most recent calculation update for this property.|
|warnings|array<string>|optional|Any warnings related to this property.|
|spectrum|TypedXASSpectrumDict \| null|optional|The XAS spectrum for this calculation.|
|task_ids|array<string> \| null|optional|List of Calculations IDs used to make this XAS spectrum.|
|absorbing_element|string|required|Absoring element.|
|spectrum_type|string|required|XAS spectrum type.|
|edge|string|required|The interaction edge for XAS.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-046, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MAT-XAS-GET

#### `MoleculesDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Molecules document (e.g., /molecules/jcesr).
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|elements|array<Element> \| null|optional|List of elements in the molecule.|
|nelements|integer \| null|optional|Number of elements in the molecule.|
|EA|number \| null|optional|Electron affinity of the molecule in eV.|
|IE|number \| null|optional|Ionization energy of the molecule in eV.|
|charge|integer \| null|optional|Charge of the molecule in +e.|
|pointgroup|string \| null|optional|Point group of the molecule in Schoenflies notation.|
|smiles|string \| null|optional|The simplified molecular input line-entry system (SMILES)             representation of the molecule.|
|task_id|string \| null|optional|Materials Project molecule ID. This takes the form mol-*****.|
|molecule|TypedMoleculeDict \| null|optional|Pymatgen molecule object.|
|formula_pretty|string \| null|optional|Chemical formula of the molecule.|
|svg|string \| null|optional|String representation of the SVG image of the molecule.|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-047, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MOL-JCESR-GET

#### `MoleculeSummaryDoc`

- **Kind:** OpenAPI-generated model (DTO)
- **Purpose:** Molecule summary document.
- **Fields / Properties**
|Field|Type|Required|Constraints / Invariants|
|---|---|---|---|
|charge|integer \| null|optional|Charge of the molecule|
|spin_multiplicity|integer \| null|optional|Spin multiplicity of the molecule|
|natoms|integer \| null|optional|Total number of atoms in the molecule|
|elements|array<string> \| null|optional|List of elements in the molecule|
|nelements|integer \| null|optional||
|nelectrons|integer \| null|optional|The total number of electrons for the molecule|
|composition|object \| null|optional|Full composition for the molecule|
|composition_reduced|object \| null|optional|Simplified representation of the composition|
|formula_alphabetical|string \| null|optional|Alphabetical molecular formula|
|formula_pretty|string \| null|optional|Cleaned representation of the formula.|
|formula_anonymous|string \| null|optional|Anonymized representation of the formula|
|chemsys|string \| null|optional|dash-delimited string of elements in the molecule|
|symmetry|PointGroupData \| null|optional|Symmetry data for this molecule|
|species_hash|string \| null|optional|Weisfeiler Lehman (WL) graph hash using the atom species as the graph node attribute.|
|coord_hash|string \| null|optional|Weisfeiler Lehman (WL) graph hash using the atom coordinates as the graph node attribute.|
|property_name|string|optional||
|property_id|string|required|The unique identifier of this property document.|
|molecule_id|string|required|The ID of the molecule, used as a reference across property documents.This comes in the form of an MPculeID (or appropriately formatted string)|
|deprecated|boolean|required|Whether this property document is deprecated.|
|deprecation_reasons|array<string> \| null|optional|List of deprecation tags detailing why this document isn't valid|
|level_of_theory|string \| null|optional|Level of theory used to generate this property document.|
|solvent|string \| null|optional|String representation of the solvent environment used to generate this property document.|
|lot_solvent|string \| null|optional|String representation of the level of theory and solvent environment used to generate this property document.|
|last_updated|string|optional|Timestamp for the most recent calculation update for this property|
|origins|array<MolPropertyOrigin>|optional|Dictionary for tracking the provenance of properties|
|warnings|array<string>|optional|Any warnings related to this property|
|molecules|object|required|The lowest energy optimized structures for this molecule for each solvent.|
|molecule_levels_of_theory|object \| null|optional|Level of theory used to optimize the best molecular structure for each solvent.|
|inchi|string \| null|optional|International Chemical Identifier (InChI) for this molecule|
|inchi_key|string \| null|optional|Standardized hash of the InChI for this molecule|
|task_ids|array<string>|optional|List of Calculation IDs associated with this molecule.|
|similar_molecules|array<string>|optional|IDs associated with similar molecules|
|constituent_molecules|array<string>|optional|IDs of associated MoleculeDocs used to construct this molecule.|
|unique_calc_types|array<string> \| null|optional|Collection of all unique calculation types used for this molecule|
|unique_task_types|array<string> \| null|optional|Collection of all unique task types used for this molecule|
|unique_levels_of_theory|array<string> \| null|optional|Collection of all unique levels of theory used for this molecule|
|unique_solvents|array<string> \| null|optional|Collection of all unique solvents (solvent parameters) used for this molecule|
|unique_lot_solvents|array<string> \| null|optional|Collection of all unique combinations of level of theory and solvent used for this molecule|
|thermo|object \| null|optional|A summary of thermodynamic data available for this molecule, organized by solvent|
|vibration|object \| null|optional|A summary of the vibrational data available for this molecule, organized by solvent|
|orbitals|object \| null|optional|A summary of the orbital (NBO) data available for this molecule, organized by solvent|
|partial_charges|object \| null|optional|A summary of the partial charge data available for this molecule, organized by solvent and by method|
|partial_spins|object \| null|optional|A summary of the partial spin data available for this molecule, organized by solvent and by method|
|bonding|object \| null|optional|A summary of the bonding data available for this molecule, organized by solvent and by method|
|multipole_moments|object \| null|optional|A summary of the electric multipole data available for this molecule, organized by solvent|
|redox|object \| null|optional|A summary of the redox data available for this molecule, organized by solvent|
|metal_binding|object \| null|optional|A summary of the metal binding data available for this molecule, organized by solvent and by method|
|has_props|object \| null|optional|Properties available for this molecule|

- **Serialization / Schema Notes:** Serde JSON derived from OpenAPI schema; nullable fields are `Option<T>`; unknown fields tolerated (DR-003).
- **Versioning / Compatibility Notes:** Schema evolves with OpenAPI; treat model changes as spec-driven. Prefer additive changes; unknown fields should not break deserialization.
- **Location:** src/data/models/<generated>.rs (generated; exact file naming depends on generator)
- **Related Requirement IDs:** FR-048, DR-002, DR-003
- **Related Test Case IDs:** CT-FR-MOL-SUMMARY-GET


### Public Interfaces

- `pub struct Response<T> { pub data: Vec<T>, pub errors: Vec<ApiErrorItem>, pub meta: Meta }`
    - Note: `data` is modeled as `Vec<T>` consistent with the RDD envelope example (RDD §7.1).
  - `pub struct ApiErrorItem { pub code: i32, pub message: String }`
  - `pub struct Meta { pub api_version: Option<String>, pub time_stamp: Option<String>, pub total_doc: Option<i64>, pub facet: Option<serde_json::Value> }`

### Internal Design

- Deserialization strategy:
    1. Attempt to deserialize into `Response<T>` (typed mode).
    2. In doc-driven raw mode, attempt:
       - `Response<serde_json::Value>` first (preferred)
       - fallback to top-level JSON (array/object) if no envelope present (per FR-049).

### Source Files & Responsibilities

#### `src/data/mod.rs`

- **Responsibility:** Data-layer module root; re-exports envelope and generated models.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `Response<T>`, `ApiErrorItem`, `Meta`
- **Related requirement IDs:** FR-004, DR-001, DR-002, DR-003
- **Related test case IDs:** UT-FR-004, UT-DR-001, UT-DR-002, UT-DR-003

#### `src/data/envelope.rs`

- **Responsibility:** Defines Response<T>, ApiErrorItem, Meta, and envelope parsing helpers.
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `Response<T>`, `ApiErrorItem`, `Meta`
- **Related requirement IDs:** FR-004, DR-001, DR-002, DR-003
- **Related test case IDs:** UT-FR-004, UT-DR-001, UT-DR-002, UT-DR-003

#### `src/data/models/mod.rs`

- **Responsibility:** OpenAPI-generated model module root (generated files per schema).
- **Description:**
  - Implements the module’s responsibility contract for the concerns owned by this file.
  - Uses deterministic, testable behavior for serialization/mapping where applicable.
  - Avoids leaking secrets in logs and error messages (where applicable).
- **Key public types/functions:** `Response<T>`, `ApiErrorItem`, `Meta`
- **Related requirement IDs:** FR-004, DR-001, DR-002, DR-003
- **Related test case IDs:** UT-FR-004, UT-DR-001, UT-DR-002, UT-DR-003


### Data Model

- N/A (in-memory only).

### Business Rules & Validation (mapped to requirement IDs)

- Schema mismatch yields `DeserializeError` (FR-004).

### Error Handling

- Use `MpApiError::DeserializeError` for JSON parse failures.
  - Use `MpApiError::ValidationError` for HTTP 422 (handled in Errors module, but deserializes using `HTTPValidationError` model).

### Logging & Metrics

- Deserialization failures are logged with correlation ID; payloads are truncated and sanitized.

### Security

- Ensure that any logged payloads do not include secrets (headers are redacted at transport level).

### Performance/Scalability Notes

- Prefer reading response to bytes then deserializing; v1.1 does not include streaming APIs. For large payloads, callers can page results and/or use raw JSON mode; streaming support may be added in a future major version (see §12).

### Dependencies

- `serde`, `serde_json`.

### Test Design

- UT-FR-004: envelope parsing happy path and error path.
  - UT-DR-001/002/003: envelope/meta/error type policy tests.

# 7. Module Traceability Appendix (module-scoped)

|Requirement ID|Module Section|File(s)|Test Case IDs|Coverage Status|
|---|---|---|---|---|
|DR-001|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-DR-001|Covered|
|DR-002|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-DR-002|Covered|
|DR-003|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-DR-003|Covered|
|FR-004|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-FR-004|Covered|
|FR-008|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-DEF-TASKS-GET|Covered|
|FR-009|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-DOI-ROOT-GET|Covered|
|FR-010|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-ABSORPTION-GET|Covered|
|FR-011|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-ALLOYS-GET|Covered|
|FR-012|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-BONDS-GET|Covered|
|FR-013|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-CHEMENV-GET|Covered|
|FR-014|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-CONVERSION_ELECTRODES-GET|Covered|
|FR-015|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-CORE-GET|Covered|
|FR-016|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-CORE_BLESSED_TASKS-GET|Covered|
|FR-017|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-CORE_FIND_STRUCTURE-POST|Covered|
|FR-018|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-CORE_FORMULA_AUTOCOMPLETE-GET|Covered|
|FR-019|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-DIELECTRIC-GET|Covered|
|FR-020|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-ELASTICITY-GET|Covered|
|FR-021|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-ELECTRONIC_STRUCTURE-GET|Covered|
|FR-022|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-ELECTRONIC_STRUCTURE_BANDSTRUCTURE-GET|Covered|
|FR-023|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-ELECTRONIC_STRUCTURE_DOS-GET|Covered|
|FR-024|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-EOS-GET|Covered|
|FR-025|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-FERMI-GET|Covered|
|FR-026|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-GRAIN_BOUNDARIES-GET|Covered|
|FR-027|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-INSERTION_ELECTRODES-GET|Covered|
|FR-028|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-MAGNETISM-GET|Covered|
|FR-029|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-OXIDATION_STATES-GET|Covered|
|FR-030|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-PHONON-GET|Covered|
|FR-031|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-PIEZOELECTRIC-GET|Covered|
|FR-032|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-PROVENANCE-GET|Covered|
|FR-033|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-ROBOCRYS-GET|Covered|
|FR-034|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-ROBOCRYS_TEXT_SEARCH-GET|Covered|
|FR-035|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-SIMILARITY-GET|Covered|
|FR-036|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-SIMILARITY_MATCH-GET|Covered|
|FR-037|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-SUBSTRATES-GET|Covered|
|FR-038|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-SUMMARY-GET|Covered|
|FR-039|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-SURFACE_PROPERTIES-GET|Covered|
|FR-040|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-SYNTHESIS-GET|Covered|
|FR-041|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-TASKS-GET|Covered|
|FR-042|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-TASKS_DEPRECATION-GET|Covered|
|FR-043|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-TASKS_ENTRIES-GET|Covered|
|FR-044|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-TASKS_TRAJECTORY-GET|Covered|
|FR-045|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-THERMO-GET|Covered|
|FR-046|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MAT-XAS-GET|Covered|
|FR-047|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MOL-JCESR-GET|Covered|
|FR-048|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|CT-FR-MOL-SUMMARY-GET|Covered|
|FR-049|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|DT-MANIFEST-001, UT-FR-049|Covered|
|FR-050|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|DT-FR-MOL-ASSOC-GET|Covered|
|FR-051|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|DT-FR-MOL-BONDING-GET|Covered|
|FR-052|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|DT-FR-MOL-CORE-GET|Covered|
|FR-053|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|DT-FR-MOL-ORBITALS-GET|Covered|
|FR-054|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|DT-FR-MOL-PARTIAL_CHARGES-GET|Covered|
|FR-055|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|DT-FR-MOL-PARTIAL_SPINS-GET|Covered|
|FR-056|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|DT-FR-MOL-REDOX-GET|Covered|
|FR-057|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|DT-FR-MOL-TASKS-GET|Covered|
|FR-058|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|DT-FR-MOL-THERMO-GET|Covered|
|FR-059|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|DT-FR-MOL-VIBRATIONS-GET|Covered|
|FR-060|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-FR-060|Covered|
|FR-061|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-FR-061|Covered|
|FR-062|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-FR-062|Covered|
|FR-063|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-FR-063|Covered|
|FR-064|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-FR-064|Covered|
|FR-065|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-FR-065|Covered|
|FR-066|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-FR-066|Covered|
|FR-067|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-FR-067|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|
|FR-068|§6 "Envelope & Models"|`src/data/mod.rs`, `src/data/envelope.rs`, `src/data/models/mod.rs`|UT-FR-068|Covered (conditional: UnsupportedBySpecification if no OpenAPI route)|


# 8. Open Questions (module-scoped)

- Optional streaming response support for very large payloads is TBD (see Index §12).

# 9. Final Self-Check (module-scoped)

- English-only content (code identifiers/proper nouns allowed): **Yes**
- Table of Contents present: **Yes**
- Covered requirements listed (primary + full appendix): **Yes**
- Responsibility contract uses SHALL/MUST language: **Yes**
- Types & Definitions includes field-level details (or explicit TBD where upstream spec is incomplete): **Yes**
- Source Files & Responsibilities enumerated for the module directory: **Yes**
- Traceability appendix includes requirement-to-test mapping: **Yes**
