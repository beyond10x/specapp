---
format: aep.planning-md/3
id: dependency-blocker:cloudspec-run-verbs
kind: dependency-blocker
status: cleared
title: cloudspec has no simulate verb that reads an architecture and answers in JSON
revision: 7
transitions:
- {from: "open", to: "cleared", at: "2026-11-28T20:15:31Z", actor: "human:operator", revision: 7}
---
cloudspec plans simulate and search on an architecture path (JSON KPIs per document and per part) as step 3 of its order, after its story:journeys-single-point-of-failure; no story id yet (2026-11-28).

Since architecture-decision-record:shared-search (2026-11-28) metaspec runs the search, so this
blocker clears once cloudspec has a `simulate` verb on one architecture with a JSON answer;
cloudspec needs no `search` verb for metaspec. cloudspec parked its own planned search work and
added story:kpi-direction-and-vary (KPI directions and `{value, vary}` places) the same day.

Since 2026-11-28 story:cloudspec-runs is planned on `check` alone and this blocker no longer
blocks it. It blocks the later story that declares `simulate` in `packs/cloudspec.yaml`, filed
once cloudspec decides the engine of its `cloudspec simulate` verb (cloudspec story:cloudspec-simulate).

Cleared 2026-11-28: cloudspec main 90e1939d21a5ef5fd0251212288e9a8178cb79b5 (cloudspec PR #31) has `cloudspec simulate --load <profile> --seed <n> <architecture>` answering one JSON line and `--batch` (limit 144); the declaration in `packs/cloudspec.yaml` is story:cloudspec-simulate.
