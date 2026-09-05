---
title: Specification
project:
  name: Ivory KLEIO
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
template:
  name: Vorlage Specification
  version: "0.3"
  url: https://dhcraft.org/Promptotyping/promptotyping-document/specification
status: draft
language: en
created: 2026-09-05
updated: 2026-09-05
related: [project, research-methodology, architecture, testing]
---

# Specification

The H01–H11 entries are requirement candidates. Each needs a precise source passage or an explicit project rationale, defined scope and observable acceptance behavior before adoption as a domain baseline. [Research methodology](research-methodology.md) governs that transition.

## Requirements

| ID | Candidate behavior | Initial basis | Acceptance direction |
|---|---|---|---|
| H01 | Address source representation and interpretation separately | Dissertation context and project inference | Trace a result to a source location and selected interpretation |
| H02 | Distinguish forms of underdetermination | Selected passages of Thaller 2020 | Keep exclusive alternatives, intervals, unknown and not-applicable values distinguishable |
| H03 | Evaluate operations in explicit historical context | Thaller 1991, pp. 197–200 | Report the unit, calendar or contextual rule used |
| H04 | Allow several interpretations of a source token | Selected passages of Thaller 2020 | Apply distinct interpretations while retaining their shared source reference |
| H05 | Specify uncertainty through computation and conditional execution | Thaller 2020 and project inference | State the meaning and limits of each supported operation chain |
| H06 | Preserve dependencies and exclusion between hypotheses | Project inference to check against the corpus | Avoid combinations forbidden by shared assumptions or mutual exclusion |
| H07 | Preserve revisions and competing scholarly perspectives | Project requirement | Locate affected derivations and retrieve earlier research states |
| H08 | Restore stored results with their semantics | Project requirement related to persistent-information literature | Reopen a collection and reproduce a specified query under the same context |
| H09 | Attribute machine proposals and subsequent decisions | Project requirement from the harness discussion | Identify the generating activity, evidence and later decisions |
| H10 | Distinguish extraction uncertainty from historical uncertainty | Project requirement | Preserve both dimensions without equating model confidence with historical validity |
| H11 | Expose inspectable, typed operations to an agent harness | Project requirement | Validate a proposed operation and context, and retain its actual result |

The [corpus inventory](../research-artefacts/corpus.csv) identifies consulted representations and reading states. These candidates are a project synthesis. Full source analysis remains pending.

## Operation requirements

Each operation needs preconditions, a result vocabulary and failure behavior. Its explicit inputs, context and rule version determine the specified core result. Unsupported or invalid input must remain distinguishable from a valid negative answer.

Source references and selected interpretations must remain available through supported processing. Exports must state what they preserve or approximate. Core operations must be callable independently of an LLM.

The research interface must support keyboard-accessible inspection of sources, alternatives and decisions. Review state must be understandable in text as well as through visual presentation.

The agent-facing contract must distinguish a generated proposal, formal validation and scholarly adoption. Retain actual model outputs and invocation records. A fresh invocation is a new activity and may produce a different proposal.

## User workflows

A researcher compares competing temporal interpretations without losing their source connection. The [temporal case](../research-artefacts/cases/temporal-alternatives.md) defines the first synthetic example.

A researcher revises a contextual assumption and identifies which results depended on it. H03, H06 and H07 govern this later workflow.

A researcher asks an agent to propose an interpretation, inspects its evidence and changes its assumptions. The [agent case](../research-artefacts/cases/agent-interpretation.md) defines a later H09–H11 evaluation target. The interpretation of a natural-language request must be visible in the proposed executable operation.

## Minimum prototype

This is a proposed experimental contract, not an adopted general historical date model. It isolates one distinction with a Rust library, a callable example or small CLI, and a JSON save/load path.

### Scope and inputs

Each record describes one synthetic event and retains a record identifier, source identifier, source locator, original wording and interpretation identifier. The source record explicitly declares its synthetic origin.

The temporal interpretation has exactly one tagged form.

| Form | Meaning | Valid input |
|---|---|---|
| Exclusive alternatives | Exactly one of the exhaustively listed years applies | A nonempty list of distinct year labels |
| Inclusive interval | Any year from the lower through the upper bound is possible | Two year labels with lower bound at most upper bound |

For this experiment, a year label is an integer from 1 through 9999 in one declared synthetic year domain. The scope includes no historical calendar conversion, partial dates, open bounds, probability distributions or unknown values. Unsupported forms require an explicit error.

Records identify that domain. JSON snapshots declare their format version. The operation result declares the rule version used. Exact field names and version identifiers will be recorded with the implementation before persistence is relied upon.

### Operation and result

`could_occur_in_year(record, year)` asks whether the query year is permitted by the recorded interpretation. For alternatives it tests membership. For an interval it tests inclusive bounds. Its Boolean result expresses possibility within this restricted, exhaustive model.

The structured result includes the Boolean answer, query year, record and interpretation references, domain, rule version and applied rule. It must expose the alternatives or bounds that justify the answer. Explanations follow from those fields and need no model call.

Invalid years, empty or repeated alternatives, reversed bounds, unsupported variants and unknown snapshot versions produce declared errors. They must not be repaired silently or converted into a false answer.

### Acceptance

The [case result table](../research-artefacts/cases/temporal-alternatives.md#expected-behavior) is the semantic oracle. For alternatives 1700 or 1710, the answer for 1705 is false. For the inclusive interval 1700 through 1710, it is true.

A complete experiment must also demonstrate the following behavior.

1. Both forms permit 1700 and 1710 and exclude 1699 and 1711.
2. A single alternative and a one-year interval permit that year and no other year.
3. Each result exposes the interpretation and applied rule.
4. JSON save and reload preserve identifiers, source wording, domain, temporal variant and answers for the same inputs and rule version.
5. Malformed or unsupported inputs return the specified errors.
6. A documented command runs the example from a fresh checkout with the declared toolchain.

This provides bounded evidence for H01, H02, H05 and H08. Snapshot round trips do not establish database transactions, revision history or crash recovery. The experiment does not implement the later agent case.

## Website and later integration

[Website specification](website-specification.md) owns public learning paths and publication criteria. A later browser demonstration should call this same operation through WebAssembly. A harness subsequently invokes tested operations through a typed interface.

## Decisions

| Subject | Position | Status |
|---|---|---|
| Name and repository | Ivory KLEIO, public DigitalHumanitiesCraft/ivory-kleio | Established |
| Repository and website language | English | Established |
| Method | Promptotyping with maintained knowledge and source analysis | Established |
| Core language | Rust | Established |
| Licensing | Original text CC BY 4.0, code MIT | Established |
| Domain baseline | Requires systematic reconstruction of the relevant primary corpus | Established research constraint |
| Current implementation | Documentation, source inventory and synthetic cases | Observed |
| First executable target | Temporal alternatives, one operation and JSON round trip | Proposed experiment |
| Research interface | Agent harness over independently callable kernel operations | Proposed direction |
| Website | Professional public documentation and learning resource | Requested |
| Site framework and browser adapter | Astro with Starlight, later Rust WebAssembly | Proposed technical choices |
| General object model, storage and harness runtime | Determine from requirements and experiments | Open |
