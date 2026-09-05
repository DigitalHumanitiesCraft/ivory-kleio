---
title: Website specification
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
related: [project, specification, architecture, design, testing]
---

# Website specification

The website publishes the Ivory KLEIO research specification and supporting documentation in English. Its purpose is to explain the subject, defined behavior, source evidence and current implementation boundary.

The website source is implemented under `site/`. Publication on GitHub Pages remains pending. [Architecture](architecture.md) owns technical details, and [Design](design.md) records the specification format requested by the project owner.

## Content and navigation

The entry page states document and software status, purpose, scope and the first experimental contract. It routes readers to the maintained specification.

| Section | Contents |
|---|---|
| Project | Specification overview, project scope, temporal explanation and kernel/harness responsibilities |
| Specification | Data and operations, architecture and the temporal acceptance case |
| Research | Scientific proposal, objectives, methodology, sources and reading records |
| Development | Contribution guidance and website build instructions |

Database installation instructions and executable tutorials follow working software. The current temporal page is an explanatory acceptance example.

## Canonical content

The maintained knowledge base owns research purpose, requirements and architecture. The preparation script renders selected Markdown files and the corpus CSV into ignored site content. It preserves source links and maps repository cross-references to their public destinations where those pages exist.

Introductory pages are separately authored for their explanatory task. They must agree with the canonical documents. Generated pages are refreshed before development, checking and production builds.

The public corpus preserves its recorded reading states. The handoff inbox and journal remain repository working documents rather than primary website navigation.

## Temporal example

The example identifies invented source wording, the selected interpretation, a precise query and its expected result. Exhaustive alternatives and an inclusive interval remain distinguishable. A positive possibility answer does not establish that the event actually occurred in the queried year.

A future executable browser demonstration will use the Rust operation through WebAssembly. Its result must identify the same inputs, interpretation and rules as the reference model.

## Acceptance

- The entry page identifies purpose, scope, document status and available software.
- Headings and text state concrete information without promotional framing.
- Canonical content and reading states remain traceable to repository sources.
- Page links, fragments and assets resolve under the GitHub Pages repository subpath.
- Search indexes the published documentation.
- Narrow and wide layouts support readable text, usable tables and keyboard navigation.
- Tutorials reproduce actual commands and results at a documented implementation revision.

The build and link checks provide technical evidence. Visual and interaction review provide separate evidence. [Testing](testing.md) records how to run checks, and [Plan](plan.md) records the work position.

## Publication

GitHub Pages is the requested hosting destination. Repository pushes and pull requests run the verification workflow. A manual workflow run can publish after Pages is configured and verification succeeds.

The site does not supply a backend harness or database service.
