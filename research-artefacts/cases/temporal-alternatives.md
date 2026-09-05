# Temporal alternatives

Synthetic case `CASE-TIME-001`, proposed for H02 and H05. The years and records below are invented to distinguish two representations.

## Inputs and interpretation

Record A represents a single event whose year is either 1700 or 1710, with the alternatives mutually exclusive.

Record B represents a single event whose year is somewhere in the inclusive interval from 1700 to 1710. Every year in the interval is permitted, and no probability distribution is assigned.

## Query

“Could this event have occurred in 1705 under its recorded temporal interpretation?”

## Expected behavior

For A, the answer is false under the explicit assumption that 1700 and 1710 are the exhaustive alternatives. For B, the answer is true under the stated interval interpretation.

A query asking whether the event definitely occurred in 1705 has a different meaning. Neither record establishes that claim. The result vocabulary and treatment of incomplete alternatives require separate specification.

## Failure cases

Converting A into B introduces possibilities absent from A. Replacing either record by a midpoint discards its interpretation. Counting the alternatives as independent events changes the represented subject.

Preserve this distinction through any supported serialization and reload operation.
