---
title: Project scope
description: What Ivory KLEIO investigates, what is available, and where to begin.
---

Ivory KLEIO is a research project for an agentic scholarly environment built around a database kernel for historical information. It investigates how evidence, historical context and uncertainty can remain explicit through computation and revision.

## Implementation status

This website presents the research foundation, proposed requirements and synthetic acceptance cases. The Rust kernel and agent harness are not yet implemented. There is currently no database package to install.

The [temporal alternatives](../concepts/temporal-alternatives/) page explains one consequence of the recorded interpretation using synthetic inputs and expected results.

## A database and its scholarly tools

The intended kernel provides types and operations for historical information. Persistence and query execution will preserve their meaning across stored collections. A researcher can use that kernel directly or through an agent harness.

The [kernel and harness](../concepts/kernel-and-harness/) have different responsibilities. A model proposes interpretations and operations. The kernel evaluates declared rules. Scholarly decisions remain attributable to the researcher.

## Research documents

The [scientific proposal](../generated/proposal/) sets out the research objectives. The [specification](../generated/specification/) records requirement candidates and the proposed minimum experiment. The [source inventory](../generated/sources/) distinguishes identified works, partial reading and full analysis.

The historical information model remains open. Small experiments can accompany primary reading, while adoption of a domain baseline requires systematic reconstruction of the relevant corpus.

## The name

KLEIO expands to *Knowledge-Led Exploration, Interpretation and Operationalisation of Historical Information*. Ivory refers to the historical information programme studied by the project. Kleio connects the name to earlier source-oriented historical computing.

The repository is [DigitalHumanitiesCraft/ivory-kleio](https://github.com/DigitalHumanitiesCraft/ivory-kleio). Original text is licensed under CC BY 4.0 and code under MIT. Third-party sources retain their own terms.
