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

The requested GitHub Pages site explains Ivory KLEIO, presents its scientific basis and teaches available functionality. It should let a visitor move from a historical question to a reproducible operation and an inspectable result.

This is a publication brief. No website or interactive demonstration is implemented yet. [Architecture](architecture.md) owns the proposed deployment stack, and [Design](design.md) owns the visual direction.

## Audiences and learning paths

| Reader | Question | Intended path |
|---|---|---|
| Historian or prospective user | What problem does this solve, and can I try it? | Overview, temporal example, Get started |
| Researcher reviewing the approach | Which claims are grounded, and what remains open? | Concepts, specification, Research |
| Developer or contributor | What can I call, test or extend? | Get started, Reference, contribution guide |

## Content structure

| Section | Purpose | Publication condition |
|---|---|---|
| Overview | Explain the research problem, intended product and actual status | Can publish with the documentation foundation |
| Get started | Run the smallest working example with expected output | Publish executable instructions with the first runnable prototype |
| Concepts | Explain source versus interpretation, context and temporal alternatives | Label the scope and evidence of each concept |
| Guides | Complete a research task such as comparing interpretations or revising assumptions | Each guide must correspond to an implemented workflow |
| Reference | Document formal contracts, data formats, operations and errors | Identify whether a contract is proposed or implemented and at which revision |
| Research | Present the proposal, source inventory, method, limitations and evaluation | Preserve reading states and evidence links |

The initial site can contain Overview, the proposed temporal example and Research. Add navigation sections when they have useful content. The expanded structure is a destination for growth.

## Overview and first example

The landing page should state the project name, descriptive line and current stage, then introduce the temporal distinction using a short comparison. It should offer clear paths to the research argument and the available example.

The explanatory example shows a source excerpt, its explicit interpretation, a query and the justified answer. Before a runnable demonstration exists, present the expected result as a synthetic case.

The first interactive version uses the minimum prototype's two temporal variants. A visitor can change the query year and inspect the Boolean result with its applied rule. Invalid input explains what needs correction. A reset restores the published fixture. The page links to the exact specification and implementation revision.

## Canonical content

The maintained knowledge base owns project purpose, requirement status and architecture. The public specification should be rendered or assembled from those canonical sources with traceable links. The site build must preserve that ownership.

Tutorials and introductory explanations are separately authored for their learning tasks. Verify their commands, fixtures and outputs against the documented implementation. Do not maintain a second editable copy of the requirement register.

Public research material comes from the repository. The internal handoff inbox and working journal are not part of the primary learning navigation. The corpus remains explicit about incomplete reading.

## Acceptance

A publishable site must meet these criteria.

- Readers can identify what is implemented, what is proposed and where the source evidence is recorded.
- Navigation, search where available, and links work under the actual GitHub Pages repository subpath.
- Reading pages and examples remain usable on narrow and wide screens, with visible keyboard focus and meaningful headings.
- Colour supports meaning that is also expressed in text. Code and long tables remain accessible without breaking the page layout.
- Every tutorial has verified prerequisites, commands, expected output and a documented version or revision.
- The browser operation agrees with the Rust reference cases and uses the same semantics.
- The production build and published artifact are checked, including assets and relevant external source links.

These criteria apply as their corresponding pages and interactions are introduced. A static site build alone does not validate a research workflow.

## Product relationship

The site teaches the research environment and can host a bounded browser example. It does not itself provide a backend agent harness. Local or separately hosted research sessions require their own connection, state and tool contracts.

Documentation should explain the actual use of the available tool. Installation instructions, agent controls and database commands will be added with working software.
