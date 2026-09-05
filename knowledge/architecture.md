---
title: Architecture
project:
  name: Ivory Kleio
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
template:
  name: Vorlage Architecture
  version: "0.4"
  url: https://dhcraft.org/Promptotyping/promptotyping-document/architecture
status: draft
language: en
created: 2026-09-05
updated: 2026-09-05
related: [specification, research-methodology, testing]
---

# Architecture

The intended architecture separates historical semantics from model-mediated research work. Component boundaries are hypotheses to test against the [specification](specification.md). The repository currently provides the research documentation on which implementation will be based.

## Stack

Rust is selected for the reference model and kernel. The harness, user interface and physical storage technology remain undecided. The first integration should expose a small, typed tool interface and reuse an appropriate existing harness runtime where possible.

## Components

| Component | Proposed responsibility |
|---|---|
| Reference model and kernel | Historical values, explicit contexts, supported operations and derivations |
| Persistence and query execution | Durable collections, revision retrieval and query evaluation |
| Agent harness | Context selection, tool execution, workflow state and recorded feedback |
| LLM | Candidate interpretations, proposed query plans and explanations |
| Research interface | Source inspection, comparison of alternatives and revision of decisions |

These responsibilities do not prescribe separate packages. The kernel is intended to remain usable without an LLM.

## Data flow

A research request becomes an inspectable proposed operation. The harness gathers evidence and supplies a candidate interpretation and explicit context. The core validates the operation's formal preconditions and evaluates it. Results retain their dependencies. A scholarly decision records how an interpretation is used.

The tool boundary must distinguish invalid input, an undefined operation, absent evidence and a result under stated assumptions. The initial object model remains open, including the relationship between token, interpretation, assertion and entity.

## External models and services

Model calls should have an attributable invocation, input references, configuration and retained output. Provider model identifiers describe the actual invocation. A subsequent call can produce a different proposal. Core reproducibility is tested using stored inputs and rule versions.

The [agent case](../research-artefacts/cases/agent-interpretation.md) defines the first proposed interaction without selecting a provider or framework.

## Security

Source documents are input data and may contain text resembling instructions. The harness must preserve this boundary. Tools enforce their declared operations and access scope. New remote services require an explicit decision about which research material they receive.

## Accessibility

The proposed interface must allow direct inspection of sources, alternatives and changes through keyboard navigation and understandable text. Review state must be expressed in content as well as any visual encoding.

## Repository structure

The [index](INDEX.md) owns navigation through the actual knowledge base and artifact locations. Rust packages will be introduced with their first executable experiment. Component extraction follows an independently meaningful interface or evaluation task.

## Build and deployment

The current foundation is Markdown and CSV. The next implementation step will select and record a Rust toolchain, add a minimal build and provide the meaningful checks required by [Testing](testing.md). No database service is deployed by this foundation.
