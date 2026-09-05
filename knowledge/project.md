---
title: Project
project:
  name: Ivory KLEIO
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

Ivory KLEIO investigates how source evidence, historical context and uncertainty can remain explicit through representation, computation and revision. Its intended product is an agentic research environment with a database kernel for historical information.

## Material basis

The [dissertation on historical financial information](https://unipub.uni-graz.at/obvugrhs/12127700) and the Bookkeeping Ontology supply the first intended application context. A specific reusable historical collection remains to be selected. The current [cases](../research-artefacts/cases/) are synthetic.

The Ivory programme and source-oriented processing in Kleio provide the principal research starting points. The [corpus inventory](../research-artefacts/corpus.csv) records the consulted representations and their incomplete reading states. Systematic reconstruction of the relevant primary corpus precedes adoption of a domain baseline.

## Research purpose

The working hypothesis is that a shared semantic core can preserve the meaning of historical values across processing chains. Its contribution must be assessed against carefully implemented alternatives using equivalent information and rules.

The research has six objectives.

1. Reconstruct the requirements of historical information from primary texts, with passage-level evidence, scope and counterexamples.
2. Specify data types and relationships that distinguish source representations, interpretations and reconstructed entities.
3. Define comparison, arithmetic, aggregation and conditional execution with explicit context and uncertainty semantics.
4. Develop a persistent kernel that supports queries, revisions and recovery of earlier research states.
5. Investigate an agentic environment in which researchers inspect proposals, compare assumptions and revise scholarly decisions.
6. Evaluate semantic preservation, auditability, modelling effort, execution performance and scholarly usability.

These objectives concern both representation and execution. A value that stores alternatives is insufficient if a later operation silently converts them to a continuous interval.

## Research questions

The specification investigates which requirements generalise across source forms, which distinctions need primitive types, and which belong to contextual rules. It also asks how competing readings, jointly applicable properties and contradictory evidence differ, how their dependencies affect operations, and how revisions identify affected results.

The database contribution concerns the semantics exposed by a reusable kernel and the persistence and query mechanisms needed to preserve them. New physical storage or indexing technology becomes a research contribution when the specified workloads justify it.

The harness contribution concerns whether model-mediated work helps researchers understand and revise interpretations over that same kernel. Formal validation establishes conformity to a declared operation. Scholarly adoption remains an attributable research decision.

## Product boundary

A database is a stored collection. A database management system provides the procedures for storing, changing and querying collections. A research tool supports activities such as source inspection or hypothesis comparison. Ivory KLEIO is the project and intended environment encompassing the kernel and its scholarly tools.

The first executable target is a small Rust reference model. A persistent embedded kernel and agent-mediated research workflow follow from demonstrated behavior. The [public website](website-specification.md) explains and teaches the project. Its browser demonstration is a learning interface to a bounded operation.

A general query language, distributed synchronisation, server deployment and dedicated physical storage each need a separate use case. A full scholarly editing platform is outside the initial scope.

## Standards and implementation

Rust is selected for the core. [Architecture](architecture.md) owns the proposed component boundaries and technical options.

[PROV-O](https://www.w3.org/TR/prov-o/) and the [Web Annotation Data Model](https://www.w3.org/TR/annotation-model/) are candidate interoperability references for derivations and source locations. A mapping must state what it preserves or approximates. Neither reference determines the internal representation.

## Name and publication

The [index glossary](INDEX.md#terms) owns the name and KLEIO expansion. The public repository is [DigitalHumanitiesCraft/ivory-kleio](https://github.com/DigitalHumanitiesCraft/ivory-kleio). Repository documentation and the planned website use English.

Original text uses [CC BY 4.0](../LICENSE), and code uses [MIT](../LICENSE-CODE). Third-party materials retain their own terms. The [scientific proposal](../research-artefacts/ivory-kleio-proposal.md) is a concise presentation of the maintained research purpose and specification.
