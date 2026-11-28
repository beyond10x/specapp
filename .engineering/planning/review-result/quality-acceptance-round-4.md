---
format: aep.planning-md/3
id: review-result:quality-acceptance-round-4
kind: review-result
status: active
title: aep:plan-critic-acceptance, round 4, initiative:measure-quality
relations:
- reviews: epic:kpi-quality
- reviews: epic:spec-quality
- reviews: story:kpi-quality-catalogue-shape
- reviews: story:kpi-quality-grounding
- reviews: story:kpi-quality-page
- reviews: story:kpi-quality-rules
- reviews: story:kpi-quality-values
- reviews: story:quality-catalogue
- reviews: story:quality-ratchet
- reviews: story:spec-quality-conformance
- reviews: story:spec-quality-drift
- reviews: story:spec-quality-layering
- reviews: story:spec-quality-mutation
- reviews: story:spec-quality-page
- reviews: story:spec-quality-refusal
- reviews: story:spec-quality-self
- reviews: story:spec-quality-static
revision: 1
---
approve

The four round-3 findings have landed:
- `kpi-quality-catalogue-shape`: the acceptance command is `-E 'test(layout_) | test(golden_)'`; its `golden_*` cases check `coverage.blocks`, `completeness.fields` and `planned` on harbor and orchard (`.engineering/planning/story/kpi-quality-catalogue-shape.md:159-167`).
- `quality-ratchet`: the acceptance names `first_run_writes_baseline`, `worse_value_exits_1` (baseline file unchanged) and `private_pack_baseline`; the committed-baselines check stays in scope and the epics' acceptance still requires those baselines (`.engineering/planning/story/quality-ratchet.md:119-125`).
- `spec-quality-mutation`: first run (one stub `ess` call per class logged, a result written for the digest, the score read back) then second run (no new log line, exit 0); a script that never calls `ess` no longer passes.
- `initiative:measure-quality`: one `task acceptance` command whose rows show a value or a named gap for every measure of both questions on the four packs (`.engineering/planning/initiative/measure-quality.md:84-89`).

The rest each have one statement, a command, test case or output to check, and a before and after. 10 measures in the KPI epic and 30 in the specification epic match the 40 ids in `quality-catalogue`.

What I read: 20 of 20 ids and `acceptance-r3.md`; every story's Acceptance and context through scope; `quality-ratchet` and `kpi-quality-catalogue-shape` whole.

What I could not establish:
- Whether the `task acceptance` rows, `task` targets and fixtures exist on the target branch; commands judged as written.
- `spec-quality-refusal` states no reading for `document_health` on orchard; the epic still requires a value or a gap.
- The two decision-blockers have no `## Acceptance`; their `## Decision` and `cleared` status serve as the check.
- Out of lane: `spec-quality-conformance` runs a real `ess verify conform run` on orgspec inside `task acceptance`, which may break the 120 s budget; the story defers this with a measure-and-move rule.

```findings
[]
```
