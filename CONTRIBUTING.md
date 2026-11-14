# Contributing

Start with [the knowledge index](knowledge/INDEX.md).

A source contribution identifies the work, the exact consulted representation, a passage locator and the reading status. An interpretation links the evidence to a requirement or unresolved question. An implementation contribution links the behavior and its checks to an existing requirement.

Discuss changes to domain semantics through the [research specification](knowledge/specification.md) before treating them as established behavior. The [temporal specification](knowledge/temporal-specification.md) owns the implemented experimental contract. Keep counterexamples that expose a limitation. Distinguish the effects of changing a model, a context, an operation or an agent harness in any evaluation.

Expected test values must be independently grounded. Preserve fixed compatibility inputs and compare results using the equivalence relation declared by the contract. Follow [Testing](knowledge/testing.md) for model enumeration, process tests and mutation auditing. Updating expected output automatically from the production evaluator is insufficient evidence for a change in semantics.

Website contributions follow the [website specification](knowledge/website-specification.md) and [design brief](knowledge/design.md). Use factual, neutral text and headings that identify their contents. Keep proposed and implemented behavior distinguishable. Tutorials need verified commands and expected output, while canonical requirements remain in the knowledge base.

Record substantive integrated changes in [the journal](knowledge/journal.md). Keep accepted next work in [the plan](knowledge/plan.md). List incoming material awaiting integration in the Open section at the beginning of the journal.

Original text contributions follow CC BY 4.0 and project code follows MIT. Record source-specific terms before adding third-party material.
