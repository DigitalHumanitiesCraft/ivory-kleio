---
title: Temporal specification
project:
  name: Ivory KLEIO
  repository: https://github.com/DigitalHumanitiesCraft/ivory-kleio
method:
  name: Promptotyping
  url: https://lisa.gerda-henkel-stiftung.de/digitale_geschichte_pollin
template:
  name: Vorlage Specification
  version: "0.3"
  url: https://dhcraft.org/Promptotyping/promptotyping-document/specification
status: draft
language: en
created: 2026-09-17
updated: 2026-09-17
related: [specification, architecture, testing, research-methodology]
---

# Temporal specification

This experimental contract isolates one distinction with a Rust library, a command-line program and a JSON save/load path. Its implementation does not establish a general historical date model. The project owner authorized the bounded experiment on 2026-09-17; scholarly adoption of a domain baseline remains open.

## Scope and inputs

Each record describes one synthetic event and retains a record identifier, source identifier, source locator, original wording and interpretation identifier. The source record explicitly declares its synthetic origin.

The temporal interpretation has exactly one tagged form.

| Form | Meaning | Valid input |
|---|---|---|
| Exclusive alternatives | Exactly one of the exhaustively listed years applies | A nonempty list of distinct year labels |
| Inclusive interval | Any year from the lower through the upper bound is possible | Two year labels with lower bound at most upper bound |

For this experiment, a year label is an integer from 1 through 9999 in one declared synthetic year domain. Rust inputs use signed 32-bit integers. JSON numbers must deserialize to that representation before domain validation; values outside that representation, fractional numbers and nonnumeric values produce `InvalidJson`. Representable integers outside 1 through 9999 produce `InvalidYear`. The scope includes no historical calendar conversion, partial dates, open bounds, probability distributions or unknown values. Unsupported forms require an explicit error.

Records identify the domain `synthetic-years-v1`. JSON snapshots declare `format_version: 1`. The operation result declares `rule_version: "could-occur-in-year-v1"`. These identifiers fix the meaning below. Changing that meaning requires a new version and an explicit compatibility decision.

## Data contract v1

The [JSON fixture](../research-artefacts/cases/temporal-alternatives.json) supplies the invented records for `CASE-TIME-001`.

| Object | Required fields | Constraints |
|---|---|---|
| Snapshot | `format_version`, `records` | Version 1; an empty collection is allowed |
| Record | `record_id`, `domain`, `source`, `interpretation` | Record IDs are unique within a snapshot; the domain must equal `synthetic-years-v1` |
| Source | `source_id`, `locator`, `wording`, `origin` | Origin is exactly `synthetic`; the other fields are nonblank strings |
| Interpretation | `interpretation_id`, `temporal` | Interpretation IDs are unique within a snapshot |
| Exclusive alternatives | `kind: "exclusive_alternatives"`, `years` | Nonempty array of distinct integer years from 1 through 9999 |
| Inclusive interval | `kind: "inclusive_interval"`, `lower`, `upper` | Integer years from 1 through 9999, with `lower <= upper` |

Identifiers must be nonblank strings. Source identifiers may recur at different locators. The prototype stores self-contained source references and has no shared source registry. Original wording and alternative order are retained exactly; no sorting, trimming, deduplication or coercion is performed. Alternative order has no effect on the answer.

All fields are required, and additional or duplicate object fields are rejected. Positional arrays cannot substitute for the objects in this table. Unknown temporal forms, source origins and incorrectly typed values produce `InvalidJson` with a diagnostic. Semantic validation distinguishes `InvalidYear`, `EmptyAlternatives`, `DuplicateAlternative`, `ReversedInterval`, `UnsupportedDomain`, `UnsupportedSnapshotVersion`, `BlankField`, `DuplicateRecordId` and `DuplicateInterpretationId`.

The supported snapshot boundary is `Snapshot::from_json` for loading and `Snapshot::to_json` for serialization. Both validate the complete snapshot. `could_occur_in_year` validates its record and query before evaluation, including values constructed directly in Rust. Public structs remain raw, editable inputs. Their generic Serde traits perform structural conversion only and must not be used as a substitute for the validated snapshot methods.

## Operation and result

`could_occur_in_year(record, year)` asks whether the query year is permitted by the recorded interpretation. For alternatives it tests membership. For an interval it tests inclusive bounds. Its Boolean result expresses possibility within this restricted, exhaustive model.

The structured result includes the Boolean answer, query year, record and interpretation references, domain, rule version and applied rule. It must expose the alternatives or bounds that justify the answer. Explanations follow from those fields and need no model call.

Result fields are `possible`, `query_year`, `record_id`, `interpretation_id`, `domain`, `rule_version`, `applied_rule`, `temporal` and the complete `source`. The applied rule is `alternative_membership` or `inclusive_bounds`. The Boolean answer states whether the interpretation permits that year; it supplies no probability or claim that the event occurred in that year. Snapshot version 1 retains the temporal and domain identifiers needed to reapply rule version 1. Stored results and rule migration are outside this experiment.

Invalid years, empty or repeated alternatives, reversed bounds, unsupported variants and unknown snapshot versions produce declared errors. They must not be repaired silently or converted into a false answer.

## Determinism and equivalence

Identical validated records, query years, domains and rule versions produce equal structured results. Evaluation uses no clock, random choice, network response or model invocation. The alternatives retained in a deterministic result still describe historical underdetermination within this synthetic experiment.

Alternative permutations preserve the Boolean possibility answer. The result retains the supplied order, so complete results from differently ordered inputs need not be equal. Expanding a valid interval preserves every previously permitted year. An interval with equal bounds and a singleton alternative permit the same year while retaining their distinct tagged forms and applied rules.

Serialization followed by validated loading preserves every record field and complete query result under the same rule version. JSON object-key order, whitespace and byte identity are outside that guarantee. Errors are compared by their declared category and relevant fields; operating-system diagnostics and parser wording are not a stable cross-platform text contract.

## Acceptance

The [case result table](../research-artefacts/cases/temporal-alternatives.md#expected-behavior) is the semantic oracle. For alternatives 1700 or 1710, the answer for 1705 is false. For the inclusive interval 1700 through 1710, it is true.

A complete experiment must also demonstrate the following behavior.

1. Both forms permit 1700 and 1710 and exclude 1699 and 1711.
2. A single alternative and a one-year interval permit that year and no other year.
3. Each result exposes the interpretation and applied rule.
4. JSON save and reload preserve identifiers, source wording, domain, temporal variant and answers for the same inputs and rule version.
5. Malformed or unsupported inputs return the specified errors.
6. A documented command runs the program from a fresh checkout with the declared toolchain.

This provides bounded evidence for H01, H02, H05 and H08. H05 evidence covers the single possibility predicate; operation chains and conditional execution remain untested. Snapshot round trips do not establish database transactions, revision history or crash recovery. The experiment does not implement the later agent case.
