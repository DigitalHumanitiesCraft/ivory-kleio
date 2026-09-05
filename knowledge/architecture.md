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

The historical kernel, persistence model and agent harness remain proposed components. The documentation website is implemented with Astro and Starlight. Rust is selected for the semantic core.

## Components

| Component | Responsibility | State |
|---|---|---|
| Reference model and kernel | Historical values, explicit contexts, operations and derivations | Proposed |
| Persistence and query execution | Durable collections, revisions and query evaluation | Proposed |
| Agent harness | Context selection, typed tool calls, workflow state and recorded feedback | Proposed |
| LLM | Candidate interpretations, query plans and explanations | Proposed integration |
| Research interface | Source inspection, alternatives and scholarly decisions | Proposed |
| Documentation website | Specification, research records and explanatory examples | Implemented source |

The kernel remains callable independently of an LLM. These responsibilities do not imply separate packages.

## Historical data flow

A research request becomes an inspectable proposed operation. The harness supplies evidence, interpretation and explicit context. The kernel validates formal preconditions and evaluates the operation. A result retains input references and applicable rules. The researcher records how an interpretation is used.

Invalid input, unsupported or undefined operations, absent evidence and valid results require distinct outcomes. A generated proposal, formal validation and scholarly adoption remain separate records.

The relationships between token, interpretation, assertion and entity remain research questions. A versioned assertion graph is a candidate structure. A graph does not by itself define uncertainty semantics.

## Reference model and persistence

The [minimum prototype](specification.md#minimum-prototype) proposes a Rust library with a callable example, tagged temporal values and JSON snapshots. Snapshot round trips test bounded semantic preservation. Transactions, recovery, indexing and general query execution require later engine mechanisms.

Existing storage components can carry the semantic kernel if they satisfy its requirements. Optimised queries must agree with a simple reference evaluation. Uncertain conditions may require a dedicated evaluator with rules distinct from ordinary host-language Boolean branching.

Distributed conflict resolution concerns synchronisation. It does not determine the scholarly meaning of contradictory sources.

## Agent integration

The [agent case](../research-artefacts/cases/agent-interpretation.md) defines a later evaluation target. Calls retain invocation identity, input references, available configuration and actual output. Repeated model calls can produce different proposals.

Source documents are research input and do not gain authority to change tool permissions. The harness invokes declared kernel operations. Credentials remain outside tracked files and public browser bundles.

## Website implementation

The site uses [Astro](https://docs.astro.build/en/guides/deploy/github/) and [Starlight](https://starlight.astro.build/) with npm and Node.js 24. Exact dependencies are declared in [package.json](../site/package.json) and the committed lockfile.

The source preparation script reads canonical repository documents and the corpus CSV. It emits ignored Markdown under `site/src/content/docs/generated/`, with source attribution and mapped cross-references. Authored explanations remain in `site/src/content/docs/`.

Shared Astro components and a custom stylesheet implement the specification layout. Pagefind supplies static search. Font packages are bundled locally. There are no model API calls in the website.

## Repository structure

| Location | Contents |
|---|---|
| `knowledge/` | Maintained project knowledge |
| `research-artefacts/` | Corpus, proposal, reading notes and acceptance cases |
| `site/src/` | Authored pages, components and styles |
| `site/scripts/` | Source synchronisation, content checks and built-link validation |
| `site/dist/` | Ignored static output |
| `.github/workflows/pages.yml` | Verification and manual publication workflow |

Rust packages will be added with the first executable experiment. A future WebAssembly adapter will reuse the same core operation in the browser.

## Build and deployment

[Testing](testing.md) owns commands and verification boundaries. [GitHub Pages](https://docs.github.com/en/pages/getting-started-with-github-pages/what-is-github-pages) hosts the static output under `/ivory-kleio/`. A [GitHub Actions workflow](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages) checks pushes and pull requests. Deployment requires a manual run and configured Pages settings.

A backend harness would run locally or in a separately hosted service. The documentation site supplies no such backend.
