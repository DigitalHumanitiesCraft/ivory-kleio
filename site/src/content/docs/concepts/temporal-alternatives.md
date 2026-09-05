---
title: Temporal alternatives
description: Why “1700 or 1710” and “between 1700 and 1710” allow different conclusions.
---

Two interpretations can share their earliest and latest dates while permitting different answers. This synthetic example is the first proposed semantic experiment for Ivory KLEIO.

## Read the source wording

| Record | Invented source wording | Explicit interpretation |
|---|---|---|
| A | “In 1700 or 1710” | Exactly one of these two years applies. The alternatives are exhaustive. |
| B | “Between 1700 and 1710, inclusive” | Any year within the interval is possible. |

Each record describes one event. Neither assigns a probability distribution.

## Ask a precise question

> Could this event have occurred in 1705 under its recorded interpretation?

For A, the expected answer is **no**. 1705 is not one of its alternatives.

For B, the expected answer is **yes**. 1705 lies within its inclusive bounds.

<details>
<summary>Why “could” matters</summary>

Possibility is the subject of this query. A positive result does not assert that the event actually occurred in 1705. Asking whether it *definitely* occurred in that year would require a different operation.

</details>

## Compare the expected results

| Query year | A, exclusive alternatives | B, inclusive interval |
|---|---|---|
| 1699 | No | No |
| 1700 | Yes | Yes |
| 1705 | No | Yes |
| 1710 | Yes | Yes |
| 1711 | No | No |

Converting A into the interval B introduces possibilities that A did not contain. Replacing either interpretation with the midpoint 1705 discards the distinction.

## Preserve the explanation

The proposed operation returns an answer with the selected interpretation, query year and applied rule. It retains a path to the original wording and source location. Saving and loading the record should preserve this meaning.

This is a worked acceptance example. It is not a running database demonstration. The first Rust implementation is still pending. The browser version will later call the same Rust operation through WebAssembly.

Read the [experimental contract](../../generated/specification/#minimum-prototype) for its input boundaries and errors, or the [complete acceptance case](../../generated/temporal-case/) for the expected persistence behavior.
