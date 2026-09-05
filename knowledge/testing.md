---
title: Testing
project:
  name: Ivory KLEIO
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

Documentation integrity, formal operation behavior and scholarly validity require different evidence. [Specification](specification.md) owns expected behavior. This document owns how to assess it.

## Documentation checks

For the current repository, check required knowledge metadata, nested project and method fields, process statuses, the index inventory and repository-relative Markdown links. Parse the corpus CSV and check its reading-state vocabulary and note paths. Review source claims against the recorded evidence.

Run `git diff --check` for whitespace errors. Check that public files contain no machine-specific paths or unresolved private-vault links. The index alone owns the common knowledge schema version.

## Reference model acceptance

The [minimum prototype](specification.md#minimum-prototype) defines a proposed bounded contract. The [temporal case](../research-artefacts/cases/temporal-alternatives.md) supplies an independently stated result table.

When implemented, meaningful checks must establish that the exhaustive alternatives and inclusive interval give different answers for 1705, agree at the endpoints, and reject years outside their represented possibilities. Check that structured explanations identify the input, interpretation and rule used.

Test save and reload for preserved identifiers, original wording, temporal variant and operation result. Invalid intervals, empty alternatives, unsupported variants and unknown snapshot versions must produce the declared errors. Parsing failure must not become a historical false result.

Add property-based checks only for justified properties. Do not weaken a case's expected result to fit an implementation.

## Engine and harness evaluation

An engine must reproduce specified queries after closing and reopening a collection. Add revision, interrupted-write and recovery checks with the corresponding persistence mechanisms. Compare indexed evaluation against the reference operation.

The [agent case](../research-artefacts/cases/agent-interpretation.md) belongs to later harness evaluation. Inspect the recorded model proposal, invocation context, formal validation and scholarly decision. Test whether a revised assumption leads to identifiable affected computations.

A retained invocation supports audit of what occurred. It does not guarantee identical output from a new model call. Core reproducibility uses stored values, explicit context and rule versions.

## Website acceptance

The [website specification](website-specification.md#acceptance) defines publication criteria. Once implemented, run the production build and check repository-subpath links and assets. Review the rendered landing page, reading page and temporal example on narrow and wide screens, including keyboard navigation.

Verify browser results against the same reference fixtures used by Rust. Tutorials must reproduce actual commands and expected output at their documented revision. A static explanatory table must not be presented as a running demonstration.

## Evidence and limits

The current repository has synthetic cases and documentation. No implemented Rust operation, database engine, harness or website has yet been evaluated.

A passing document check establishes consistency. A passing operation test establishes the specified behavior within its scope. Historical validity requires source interpretation and domain review. Claims of improved performance or usability require comparative evidence.

## Executable commands

The only current generic repository command is `git diff --check`. Compilation, test and website commands will be documented here when their build configuration and implementation exist. [Plan](plan.md) records the work position, and [Journal](journal.md) records substantive transitions.
