# Ivory Kleio

An agentic research environment with a database kernel for historical information.

Ivory Kleio investigates how historical information can be represented and processed while preserving its source basis, contextual meaning and uncertainty. Historical dates may admit competing readings, monetary values may depend on regional conventions, and scholarly interpretations may change with new evidence. The central research question is how a computational system can retain these conditions through queries, calculations and revisions.

The project connects the Ivory programme with source-oriented processing in Kleio. Its initial theoretical basis includes *The Need for a Theory of Historical Computing* (Thaller 1991) and *On vagueness and uncertainty in historical data* (Thaller 2020). The dissertation on modelling historical financial information (Pollin 2025) provides an application context. The [source inventory](corpus.csv) records the representations consulted and the incomplete state of primary analysis.

The research has six objectives.

1. Reconstruct the requirements of historical information through systematic primary reading. Each requirement needs a passage, a defined scope and an observable acceptance criterion.
2. Specify data types and structures that distinguish source representations, interpretations and reconstructed entities. Different forms of underdetermination and dependencies between hypotheses must remain expressible.
3. Define the semantics of operations. Comparison, arithmetic, aggregation and conditional execution need explicit rules for context and uncertainty.
4. Develop a persistent kernel that supports queries and revisions. A result should remain traceable to its sources, assumptions and rule versions, and earlier research states should be recoverable.
5. Investigate an agentic research environment. An LLM can propose interpretations and query plans, while a harness mediates tools and workflow state. The system must distinguish a generated proposal, formal validation and scholarly adoption.
6. Evaluate the combined approach on historical source cases. Comparisons with relational and graph-based implementations will use equivalent information and rules. Harness evaluation will examine whether researchers can inspect proposals, detect errors and revise decisions.

Implementation begins with an executable reference model in Rust. A persistent embedded kernel follows from justified semantics. An agent harness will interact through explicitly defined operations. Its technical framework, the core object model and physical storage technology remain open design questions.

Small experiments can accompany the reading, while the domain baseline depends on complete reconstruction of the relevant corpus. Historical financial sources are the first intended demonstrator, and contrasting source forms will test the scope of the conclusions. Synthetic cases make early assumptions explicit before implementation.

The expected contribution is a source-grounded requirements specification and an evaluated computational account of historical information, together with an inspectable agentic workflow for scholarly use. Originality and practical value must be established through the comparison, rather than inferred from the use of Rust or an LLM.

## References

- Thaller, M. (2017 [1991]). *The Need for a Theory of Historical Computing*. HSR Supplement 29, 193–202. https://doi.org/10.12759/hsr.suppl.29.2017.193-202.
- Thaller, M. (2020). *On vagueness and uncertainty in historical data*. https://ivorytower.hypotheses.org/88.
- Pollin, C. (2025). *Modelling, operationalising and exploring historical information. Using historical financial sources as an example*. University of Graz. https://unipub.uni-graz.at/obvugrhs/12127700.
