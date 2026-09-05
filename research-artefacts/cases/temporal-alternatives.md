# Temporal alternatives

Synthetic case `CASE-TIME-001`, proposed for H01, H02, H05 and bounded snapshot evidence for H08. All records and years are invented. The [minimum prototype](../../knowledge/specification.md#minimum-prototype) owns the experimental contract.

## Inputs and interpretation

Record A describes one event whose year is either 1700 or 1710. The alternatives are exhaustive and mutually exclusive. Its original synthetic wording is “in 1700 or 1710”.

Record B describes one event that occurred somewhere in the inclusive interval from 1700 to 1710. Every year in the interval is possible. Its original synthetic wording is “between 1700 and 1710, inclusive”.

No probability distribution is assigned. Both use the same synthetic year domain and retain distinct source locators and interpretation identifiers.

## Query

“Could this event have occurred in the queried year under its recorded temporal interpretation?”

## Expected behavior

| Query year | A, exclusive alternatives | B, inclusive interval |
|---|---|---|
| 1699 | false | false |
| 1700 | true | true |
| 1705 | false | true |
| 1710 | true | true |
| 1711 | false | false |

A query asking whether the event definitely occurred in 1705 has a different meaning. Neither record establishes that claim. Incomplete alternatives and unknown values fall outside this prototype.

A single alternative of 1700 and a one-year interval from 1700 to 1700 both permit 1700 and exclude 1699 and 1701.

Each result identifies the selected interpretation, query year, domain, rule version and applied membership or bounds rule. Source references and wording remain inspectable.

## Persistence evidence

Save and reload the records using the declared JSON format version. The temporal variant, identifiers, source wording and domain must remain unchanged in meaning, and the same query with the same rule version must produce the same answer and justification.

Byte-for-byte JSON identity is unnecessary. Meaning and source attribution are the properties being checked. Transactions, revision history and crash recovery need later engine cases.

## Failure cases

Converting A into B introduces possibilities absent from A. Using a midpoint loses the recorded interpretation. Counting alternatives as independent events changes the represented subject.

An empty alternatives list, repeated year, reversed interval or out-of-domain year must yield a validation error. An unsupported temporal variant or unknown snapshot version must yield an explicit error. Neither error may be reported as a valid false result.
