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

Documentation integrity, operation behavior and scholarly validity require different evidence. [Specification](specification.md) owns expected behavior. This document owns verification procedures.

## Website commands

Use Node.js 24 and npm. From the repository root:

```sh
cd site
npm ci
npm run check
npm test
npm run build
npm run check:links
```

Development uses `npm run dev`. The preparation hooks regenerate public content from canonical documents before development, checking and builds. Restart development after changing canonical documents outside the site.

The content tests check source attribution, removal of duplicate titles and resolution of canonical cross-references. The built-link checker inspects page landmarks, local links, fragment destinations and assets under `/ivory-kleio/`.

Run the production build to completion before accepting the link-check result. Pagefind and the sitemap are part of that build. The current session's verification state and remaining issues are recorded in [Plan](plan.md).

## Documentation checks

Check knowledge metadata, index coverage and repository-relative links. Validate corpus structure and reading states against recorded evidence. Run `git diff --check`. Public files must contain no machine-specific paths or unresolved private-vault links.

The index owns the common knowledge schema version. Generated website Markdown remains ignored and must be regenerated from its canonical source.

## Website review

The [website specification](website-specification.md) defines acceptance. Review the rendered specification entry, a reading page and the temporal example on narrow and wide screens. Verify keyboard navigation, focus, theme contrast, tables and search.

A static explanatory example must remain identifiable as an acceptance case. An executable browser operation will later need agreement with the Rust reference fixtures.

## Reference model acceptance

The [temporal case](../research-artefacts/cases/temporal-alternatives.md) independently specifies the expected results. When implemented, check the difference for 1705, endpoint agreement, exclusion outside represented possibilities and explicit result explanations.

Save and reload must preserve identifiers, wording, domain and temporal variant. Invalid or unsupported inputs must yield declared errors. Add property checks only for justified properties.

## Engine and harness evaluation

An engine must reproduce specified queries after reopening a collection. Add revision, interrupted-write and recovery checks with the corresponding mechanisms. Compare indexed queries against the reference evaluator.

The [agent case](../research-artefacts/cases/agent-interpretation.md) requires recorded proposals, invocation context, formal validation and scholarly decisions. A repeated model call is a new activity and may produce a different proposal.

## Evidence boundary

A passing document check establishes consistency. An operation test establishes behavior within its defined scope. Historical validity requires primary evidence and domain review. Comparative claims about performance or usability require separate evaluation.
