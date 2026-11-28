---
format: aep.planning-md/3
id: review-result:quality-scope-round-2
kind: review-result
status: active
title: aep:plan-critic-scope, round 2, initiative:measure-quality
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

epic:spec-quality — the measure `drift` (and its story `story:spec-quality-drift`, blocker `decision-blocker:pack-drift-source`, and a new manifest verb) is traceable to no sentence of `initiative:measure-quality`, whose "Valid, layered, complete, held by its conformance suite, honest, able to refuse, adoption level" list has no generated-files measure; the cleared blocker records an agent's pick of option A over "C: drop drift", not an ask in the initiative, so either the initiative gains drift or the epic records who asked — .engineering/planning/epic/spec-quality.md:48
epic:kpi-quality — "Each value is held by the ratchet against a baseline" and the whole `story:quality-ratchet` (baseline files in every measure story, a private-pack baseline location) are not asked for anywhere in the initiative, which says nothing of a baseline or ratchet; the epic should cite where the ratchet was asked or the initiative should gain it — .engineering/planning/epic/kpi-quality.md:19
epic:kpi-quality — the measure definitions say `grounding` is a "share of KPIs whose `reads` name only events…", `discrimination` a "share of KPIs whose value differs", and `redundancy` "pairs of KPIs whose values agree on every document", while the stories narrow the denominators and sets (reads non-empty only, alive KPIs only, non-flat pairs only) and nothing in the epic records the narrowing — .engineering/planning/epic/kpi-quality.md:26

What I read: 20 artifacts, in full. They are the initiative, 2 epics, 15 stories and 2 decision-blockers. I wrote the initiative's 18 promises down before opening any item, and traced all 18 to an item (promises 17 and 18 hold as constraints). I also traced every measure id of both epics: 10 KPI-question and 30 specification-question ids, with each id in exactly one measure story and one row of `story:quality-catalogue`. Commands: `aep plan artifact graph|kinds|relations|show`, `design/specapp.md` grep.

What I could not establish:
- Whether the owner asked for drift or the ratchet outside the store. Neither appears in the initiative, and the ratchet appears in `design/specapp.md:380` only for UX scenarios.
- Out of my lane, so they do not set my verdict:
  - `story:quality-ratchet` decomposes `epic:kpi-quality` but is depended on by the spec-epic stories, and the spec epic's own child list omits it (design).
  - `story:kpi-quality-page` says "every measure the report holds is shown", while `story:spec-quality-page` later extends it to the specification question (parallel-safety).
  - The `gate_fidelity` story reads "moves on no valid example" as "same on every valid document" (acceptance).

```findings
- file: .engineering/planning/epic/spec-quality.md
  line: 48
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the measure drift, its story, its blocker and a new manifest verb are traceable to no sentence of initiative:measure-quality, and the cleared blocker records an agent's pick of option A over 'C: drop drift', not an ask in the initiative; either the initiative gains drift or the epic records who asked"
- file: .engineering/planning/epic/kpi-quality.md
  line: 19
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "'Each value is held by the ratchet against a baseline' and the whole story:quality-ratchet are not asked for anywhere in the initiative, which says nothing of a baseline or ratchet; the epic should cite where the ratchet was asked or the initiative should gain it"
- file: .engineering/planning/epic/kpi-quality.md
  line: 26
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the epic defines grounding, discrimination and redundancy over all KPIs or all pairs, while the stories narrow them to KPIs with non-empty reads, alive KPIs and non-flat pairs, and nothing in the epic records the narrowing"
```
