---
format: aep.planning-md/3
id: review-result:search-acceptance-pass-4
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 4, epic:shared-search
relations:
- reviews: epic:shared-search
- reviews: story:search-batch-scoring
- reviews: story:search-beam
- reviews: story:search-candidate-space
- reviews: story:search-design
- reviews: story:search-dominance
- reviews: story:search-enumerate
- reviews: story:search-gate
- reviews: story:search-hill-climb
- reviews: story:search-orgspec
- reviews: story:search-result-best
- reviews: story:search-result-format
- reviews: story:search-result-page
- reviews: story:search-score
- reviews: story:search-simulate-adapter
revision: 1
---
needs-revision

Pass 3's decisions hold, including A6 on one marked place. I found eight new acceptance defects.

- **Pass-3 decisions that hold:**
  - A6: precedence is in search-enumerate.md:68-74. `budget`, `no_better_neighbour` and `exhausted` come out distinct on a line of 12 points. I traced the counts: 11 variants and `runs` 12 for the climb stub, and `runs` 4 at `--budget 3`.
  - A7: result-best names `fixtures/expected/search/best/beam-best.json`.
  - A8: the result-page acceptance is the `test(search_page)` cargo case. It checks the refusals and the failed-variant page JSON, and the vitest cases moved to Scope. The Run-section condition is stated and matches documents.rs:435-441.
  - A9: the grep prints `1 1 1 0 0 0 0 0` today. I ran it on the tree.
  - A10 and A11: both are present. The compare(a, b) wording in dominance is consistent.
- **New findings:** the peak counts in hill-climb and beam cannot be derived. The seeded order and the beam cut are never observed. Several pass-3 behaviours have no check. The design story cites stale line numbers.

story:search-hill-climb — the peak case's counts cannot be derived. The stub is fixed at `6 - |v - 6|`, while the marked values are `3m` (harbor) and `2` (orchard). `min`, `max` and `step` are unnamed ("starting at its original value"), so the peak is not at the "sixth point". The stub's validity answer is also unstated, and harbor's own tool refuses its document. "(points 2 to 7)" contradicts point 0 = original in `Space::point(n)` and enumerate. — .engineering/planning/story/search-hill-climb.md:92,123,127-128; fixtures/packs/harbor/harbors/inlet.yaml:7; fixtures/packs/orchard/orchards/hillside.yaml:9

story:search-hill-climb — the line starts at the original value. Every step has exactly one unscored neighbour, so "first neighbour in the seeded order" and `--seed 3` are never exercised. Byte-equality on a second run holds for any seed. No case can tell first-improvement in seeded order from any other rule. — .engineering/planning/story/search-hill-climb.md:123-128

story:search-beam — it repeats the peak counts ("points 2 to 7", `runs` 7) on hill-climb's scratch copy. They are not derivable, as above. — .engineering/planning/story/search-beam.md:111

story:search-beam — on a line the beam never holds more than one member and never has more than one unscored neighbour. `--beam-width 2` therefore never cuts, so these cases pass on hill climbing relabelled `beam`. No case observes the cut to `--beam-width` in the seeded order. — .engineering/planning/story/search-beam.md:105-111

story:search-gate — the acceptance has no case for "a gated KPI the baseline answered and the candidate does not answer fails the gate". That rule is pass 3's S1 and sits in Scope. It also does not say that a candidate equal or better on every gated KPI has `feasible` true, nor that the re-recorded files carry `gate_failed` and `feasible`. They validate before the re-recording too, because both properties are optional. — .engineering/planning/story/search-gate.md:112-118

story:search-dominance — `order::better` has no case for the order moved here by G4. That order is feasible beats infeasible. Among infeasible candidates, one that did not run ranks below one that ran, then fewer gated findings win, then fewer failed gates. The acceptance checks only front membership of one infeasible variant. — .engineering/planning/story/search-dominance.md:131-137

story:search-result-best — the baseline member of `front` is drawn as the baseline row (D15). `beam-best.json` has a front of one variant that dominates the baseline, so the baseline never appears in `front`. The acceptance never observes that drawing. — .engineering/planning/story/search-result-best.md:53, 102-107

story:search-design — it cites the old fragments on "lines 233, 350 and 434". After the cloudspec-manifest merge they are on lines 242, 359 and 443 of design/metaspec.md. — .engineering/planning/story/search-design.md:113-114; `grep -n 'the agent and the tool.s search' design/metaspec.md` prints line 443

**What I read:** 14 of 14 drafts, through `aep plan artifact show` on each (dumped to scratch in two batches). I also read the criteria file, the pass-3 brief, the fixture packs, the harbor and orchard catalogues, and documents.rs. I ran the A9 grep.

**What I could not establish:**
- Whether a `search_run`, `search_climb` or `search_beam` test in `-p metaspec-core` can start `metaspec search`, which lives in crates/metaspec/src/search.rs. That belongs to the design and scope critics.
- Design paragraphs owned by gate, dominance, hill-climb, beam, result-best and the orgspec row are Scope items with no grep in their acceptance. I did not make that a finding. It may deserve a look.
- The `compare` verdicts `same` and `not_ordered` are observed by no acceptance (story:search-score). I did not make that a finding either.
- Out of my lane: result-best's Constraints still say the tests read the `<pack>.json` results, while its acceptance uses `beam-best.json`.

```findings
[
{"file":".engineering/planning/story/search-hill-climb.md","line":127,"category":"acceptance","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the peak case's counts cannot be derived: the peak stub is fixed at 6 - |v - 6| while the marked values are 3m (harbor) and 2 (orchard), min/max/step are unnamed, the stub's validity answer is unstated (harbor's tool refuses its document), and '(points 2 to 7)' contradicts point 0 = original"},
{"file":".engineering/planning/story/search-hill-climb.md","line":123,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the line starts at the original value so each step has one unscored neighbour; the seeded order and first-improvement rule and the seed itself are never observed, and byte-equality on a second run holds for any seed"},
{"file":".engineering/planning/story/search-beam.md","line":111,"category":"acceptance","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the peak case repeats counts (points 2 to 7, runs 7) that cannot be derived from the peak stub, the fixture values and the unnamed range of the scratch copy"},
{"file":".engineering/planning/story/search-beam.md","line":105,"category":"acceptance","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"on a line the beam never holds more than one member, so --beam-width 2 never cuts and the cases pass on hill climbing relabelled beam; no case observes the cut to the beam width in seeded order"},
{"file":".engineering/planning/story/search-gate.md","line":112,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the acceptance has no case for a gated KPI the baseline answered and the candidate does not answer (S1), none for feasible true on a candidate equal or better on every gated KPI, and the re-recorded files validate before the re-recording too since gate_failed and feasible are optional"},
{"file":".engineering/planning/story/search-dominance.md","line":131,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"order::better (feasible beats infeasible; among infeasible a run beats a non-run, then fewer gated findings, then fewer failed gates) moved here by G4 has no case; only front membership of one infeasible variant is checked"},
{"file":".engineering/planning/story/search-result-best.md","line":102,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the drawing of the baseline member of front as the baseline row (D15) is in Scope but beam-best.json has a front of one variant that dominates the baseline, so the acceptance never observes it"},
{"file":".engineering/planning/story/search-design.md","line":113,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the acceptance cites the old fragments on lines 233, 350 and 434 but they are on lines 242, 359 and 443 of design/metaspec.md after the cloudspec-manifest merge"}
]
```
