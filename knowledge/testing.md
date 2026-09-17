---
title: Testing
project:
  name: Ivory KLEIO
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
template:
  name: Vorlage Testing
  version: "0.3"
  url: https://dhcraft.org/Promptotyping/promptotyping-document/testing
status: draft
language: en
created: 2026-09-05
updated: 2026-09-17
related: [specification, temporal-specification, architecture, research-methodology, journal]
---

# Testing

This document owns verification procedures and their evidence limits. [Temporal specification](temporal-specification.md) defines the implemented behavior; [Specification](specification.md) owns the broader research requirements. [Journal](journal.md) records observed runs and review findings. A passing test establishes conformity within its declared scope. Historical adequacy needs identified source evidence and scholarly review.

## Deterministic strategy

The semantic core receives all relevant inputs explicitly. Equal validated ordered input, query, domain and rule version must produce equal structured output. A deterministic output can preserve several possible historical interpretations. [The contract](temporal-specification.md#determinism-and-equivalence) defines which comparisons require full result equality and which preserve only the possibility answer.

Tests use fixed inputs or deterministic finite enumeration. The oracle must be independently stated or implemented through a different representation; it must not call the production operation to construct expected answers. No test depends on a live model, network response, current time or random historical interpretation. Random temporary directory names isolate file operations and never enter the compared research result.

## Reference model acceptance

The [temporal case](../research-artefacts/cases/temporal-alternatives.md) states the synthetic assumptions and expected answers before implementation. The [JSON input](../research-artefacts/cases/temporal-alternatives.json) encodes those records. The exact data and operation versions belong to the temporal specification.

Use rustup and the compiler declared in [rust-toolchain.toml](../rust-toolchain.toml), currently Rust 1.98.1 with rustfmt and Clippy. Cargo.lock fixes dependency resolution. From the repository root, fetch dependencies once, then run the checks offline:

```sh
cargo fetch --locked
cargo fmt --all -- --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo test --locked --offline --all-targets
cargo test --locked --offline --doc
```

`cargo test` includes the actual `ivory-temporal` process tests and can be repeated without changing output paths. The tests receive Cargo's built binary directly and use fresh temporary directories. They invoke no nested Cargo process. Run the human-inspectable example separately:

```sh
cargo run --locked --offline --bin ivory-temporal -- research-artefacts/cases/temporal-alternatives.json 1705 target/temporal-roundtrip.json
```

The three arguments are an input snapshot, an integer query year and a new output path. The program saves, reopens and compares complete records and results. Its JSON report gives `false` for record A and `true` for B at 1705, with `round_trip_verified: true`. For another manual run choose a new output filename. Existing destinations are refused. Invalid input produces a nonzero exit status and an error on standard error. File writes have no transaction or recovery guarantee.

## Test layers

| Layer | Independent basis and assertion | Executable source |
|---|---|---|
| Semantic examples and rejection | Written case table, exact error categories, source retention, schema failures and validation of direct Rust inputs | [temporal_contract.rs](../tests/temporal_contract.rs) |
| Finite model comparison | Every nonempty subset and interval in fixed eight-year windows, checked against bitmasks or explicitly materialized sets | [model_space.rs](../tests/model_space.rs) |
| Justified transformations | Alternative permutation preserves possibility and the supplied order; interval widening preserves permitted years; equal possibility sets retain distinct temporal forms | [model_space.rs](../tests/model_space.rs) |
| Fixed file compatibility | Independently written expected results and frozen v1 input, checked before and after serialization | [reference_files.rs](../tests/reference_files.rs) |
| Actual process and filesystem | Isolated execution, replay of saved data, output channels, invalid input and preservation of existing files | [cli.rs](../tests/cli.rs) |

The model windows are 1–8, 1700–1707 and 9992–9999. Within each, all nonempty subsets and inclusive intervals are enumerated. Queries cover the window, available immediate neighbors and domain endpoints. Selected permutations are tested; the suite does not enumerate every ordering. Existing contract tests also query every year from 1 through 9999 for selected records. These finite checks do not establish behavior for every possible interpretation over the full domain.

Expected positive outputs are compared as structured values. JSON formatting and object-key order are irrelevant. Existing input or destination files are compared byte-for-byte when the guarantee is that the program leaves them untouched. Operating-system error text is not asserted; declared semantic error categories are.

## Regression references

[temporal-results-v1.json](../tests/fixtures/temporal-results-v1.json) contains the complete independently authored report for the original case at 1705. [snapshot-v1-source-fidelity.json](../tests/fixtures/snapshot-v1-source-fidelity.json) freezes additional v1 input with unsorted alternatives, boundary years, Unicode and significant whitespace. The latter was introduced with this strategy; it is not evidence of an earlier software release.

The test compares serialized data with the fixed input as well as comparing query results with fixed expectations. This prevents a writer and reader from passing by sharing the same lossy transformation. Input and output references must never be automatically regenerated from the production evaluator. A change to expected behavior needs its rationale in the owning contract and journal. A new version preserves earlier fixtures and either loads them under an explicit compatibility policy or rejects them by version.

## Mutation checks

[cargo-mutants](https://mutants.rs/) introduces code changes and observes whether the tests fail. Install the selected audit version and run from the repository root after the normal checks pass:

```sh
cargo install cargo-mutants --version 27.1.0 --locked
cargo mutants --no-shuffle --no-times --output target/mutation-audit
```

[The configuration](../.cargo/mutants.toml) scopes automatic mutations to `src/lib.rs` and passes `--locked --offline` to Cargo. The standard error-source chaining method is excluded because it has no declared historical-semantic guarantee. Error display remains in scope because CLI tests depend on reported error categories. The tool mutates scratch copies; never run this audit with `--in-place`.

A caught mutant must compile and then fail a test. An unviable mutant that fails compilation supplies no detection evidence. A timeout is inconclusive and requires inspection. For a surviving mutation, determine whether it changes a promised behavior; add an independently grounded test for a real gap, or document why the mutation is equivalent or outside the contract. Do not assert a universal coverage percentage from this audit. Its generated mutation set does not cover every conceivable semantic defect.

## Adding an operation

Before implementation, extend the relevant operation specification with its requirement ID, source passage or explicit project rationale, input/context boundaries, result meaning and failure categories. State an example and a counterexample with independently determined outcomes. Identify the equivalence relation used by each test and justify proposed properties such as monotonicity or commutativity for that operation.

Add bounded model comparison where finite enumeration is informative, fixed compatibility data when persistence changes, and process checks when the external interface changes. A discovered defect becomes a minimized permanent regression case. Update this document only when the procedure or evidence boundary changes; record observed results and unresolved findings in the journal. Future agent integration replays recorded proposals as test inputs. Fresh LLM responses cannot serve as deterministic expectations.

## Requirement coverage and limits

| Requirement | Current evidence | Open scope |
|---|---|---|
| H01 | Exact source IDs, locators and wording survive evaluation and file reload | Historical source adequacy |
| H02 | Alternatives, intervals, singletons and boundaries retain distinct representations | Missingness, probability and historical calendars |
| H05 | A possibility query agrees with fixed and enumerated oracles | Operation chains and conditional execution |
| H08 | Frozen data and structured results survive JSON save/reload | Transactions, revisions, recovery and a storage engine |

[The Rust workflow](../.github/workflows/rust.yml) runs the normal checks on Linux and Windows. [The mutation workflow](../.github/workflows/mutations.yml) runs the bounded audit separately on manual invocation. Configuration alone does not establish a successful hosted run. The journal distinguishes actual local evidence from unobserved platforms.

## Website commands

Use Node.js 24 and npm. From the repository root:

```sh
cd site
npm ci
npm run check
npm test
npm run build
npm run check:links
```

Preparation hooks regenerate public content from canonical documents before development, checking and builds. Restart development after changing canonical documents outside the site. Content tests check source attribution and canonical cross-reference routing. The built-link checker inspects page landmarks, links, fragments and assets under `/ivory-kleio/`. Run the production build to completion before interpreting its result. [Plan](plan.md) records remaining website findings.

## Documentation checks

Check knowledge metadata, index coverage and repository-relative links after a refactor. Keep old anchors as short routes where public links already depend on them. Check the content map and website navigation when a canonical document is added or moved. Run `git diff --check`. Public files contain no machine-specific paths or unresolved private-vault links. Generated website Markdown remains ignored.

## Website review

[Website specification](website-specification.md) owns acceptance. Review the rendered entry, a reading page and the temporal case on narrow and wide screens, including keyboard navigation, focus, theme contrast, tables and search. A static explanation remains identifiable as such. A future executable browser adapter must agree with the Rust reference fixtures.

## Engine and harness evaluation

Add revision, interrupted-write and recovery cases when those mechanisms exist. Compare indexed queries with the simple reference evaluator. The [agent case](../research-artefacts/cases/agent-interpretation.md) requires recorded proposals, invocation context, formal validation and scholarly decisions. Repeated model calls are distinct activities. Comparative performance, usability and historical claims require separate source-based evaluation.
