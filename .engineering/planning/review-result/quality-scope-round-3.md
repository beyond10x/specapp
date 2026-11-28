---
format: aep.planning-md/3
id: review-result:quality-scope-round-3
kind: review-result
status: active
title: aep:plan-critic-scope, round 3, initiative:measure-quality
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
needs-revision

- `epic:kpi-quality` — the "page per pack" (`story:kpi-quality-page`, the `--quality` flag of `render`, and `story:spec-quality-page`, which adds the specification report to the same page) is not in the initiative. The initiative's Outcome asks only that metaspec "answers two questions with numbers", and its "Additions decided in planning" lists only drift and the ratchet. A page is a third addition, and the pages and their goldens are two stories of work. Either list the page under "Additions decided in planning", or the epics record who asked for it. — `.engineering/planning/epic/kpi-quality.md:16`. The same phrase is at `.engineering/planning/epic/spec-quality.md:16`.
- `epic:kpi-quality` — the measure `completeness.fields` (share of KPIs with unit, direction, a direction per mode and a gate threshold) is not in the initiative's definition of complete: "Complete (every block, rule and lifecycle has a measure)" (`.engineering/planning/initiative/measure-quality.md:18`). Field completeness of a KPI row is a different promise, and it is the only one of the ten KPI measures that no sentence of the initiative asks for. Either the initiative gains it, or the epic records it as an addition. — `.engineering/planning/epic/kpi-quality.md:32`

What I read: 21 artifacts in full (initiative, 2 epics, 15 stories, 2 blockers). Traced all 17 promises of the initiative; the 2 exclusions hold as constraints. The epics' 10 KPI-side and 30 specification-side measure ids match the 40 rows of `story:quality-catalogue`. Round-2 findings are resolved.

What I could not establish:
- Whether the owner asked for the page or for `completeness.fields` outside the store.
- `planned` and `document_health` are not named in the initiative; read as the "grounded" and "valid" qualities, not findings.
- Out of lane: `story:kpi-quality-page` "Every measure the report holds is shown" vs `story:spec-quality-page` changing the same file (parallel-safety); size of `story:kpi-quality-catalogue-shape` (design).

```findings
- file: .engineering/planning/epic/kpi-quality.md
  line: 16
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the 'page per pack' (story:kpi-quality-page, the render --quality flag, story:spec-quality-page) is not asked in initiative:measure-quality, whose Outcome asks only for numbers, and its 'Additions decided in planning' lists only drift and the ratchet; the initiative should list the page as an addition or the epics should record who asked"
- file: .engineering/planning/epic/kpi-quality.md
  line: 32
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "completeness.fields (unit, direction, per-mode direction, gate threshold on KPI rows) is not in the initiative's 'Complete (every block, rule and lifecycle has a measure)', so no sentence of the initiative asks for it; the initiative should gain it or the epic should record it as an addition"
```
