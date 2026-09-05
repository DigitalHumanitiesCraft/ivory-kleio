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

The repository contains the research foundation and an Astro/Starlight website implementation. The entry page was revised into a neutral research specification following the project owner's feedback. The Rust kernel and harness remain unimplemented.

## Work position

The session closed on 2026-09-05 at the project owner's request. Type checking and the content transformation tests passed. The local development route returned HTTP 200.

The production build generated HTML but failed while starting Pagefind. The final local error was `spawn EFTYPE`, following an earlier `spawn EBUSY` during dependency installation. The output is incomplete and must not be published.

The built-link check reports the missing sitemap from that incomplete build and a 404 canonical destination mismatch. Visual review of the revised specification layout and keyboard review remain pending.

GitHub Pages has not been configured or published. The workflow checks pushes and pull requests. Publication requires a manual run after the build and review criteria are satisfied.

## Next work

1. Restore and verify the local Pagefind installation using the committed package manifest and lockfile. Complete the production build, resolve the 404 canonical destination and rerun built-link checks.
2. Review the neutral specification layout on narrow and wide screens, including navigation, search and keyboard use. Configure GitHub Pages and publish through the manual workflow after verification.
3. Expand the [corpus inventory](../research-artefacts/corpus.csv) and complete the relevant primary reading. Ground dissertation-derived requirements in chapter-level evidence.
4. Implement the proposed [minimum prototype](specification.md#minimum-prototype) with an explicitly selected Rust toolchain and the specified acceptance cases.

## Research dependencies

Adoption of a domain baseline depends on systematic reconstruction of the relevant primary corpus. Small semantic experiments can accompany that reading as bounded hypotheses. Browser execution follows a tested Rust operation. Harness integration follows an explicit tool contract.

The general object model, uncertainty semantics, storage architecture and harness runtime remain open. A historical demonstrator needs an identified source and reuse conditions.

## Continuity

[Testing](testing.md) owns commands and their limits. [Journal](journal.md) records the website implementation and the owner's design correction. The handoff inbox is reserved for incoming material still awaiting integration. The continuation work above is already integrated.
