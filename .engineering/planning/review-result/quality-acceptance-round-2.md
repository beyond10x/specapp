---
format: aep.planning-md/3
id: review-result:quality-acceptance-round-2
kind: review-result
status: active
title: aep:plan-critic-acceptance, round 2, initiative:measure-quality
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

story:quality-catalogue — the acceptance pins `drift` to status `planned`, but `story:spec-quality-self` rewrites every row to `measured`, so `cargo nextest run -p specapp-core quality::catalogue` can only pass until that story lands — .engineering/planning/story/quality-catalogue.md:122 (rewrite at story/spec-quality-self.md:67)
story:quality-ratchet — the acceptance joins two independent outcomes with "and" (the `quality::ratchet` cases, then `task acceptance` exiting 0 against committed baselines), so either can pass while the other fails — .engineering/planning/story/quality-ratchet.md:211-217
story:quality-ratchet — the acceptance names no check that a private pack's baseline lands under `$XDG_STATE_HOME/specapp/quality/baselines/` and nothing under the checkout, which the cleared `decision-blocker:private-pack-baseline` option A requires and the scope tests, so the story can close with the location wrong — .engineering/planning/story/quality-ratchet.md:129-132 (acceptance at :209-217)
story:kpi-quality-catalogue-shape — the acceptance covers only the three measures and names nothing of the layout every later story fills (stubs, `--quality` wiring, registry check refusing a `measured` row with a stub), so the part the story calls its main deliverable has no check — .engineering/planning/story/kpi-quality-catalogue-shape.md:239-245 (registry check at :166)
story:spec-quality-mutation — the acceptance joins "a second `task slow:mutation` ... starts no `ess` job and exits 0" with "`metaspec quality spec` prints `mutation.score` ... as values", which are independent outcomes, and names no observation for "starts no `ess` job" (the stub-`ess` script test that shows it is listed in scope, not in the acceptance) — .engineering/planning/story/spec-quality-mutation.md:263-266 (scope test at :207)
story:spec-quality-layering — the acceptance joins the four-pack acceptance row (`layering` 0) and the `quality::layering` unit case (`layering` 3 on `fixtures/ess/layering/compile.json`) with "and", so one can pass while the other fails — .engineering/planning/story/spec-quality-layering.md:106-110
story:kpi-quality-page — the acceptance joins the `task check` byte comparison on fixtures with the `task acceptance` page check on orgspec and agenticspec with "and", two independent outcomes — .engineering/planning/story/kpi-quality-page.md:104-108
story:spec-quality-page — the acceptance joins the `task check` byte comparison on fixtures with the `task acceptance` page check on orgspec and agenticspec with "and", two independent outcomes — .engineering/planning/story/spec-quality-page.md:93-97
epic:spec-quality — the acceptance joins the `task acceptance` outcome for four packs with a second command, "`task quality:self` exits 0 with the same for metaspec's own `spec/`", via "; and", two independent outcomes — .engineering/planning/epic/spec-quality.md:55-60

What I read: 20 of the 20 given artifacts (initiative:measure-quality, epic:kpi-quality, epic:spec-quality, the 15 stories, the 2 decision-blockers). I used `aep plan artifact kinds`, `lifecycle story`, `list`, `show` for the initiative, epics and blockers, and the full story bodies and Acceptance sections from `.engineering/planning/story/*.md`. I did not read earlier review-result records.

What I could not establish:
- Whether the `task acceptance` rows exist on the branch `invert-pack-knowledge`. The worktree is not on that branch, so the commands the acceptances name could not be run or grepped.
- A weaker unease, not listed because I could not name a one-line fix: "written by the acceptance run's ratchet and committed here" is in several measure stories' scope. `task acceptance` exits 0 on a first run that writes the baselines, so exit 0 alone does not show they were committed.
- Out of my lane, not counted in the verdict: `story:spec-quality-conformance` runs a real `ess verify conform run` on orgspec (892 scenarios) inside `task acceptance`, which may break the 120 s budget. `plan-critic-design` or the parallel-safety critic can judge it.

```findings
- file: .engineering/planning/story/quality-catalogue.md
  line: 122
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance pins `drift` to status `planned`, but story:spec-quality-self rewrites every row to `measured`, so the check can only pass until that story lands"
- file: .engineering/planning/story/quality-ratchet.md
  line: 211
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance joins two independent outcomes with 'and' (the quality::ratchet cases, then task acceptance exiting 0 against committed baselines), so either can pass while the other fails"
- file: .engineering/planning/story/quality-ratchet.md
  line: 129
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance names no check that a private pack's baseline lands under $XDG_STATE_HOME/specapp/quality/baselines/ and nothing under the checkout, which the cleared decision-blocker:private-pack-baseline option A requires and the scope tests, so the story can close with the location wrong"
- file: .engineering/planning/story/kpi-quality-catalogue-shape.md
  line: 239
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance covers only the three measures and names nothing of the layout every later story fills (stubs, --quality wiring, the registry check refusing a measured row with a stub), so the part the story calls its main deliverable has no check"
- file: .engineering/planning/story/spec-quality-mutation.md
  line: 263
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance joins 'a second task slow:mutation starts no ess job and exits 0' with 'quality spec prints mutation.score ... as values', two independent outcomes, and names no observation for 'starts no ess job'"
- file: .engineering/planning/story/spec-quality-layering.md
  line: 106
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance joins the four-pack acceptance row (layering 0) and the quality::layering unit case (layering 3 on fixtures/ess/layering/compile.json) with 'and', so one can pass while the other fails"
- file: .engineering/planning/story/kpi-quality-page.md
  line: 104
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance joins the task check byte comparison on fixtures with the task acceptance page check on orgspec and agenticspec with 'and', two independent outcomes"
- file: .engineering/planning/story/spec-quality-page.md
  line: 93
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance joins the task check byte comparison on fixtures with the task acceptance page check on orgspec and agenticspec with 'and', two independent outcomes"
- file: .engineering/planning/epic/spec-quality.md
  line: 55
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance joins the task acceptance outcome for four packs with a second command, 'task quality:self exits 0 with the same for metaspec's own spec/', via '; and', two independent outcomes"
```
