---
title: Research methodology
project:
  name: Ivory KLEIO
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
updated: 2026-10-15
knowledge-sources:
  methods:
    ivory-blog: https://ivorytower.hypotheses.org/
    historical-computing: https://doi.org/10.12759/hsr.suppl.29.2017.193-202
related: [project, specification, temporal-specification, testing]
---

# Research methodology

The dissertation establishes the existing conceptual foundation. The [requirements integration](requirements-integration.md) records its passage-level correspondence to both requirement registers and preserves requirements with no direct operational counterpart. Source-specific primary-text analysis establishes further domain requirements. The project distinguishes a historical problem, an author's proposed solution and its own formal interpretation. Selected passages inform the [specification](specification.md), while complete reconstruction remains research work.

## Rationale

Bibliographically inventory the Ivory blog and related works, then read all contributions relevant to historical information representation, processing and implementation in full. Record inclusion and exclusion reasons as the corpus expands. Follow references to earlier source-oriented processing, Kleio manuals and surviving implementations.

Compare the development of concepts across publications. Existing summaries help locate evidence. Attributed requirements require inspection of the primary text. Each attributed claim requires the relevant source analysis. A proposed general baseline needs evidence for its actual scope; the existing dissertation-backed cases can be developed while other source reconstructions remain open. Small executable experiments can accompany reading as explicitly bounded hypotheses.

## Phenomena and their treatment

Distinguish intervals, exclusive alternatives, missingness, linguistic vagueness, contextual interpretation and dependencies. Probability, possibility and linguistic uncertainty each require a stated meaning. Their combination needs justification.

Investigate event time, validity time, source recording time, research decision time and system time as potentially different dimensions. Establish which dimensions a case needs and how they relate. These distinctions are inherited project questions, not a fully reconstructed taxonomy attributed to the primary corpus.

For each operation, define what its inputs mean, which context it needs, its result vocabulary and how it composes with later operations. Preserve counterexamples involving dependent hypotheses or mutual exclusion. Independence, distributivity and ordinary equality semantics must be justified for the chosen model. A deterministic evaluator can retain uncertainty in its output; reproducible computation supplies no additional certainty about the historical event.

Before implementation, state expected cases and relevant equivalence relations independently of the evaluator. Finite model comparison can test an experimental formalization against a different representation. Its conclusion remains bounded by the generated models and declared assumptions. [Temporal specification](temporal-specification.md) owns the current experiment, and [Testing](testing.md) owns how its claims are checked. Fixed test expectations change only after the underlying interpretation or contract has been reviewed and the reason recorded.

## Evidence and requirement mapping

| Evidence class | Treatment |
|---|---|
| Explicit source requirement | Attribute a faithful paraphrase or short quotation to a precise passage |
| Author-proposed implementation | Assess it as a proposed solution to an identified problem |
| Project inference | State the inferential step and motivating case |
| Synthetic case | Label invented inputs and fix the query before implementation |
| Historical case | Identify the source location, interpretation and reuse conditions |

Each requirement record needs a stable ID, source or project rationale, passage locator where applicable, historical problem, scope, formal meaning, counterexample and expected observable behavior. Link it to experiments and acceptance evidence as these become available. The H01–H11 register belongs to [Specification](specification.md).

A reading record contains a source ID, consulted representation, inspected passages, paraphrase, unresolved interpretation and requirement links. Reading notes supply the evidence behind the register.

## Source records

The [corpus CSV](../research-artefacts/corpus.csv) owns source identities and reading states. Its vocabulary is `identified`, `partial`, `secondary_only` and `fully_analyzed`. Full analysis requires complete relevant reading and an integrated account of the implications.

Distinguish the original publication from later access copies. A local acquisition records its representation URL, retrieval date and content hash before it is relied upon. [Source material](../source-material/README.md) owns acquisition and reuse conventions.

## Unresolved research

The basic information unit, compatibility of uncertainty calculi, dependencies between interpretations and meaning of conditional execution remain open. Neither a factoid model nor a graph representation determines the answer in advance.

The 2020 record needs completion. The 2018 contribution, graph presentation and Kleio documentation need direct primary analysis. The dissertation has been read through its substantive chapters for the common requirement mapping. Remaining historical fixture work concerns exact source acquisition and declared operations, rather than repeating the conceptual foundation. The dissertation reading and its integration are recorded in the source inventory; other corpus reading states remain unchanged.

## Comparative evaluation

Start with a documented dissertation example and establish its exact historical source location and reuse conditions before adopting it as a fixture. Include contrasting source forms when testing generality.

Compare relational, RDF or property-graph implementations using equivalent information, contexts and operations. Select concrete systems after the requirements are precise enough to make the comparison informative. Assess semantic preservation and modelling effort as well as performance.

Hold the kernel and source cases fixed when evaluating the harness. Examine whether researchers can understand proposals, identify errors and revise decisions. A successful core test does not establish historical adequacy or prove that an agent interface improves scholarly work.
