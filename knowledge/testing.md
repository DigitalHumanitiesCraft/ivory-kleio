---
title: Testing
project:
  name: Ivory Kleio
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
template:
  name: Vorlage Testing
  version: "0.3"
  url: https://dhcraft.org/Promptotyping/promptotyping-document/testing
status: draft
language: en
created: 2026-09-05
updated: 2026-09-05
related: [specification, architecture, plan]
---

# Testing

A check establishes a specified property. Documentation integrity, formal operation behavior and scholarly validity require different evidence. The [specification](specification.md) identifies the behavior to assess.

## Test strategy

Validate document metadata, internal links, the index inventory and CSV structure for the documentation foundation. Review reading-status claims against the recorded source evidence.

For the executable reference model, implement the smallest meaningful examples and counterexamples for each operation. Add property-based checks only for properties justified by the model.

## Guarantees

A successful metadata or link check establishes document consistency. A successful core test establishes the specified behavior on its defined cases or properties. Scholarly validity requires a separate assessment of the source interpretation and the operation's relevance.

## Acceptance

The [temporal case](../research-artefacts/cases/temporal-alternatives.md) fixes a query and expected results before code. The [agent case](../research-artefacts/cases/agent-interpretation.md) fixes provenance and decision boundaries.

An engine experiment must preserve specified query behavior across closing and reopening a collection. Test revisions and interrupted writes when persistence is introduced. Compare indexed queries with a simple reference evaluation.

## Evaluation boundary

A stored proposal and invocation record support audit of what occurred. They do not guarantee that a new model invocation recreates that proposal. Core reproducibility uses explicit stored values, context and rule versions.

## How to run

For the documentation foundation, run `git diff --check`, parse the knowledge frontmatter and corpus CSV, and resolve repository-relative Markdown links. Confirm that the index covers every actual knowledge document.

Commands for compiling or testing Rust enter this section with the first implemented experiment and selected toolchain.

## Checks with the corresponding change

Change the acceptance behavior only with a documented semantic reason. Include the associated meaningful check with an implemented operation. A passing test must not be produced by weakening an independently established expected result.

## Case-first workflow

Define the research question, data interpretation, assumptions and expected observable result. Implement the bounded operation and examine counterexamples. Preserve cases that distinguish competing models.

## Known limits

The current cases are synthetic. They establish research questions rather than empirical performance or general historical adequacy. Neither the kernel nor the harness has an implementation in this initial foundation.

## Components

Core evaluation compares semantic preservation, auditability, modelling effort and execution performance. Harness evaluation holds the core fixed and examines whether researchers can understand proposals, identify errors and revise decisions.

## Current evidence

The work position is maintained in [Plan](plan.md), and completed substantive transitions are recorded in [Journal](journal.md). Claims of implemented behavior require reproducible evidence from actual code.
