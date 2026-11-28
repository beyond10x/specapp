---
format: aep.planning-md/3
id: dependency-blocker:cloudspec-run-verbs
kind: dependency-blocker
status: open
title: cloudspec has no simulate verb that reads an architecture and answers in JSON
revision: 5
---
cloudspec plans simulate and search on an architecture path (JSON KPIs per document and per part) as step 3 of its order, after its story:journeys-single-point-of-failure; no story id yet (2026-11-28).

Since architecture-decision-record:shared-search (2026-11-28) metaspec runs the search, so this
blocker clears once cloudspec has a `simulate` verb on one architecture with a JSON answer;
cloudspec needs no `search` verb for metaspec. cloudspec parked its own planned search work and
added story:kpi-direction-and-vary (KPI directions and `{value, vary}` places) the same day.

Since 2026-11-28 story:cloudspec-runs is planned on `check` alone and this blocker no longer
blocks it. It blocks the later story that declares `simulate` in `packs/cloudspec.yaml`, filed
once cloudspec decides the engine of its `cloudspec simulate` verb (cloudspec story:cloudspec-simulate).
