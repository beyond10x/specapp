---
format: aep.planning-md/3
id: review-result:quality-acceptance-round-1
kind: review-result
status: active
title: aep:plan-critic-acceptance, round 1, initiative:measure-quality
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
epic:spec-quality — has no Acceptance section, and its Outcome (the only check) names a landing-page table over all packs and a drift measure that no child's acceptance delivers, so the epic can never be shown done — .engineering/planning/epic/spec-quality.md:15
epic:kpi-quality — has no Acceptance section, and its measures are headed "candidates", so nothing fixes the set whose presence would close the epic — .engineering/planning/epic/kpi-quality.md:21
story:kpi-quality-values — the acceptance reads the same before and after the work, because orgspec's 38 documents already ran through the `sed`-derived manifest, and it never observes the `expect: invalid` field, the orgspec broken-class or the harbor broken plan that `story:kpi-quality-rules` and `story:spec-quality-refusal` read — .engineering/planning/story/kpi-quality-values.md:91
story:spec-quality-static — the acceptance observes ESS completeness, adoption level and the `check` verb gap but not validity (refusals, warnings) or honesty (`UNMAPPED:` markers), which the title and scope promise — .engineering/planning/story/spec-quality-static.md:80
story:spec-quality-layering — the acceptance observes only layering 0 and "the one upward edge", so the scope's second half, "edges on a cycle inside one layer", has no recorded case and can pass untested — .engineering/planning/story/spec-quality-layering.md:67
story:spec-quality-mutation — the acceptance joins three independent outcomes: score with survivors after a run, a gap for a digest with no run, and "`task check` runs no `ess verify conform mutate`". It also never observes the scope's "a digest with a result is skipped" — .engineering/planning/story/spec-quality-mutation.md:73
story:quality-catalogue — the acceptance checks only that the loader refuses three bad rows, so a catalogue holding one row passes, while the story's outcome is one row per measure of both epics — .engineering/planning/story/quality-catalogue.md:78
story:quality-ratchet — the acceptance says "better" and "worse" without naming a `lower_is_better` measure, so a ratchet that treats every measure as `higher_is_better` passes, although the scope takes the direction from each row — .engineering/planning/story/quality-ratchet.md:79

Read: 19 artifacts, all fully (the initiative, 2 epics, 14 stories, 2 decision-blockers). I ran `aep plan artifact kinds`, `list`, `show` for each id, `lifecycle story|epic|initiative` and `validate` (valid). I also read the files under `.engineering/planning/`, and checked that harbor and orchard have numbered block directories.

Could not establish:
- The initiative has no Acceptance section either. I judged its Outcome as its acceptance and raised no finding on it.
- The acceptances that say a measure prints "as values" (grounding, coverage.lifecycles, discrimination, redundancy) check that a number appears, not which number. I counted that as observable and raised no finding.
- Whether agenticspec's catalogue has a `reads` column (the grounding acceptance needs a value there) is not in the store.
- Out of my lane, scope: both decision-blockers are `cleared` with option A. Even so, `story:quality-ratchet` and `story:quality-catalogue` still call both questions "undecided". No story builds the private-pack baseline in the user's state directory or the drift adapter verb. That is why `epic:spec-quality` cannot be met.
- Out of my lane, parallel-safety: `story:kpi-quality-rules` and `story:kpi-quality-values` both change `fixtures/packs/harbor/`, and the two stories describe different harbor broken-plan fixtures.

```findings
- file: .engineering/planning/epic/spec-quality.md
  line: 15
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the epic has no Acceptance section, and its Outcome (the only check) names a landing-page table over all packs and a drift measure that no child's acceptance delivers, so the epic can never be shown done"
- file: .engineering/planning/epic/kpi-quality.md
  line: 21
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the epic has no Acceptance section, and its measures are headed 'candidates', so nothing fixes the set whose presence would close the epic"
- file: .engineering/planning/story/kpi-quality-values.md
  line: 91
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance reads the same before and after the work, because orgspec's 38 documents already ran through the sed-derived manifest, and it never observes the expect: invalid field, the orgspec broken-class or the harbor broken plan that story:kpi-quality-rules and story:spec-quality-refusal read"
- file: .engineering/planning/story/spec-quality-static.md
  line: 80
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance observes ESS completeness, adoption level and the check verb gap but not validity (refusals, warnings) or honesty (UNMAPPED markers), which the title and scope promise"
- file: .engineering/planning/story/spec-quality-layering.md
  line: 67
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance observes only layering 0 and the one upward edge, so the scope's second half, edges on a cycle inside one layer, has no recorded case and can pass untested"
- file: .engineering/planning/story/spec-quality-mutation.md
  line: 73
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance joins three independent outcomes (score with survivors after a run, a gap for a digest with no run, task check running no mutate) and never observes the scope's 'a digest with a result is skipped'"
- file: .engineering/planning/story/quality-catalogue.md
  line: 78
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance checks only that the loader refuses three bad rows, so a catalogue holding one row passes, while the story's outcome is one row per measure of both epics"
- file: .engineering/planning/story/quality-ratchet.md
  line: 79
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance says better and worse without naming a lower_is_better measure, so a ratchet that treats every measure as higher_is_better passes, although the scope takes the direction from each row"
```
