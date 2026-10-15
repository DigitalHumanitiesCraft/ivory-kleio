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
updated: 2026-10-15
related: [project, specification, plan, handoff]
---

# Journal

Entries record substantive transitions and the evidence behind them. Future work is maintained in [Plan](plan.md), and incoming deltas are processed through [Handoff](handoff.md).

## Entries

### 2026-10-15 integrated dissertation and requirement consolidation

The owner commissioned consolidation of the existing dissertation, hist-info-model and Ivory KLEIO. Chapters 1–7 of the published 2025 dissertation were read against the two requirement registers. The [source record](../research-artefacts/reading-notes/dissertation-requirements.md) identifies the consulted PDF. [Requirements integration](requirements-integration.md) now owns the complete R/H correspondence, passage locators, existing examples, evidence boundaries and specific open questions. The dissertation supplies the foundation; the primitive reconstruction and comparative gap claims retain hypothesis status.

The theoretical repository's current knowledge, navigation and manuscript outline were brought into agreement. The local knowledge model and operational contract retain distinct responsibilities. The source inventory now records the relevant dissertation analysis, while the other primary-source reading states remain unchanged. Historical fixture adoption and scholarly acceptance remain open. The Rust implementation and temporal contract were inspected for the mapping; their semantics were not changed.

Independent review checked source claims, identifier coverage, preservation of theory examples and the temporal implementation boundary. Integration repaired missing reverse correspondences and replaced an unverified DOI association with the published record/URN. Targeted checks cover every R/H identifier, both mapping directions, current knowledge links/anchors and code fences. The theoretical data validator, website content tests, Astro diagnostics, static build and built-link check pass. The pre-existing overlapping 404-route warning remains. No deployment or publication was performed.

### 2026-09-17 integrated deterministic testing and knowledge refactor

The project owner requested implementation of the deterministic test strategy and a refactor of the knowledge base. Separate GPT-6 tasks authored finite-model/reference tests and process tests; another task reviewed their oracles, contracts and document boundaries. The integration pass exercised the combined result.

The temporal contract moved from the general specification into [Temporal specification](temporal-specification.md) without losing its input, error, provenance or acceptance obligations. It now states determinism and the equivalence relation for each comparison. The general specification retains H01–H11, adoption criteria and a short `minimum-prototype` route. Its duplicate decision table became an ownership map. [Index](INDEX.md) identifies six core documents; website documents retain their separate purpose, and the handoff inbox contains only its empty state. Material descriptions remain in the corpus, cases and fixtures. No additional governance, data or verification document was justified by this experiment. Agent routing, contributing guidance, canonical cross-references and the website map/sidebar were updated together.

The command-line example became the Cargo binary `ivory-temporal` so integration tests can invoke the built process directly. Isolated temporary directories make those tests repeatable. Fixed v1 inputs and independently written structured expected results check both serialization and evaluation. The generated model tests enumerate all nonempty alternative subsets and inclusive intervals in the windows 1–8, 1700–1707 and 9992–9999, with selected permutations, neighboring queries and domain endpoints. Bitmasks and materialized sets provide independent oracles. Review corrected success-path CLI comparisons from bytes to structured JSON equality; unchanged input and protected destination files retain byte-preservation assertions.

Observed local evidence uses Rust 1.98.1 on Linux through WSL. All 31 tests passed, including 8 real-process tests, 4 finite-model tests, 3 fixed-reference tests and the existing 16 contract tests. Repeated runs passed with `--locked --offline`. Formatting and Clippy with warnings denied also passed. The generated domains contain 765 nonempty subsets and 108 intervals. The evidence is exhaustive within those windows and declared query sets, with selected orderings; it is not a proof over all interpretations of the full year domain.

Cargo-mutants 27.1.0 audited the configured library scope in scratch copies with an unmutated baseline. Of 20 generated mutations, 17 compiled and were caught by failing tests; 3 were unviable and supply no detection evidence. There were no surviving or timed-out mutations. The standard error-source chaining method is excluded from this audit's semantic scope. The report is reproducible under ignored `target/mutation-audit/`; its mutation set does not enumerate all possible defects. The normal cross-platform workflow and a separate manual mutation workflow encode the procedures. Hosted CI and native Windows execution have not been observed.

The website's type check and three content tests passed. Its complete production build produced the new temporal-contract route, search index and sitemap. The existing 404 canonical mismatch was reproduced and corrected in that page's head metadata to point to 404.html; the built-link checker then passed for 20 pages and 1000 local links/assets. The framework still warns about overlapping 404 routes. Visual/keyboard acceptance and publication remain open. This transition added no primary-source reading or scholarly acceptance.

### 2026-09-17 integrated temporal reference model

The project owner authorized implementation of the bounded dating experiment and the use of GPT-6 subagents. Implementation, contract-test authorship and conceptual/code review were assigned separately. The integration pass compiled and exercised their combined result. The existing temporal case supplied the semantic oracle; this work added no primary-source reading.

[Temporal specification](temporal-specification.md#data-contract-v1) now defines the versioned JSON shape, synthetic domain, rule identifier, result fields and errors. A single Rust crate implements raw temporal records, semantic validation and `could_occur_in_year`. The file-based example saves and reopens a snapshot and compares both records and complete results. The fixture retains the invented wording and locators of `CASE-TIME-001`. The compiler is pinned to Rust 1.98.1, and Cargo.lock fixes dependency resolution.

Independent review identified an unspecified integer representation boundary, Serde's positional-array decoding and an overly broad serialization-validation claim. The contract now states the signed 32-bit representation and distinguishes parser failures from out-of-domain years. The supported loader rejects positional arrays while preserving duplicate-field diagnostics. Both the contract and API documentation identify `Snapshot::from_json`, `Snapshot::to_json` and the query function as validated entry points. Generic Serde conversion remains structural. Focused rereview found these issues resolved.

Local evidence was obtained on Linux through WSL. All 16 contract tests passed, including the full finite year domain, singleton boundaries, malformed inputs and exact source preservation. Formatting and Clippy with warnings denied passed. The actual file example returned false for alternatives and true for the interval at 1705, with `round_trip_verified: true`. Additional CLI checks rejected invalid records, invalid queries including an empty collection, and missing input without creating an output. An existing destination remained byte-for-byte unchanged. Reopening a previously saved snapshot and processing a valid empty snapshot also succeeded. The website's three content transformation tests passed after its status text was updated.

The Rust workflow is configured for Linux and Windows, but hosted CI and native Windows execution have not been observed. The website production build and publication remain open. Evidence for H05 covers one possibility predicate; H08 covers a JSON round trip. General database behavior, historical adequacy and scholarly acceptance remain outside the verified result. The maintained specification, architecture, testing document, README and website source now describe that same boundary.

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
