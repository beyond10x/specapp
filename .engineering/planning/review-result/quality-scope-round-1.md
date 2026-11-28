---
format: aep.planning-md/3
id: review-result:quality-scope-round-1
kind: review-result
status: active
title: aep:plan-critic-scope, round 1, initiative:measure-quality
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
- reviews: story:spec-quality-layering
- reviews: story:spec-quality-mutation
- reviews: story:spec-quality-page
- reviews: story:spec-quality-refusal
- reviews: story:spec-quality-self
- reviews: story:spec-quality-static
revision: 1
---
needs-revision
epic:spec-quality — the "drift" measure ("**drift**: generated files of the pack that differ from a fresh generation") is promised, and no story claims it, though `decision-blocker:pack-drift-source` is cleared with option A (a new adapter verb mapped like `check`); a story, for example `story:spec-quality-drift`, should take it — `aep plan artifact show epic:spec-quality` (Measures, last bullet); `aep plan artifact show decision-blocker:pack-drift-source` (Decision)
story:quality-catalogue — the body says "Drift has no row: how metaspec reads it is undecided (`decision-blocker:pack-drift-source`)", which is stale because the blocker is cleared; the body must add a drift row, with status `planned` until the drift story measures it — `aep plan artifact show story:quality-catalogue` (Scope, bullet "One row per number the reports print")
story:quality-ratchet — the body says a private pack's baseline location "is undecided" and writes none, but `decision-blocker:private-pack-baseline` is cleared with option A (state directory, keyed by pack name); the story must claim that case, or another story must — `aep plan artifact show story:quality-ratchet` (Scope, last bullet); `aep plan artifact show decision-blocker:private-pack-baseline` (Decision)

What I read: 19 artifacts (the initiative, 2 epics, 14 stories, 2 blockers) with `aep plan artifact show` on each, plus `aep plan artifact graph`, `kinds` and `relations`, and the ESS `quality.yaml` for Baseline. I extracted 24 promises from the initiative and the two epics and traced 22 to an item. The two untraced are the drift measure and the private-pack baseline. A third, the landing-page table, is untraced but named as an omission (see below).

What I could not establish:
- The landing-page table over all packs is promised in `epic:spec-quality` (Outcome). `story:spec-quality-page` names it as "Not here" with a reason, but points to a "report of the decomposition" that is not in the store. I treated it as an honest omission and did not count it as a finding.
- The ESS Baseline summary in `quality.yaml` still says "kept in a file committed in metaspec". The cleared blocker says it "gets the same split". That edit lies outside the plan store, so I left it out of scope.
- I found nothing that reaches beyond the parent or claims one outcome twice. The `expect: invalid` manifest field, `blocks.known_failing`, `packs/metaspec.yaml` and the ratchet all trace to the parent's sentences. I did not check the adoption-level "features the pack cannot use yet" promise against `design/specapp.md:175`, so verb gaps may be a narrowing of it.
- Dependency order, acceptance checkability and parallel safety are out of my lane, and I did not judge them. One thing I noticed in passing: `epic:spec-quality` depends on `story:quality-ratchet`, which depends on `story:spec-quality-static`, a child of the same epic.

```findings
- file: "aep plan artifact show epic:spec-quality"
  line: 35
  category: scope
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the drift measure (\"generated files of the pack that differ from a fresh generation\") is promised, and no story claims it, though decision-blocker:pack-drift-source is cleared with option A (new adapter verb); a story such as story:spec-quality-drift should take it"
- file: "aep plan artifact show story:quality-catalogue"
  line: 34
  category: scope
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the body says drift has no row because it is undecided, but decision-blocker:pack-drift-source is cleared; the body must add a drift row (status planned until measured)"
- file: "aep plan artifact show story:quality-ratchet"
  line: 41
  category: scope
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the body treats a private pack's baseline location as undecided and writes none, but decision-blocker:private-pack-baseline is cleared with option A (user state directory keyed by pack name); the story or another story must claim that baseline"
```
