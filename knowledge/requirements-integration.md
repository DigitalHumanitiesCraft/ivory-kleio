---
title: Requirements integration
project:
  name: Ivory KLEIO
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
status: draft
language: en
created: 2026-10-15
updated: 2026-10-15
related: [project, specification, temporal-specification, research-methodology, plan]
---

# Requirements integration

The dissertation supplies the conceptual foundation and documented research workflows. The R requirements in [hist-info-model](https://github.com/chpollin/hist-info-model/blob/main/knowledge/01-theory/Requirements.md) articulate representation needs. The H requirements in [Specification](specification.md) articulate operational behavior. Their relationship is many-to-many. A link below is an analytical correspondence, not proof of equivalence, implementation or scholarly acceptance.

## Evidence basis

Pollin (2025), *Modelling, Operationalising and Exploring Historical Information. Using Historical Financial Sources as an Example*, University of Graz, [published record and PDF](https://unipub.uni-graz.at/obvugrhs/12127700), URN `urn:nbn:at:at-ubg:1-220602`, supplies the page references. Printed and PDF page numbers coincide. Chapters 1–7 were read for this integration; the bibliography's underlying works were not independently revalidated. The acquired PDF has SHA-256 `254f5edf203cd1168833deb9ac2a85568f519891995acb2171e33f49f6057cf5`. Acquisition and reading are recorded in the [dissertation reading record](../research-artefacts/reading-notes/dissertation-requirements.md).

The dissertation's four elaborated characteristics are human agency, context sensitivity, multiperspectivity and uncertainty (§3.2, pp. 35–43). Source veto is a historiographic principle on p. 33 and part of the definition on p. 55. The later three primitives and their property graph are reconstruction hypotheses. They do not supply an adopted axiomatic derivation. Completing that derivation is not a prerequisite for evaluating an existing source workflow.

Conceptual argument, a documented application, an illustrative example, a synthetic test and a current execution check establish different things. The tables preserve these distinctions. A passage supporting the motivation of a requirement does not establish the exact later requirement or its generality.

## Existing examples

| Key | Material and location | What it supports | Evidence boundary |
|---|---|---|---|
| D01 | Contextual meaning of `bushell corn`, pp. 36–37 | Place, period, commodity and practice affect interpretation | Context illustration; no single ready-to-import fixture |
| D02 | Baltic timber measurement studies, pp. 37–38 | Different evidence and methods produce plausible conversions | Literature-based comparison; not a controlled rereading of one identical entry |
| D03 | Deep dives, collective tax actors, rights and AgentMention, pp. 89–98 | Source problems and research activities motivate model elements | Documented development and user stories; not a general effectiveness evaluation |
| D04 | Bookkeeping layers, pp. 100–108 | Economic reality, source documentation and interpretation; UnitMention and Conversion | Conceptual architecture; v1.3 is not deployed in the described DEPCHA workflow |
| D05 | RDF-star/PROV transaction and alternative amount, pp. 123–125 | Attribution and distinct assertions with time information | Explanatory code example; numerical certainty values are illustrative, not measurements |
| D06 | Wheaton pension-entry hypergraph, pp. 151–154, source p. 100 | Alternative groupings, source context and multi-entity relations | Historical transcription with a proposed representation; no executed database comparison |
| D07 | TEI annotation and transformations, pp. 158–164; CSV workflow pp. 166–168 | Source wording, normalized values, units, classifications and transformations | Documented implementation examples; lossless general conversion is not established |
| D08 | Wheaton network and Basel treemap, pp. 198–201 | Role relations, aggregation, drill-down and source-directed exploration | Implemented views with explicit limitations; no broad user-effectiveness study |
| D09 | Iterative research and its limits, pp. 202–204, 217–219 | Exploration informs remodelling; uncertainty and evaluation remain incomplete | Method and outlook; automatic dependency revision is not implemented |
| D10 | LLM-assisted modelling and transformation, pp. 207–218 | Machine proposals, expert review, formal checks and prompt-based development | Reported workflows and proposals; general efficacy remains unvalidated |
| S-A | [Synthetic account book](https://github.com/chpollin/hist-info-model/blob/main/data/examples/scenario_a.json) | Uncertain amount, historical units and emic/etic labels | Invented teaching data; no historical evidence |
| S-B | [Synthetic prosopography](https://github.com/chpollin/hist-info-model/blob/main/data/examples/scenario_b.json) | Missingness, exclusions, source loss, variants and identities | Invented teaching data; no standard capability assessment |
| S-C | [Synthetic edition](https://github.com/chpollin/hist-info-model/blob/main/data/examples/scenario_c.json) | Witnesses, material features and textual variants | Invented teaching data |
| S-D | [Synthetic guild dispute](https://github.com/chpollin/hist-info-model/blob/main/data/examples/scenario_d.json) | Categories, perspectives and interpretive chains | Invented teaching data |
| T01 | [CASE-TIME-001](../research-artefacts/cases/temporal-alternatives.md) | Exclusive years versus an inclusive interval, one possibility query and JSON round trip | Implemented synthetic Rust contract; current execution evidence belongs in the journal |
| A01 | [CASE-AGENT-001](../research-artefacts/cases/agent-interpretation.md) | An attributed proposal for “about 1700”, formal validation and subsequent adoption | Proposed synthetic workflow; harness unimplemented |

## Representation requirements

Every R identifier remains addressable. “No direct H” preserves a requirement whose specific concern is outside the present operation register. Existing synthetic examples illustrate a distinction, but do not validate the requirement or prove a deficiency in another system.

| R requirement | Dissertation basis | Existing example | H correspondence | Specific open question |
|---|---|---|---|---|
| R-E1.1 Uncertainty forms and degrees | §3.2.4, pp. 38–43 | D02, D05, S-A, T01 | H02, H05 | Which meaning belongs to each uncertainty value, and how does each supported operation preserve it? A single confidence scale is not established. |
| R-E1.2 Alternative interpretations | pp. 37–38, 74–77, 151–154 | D02, D06, S-B, T01 | H04, H06 | Which readings exclude one another, and which can jointly hold? T01 answers only an exhaustive year-alternative case. |
| R-E1.3 Explicit knowledge gaps | Missing and not-applicable values p. 41; negation pp. 42–43; OWA/CWA pp. 140–142 | S-B; D07 provides contextual defaults | H02 | How are unknown, not applicable, not searched and not recorded distinguished without inventing missing content? No complete historical gap fixture is integrated. |
| R-E2.1 Provenance | pp. 77–80, 100, 123–125 | D04, D05, D07, T01 | H01, H09 | Which source locator and derivation activity must survive a chain of operations? T01 retains a reference, not a full derivation graph. |
| R-E2.2 Epistemic distance | Traces and interpretation, pp. 30–34, 53–55, provide a motivation | S-D | H01, H09, partial | What does distance mean operationally? Source/derivation paths can be recorded; a valid scalar measure is not established. |
| R-E3.1 Coexisting perspectives | pp. 37–38, 74–77, 151–154 | D02, D06, S-D | H04, H07 | How are perspectives separately addressable and compared without silently merging their assumptions? |
| R-E3.2 Relations between perspectives | pp. 151–154 motivate relationships; no complete relation vocabulary | D06, S-B, S-D | H06, H07 | How are dependency, exclusion, compatibility and disagreement declared and evaluated? Hyperedge overlap alone does not settle them. |
| R-E4.1 Interpretation metadata | pp. 74–80, 123–125 | D05, S-B, A01 | H01, H03, H09 | Which interpreter, method, context and decision state are needed for this operation? |
| R-E5.1 Historically situated categorization | Context/political categories pp. 36–37; accounting categories p. 105 | D07, S-A, S-D | H03, partial | How does category validity change by period and framework? This is not merely a timestamp on a category. |
| R-E5.2 Emic/etic category provenance | pp. 36–37 motivate context; normalization examples pp. 160–164 provide a partial anchor | S-A, S-D | H03, H09, partial | How is a source's term separated from an analyst's classification and its attribution? No complete emic/etic formalism is established by those passages. |
| R-M1.1 Source type | Financial source forms, pp. 14–21; annotation cases pp. 158–164 | D03, D07, S-C | No direct H | Which source-type distinctions change interpretation or operations and must therefore be explicit? |
| R-M2.1 Creation purpose | Financial documentation context, pp. 14–21; source-driven modelling pp. 89–98 | D03, D07, S-A | H03, partial | How is documented creation purpose represented separately from present research purpose and used as context? |
| R-M2.2 Systematic omissions | Source and research context motivate the concern; no direct formal treatment establishes this later requirement | S-B | No direct H | What evidence establishes an exclusion policy, its scope and exceptions? An absent entry alone cannot establish systematic exclusion. |
| R-M3.1 Transmission gaps | Fragmentary traces pp. 30–34, 55 motivate the concern; no direct loss-operation contract | S-B | No direct H | What evidence identifies a lost source and what was knowable about its content? Keep source loss separate from an unknown field. |
| R-M4.1 Carrier/content separation | Signal/carrier distinctions pp. 28–30; bibliographic levels p. 67; annotation model pp. 80–81 | D04, S-C | H01, partial | Which carrier, digital representation and content references remain addressable through transformations? H01 is narrower than this material distinction. |
| R-M4.2 Material as information | Material observation p. 28; non-textual sources p. 148; facsimile access pp. 194–195; no complete material semantics | S-C; D07 facsimile/source references | No direct H | Which material feature affects a specific reading, and how is that observation attributed? A measurement ontology alone does not establish carrier semantics. |
| R-S1.1 Temporal contextualization | Context pp. 36–37; assertion/event/recording time pp. 73–74; interpretation time pp. 124–125 | D05, S-A, S-D, T01 | H03, H07 | Which temporal dimensions does the operation need? T01 has one synthetic year domain, not historical multi-time semantics. |
| R-S1.2 Historical reference systems | Context pp. 36–38; Measurement Framework pp. 106–108; units pp. 159–160 | D01, D02, D04, D07 | H03, H05 | How are the chosen unit, calendar or conversion rule and its validity retained? Existing deterministic conversions do not settle uncertain conversion chains. |
| R-S2.1 Readings and normalization | Source strings/mentions pp. 100–108; TEI choice and measure examples pp. 159–164 | D04, D07, S-B, S-C | H01, H04; H10 for machine extraction | What exact transformation links each source form to its interpretation, and which alternatives or losses remain? The basic TEI/semantic connection already exists. |
| R-S3.1 Source relations | Factoid/STAR pp. 71–77; contextual graph pp. 151–154 | D06, S-C | H06, partial | Which copying, quotation or common-origin relationship warrants dependent rather than independent evidence? A generic graph edge does not establish this. |
| R-A1 Separation of concerns | Bookkeeping architecture p. 100; validation and historical interpretation pp. 140–144 | D04, D05, D07, T01 | H01, H09, H10 | Are source representation, interpretation, executable result, machine proposal and adoption independently identifiable? |
| R-A2 Event centering | Event modelling pp. 64–70, 100–103; hypergraph pp. 151–154 | D04, D06, S-D | No mandatory H; H01/H03 may use it | Which operations require an event as their organizing entity? Event centering and assertion-based attribution can coexist. |
| R-A3 Monotonicity candidate | Iterative arrangements pp. 151–154, 202–204 | D06, D09, S-B | H07, H08 | How do retained evidence, changed acceptance and recomputed results relate? Logical monotonicity, append-only history and reversible scholarly decisions are different properties. |
| R-A4 Assertion basis | Factoid/STAR pp. 71–77; RDF-star pp. 123–125 | D05, D06 | H01, H04, H09 | What is an attributed assertion and how does it relate to a token, entity and operation? T01 does not adopt a general assertion model. |

## Operational requirements

The H identifiers and their normative candidate wording remain in [Specification](specification.md). This reverse view makes additions and partial correspondences explicit.

| H | Existing foundation and R links | Current support and remaining work |
|---|---|---|
| H01 | D04/D05/D07; R-E2.1, R-E4.1, R-S2.1, R-A1, R-A4; partial R-E2.2 and R-M4.1; optional event organization R-A2 | T01 retains source and interpretation references. General entities, carriers and derivation chains remain to specify. |
| H02 | §3.2.4; R-E1.1, R-E1.3 | T01 distinguishes alternatives and intervals. Unknown, not-applicable, negation, vagueness and polyvalence require separate meanings. |
| H03 | D01/D02/D04/D07; R-E4.1, R-E5.1, R-E5.2, R-M2.1, R-S1.1, R-S1.2; optional event organization R-A2 | Unit definitions and context precede the kernel. No historical calendar or unit evaluator is implemented in T01. |
| H04 | D02/D05/D06; R-E1.2, R-E3.1, R-S2.1, R-A4 | T01 contains one interpretation per record. Comparing several attributed readings remains a broader operation. |
| H05 | Uncertainty discussion pp. 38–43 and conversion examples; R-E1.1, R-S1.2 | T01 has one possibility predicate. Arithmetic, aggregation and uncertain conditional execution remain open. |
| H06 | D06; R-E1.2, R-E3.2, R-S3.1 | Shared premises and mutual exclusion have no implemented cross-record semantics. |
| H07 | D06/D09; R-E3.1, R-E3.2, R-S1.1, R-A3 | Historical research already requires remodelling. Revision identities, changed assumptions and affected-result discovery remain unimplemented. |
| H08 | DEPCHA preservation workflow pp. 155–157; R-A3 gives a partial motivation | T01 tests JSON persistence under a fixed contract. Transactions, recovery and retained historical queries require separate behavior. |
| H09 | D05/D10; R-E2.1, R-E4.1, R-A1, R-A4; partial R-E2.2 and R-E5.2 | A01 proposes attributed machine output and later decisions. Formal validity and scholarly adoption remain separate. |
| H10 | Uncertainty §3.2.4 and LLM validation pp. 206–216; R-A1/R-S2.1 partly overlap | A01 motivates the explicit separation of extraction error and historical underdetermination. Neither is a substitute probability for the other. |
| H11 | Function/tool use pp. 209–212 and D10; no equivalent general R requirement | T01 exposes a typed Rust operation; A01's harness tool contract and workflow remain proposed. |

## Consolidation decisions

The dissertation remains the foundation for historical information, the Bookkeeping architecture and scholar-centred workflows. The theoretical repository maintains its R vocabulary, primitive hypotheses, approach notes and teaching scenarios. Ivory KLEIO maintains the H candidates, executable contracts and this cross-repository integration. Stable identifiers survive editorial corrections.

Requirements receive a concrete operation and expected counterexample before an implementation or comparison is commissioned. Existing dissertation examples are examined before adding a new source. An illustrative transcription becomes a historical fixture only after locating its precise source, checking transcription and reuse, declaring interpretations and fixing expected behavior. No new historical dataset is commissioned by this integration.

The comparative assessment distinguishes semantic expressiveness, convenience of a dedicated construct, workflow integration and execution cost. A missing implementation or a generic mechanism does not prove impossibility. Existing ratings and scenario displays in hist-info-model remain unvalidated exploratory artifacts until equivalent encodings and operations have been checked against cited specifications.

The four characteristics, source veto and the optional primitive reconstruction can be consolidated now. The representations and operations remain linked but separately specified. Source types, intentional omissions, carrier properties and category provenance remain visible even where no direct H candidate exists. Theoretical comparison and executable semantics are connected strands of the same undertaking; repository boundaries alone do not establish independent subprojects.

## Open questions from the existing material

- Whether the primitive reconstruction contributes an explanatory or practical distinction beyond the dissertation and the R vocabulary.
- Whether a relation joins simultaneously valid assertions, exclusive alternatives or evidence that shares an assumption, and what operation follows from each case.
- Which original and normalized representations an existing transformation preserves, approximates or loses.
- Which revision changes an assumption, which decision accepts it, and which previously computed results depend on it.
- Which narrowly defined capability is unsupported by an equivalent implementation, rather than merely absent from the project's current prototype.

These questions are developed with the existing examples and contracts. Their resolution and scholarly acceptance remain substantive research work.
