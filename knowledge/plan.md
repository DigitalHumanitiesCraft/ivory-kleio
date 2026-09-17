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
updated: 2026-09-17
related: [specification, temporal-specification, research-methodology, testing, journal, handoff]
---

# Plan

The bounded Rust temporal experiment and deterministic test strategy are implemented. The knowledge base separates the research requirements from the versioned temporal contract. [Testing](testing.md) owns repeatable procedures; [Journal](journal.md#2026-09-17-integrated-deterministic-testing-and-knowledge-refactor) records observed checks and their limits.

## Work position

The contract, finite-model, fixed-reference and actual process tests pass locally on Linux through WSL. The bounded mutation audit detected every compilable generated mutation in its configured scope. Native Windows and hosted workflow runs remain unobserved. Historical adequacy and scholarly acceptance remain open.

The documentation website passes its type/content checks, production build including search and sitemap, and built-link validation after correcting the 404 page's canonical URL. The former Pagefind installation failure was not reproduced in this checkout. Astro still reports overlapping generated 404 routes while producing the expected 404.html. Visual and keyboard review have not been repeated. No website publication is part of this work.

## Open work

- Review the [temporal contract](temporal-specification.md) for scholarly adequacy within its stated synthetic scope. Select an addressable historical source and reuse conditions before expanding it to a historical demonstrator.
- Expand the [corpus inventory](../research-artefacts/corpus.csv), complete the relevant primary reading and ground dissertation-derived requirements in chapter-level evidence.
- Review the website layout, navigation, search and keyboard use on narrow and wide screens. Publication follows the website acceptance criteria and the owner's authorization.

## Research boundaries

The general object model, broader uncertainty semantics, database persistence architecture and harness runtime remain open. A domain baseline requires systematic source reconstruction. Browser execution must agree with the tested Rust operation; harness integration requires an explicit tool contract. Transactions, revisions and recovery need their own executable cases when implemented.
