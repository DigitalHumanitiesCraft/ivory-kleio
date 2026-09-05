---
title: Project
project:
  name: Ivory Kleio
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
template:
  name: Vorlage Projekt-Wissensdokument
  version: "0.3"
  url: https://dhcraft.org/Promptotyping/promptotyping-document/project
status: draft
language: en
created: 2026-09-05
updated: 2026-09-05
knowledge-sources:
  methods:
    historical-computing: https://doi.org/10.12759/hsr.suppl.29.2017.193-202
    ivory: https://ivorytower.hypotheses.org/88
  standards:
    provenance: https://www.w3.org/TR/prov-o/
    web-annotation: https://www.w3.org/TR/annotation-model/
---

# Project

Ivory Kleio investigates a database technology for historical information and an agentic environment for its scholarly use. The central question is how evidence, context and uncertainty can remain explicit through representation, computation and revision.

## Material basis

Historical financial sources provide the first intended application context through [the dissertation](https://unipub.uni-graz.at/obvugrhs/12127700) and the Bookkeeping Ontology. A specific reusable source collection remains to be selected. The initial [cases](../research-artefacts/cases/) are explicitly synthetic. Primary literature is registered in the [corpus inventory](../research-artefacts/corpus.csv).

## Broader context

The project connects the Ivory programme with source-oriented processing in Kleio. Relevant starting points include *The Need for a Theory of Historical Computing* (1991) and *On vagueness and uncertainty in historical data* (2020). The intellectual connection is a research basis, rather than evidence that a particular graph or storage structure has already been established.

## Research purpose

The project has two connected research contributions.

First, reconstruct the requirements of historical information, specify appropriate data types and operations, and evaluate an executable database kernel.

Second, investigate whether an agentic environment helps researchers formulate, inspect and revise work over that kernel while preserving source attribution and competing interpretations.

The initial hypothesis is that a shared semantic core can preserve the meaning of historical values across processing chains. Its contribution must be assessed against carefully implemented alternatives.

## Standards

[PROV-O](https://www.w3.org/TR/prov-o/) and the [Web Annotation Data Model](https://www.w3.org/TR/annotation-model/) are candidate interoperability references for derivations and source locations. Their relevance does not determine the kernel's internal representation. A mapping will require an explicit semantic contract.

## Technical implementation

Rust is the selected language for the semantic core. The [architecture](architecture.md) describes the proposed boundary between the core, persistence, agent harness and scholarly interface. Physical storage and the harness implementation remain open choices.

## Functional scope

The intended system will represent historical values with their contexts, perform explicit operations, preserve revisions and expose inspectable results. Agent tools should help retrieve evidence, propose interpretations and compare outcomes under different assumptions.

The [specification](specification.md) owns candidate requirements and acceptance behavior.

## Boundaries

The first executable research target is a reference model. A persistent embedded engine follows from demonstrated behavior. A scholarly interface develops from observed research tasks. Distributed operation, a new general programming language and dedicated physical storage each require separate justification.

## Terms

Ivory KLEIO is the working project name and `ivory-kleio` is the repository name. KLEIO expands to Knowledge-Led Exploration, Interpretation and Operationalisation, qualified by the domain of historical information. Ivory retains its connection to the published research programme. Compatibility with historical Kleio formats would be a separately specified feature.

## License

Original project text uses [CC BY 4.0](../LICENSE), and project code uses [MIT](../LICENSE-CODE). Source-specific reuse conditions remain attached to the materials.
