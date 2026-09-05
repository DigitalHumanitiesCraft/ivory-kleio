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

The proposed website should read as a scholarly software publication. Source evidence, readable explanations and inspectable operations guide the design.

This brief is a draft direction from the project discussion. It has not been validated through rendered pages or user observation. [Website specification](website-specification.md) owns content and acceptance, while [Architecture](architecture.md) owns the proposed stack.

## Design approach

Use generous reading space and a clear hierarchy. Introduce the historical problem through an example, then reveal the formal distinction and operation. Keep citations close to the claims they support.

The overview can have a distinctive project identity. Documentation pages need predictable navigation and enough space for sources, code and result explanations. Show project status plainly wherever an unavailable capability could otherwise be inferred.

## Design system

A warm ivory background, dark text and a restrained petrol accent form the proposed palette. The accent supports navigation, focus and selected states. Contrast must be checked on rendered pages.

Serif headings can connect the research presentation to scholarly reading. A legible sans-serif face serves navigation and body documentation, with a monospace face for code and exact values. Concrete font choices and design tokens belong to the implementation after visual review.

Use a compact top navigation, section navigation on the left and a page contents panel on the right where space permits. Narrow layouts should retain a clear route through the document and place supplementary navigation behind accessible controls.

Tables explain comparisons. Code blocks show real commands or labelled proposed structures. Status text distinguishes experimental contracts from implemented behavior.

## Interaction patterns

The temporal example keeps source wording, interpretation, query and result visibly connected. The selected interpretation must remain explicit when the query changes.

The result panel displays the answer together with the applied rule and input values. An error explains which input is invalid. Neither a negative answer nor a validation error should visually imply a scholarly judgement beyond the declared operation.

A future harness interface should make proposed actions, formal results and scholarly decisions individually inspectable. Its detailed design follows observed research workflows and is outside the initial documentation implementation.

## Visualisation logic

Use aligned alternatives and interval bounds to show the temporal distinction. Any graphic must have an equivalent textual explanation. An interval drawing must not imply that exhaustive alternatives include the intervening years.

Use diagrams where they explain dependencies or component boundaries. Source and derivation views should retain accessible text and links to their evidence.

## Action-layer integration

When implementing or changing the website, read this brief together with the website specification. Use the actual rendered result to assess the proposed direction, then record any adopted design choices here and their concrete values in code.

Check keyboard navigation, visible focus, contrast, narrow-screen reading and the clarity of example states. Keep the current draft status until those checks and design review provide evidence for a stronger claim.
