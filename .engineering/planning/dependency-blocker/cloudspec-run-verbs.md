---
format: aep.planning-md/3
id: dependency-blocker:cloudspec-run-verbs
kind: dependency-blocker
status: open
title: cloudspec has no simulate or search verb that reads an architecture and answers in JSON
relations:
- blocks: story:cloudspec-runs
revision: 2
---
cloudspec plans simulate and search on an architecture path (JSON KPIs per document and per part) as step 3 of its order, after its story:journeys-single-point-of-failure; no story id yet (2026-11-28).
