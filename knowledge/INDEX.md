---
title: Index
project:
  name: Ivory KLEIO
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
template:
  name: Vorlage Index
  version: "0.4"
  url: https://dhcraft.org/Promptotyping/promptotyping-document/index
status: draft
language: en
created: 2026-09-05
updated: 2026-10-15
version: "0.1"
related: [project, research-methodology, requirements-integration, specification, temporal-specification, architecture, testing, website-specification, design, plan, journal, handoff]
---

# Index

Ivory KLEIO connects historical information modelling with an agentic research environment. Researchers and coding agents use the documents below to select the relevant evidence, requirements and work context.

## Core knowledge

| Document | Function and routing question | Update trigger |
|---|---|---|
| [Project](project.md) | What are the scientific purpose and product boundaries? | Scope or contribution changes |
| [Research methodology](research-methodology.md) | How are sources read and concepts formalized? | A research procedure or source criterion changes |
| [Requirements integration](requirements-integration.md) | How do dissertation findings, R representation needs, H operations and existing examples connect? | A mapped requirement, source passage, example or evidence boundary changes |
| [Specification](specification.md) | Which research requirements and adoption criteria apply? | A sourced requirement, product boundary or adoption decision changes |
| [Temporal specification](temporal-specification.md) | What exactly do the implemented data, operation and versions mean? | A temporal field, rule, error, equivalence or acceptance criterion changes |
| [Architecture](architecture.md) | How is the implementation organized and what remains proposed? | A component, dependency or technical choice changes |
| [Testing](testing.md) | How is each promised behavior checked and what does that establish? | A test procedure or evidence boundary changes |

The general specification retains provisional research requirements. The temporal specification owns the implemented experimental contract independently of their future adoption. Expected examples live in the synthetic case, test procedures in testing, and observed runs in the journal. This separates their update triggers while keeping each fact at one maintained location.

## Website and continuity

| Document | Function and routing question | Update trigger |
|---|---|---|
| [Website specification](website-specification.md) | What should visitors learn and which pages can be published? | A learning path or publication criterion changes |
| [Design](design.md) | How should the website present research and interactions? | A visual or interaction choice gains evidence |
| [Plan](plan.md) | What is the current work position and accepted next work? | Work or a dependency changes |
| [Journal](journal.md) | Which substantive transitions occurred and why? | A change is integrated, corrected or rejected |
| [Handoff](handoff.md) | Which incoming deltas await integration? | A delta arrives or is processed |

## Storage zones

`knowledge/` contains maintained project knowledge. `research-artefacts/` contains the [corpus inventory](../research-artefacts/corpus.csv), reading records and synthetic cases. [Source material](../source-material/README.md) records acquisition and reuse. Local downloads use the ignored `source-material/local/` location.

`site/` contains the website source. Its preparation script renders selected canonical documents and the corpus into ignored generated Markdown. [Build this website](../site/src/content/docs/contributing/website.md) describes maintenance.

`src/` contains the bounded Rust temporal experiment. `src/bin/ivory-temporal.rs` supplies its command-line execution path. `tests/` checks the contract, and `tests/fixtures/` retains fixed inputs and independently written expectations. Cargo writes build output under ignored `target/`; mutation reports also live there. [Testing](testing.md#reference-model-acceptance) owns the commands and evidence limits.

The root [AGENTS.md](../AGENTS.md) provides the shared action layer. [CLAUDE.md](../CLAUDE.md) routes to it for runtimes that load that filename.

## Reading paths

Start a session with this index, then the handoff inbox. For orientation, continue with project, requirements integration and specification. For source work, use research methodology and the corpus inventory. For temporal implementation, read temporal specification, architecture and testing; consult the general specification for its requirement rationale. For website work, use website specification and design. Consult the journal for observed evidence or the reason for a transition, and plan for open work.

## Document boundaries

Material descriptions are currently owned by the corpus inventory, source-acquisition record and synthetic cases. Their concrete schemas belong to the temporal specification and fixed test inputs. A separate data document becomes useful when an actual historical collection introduces its own acquisition, structure and reuse decisions. Empirical comparison or novelty claims would justify a verification document with addressable evidence. Existing authorization and contribution rules remain in the root action layer and contributing guide. No additional roadmap, governance or decision-log file is required for the current experiment.

## Convention

The knowledge base follows the [Promptotyping document templates](https://dhcraft.org/Promptotyping/). English section titles adapt the referenced templates to this research project.

The common document schema version is owned by this index. Sibling documents inherit it. Their content dates and template versions remain independent. The temporal contract separately versions its snapshot format, synthetic domain and operation rule.

This repository owns the current project purpose, requirements, architecture and their evidence. The [scientific proposal](../research-artefacts/ivory-kleio-proposal.md) and README present that maintained knowledge for different reading tasks. The website renders canonical specifications and separately authored explanations. Executable tutorials follow working software. Private workspace notes route to this repository and retain broader theoretical connections. Public contributors need no private workspace access.

## Terms

- Agent harness: the runtime that selects context, mediates tools and maintains an agent's work state.
- CLI: command-line interface.
- Historical context: explicit interpretive conditions such as calendar, place, unit system or rule version.
- Interpretation: an attributed reading or reconstruction tied to evidence and assumptions.
- Ivory KLEIO: the project and intended research environment. KLEIO expands to Knowledge-Led Exploration, Interpretation and Operationalisation of Historical Information. The repository name is `ivory-kleio`.
- Ivory: the historical-information research programme studied by this project.
- Kleio: the source-oriented historical computing system referenced by the project name. Compatibility with its formats would require a separately specified feature.
- Kernel: the reusable implementation of declared historical semantics and, as the engine develops, persistence and query behavior.
- Prototype: the first bounded semantic experiment. Its success does not establish a general database system.
- LLM: large language model.
- Reference model: an executable, inspectable implementation of specified behavior used to assess alternatives.
- Deterministic operation: equal validated input and declared context produce equal structured output under the same rule version. Its output can retain uncertainty about the represented event.
- Test oracle: an independently specified expected outcome or reference representation used to judge the implementation.
- Source uncertainty: underdetermination or vagueness in historical evidence and its interpretation. It is distinct from uncertainty in a machine extraction.
- Token: a candidate term from the source literature whose precise role will be reconstructed before adoption.
