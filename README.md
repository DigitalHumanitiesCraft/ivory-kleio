# Ivory Kleio

An agentic research environment with a database kernel for historical information.

Ivory Kleio investigates how source evidence, historical context and different forms of uncertainty can remain explicit throughout representation, computation and revision. The name connects the Ivory research programme with source-oriented processing in Kleio.

This initial repository contains a research specification, a preliminary source inventory and synthetic acceptance cases. It is a documentation foundation. A database engine and an agentic application remain research and implementation goals.

## Start here

Read the [scientific proposal](research-artefacts/ivory-kleio-proposal.md) for a concise overview, then use [Project](knowledge/project.md) and the [knowledge index](knowledge/INDEX.md) to find requirements, architectural hypotheses and the research method. The [work plan](knowledge/plan.md) identifies the next research task.

Contributors and coding agents start with [AGENTS.md](AGENTS.md). The knowledge base follows [Promptotyping](https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin).

## Research direction

The intended Rust kernel will provide historical value types and explicitly specified operations. A separately integrated agent harness will translate research requests into inspectable actions over that kernel. The system should preserve the distinction between source uncertainty, machine-generated proposals and scholarly decisions.

The relevant primary texts must be systematically read before the domain architecture is established. Small experiments can test individual interpretations during that reading.

## Repository

- [knowledge/](knowledge/INDEX.md) holds the maintained project knowledge.
- [research-artefacts/corpus.csv](research-artefacts/corpus.csv) registers source works and reading evidence.
- [research-artefacts/reading-notes/](research-artefacts/reading-notes/) holds reading records.
- [research-artefacts/cases/](research-artefacts/cases/) holds explicit test cases.
- [source-material/](source-material/README.md) documents source acquisition and reuse.

The intended public home is `DigitalHumanitiesCraft/ivory-kleio`. Remote publication is tracked in the work plan until verified.

## License

Original documentation and original research notes are licensed under [CC BY 4.0](LICENSE). Project code, when introduced, is licensed under [MIT](LICENSE-CODE). Third-party materials retain their own terms, recorded per source.
