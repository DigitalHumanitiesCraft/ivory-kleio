---
title: Architecture
project:
  name: Ivory KLEIO
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

This document describes proposed component boundaries and their technical implications. Rust is the selected core language. The historical object model, storage technology and harness runtime remain open. The repository currently contains documentation and research artifacts.

## Stack

The reference model starts as a small Rust library with a callable example or a small CLI. Its first operation is specified in [Specification](specification.md#minimum-prototype). Separate crates follow when an interface or evaluation task justifies them.

Astro with Starlight is the proposed documentation stack. A later WebAssembly build would expose the same Rust operation in the browser. Framework selection and toolchain versions will be recorded with implementation.

## Components

| Component | Proposed responsibility | Boundary |
|---|---|---|
| Reference model and kernel | Historical values, explicit contexts, operations and derivations | Callable independently of an LLM |
| Persistence and query execution | Durable collections, revision retrieval and query evaluation | Preserve the reference semantics |
| Agent harness | Context selection, tool execution, workflow state and recorded feedback | Invoke declared operations with inspectable inputs |
| LLM | Candidate interpretations, query plans and explanations | Produce attributable proposals |
| Research interface | Source inspection, alternatives and scholarly decisions | Expose evidence and dependencies |
| Documentation website | Project explanation, tutorials and reference material | Teach behavior supported by the current implementation |

These responsibilities do not imply separate packages or products. The harness is a proposed primary research interface. It can use an existing runtime where its tool and provenance boundaries fit the specification.

## Data flow

A research request becomes an inspectable proposed operation. The harness gathers evidence and supplies an interpretation with explicit context. The kernel validates formal preconditions and evaluates the operation. A result retains its input references and applicable rules. The researcher can inspect the result and record a decision about its use.

The tool contract must distinguish invalid input, an unsupported or undefined operation, absent evidence and a computed result under stated assumptions. A model proposal, formal validation and scholarly adoption are separate records.

The underlying relationships between token, interpretation, assertion and entity remain research questions. A versioned assertion graph is a candidate structure. A graph alone does not define uncertainty semantics or the meaning of execution.

## Reference model and persistence

The first experiment needs tagged temporal values, source references, one operation and a versioned JSON snapshot. Saving and loading that snapshot tests semantic preservation. It does not establish transactions, crash recovery, indexing or general query execution.

An embedded engine can initially use an existing storage component under the semantic core. Its choice follows the requirements and workloads. Optimised query evaluation must agree with a simple reference evaluation.

Rust types can make distinctions explicit, but the project must still define their operations. Uncertain conditions may require a dedicated evaluator. Ordinary host-language Boolean branching must not silently determine historical uncertainty semantics. Distributed conflict resolution addresses synchronisation and does not decide the scholarly meaning of contradictory sources.

## Agent integration

The [agent case](../research-artefacts/cases/agent-interpretation.md) defines a later evaluation target. Each model call should retain its invocation identity, input references, available configuration and actual output. A repeated invocation may produce a different proposal.

The harness uses the tested core through a typed tool interface. Selecting that transport and a harness runtime follows an executable kernel operation. Natural-language explanations accompany the structured result and must preserve its assumptions.

Source documents are research input. Text within them does not gain authority to change tool permissions or workflow instructions. Remote model services require a defined scope for the research material they receive. Credentials remain outside tracked content and public browser bundles.

## Website and browser demonstration

[Starlight](https://starlight.astro.build/) supplies a proposed basis for navigation, search and documentation components. [Astro's GitHub deployment guide](https://docs.astro.build/en/guides/deploy/github/) describes deployment to Pages. The implementation must account for the repository subpath and verify asset and documentation links in that deployment context.

[GitHub Pages](https://docs.github.com/en/pages/getting-started-with-github-pages/what-is-github-pages) hosts static HTML, CSS and JavaScript. A browser demonstration can execute the bounded Rust operation through WebAssembly. The [Rust to WebAssembly guide](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm) provides an implementation starting point. Browser-side code must reuse the kernel semantics.

A backend harness would run locally or in a separately hosted service. The documentation site does not supply that backend. The [website specification](website-specification.md) owns its learning paths and publication criteria, while [Design](design.md) owns the proposed presentation.

## Repository structure

The [index](INDEX.md) inventories the actual files. The following additions describe a possible evolution and do not exist yet.

| Stage | Candidate addition | Reason |
|---|---|---|
| Reference model | Root Cargo package with `src/`, `examples/` and focused tests | Keep one operation easy to inspect and run |
| Documentation website | `site/` and a Pages workflow under `.github/workflows/` | Publish maintained explanations and examples |
| Browser demonstration | A WebAssembly adapter around the same library | Teach the operation without duplicated semantics |
| Embedded engine and harness | Additional modules or packages as needed | Separate responsibilities after their contracts are demonstrated |

## Build and deployment

There is currently no Rust build, website build or deployed database service. The first implementation will record the actual toolchain and reproducible commands in [Testing](testing.md).

The proposed site deployment uses a [GitHub Actions Pages workflow](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages). Publication requires a successful build, link and deployment-path checks, and review of the rendered pages.
