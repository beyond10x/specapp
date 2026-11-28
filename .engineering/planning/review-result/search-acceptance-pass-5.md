---
format: aep.planning-md/3
id: review-result:search-acceptance-pass-5
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 5, epic:shared-search
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

- story:search-beam — the case 1 cut is not observable as seeded: for seed 1 the shuffle of `[p2, p5, p8]` is the identity (`r1 mod 3 = 2`, `r2 mod 2 = 1`, no swap). A cut that takes the first 2 in ascending order, with no shuffle, therefore passes byte for byte, and no other case has a cut. Use a seed where the shuffle swaps. By my probe, seed 2 gives `[p8, p2, p5]` and keeps p8 and p2. That run scores 15 variants with `runs` 16 and ends `exhausted`, and it reuses the seed-2 outputs `10905525725756348110` and `13819372491320860226` already in the hill-climb story. Seed 3 also swaps (keeps p8 and p5, `no_better_neighbour`). The acceptance must name which members the cut keeps, and the derivation must be rewritten for that seed. — .engineering/planning/story/search-beam.md:92 and :149
- story:search-enumerate — `--budget 1 --seed 7` does not name which candidate is scored, and the shuffle is the identity for seed 7 (first output `7191089600892374487` is odd, so `j = 1` with no swap). The branch "space larger than the budget: shuffled, first `--budget` scored" cannot be told from odometer order. Pick a seed whose first output is even (seed 2 gives `10905525725756348110`), so the shuffle swaps the two candidates, and name the variant the acceptance expects. — .engineering/planning/story/search-enumerate.md:155
- story:search-gate — the cases "gated KPI worse than the original" and "gated KPI the baseline answered and the candidate does not" need stub answers that no story defines. The Scope claims "the stubs compute their values from the marked values, so a candidate moves the gated KPI of each fixture pack the wrong way". But search-enumerate files the stubs only as "answer in the shape of each fixture pack", and the gate story's Surfaces list no stub. Say which stub or hand-written scores give a worse, a missing and an equal-or-better gated value, with the values written out. — .engineering/planning/story/search-gate.md:70 and :118
- story:search-hill-climb — the acceptance says the searches "end on p1 and on p4", but the result has no member that records the current candidate (the format has only `variants`, `trace` in evaluation order, and `stop`). Name the observable instead: variants `v1..v4` = p1, p4, p2, p5 for seed 1 and p1, p4, p5, p8 for seed 2. — .engineering/planning/story/search-hill-climb.md:191

I checked the derived numbers by hand. Everything else in the hill-climb and beam derivations reproduces exactly, and I raise no finding on it.
- **SplitMix64 outputs:** the seed 1 and seed 2 stream values match.
- **Shuffle outcomes:** the shuffle results and the neighbour lists (p5 has 1, 4, 6, 9) match.
- **Hill climb:** case 1 gives variants p1, p4, p2, p5, `runs` 5, `no_better_neighbour`, and the current candidate p1. Case 2 gives p1, p4, p5, p8 and the current candidate p4. Case 3 gives p1, p4, p2, `runs` 4, `budget`.
- **Beam:** case 1 gives 14 variants, `runs` 15, front and best p15, with p12 unscored. Case 2 gives variants p1, p4, p2, p5, p8, `runs` 6, `budget`.
- **Fixtures:** `reach` on `lone_berth` and `draught` on `barge` are in document order, as are `age` on `apple` and `hands` on `picker`. `crane_lift_rate` and `tree_yield_kilograms` are the first KPIs with direction higher. No gated floor can fail, because every k is at least 0.
- **A12 to A15:** the gate cases, the `order::better` cases, the second hand-written result with its two named files, and the "at the time of writing" line citations are all present. The old-fragment grep prints `1 1 1 0 0 0 0 0` today (lines 242, 359, 443).
- **Test time:** the strategy tests are at most 16 stub runs each, so none is near 60 s.

**What I read:** 14 of 14 ids with `aep plan artifact show`, plus `waves --kind story --status draft`. I also read `fixtures/manifests/*.yaml`, the fixture catalogues and documents, and the old fragments in `design/metaspec.md`. The probe ran from `<scratch>/crit5` with `python3 -I -c`, so nothing is in the repository.

**What I could not establish:**
- Whether the stub formulas named in search-gate exist anywhere else; I found none in the 14 drafts.
- Out of my lane, not counted in the verdict: the co-writer lines of P6 and P7 are present, and the `waves` command still prints collisions among the search stories on shared files such as `CHANGELOG.md`, `search/run.rs` and `tests/cases/mod.rs`.

```findings
[
{"file":".engineering/planning/story/search-beam.md","line":92,"category":"acceptance","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the round-2 cut of case 1 is not observable as seeded: `k = 2`, `r1 mod 3 = 2`, no swap; `k = 1`, `r2 mod 2 = 1`, no swap leaves `[p2, p5, p8]` unchanged, so a cut by ascending order with no shuffle passes byte for byte; use a seed whose shuffle swaps (seed 2 keeps p8 and p2 and ends exhausted with 15 variants and runs 16) and name the kept members in the acceptance"},
{"file":".engineering/planning/story/search-enumerate.md","line":155,"category":"acceptance","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"`with `--budget 1` it has one variant and `stop.reason` `budget`` does not name which candidate is scored, and with seed 7 the shuffle is the identity (first output 7191089600892374487 is odd), so the shuffled-prefix branch cannot be told from odometer order; use a seed whose first output is even (seed 2) and name the variant"},
{"file":".engineering/planning/story/search-gate.md","line":70,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"undecided","message":"the worse, missing and equal-or-better gated KPI cases rely on stubs that \"compute their values from the marked values, so a candidate moves the gated KPI of each fixture pack the wrong way\", but story:search-enumerate files the stubs only as \"answer in the shape of each fixture pack\" and this story's Surfaces list no stub; name the stub or the hand-written scores and the values that produce each case"},
{"file":".engineering/planning/story/search-hill-climb.md","line":191,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the acceptance says the climbs \"end on p1 and on p4\" but metaspec-result/1 records no current candidate (variants, trace in evaluation order, stop only); state the observable instead: variants v1..v4 = p1, p4, p2, p5 for seed 1 and p1, p4, p5, p8 for seed 2"}
]
```
