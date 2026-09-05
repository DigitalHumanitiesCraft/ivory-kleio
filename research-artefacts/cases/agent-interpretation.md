# Agent interpretation

Synthetic case `CASE-AGENT-001`, proposed for H09, H10 and H11. The date and wording are invented. This case belongs to later harness evaluation and is outside the [minimum temporal prototype](../../knowledge/specification.md#minimum-prototype).

## Input and task

A source transcription contains “about 1700”. A researcher asks an agent to propose a machine-processable temporal interpretation.

## Proposed behavior

The system retains the transcription and source reference. Any proposed interval is an attributed interpretation with its generating activity, supplied context and explanation of the chosen bounds. It records whether those bounds come from a project rule, another source or a model proposal.

Formal validation can establish that an interval is well formed. Scholarly adoption is a separate decision. The stored result must preserve the relationship between the wording and its interpretation.

If the researcher changes the interval, the system records the revision and can identify computations that used the earlier bounds.

## Expected evidence

- The source representation remains addressable.
- The actual model proposal and the invocation context are retained. A later model call is a separate activity and need not reproduce the proposal.
- The proposal's interpretation of “about” is explicit.
- The formal check and scholarly decision are distinguishable.
- A later operation identifies which interpretation and rule version it used.

## Failure cases

Storing an exact year silently loses the qualifier. An invented interval presented as a source assertion obscures its origin. A model's confidence in its extraction does not quantify the historical distribution of possible event dates.
