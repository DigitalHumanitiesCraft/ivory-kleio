# Ivory KLEIO

*Knowledge-Led Exploration, Interpretation and Operationalisation of Historical Information*

An agentic research environment with a database kernel for historical information.

Historical dates may admit competing readings, monetary values depend on regional conventions, and scholarly interpretations change with new evidence. Ivory KLEIO asks how a computational system can preserve these conditions through representation, queries and revision. Its working hypothesis is that a shared semantic core can retain historical meaning across processing chains.

The project connects the Ivory programme with source-oriented processing in Kleio. Initial references include *The Need for a Theory of Historical Computing* (Thaller 1991) and *On vagueness and uncertainty in historical data* (Thaller 2020). The dissertation on historical financial information (Pollin 2025) supplies the conceptual foundation and existing source-driven examples. The [requirements integration](../knowledge/requirements-integration.md) connects these with the theoretical R register and H operation candidates. The [corpus inventory](corpus.csv) records the incomplete primary analysis. Systematic reconstruction of the relevant corpus precedes adoption of a domain baseline.

The research has six objectives.

1. Reconstruct historical information requirements with passage-level evidence, scope and acceptance criteria.
2. Specify data types and relationships that distinguish source representations, interpretations and reconstructed entities, including uncertainty and dependencies.
3. Define comparison, arithmetic, aggregation and conditional execution under explicit contextual rules.
4. Develop a persistent kernel with queries, revisions and recoverable research states, retaining sources, assumptions and rule versions.
5. Investigate an agentic environment in which a harness mediates context and tools while an LLM proposes interpretations and query plans. Generated proposals, formal validation and scholarly adoption remain distinguishable.
6. Evaluate semantic preservation, auditability, modelling effort, performance and scholarly usability against carefully implemented alternatives.

The first implemented experiment is a bounded Rust reference model. It distinguishes an event occurring in either 1700 or 1710 from one occurring anywhere in that inclusive interval. A query about 1705 yields different answers. The executable example checks that source wording, interpretation and a structured explanation survive JSON save and reload. This tests one semantic distinction before a general database engine is designed.

A persistent embedded kernel follows from demonstrated semantics. Its object model, physical storage and harness runtime remain research questions. The kernel stays callable without an LLM. Model confidence does not determine historical certainty, and a retained invocation records an actual proposal rather than guaranteeing its regeneration.

Historical financial sources are the first intended demonstrator. Contrasting source forms will test generality. Relational and graph-based comparisons will use equivalent information and rules. Harness evaluation will hold the core fixed and examine whether researchers understand proposals, detect errors and revise decisions. The documentation website presents the research specification. Tutorials for executable operations follow working software.

The expected contribution is a source-grounded requirements specification and an evaluated computational account of historical information, together with an inspectable agentic workflow. Claims of originality and practical value depend on those comparisons.

## References

- Thaller, M. (2017 [1991]). *The Need for a Theory of Historical Computing*. HSR Supplement 29, 193–202. [DOI](https://doi.org/10.12759/hsr.suppl.29.2017.193-202).
- Thaller, M. (2020). *On vagueness and uncertainty in historical data*. [A Digital Ivory Tower](https://ivorytower.hypotheses.org/88).
- Pollin, C. (2025). *Modelling, operationalising and exploring historical information. Using historical financial sources as an example*. University of Graz. [Dissertation record](https://unipub.uni-graz.at/obvugrhs/12127700).

This one-pager summarises [Project](../knowledge/project.md) and [Specification](../knowledge/specification.md). Those documents own the maintained research purpose and requirements.
