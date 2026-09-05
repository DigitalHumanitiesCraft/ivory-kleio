---
title: Design
project:
  name: Ivory KLEIO
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
template:
  name: Vorlage Design
  version: "0.2"
  url: https://dhcraft.org/Promptotyping/promptotyping-document/design
status: draft
language: en
created: 2026-09-05
updated: 2026-09-05
related: [website-specification, architecture, testing]
---

# Design

The website presents a research specification. The project owner requested factual, neutral and precise text and rejected a marketing-style landing page during review of the first local preview.

## Design approach

The entry page states the subject, document status, software status and scope, then links to the formal documents. Headings identify their contents. Claims distinguish implemented behavior, proposed contracts and open research.

The design uses a continuous document layout with section navigation and a table of contents. Tables compare requirements, component responsibilities and expected results. Decorative feature blocks, slogans and promotional calls to action are excluded.

## Design system

Source Serif 4 supplies document headings and the subtitle. DM Sans supplies body text and navigation. Fonts are served from the website. A restrained neutral background, dark text and petrol links form the light palette. A corresponding dark palette follows the reader's theme preference.

Concrete typography, spacing and colour values are maintained in [the stylesheet](../site/src/styles/theme.css). Borders distinguish document sections and table rows. The interface uses square or minimally rounded controls.

## Interaction patterns

Navigation opens specifications, research records and worked examples. The search indexes published text. The temporal example presents documented expected results and identifies its synthetic inputs. It does not imply that a database operation runs in the browser.

The later harness interface requires its own workflow design. Its proposed controls are not part of the current website.

## Review state

The initial marketing presentation was replaced after the project owner's feedback. The revised specification layout still needs visual review on narrow and wide screens and keyboard review. Its implementation status and reproducible checks are recorded in [Plan](plan.md) and [Testing](testing.md).

## Action-layer integration

Website changes follow this direction and the [website specification](website-specification.md). Use precise headings and prose. Keep canonical research claims in their owning knowledge documents, and verify their public rendering after changes.
