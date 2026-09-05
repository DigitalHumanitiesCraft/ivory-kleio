# Ivory KLEIO

*Knowledge-Led Exploration, Interpretation and Operationalisation of Historical Information*

Ivory KLEIO is a research project for an agentic scholarly environment built around a database kernel for historical information. It investigates how source evidence, historical context and uncertainty can remain explicit throughout computation and revision.

The name connects the [Ivory research programme](https://ivorytower.hypotheses.org/88) with source-oriented processing in [Kleio](https://www.uni-trier.de/fileadmin/fb3/GES/Grotum/kleio/HGR-A23.pdf). The project brings this connection into dialogue with the modelling of historical financial sources and with agent-mediated research.

## Project status

The repository currently contains the research specification, a preliminary source inventory, reading notes and synthetic acceptance cases. The database kernel and agentic application are proposed research targets. Rust is the selected language for the kernel.

Start with the [scientific proposal](research-artefacts/ivory-kleio-proposal.md) for the research argument or the [knowledge index](knowledge/INDEX.md) for the working documentation. The [work plan](knowledge/plan.md) records the current research task.

## The research problem

Historical information is interpreted under conditions that affect the meaning of an operation. A monetary amount depends on its unit and historical context. A date may admit several readings. A conclusion may depend on an assumption that is later revised.

Consider a synthetic temporal example:

| Recorded interpretation | Could the event have occurred in 1705? |
|---|---|
| The event occurred in either 1700 or 1710, with these alternatives exhaustive | No |
| The event occurred sometime in the inclusive interval from 1700 to 1710 | Yes |

The query asks about possibility under the recorded interpretation. Replacing the alternatives with an interval changes the answer. Replacing either representation with a midpoint loses its uncertainty. The [complete case](research-artefacts/cases/temporal-alternatives.md) makes these assumptions explicit.

Ivory KLEIO investigates how such distinctions can be represented in data types and preserved by operations, queries and persistent storage.

## Research objectives

- Reconstruct the requirements of historical information through systematic primary-text analysis of the relevant Ivory and Kleio literature.
- Formalise historical values and contexts, including different forms of uncertainty and dependencies between interpretations.
- Define operations whose results retain their assumptions and source connections.
- Develop a persistent kernel that supports queries, revisions and recovery of earlier research states.
- Investigate an agentic environment in which researchers can inspect machine proposals and revise their decisions.
- Evaluate the approach against carefully implemented alternatives using equivalent information and rules.

The [specification](knowledge/specification.md) distinguishes candidate requirements from established decisions. Historical financial sources provide the first intended demonstrator through the [dissertation on modelling historical information](https://unipub.uni-graz.at/obvugrhs/12127700) and the Bookkeeping Ontology. Additional source forms will be needed to assess generality.

## Proposed architecture

| Component | Intended responsibility |
|---|---|
| Rust kernel | Historical value types, explicit contexts, defined operations and derivations |
| Persistence and query execution | Durable collections, revision retrieval and evaluation of queries |
| Agent harness | Context selection, tool execution, workflow state and recorded feedback |
| Large language model | Candidate interpretations, proposed query plans and explanations |
| Research interface | Source inspection, comparison of alternatives and revision of scholarly decisions |

An agent harness is the runtime that connects a model to its tools and working environment. It would give an LLM access to explicitly defined operations over the kernel.

A researcher could ask the system to compare expenditure across account books and identify which differences depend on uncertain currency assignments. The agent would propose interpretations and operations. The kernel would evaluate them under stated assumptions, and the interface would expose the evidence and resulting dependencies.

Machine-generated proposals, formal validation and scholarly adoption must remain distinguishable. Model confidence cannot be treated directly as a measure of historical certainty. The [agent interpretation case](research-artefacts/cases/agent-interpretation.md) develops this distinction.

The kernel is intended to remain usable independently of an LLM. The underlying data model, physical storage technology and harness implementation remain open questions in the [architecture](knowledge/architecture.md).

## Development through Promptotyping

The project follows [Promptotyping](https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin). Its maintained knowledge documents supply the research context and specifications from which implementation proceeds.

Primary reading establishes the requirements. Small experiments can test interpretations during that reading. A domain baseline requires documented evidence, defined behavior and review.

| Location | Contents |
|---|---|
| [knowledge/](knowledge/INDEX.md) | Project purpose, requirements, architecture, research method and work continuity |
| [research-artefacts/](research-artefacts/) | Scientific proposal, source inventory, reading notes and acceptance cases |
| [source-material/](source-material/README.md) | Acquisition and reuse conventions for research inputs |

The [corpus inventory](research-artefacts/corpus.csv) records the source representations and their reading states. Complete analysis remains part of the research programme.

## Contributing

Contributions can develop the source analysis, formalise an operation, supply a historical counterexample or implement a specified behavior. Each contribution should make its evidence and assumptions inspectable.

Read [CONTRIBUTING.md](CONTRIBUTING.md) for the contribution process. Coding agents begin with [AGENTS.md](AGENTS.md), then the knowledge index and handoff inbox. The [testing strategy](knowledge/testing.md) distinguishes document consistency, formal correctness and scholarly validity.

## License

Original documentation and original research notes are licensed under [CC BY 4.0](LICENSE). Project code is licensed under [MIT](LICENSE-CODE). Third-party materials retain their own terms, recorded with their source.
