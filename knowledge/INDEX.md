---
title: Index
project:
  name: Ivory Kleio
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
updated: 2026-09-05
version: "0.1"
related: [project, specification, architecture, research-methodology, testing, plan, journal, handoff]
---

# Index

Ivory Kleio connects historical information modelling with an agentic research environment. Researchers and coding agents use the documents below to select the relevant evidence, requirements and work context.

## Documents

| Document | Function and routing question | Update trigger |
|---|---|---|
| [Project](project.md) | What are the scientific purpose and product boundaries? | Scope or contribution changes |
| [Specification](specification.md) | What behavior is required and on what grounds? | A sourced requirement or acceptance case changes |
| [Architecture](architecture.md) | How do the proposed components interact? | A technical decision gains evidence |
| [Research methodology](research-methodology.md) | How are sources read and concepts formalized? | A research procedure or source criterion changes |
| [Testing](testing.md) | What does each check establish? | Implemented behavior or evaluation design changes |
| [Plan](plan.md) | What is the current work position and accepted next work? | Work or a dependency changes |
| [Journal](journal.md) | Which substantive transitions occurred and why? | A change is integrated, corrected or rejected |
| [Handoff](handoff.md) | Which incoming deltas await integration? | A delta arrives or is processed |

## Storage zones

`knowledge/` contains maintained project knowledge. `research-artefacts/` contains the [corpus inventory](../research-artefacts/corpus.csv), reading records and synthetic cases. [Source material](../source-material/README.md) records acquisition and reuse. Local downloads use the ignored `source-material/local/` location.

The root [AGENTS.md](../AGENTS.md) provides the shared action layer. [CLAUDE.md](../CLAUDE.md) routes to it for runtimes that load that filename.

## Reading paths

Start a session with this index, then the handoff inbox. For orientation, continue with project and specification. For source work, use research methodology and the corpus inventory. For implementation, read specification, architecture and testing. Consult the journal when the reason for an earlier transition matters.

## Convention

The knowledge base follows the [Promptotyping document templates](https://dhcraft.org/Promptotyping/). English section titles adapt the referenced templates to this research project.

The common document schema version is owned by this index. Sibling documents inherit it. Their content dates and template versions remain independent. A future formal data or operation contract will have its own explicitly declared version.

The repository owns technical requirements and their evidence. The separate scientific proposal is maintained by the project owner. Public contributors can understand and extend the repository without access to a private workspace.

## Terms

- Agent harness: the runtime that selects context, mediates tools and maintains an agent's work state.
- CLI: command-line interface.
- Historical context: explicit interpretive conditions such as calendar, place, unit system or rule version.
- Interpretation: an attributed reading or reconstruction tied to evidence and assumptions.
- Ivory: the historical-information research programme studied by this project.
- Kleio: the source-oriented historical computing system referenced by the project name.
- LLM: large language model.
- Reference model: an executable, inspectable implementation of specified behavior used to assess alternatives.
- Source uncertainty: underdetermination or vagueness in historical evidence and its interpretation. It is distinct from uncertainty in a machine extraction.
- Token: a candidate term from the source literature whose precise role will be reconstructed before adoption.
