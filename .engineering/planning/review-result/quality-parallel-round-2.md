---
format: aep.planning-md/3
id: review-result:quality-parallel-round-2
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, round 2, initiative:measure-quality
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

I found no pair of stories that can run at once and share a surface. The 55 collisions that `aep plan artifact waves` lists are all ordered by a `depends_on` edge and admitted in the later story's "Shared files" section. No reader sits beside its producer without an edge.

**What you read.** I read 15 stories and the graph. I ran `aep plan artifact waves`, `graph`, `relations` and `show` on every story, plus `git grep` in the plan-quality tree.
- Surfaces: 15 of 15 stories are placed from paths their own bodies cite, so none are unplaced.
- Inferred entries: 7 individual entries across 4 stories are inferred, namely `fixtures/expected/harbor/pages/plan` in `story:kpi-quality-values`, 4 harbor alive, discrimination and redundancy files in `story:kpi-quality-rules`, `taskfiles/slow.yml` in `story:spec-quality-conformance`, and `scripts/check-vocabulary.sh` in `story:spec-quality-self`. Every collision resting on one of them is already ordered by an edge.
- Same-wave pairs: waves 3 to 7 hold 13 stories, and no two in the same wave overlap in their cited scopes.
- Acceptance reads: I traced 16 reads to a producer in the set. All 16 are recorded by a `depends_on` edge; 14 directly and 2 only transitively (`story:kpi-quality-rules` and `story:spec-quality-mutation` reading the ratchet baselines).

**What you could not establish.**
- The plan-quality tree is `main`, which has no `adapter.rs`, `packs/`, `kpis/` or `quality/`. Surfaces on the `invert-pack-knowledge` branch could not be checked against files, so they rest on the bodies' own citations.
- Out of my lane, and not setting the verdict:
  - `story:spec-quality-mutation` says the shared stub `ess` refuses `mutate` outside its script test. `story:spec-quality-static` says that stub prints `fixtures/ess/<pack>/<verb>.json` and is never changed, and `fixtures/ess/harbor/mutate.json` exists. Which file makes the stub refuse is not stated. The pair is ordered through conformance, so there is no concurrency harm. Whether this is a design or acceptance defect is for those critics.
  - `story:spec-quality-drift` edits `fixtures/packs/orchard/tool/orchard-tool`, which the orchard check passes of `story:spec-quality-refusal`, `story:kpi-quality-values` and `story:kpi-quality-rules` execute. The edit is additive (a new `regenerate` answer) and refusal does not list the file, so I did not make it a finding.

```findings
[]
```
