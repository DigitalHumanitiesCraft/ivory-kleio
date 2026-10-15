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
updated: 2026-10-15
related: [project, temporal-specification, research-methodology, architecture, testing]
---

# Specification

The [requirements integration](requirements-integration.md) connects H01–H11 to the dissertation, the R representation requirements and existing examples. The H01–H11 entries remain requirement candidates. Each needs a precise source passage or an explicit project rationale, defined scope and observable acceptance behavior before adoption as a domain baseline. [Research methodology](research-methodology.md) governs that transition.

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

The implemented temporal experiment addresses H01, H02, one possibility predicate under H05 and a JSON round trip under H08. Its versioned [temporal specification](temporal-specification.md) owns all input forms, operation semantics, result fields, errors and acceptance criteria. The [synthetic case](../research-artefacts/cases/temporal-alternatives.md) supplies independently stated examples. General historical validity and adoption of a domain baseline remain open.

## Website and later integration

[Website specification](website-specification.md) owns public learning paths and publication criteria. A later browser demonstration should call this same operation through WebAssembly. A harness subsequently invokes tested operations through a typed interface.

## Decisions and ownership

Requirement candidates become a domain baseline through the source-analysis and review procedure in [Research methodology](research-methodology.md). Implementation evidence alone does not adopt a general historical model. The bounded temporal experiment is authorized as an executable research hypothesis.

| Subject | Owning document |
|---|---|
| Purpose, name, intended environment and licensing | [Project](project.md) |
| Experimental temporal behavior and version policy | [Temporal specification](temporal-specification.md) |
| Rust implementation, storage boundaries, website stack and proposed adapters | [Architecture](architecture.md) |
| Website purpose and publication criteria | [Website specification](website-specification.md) |
| Visual and interaction requirements | [Design](design.md) |
| Test procedures and evidence limits | [Testing](testing.md) |
| Current work position and open work | [Plan](plan.md) |
