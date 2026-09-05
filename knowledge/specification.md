---
title: Specification
project:
  name: Ivory Kleio
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

The entries below are requirement candidates. Each must acquire a precise source passage or an explicitly recorded project rationale, a defined scope and observable acceptance behavior before adoption as a domain baseline. The [research method](research-methodology.md) governs that transition.

## Requirements

### Functional candidates

| ID | Candidate behavior | Initial basis | Acceptance direction |
|---|---|---|---|
| H01 | Address source representation and interpretation separately | Dissertation context and project inference | Trace a result to a source location and the selected interpretation |
| H02 | Distinguish forms of underdetermination | Selected passages of Thaller 2020, pending full analysis | Keep exclusive alternatives, intervals, unknown and not-applicable values distinguishable |
| H03 | Evaluate operations in an explicit historical context | Thaller 1991, pp. 197–200, pending formalization | Report the unit, calendar or contextual rule used by an operation |
| H04 | Allow several interpretations of a source token | Selected passages of Thaller 2020, pending terminology reconstruction | Apply distinct interpretations without losing their shared source reference |
| H05 | Specify uncertainty through computation and conditional execution | Thaller 2020 and project inference | State the meaning and limitations of every supported operation chain |
| H06 | Preserve dependencies and exclusion between hypotheses | Project inference to be checked against the corpus | Avoid combinations forbidden by shared assumptions or mutual exclusion |
| H07 | Preserve revisions and competing scholarly perspectives | Project requirement | Locate affected derivations after changing a premise and retrieve earlier research states |
| H08 | Restore stored results with their semantics | Project requirement, related to persistent-information literature | Reopen a collection and reproduce a specified query under the same explicit context |
| H09 | Attribute machine proposals and subsequent decisions | Project requirement from the harness discussion | Identify a proposal's generating activity, evidence and later decisions |
| H10 | Distinguish extraction uncertainty from historical uncertainty | Project requirement | Preserve both dimensions without equating model confidence with historical validity |
| H11 | Expose inspectable, typed operations to an agent harness | Project requirement | Show the proposed operation and context, validate its inputs and retain the actual result |

The [corpus inventory](../research-artefacts/corpus.csv) identifies the consulted source representations. Candidate wording is a project synthesis. It does not claim complete corpus reading.

### Non-functional candidates

An accepted operation has documented preconditions and failure behavior. The same context and rule version should yield the specified core result. A model-generated narrative may vary and is recorded as an actual artifact rather than reconstructed from a later model invocation.

Core semantics must be callable independently of an LLM. Interoperability exports must state what they preserve or approximate. The agent interface should support source inspection and keyboard-accessible review. These requirements become executable checks as corresponding components are introduced.

## Epics and user stories

### Historical interpretation

As a researcher, I want to compare competing temporal interpretations while preserving their source connection. [Temporal alternatives](../research-artefacts/cases/temporal-alternatives.md) defines the first synthetic case for H02 and H05.

As a researcher, I want to revise a contextual assumption and identify which results depended on it. H03, H06 and H07 govern this behavior.

### Agent-mediated research

As a researcher, I want an agent to propose an interpretation with evidence, then inspect and revise it. [Agent interpretation](../research-artefacts/cases/agent-interpretation.md) separates generation, formal validation and scholarly adoption for H09–H11.

As a researcher, I want to compare results under alternative assumptions and inspect the underlying operations. The interpretation of a natural-language request must be visible in an executable query plan.

## Functional scope

The first implementation will formalize a bounded historical value and operation. Engine behavior requires persistence and query execution. The agentic environment requires a tool contract and a recorded workflow over the core.

A proposed workflow retrieves evidence, produces attributed candidate interpretations, validates a typed operation, computes with explicit context and presents results with their dependencies. The specific object model and status transitions remain to be formalized.

## Decisions

| Decision | Position |
|---|---|
| Working name | Ivory Kleio |
| Repository destination | Public repository under DigitalHumanitiesCraft, authorized on 2026-09-05 |
| Method | Promptotyping with maintained knowledge, evidence and explicit source analysis |
| Core language | Rust |
| Initial repository | Documentation, source inventory and synthetic cases |
| Domain architecture | Established after systematic reconstruction of the relevant primary corpus |
| Harness role | Proposed research-facing integration over the kernel |
| Harness implementation, graph model and storage engine | Open research and engineering choices |

The first repository commit is a documentation foundation. Rust build configuration will be added with the first justified executable experiment and its actual toolchain.
