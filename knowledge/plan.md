---
title: Plan
project:
  name: Ivory KLEIO
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
template:
  name: Vorlage Plan
  version: "0.3"
  url: https://dhcraft.org/Promptotyping/promptotyping-document/plan
status: active
language: en
created: 2026-09-05
updated: 2026-09-05
related: [specification, research-methodology, testing, journal, handoff]
---

# Plan

The documentation foundation is published in [DigitalHumanitiesCraft/ivory-kleio](https://github.com/DigitalHumanitiesCraft/ivory-kleio). The knowledge base now includes the proposed minimum prototype and the public website brief. No kernel, harness or website is implemented.

## Target

Develop an executable model of historical information, then an embedded database kernel and an agentic research workflow evaluated on explicit source cases. Publish documentation that teaches the behavior actually available.

## Phases and evidence

| Stage | Work | Evidence for proceeding |
|---|---|---|
| Source reconstruction | Inventory the relevant primary corpus, complete reading and compare terminology | Passage-level evidence, scope and justified requirements |
| Reference semantics | Implement a bounded value and operation in Rust | Declared contract, distinguishing cases and preserved snapshot semantics |
| Public documentation | Publish project explanation and research basis, then an executable tutorial | Accurate status, usable reading paths and a verified site build |
| Database kernel | Add durable collections, queries and revisions | Reopened collections, recovery evidence and reproducible queries |
| Agentic research | Connect a harness through explicit operations | Attributed proposals, inspectable outcomes and revision workflows |
| Evaluation | Compare implementations and scholarly workflows | Evidence for each claimed improvement and its limits |

Small semantic experiments and website preparation can accompany the reading. Their existence does not establish the general domain model. Browser integration follows a working core operation. Harness integration follows a tested tool contract.

## Next work

1. Expand the [corpus inventory](../research-artefacts/corpus.csv), complete the [2020 reading record](../research-artefacts/reading-notes/thaller-2020-vagueness.md), and reconstruct the 2018 contribution and Kleio documentation. Ground dissertation-derived requirements in chapter-level evidence.
2. Use the [minimum prototype](specification.md#minimum-prototype) as the proposed first implementation experiment. Record the Rust toolchain, implement one library and callable example, and verify the declared cases and JSON round trip.
3. Implement the [website brief](website-specification.md) using the [proposed design](design.md). Record the framework decision and validate the initial rendered pages. Add runnable usage instructions when the reference model exists.

These are separate workstreams with stated dependencies. Current implementation work remains pending.

## Open decisions

The general object model, uncertainty semantics, storage architecture and harness runtime remain open. Astro with Starlight and a WebAssembly demonstration are technical proposals. The visual direction needs review against rendered pages.

A historical demonstrator requires an identified source and reuse conditions. The first temporal experiment deliberately uses synthetic year labels. Its success cannot establish historical calendar handling.

## Work continuity

The public repository connection is verified. [Journal](journal.md) records the documentation consolidation and earlier transitions. [Handoff](handoff.md) contains only incoming changes still awaiting integration.
