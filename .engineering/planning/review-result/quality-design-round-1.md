---
format: aep.planning-md/3
id: review-result:quality-design-round-1
kind: review-result
status: active
title: aep:plan-critic-design, round 1, initiative:measure-quality
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

- story:spec-quality-self — its acceptance reads a value or listed gap for every row of `kpis/catalogue.yaml`, which only exists once `story:kpi-quality-grounding`, `-values`, `-rules`, `story:spec-quality-conformance`, `-layering`, `-mutation` and `-refusal` have each added their measure, and no `depends_on` edge runs from it to any of them (it declares edges only to static, shape and ratchet). Add those `depends_on` edges, or narrow the acceptance to the rows the runner prints at that time. — .engineering/planning/story/spec-quality-self.md:78, .engineering/planning/story/spec-quality-self.md:10-12
- epic:spec-quality — it declares `depends_on story:quality-ratchet`, and the ratchet itself `depends_on story:spec-quality-static`, which `decomposes` this epic. Following every edge from the epic returns to it (epic:spec-quality, story:quality-ratchet, story:spec-quality-static, epic:spec-quality). It is not a cycle in needs-first edges alone. The only spec-epic story that reads the ratchet is `story:spec-quality-self`, which already has the edge, so drop the epic-level edge. — .engineering/planning/epic/spec-quality.md:10, .engineering/planning/story/quality-ratchet.md:11
- story:quality-catalogue — one of its test cases, refusing a `measured` row that no measure of the runner prints, reads the runner's registry. That registry is created by `story:kpi-quality-catalogue-shape` (`quality/kpis/mod.rs`) and `story:spec-quality-static` (`quality/spec/mod.rs`), and both depend on the catalogue, so the case cannot run when the catalogue lands. This is a split abstraction: the catalogue holds half of a catalogue-versus-registry check. Move the case into `story:kpi-quality-catalogue-shape`, or into the first story that sets a row to `measured`. — .engineering/planning/story/quality-catalogue.md:46-47, `aep plan artifact graph`

**What I read:** 19 artifacts. I ran `aep plan artifact list`, `relations`, `graph`, `validate` (valid) and `show` on the initiative, both epics, both decision-blockers and all 14 stories, then `waves`. I also read `quality.yaml` in the ESS spec.

I walked all 44 declared edges, including those to the epics, the initiative and the decision-blockers. I found no cycle among the 23 `depends_on` edges alone. The only loop is the mixed one above. The set is not a chain: `story:kpi-quality-catalogue-shape` and `story:spec-quality-static` are the two hubs and the rest fan out from them. The cut is by capability, with the KPI command as tracer bullet and the spec command as the second, so I found no horizontal slice.

I traced 19 acceptance reads to a producer in the set, and 17 of them have an edge. The two without an edge are the self-acceptance reads above (about 7 producers in one finding) and the catalogue's runner-registry case, which is a scope line rather than an acceptance line.

**What I could not establish:**
- Out of my lane (scope/acceptance): both decisions are `cleared`, but no story carries them out.
  - Drift: `decision-blocker:pack-drift-source` picked option A (a new adapter verb), yet no story implements drift. `story:quality-catalogue` still says drift "is undecided" (`quality-catalogue.md:37`).
  - Private-pack baseline: `decision-blocker:private-pack-baseline` picked option A (the user's state directory), yet `story:quality-ratchet` still says it is "undecided" and writes no private baseline (`quality-ratchet.md:36-37`). The ESS `Baseline` summary also still says "committed in metaspec" (`quality.yaml`).
- Out of my lane (parallel-safety):
  - `story:kpi-quality-values` and `story:spec-quality-conformance` both change `manifest.rs` and `specapp-pack-2.schema.json`, with no edge between them.
  - Several stories share the harbor fixtures, `kpis/catalogue.yaml` and `scripts/acceptance.sh`.
  - Later measure stories add measures that the committed baseline files of `story:quality-ratchet` must also gain, but their Surfaces lists omit `quality/baselines/`.
  - No story has recorded a scope, so `aep plan artifact waves` reports all 14 as unassessed.
- The dependency of `story:spec-quality-self` on the ESS `spec/` landing is outside the set, so it is not a design finding.

```findings
- file: .engineering/planning/story/spec-quality-self.md
  line: 78
  category: design
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance reads a value or listed gap for every row of kpis/catalogue.yaml, which exists only after the grounding, values, rules, conformance, layering, mutation and refusal stories add their measures, and no depends_on edge runs from story:spec-quality-self to any of them; add the edges or narrow the acceptance to the rows the runner prints"
- file: .engineering/planning/epic/spec-quality.md
  line: 10
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the epic depends_on story:quality-ratchet, which depends_on story:spec-quality-static, a child that decomposes this epic, so following all edges returns to the epic; only story:spec-quality-self reads the ratchet and already has the edge, so drop the epic-level depends_on"
- file: .engineering/planning/story/quality-catalogue.md
  line: 47
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the test case refusing a measured row that no measure of the runner prints reads a runner registry that story:kpi-quality-catalogue-shape and story:spec-quality-static create after this story, so the case cannot run when the catalogue lands; move it into story:kpi-quality-catalogue-shape or the first story that sets a row to measured"
```
