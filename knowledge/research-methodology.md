---
title: Research methodology
project:
  name: Ivory Kleio
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
template:
  name: Vorlage Domänenwissen
  version: "0.3"
  url: https://dhcraft.org/Promptotyping/promptotyping-document/domain-knowledge
status: draft
language: en
created: 2026-09-05
updated: 2026-09-05
knowledge-sources:
  methods:
    ivory-blog: https://ivorytower.hypotheses.org/
    historical-computing: https://doi.org/10.12759/hsr.suppl.29.2017.193-202
related: [project, specification, testing]
---

# Research methodology

Primary-text analysis establishes the domain requirements. The project distinguishes a historical problem, an author's proposed solution and its own formal interpretation. Selected passages already inform the [specification](specification.md), while complete reconstruction remains research work.

## Rationale

Bibliographically inventory the Ivory blog and related works, then read all contributions relevant to historical information representation, processing and implementation in full. Use backward references to trace earlier source-oriented processing and Kleio. The initial inventory is a starting set, not the complete corpus.

## Phenomena and their treatment

Record intervals, exclusive alternatives, missingness, linguistic vagueness, contextual interpretation and dependencies as distinct candidate phenomena. For each, identify what the source explicitly claims and what the project infers.

A reading record contains a source ID, consulted representation, passage locator, paraphrase, open interpretation and links to candidate requirements. Preserve counterexamples and unresolved terminology.

## Mapping tables

| Evidence class | Interpretation in project work |
|---|---|
| Explicit source requirement | Attribute the wording or faithful paraphrase to a precise passage |
| Author-proposed implementation | Record as a solution proposal that can be assessed |
| Project inference | Mark the inferential step and the case that motivates it |
| Synthetic case | Label invented inputs and define the query before implementation |
| Historical case | Identify the real source location and reuse conditions |

## Header and schema declarations

The [corpus CSV](../research-artefacts/corpus.csv) owns source identities and reading status. Its `reading_status` vocabulary is `identified`, `partial`, `secondary_only` and `fully_analyzed`. A note registers the inspected sections. Full analysis requires both complete relevant reading and an integrated account of its implications.

Original publication and later access copies share a work ID but have separately described representation URLs. A local acquisition records its content hash and retrieval date in its source record before it is relied upon.

## Unresolved phenomena

The basic unit, compatibility of uncertainty calculi, dependence between interpretations and meaning of conditional execution remain open. Algebraic properties such as distributivity or independence are established for a specified model rather than assumed.

The 2020 reading record needs completion. The 2018 contribution, Kleio system description and tutorial need primary-text analysis. The dissertation's role must be grounded in chapter-level evidence beyond the accessible abstract and earlier excerpts.

## Project-wide conventions

Use the requirement IDs in the specification as the authoritative register. Reading notes supply evidence, and tests reference the relevant IDs.

Select a historical financial source for the demonstrator, then include contrasting source forms when evaluating generality. Compare implementations using the same information, contexts and rules. Evaluate a harness separately from core semantics by keeping the underlying core and cases fixed.
