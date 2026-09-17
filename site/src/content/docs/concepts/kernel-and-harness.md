---
title: The kernel and the harness
description: How a model, a tool runtime and a historical information kernel would work together.
---

Ivory KLEIO proposes an agent harness as a primary scholarly interface over a separately callable Rust kernel. The harness connects a model to tools, context and a record of its work.

## From a question to an operation

A researcher might ask for a machine-processable interpretation of “about 1700”. An agent can propose an interval and explain its bounds. The original wording and the proposal's assumptions remain available for inspection.

The harness passes a declared operation to the kernel. The kernel checks its formal preconditions and computes under explicit rules. The result identifies the interpretation and context used.

## Distinct responsibilities

| Component | Responsibility |
|---|---|
| LLM | Propose interpretations, query plans and explanations |
| Agent harness | Select context, mediate tool calls and record workflow state |
| Rust kernel | Validate and evaluate declared operations |
| Research interface | Expose source evidence, alternatives and decisions |
| Researcher | Assess interpretations and decide how to use them |

## Formal validity and scholarly judgement

A well-formed interval may still be a poor interpretation of a source. Formal validation can check its structure. It cannot establish whether the chosen bounds are historically justified.

A model's confidence in its extraction does not quantify the historical distribution of possible dates. The system must retain the actual proposal, its generating activity and later decisions. Repeating the model call is a new activity and may yield a different output.

## A core usable on its own

A library or command-line tool can call the same kernel without an LLM. This boundary makes it possible to evaluate the core separately and compare agent-mediated workflows against direct use.

The Rust reference model implements one synthetic temporal query with source attribution and JSON snapshots. The general kernel and harness remain proposed components. The [architecture](../../generated/architecture/) records the current technical direction. The [agent interpretation case](../../generated/agent-case/) defines the proposed evidence for a later workflow.
