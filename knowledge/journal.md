---
title: Journal
project:
  name: Ivory KLEIO
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
template:
  name: Vorlage Journal
  version: "0.4"
  url: https://dhcraft.org/Promptotyping/promptotyping-document/journal
status: active
language: en
created: 2026-09-05
updated: 2026-09-05
related: [project, specification, plan, handoff]
---

# Journal

Entries record substantive transitions and the evidence behind them. Future work is maintained in [Plan](plan.md), and incoming deltas are processed through [Handoff](handoff.md).

## Entries

### 2026-09-05 integrated website source and session close

The project owner requested website implementation, then rejected the first preview's marketing presentation. The entry page was replaced with a neutral research specification describing purpose, scope, implementation status, documents and the temporal contract. The design brief now requires factual, precise language and excludes slogans and promotional feature blocks.

The website uses Astro and Starlight, self-hosted font packages, canonical Markdown generation, a corpus rendering and Pagefind search integration. npm scripts and a GitHub Actions workflow record its verification and manual publication path. The Rust kernel and harness remain unimplemented, and source reading states are unchanged.

The owner then requested a controlled session close, updated project Markdown and ACTIVE-WORK, and commits and pushes in both repositories. Type checking and content tests passed. The production build remained incomplete because the local Pagefind program failed to start. The built-link check additionally identified the resulting missing sitemap and a 404 canonical destination mismatch. Visual review and publication remain in [Plan](plan.md). Local task processes are stopped for closure.

### 2026-09-05 integrated documentation refactor

The project owner requested consolidation after the naming, harness, website and prototype discussion. The knowledge base now uses Ivory KLEIO consistently and separates scientific purpose, requirements, architecture, website content and design. The README and one-pager present the same maintained project scope. Private workspace project notes route to the public repository rather than maintaining a competing specification.

The temporal experiment now has a proposed input and operation contract, expected results and JSON round-trip criteria. The website brief defines staged learning paths, and the design brief records an unvalidated visual direction. Astro with Starlight and WebAssembly remain proposed technical choices. Reading states were preserved, and this transition adds no primary analysis or software implementation.

### 2026-09-05 integrated public repository

The project owner created the public DigitalHumanitiesCraft/ivory-kleio repository. Its empty remote was connected to the local research repository, and the existing history was published on main. The remote branch was checked against the local commit before the repository mapping and work position were updated.

### 2026-09-05 integrated public README

The project owner's request established an English README using the KLEIO expansion. The README now connects the research problem and objectives to the proposed kernel and agent harness, identifies the documentation foundation as the current implementation stage, and routes readers to the existing evidence and knowledge base.

### 2026-09-05 integrated research foundation

The project owner's proposal and follow-up discussion were distilled into the project, specification, architecture and research methodology. The working name is Ivory KLEIO. The owner authorized a public repository under DigitalHumanitiesCraft. Rust remains selected for the core, and an agentic research environment is represented as an architectural hypothesis. The initial source inventory records partial reading honestly. Synthetic temporal and agent-provenance cases make the first acceptance questions explicit.

The knowledge base follows the Promptotyping templates referenced in its headers. Original documentation uses CC BY 4.0 and project code uses MIT according to the owner's standing licensing instruction.
