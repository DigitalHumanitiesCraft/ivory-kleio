---
title: Plan
project:
  name: Ivory Kleio
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

The next research task is to reconstruct the relevant primary corpus and establish precise requirements. The implementation sequence depends on that evidence.

## Target

Develop an executable model of historical information, then a persistent kernel and an agentic research workflow evaluated on explicit source cases.

## Phases and milestones

| Phase | Work | Evidence for proceeding |
|---|---|---|
| Source reconstruction | Complete the relevant primary reading and terminology comparison | Passage-level evidence and justified requirement candidates |
| Reference semantics | Formalize a bounded value and its operations in Rust | Executable cases and stated algebraic properties |
| Database kernel | Add persistence, query execution and revision behavior | Reopened collections and reproducible queries |
| Agentic research | Connect a harness through explicit operations | Attributed proposals, inspectable outcomes and revision workflows |
| Evaluation | Compare alternatives on the same source cases | Evidence for each claimed improvement and its limits |

Small semantic experiments can accompany the reading while retaining their hypothesis status.

## Work position

The documentation foundation is published in [DigitalHumanitiesCraft/ivory-kleio](https://github.com/DigitalHumanitiesCraft/ivory-kleio). The local main branch tracks the verified origin. The project owner created the public repository, and the initial research history was published on 2026-09-05.

Next, expand the [corpus inventory](../research-artefacts/corpus.csv) and complete the [2020 reading record](../research-artefacts/reading-notes/thaller-2020-vagueness.md). Read the 2018 contribution and Kleio documentation, then revise candidate requirements using precise passage locators.

## Open decisions and dependencies

The domain model, uncertainty semantics, storage architecture and harness runtime remain open. A historical demonstrator requires an identified source and recorded reuse conditions. The first Rust experiment requires a selected available toolchain and a defined operation.

## Deviations

The first repository is a documentation foundation. The previously sketched Cargo workspace follows with the first justified executable experiment, so its initial checks can exercise actual behavior.
