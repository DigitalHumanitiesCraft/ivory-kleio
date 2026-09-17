---
title: Temporal alternatives
description: Why “1700 or 1710” and “between 1700 and 1710” allow different conclusions.
---

Two interpretations can share their earliest and latest dates while permitting different answers. This synthetic example is implemented in the first Rust semantic experiment for Ivory KLEIO.

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

The operation returns an answer with the selected interpretation, query year and applied rule. It includes the original wording and source location. The executable example saves the records, reopens them and checks that both the records and complete query results remain equal.

This page is a static explanation of the acceptance case. Run the Rust implementation from a local checkout using the [example command](../../generated/testing/#reference-model-acceptance). Browser execution through WebAssembly remains proposed.

Read the [experimental contract](../../generated/temporal-specification/) for its input boundaries and errors, or the [complete acceptance case](../../generated/temporal-case/) for the expected persistence behavior.
