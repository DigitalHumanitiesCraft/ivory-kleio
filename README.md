# Ivory KLEIO

*Knowledge-Led Exploration, Interpretation and Operationalisation of Historical Information*

An agentic research environment with a database kernel for historical information.

Ivory KLEIO investigates how source evidence, historical context and uncertainty can remain explicit through computation and revision. Its name connects the [Ivory programme](https://ivorytower.hypotheses.org/88) with source-oriented processing in [Kleio](https://www.uni-trier.de/fileadmin/fb3/GES/Grotum/kleio/HGR-A23.pdf).

## Current status

This repository contains the research foundation, a preliminary source inventory, reading notes and synthetic acceptance cases. Rust is selected for the core. The kernel, agent harness and GitHub Pages website are proposed development targets.

Start with the [scientific one-pager](research-artefacts/ivory-kleio-proposal.md) for the argument, the [knowledge index](knowledge/INDEX.md) for maintained documentation, or the [work plan](knowledge/plan.md) for next work.

## The research problem

A date may admit competing readings. A monetary amount depends on historical units and context. A conclusion can depend on an assumption that later changes. These conditions affect what a computation means.

Consider this synthetic example.

| Recorded interpretation of one event | Could it have occurred in 1705? |
|---|---|
| Either 1700 or 1710, with the alternatives exhaustive | No |
| Any year in the inclusive interval from 1700 to 1710 | Yes |

The query asks about possibility under the recorded interpretation. Converting alternatives into an interval changes the answer. Replacing either with a midpoint discards its uncertainty. The [case](research-artefacts/cases/temporal-alternatives.md) states the assumptions and expected results.

The research therefore concerns data structures and the semantics of operations, queries and persistent storage.

## Research objectives

1. Reconstruct historical information requirements through systematic primary reading of the relevant Ivory and Kleio corpus.
2. Formalise values and contexts, including forms of uncertainty and dependencies between interpretations.
3. Define operations whose results preserve their assumptions and source connections.
4. Develop a persistent kernel with queries, revisions and recoverable research states.
5. Investigate an agentic environment for inspecting proposals and revising scholarly decisions.
6. Compare the approach with alternative implementations using equivalent information and rules.

Historical financial sources provide the first intended application context through the [dissertation](https://unipub.uni-graz.at/obvugrhs/12127700) and the Bookkeeping Ontology. A concrete historical collection remains to be selected. The [source inventory](research-artefacts/corpus.csv) records incomplete reading states.

## Database and research environment

The Rust kernel will expose defined historical operations. Persistence and query execution will preserve their semantics across stored collections. An agent harness is the proposed primary research interface, connecting an LLM to those operations and maintaining context, tools and workflow state.

The model can propose interpretations, query plans and explanations. The kernel evaluates declared operations under explicit assumptions. The research interface makes the sources and decisions inspectable. A model proposal, formal validation and scholarly adoption remain distinguishable.

Model confidence does not directly quantify historical certainty. Retained model outputs document what occurred, while a new invocation may produce a different proposal. The kernel remains callable independently of an LLM.

The [architecture](knowledge/architecture.md) describes these boundaries. The general information model, storage technology and harness runtime remain open.

## First prototype and website

The proposed [minimum prototype](knowledge/specification.md#minimum-prototype) is a Rust reference model for the temporal example. It retains source wording and interpretation, answers one year-membership query with a structured explanation, and preserves the result through JSON save and reload.

This is the first semantic experiment. A database engine requires further persistence, query and revision behavior.

The planned GitHub Pages site will explain the project, publish its specification and teach working examples. [Website specification](knowledge/website-specification.md) defines the learning paths, and [Design](knowledge/design.md) describes a proposed scholarly visual direction. A later browser demonstration should reuse the Rust operation through WebAssembly.

## Repository structure

| Location | Purpose |
|---|---|
| [knowledge/](knowledge/INDEX.md) | Maintained purpose, requirements, architecture, website brief, design, method and work continuity |
| [research-artefacts/](research-artefacts/) | Scientific proposal, source inventory, reading notes and acceptance cases |
| [source-material/](source-material/README.md) | Acquisition and reuse conventions for research inputs |
| [AGENTS.md](AGENTS.md) | Shared working instructions for coding agents |

The project follows [Promptotyping](https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin). Small experiments can accompany primary reading. Adoption of the domain baseline requires documented evidence, defined behavior and review.

## Contributing and licensing

Contributions can develop source analysis, formalise an operation, supply a counterexample or implement specified behavior. Read [CONTRIBUTING.md](CONTRIBUTING.md) and the [testing strategy](knowledge/testing.md).

Original text uses [CC BY 4.0](LICENSE). Code uses [MIT](LICENSE-CODE). Third-party materials retain their own terms.
