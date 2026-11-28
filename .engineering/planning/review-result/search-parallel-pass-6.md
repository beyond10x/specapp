---
format: aep.planning-md/3
id: review-result:search-parallel-pass-6
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, pass 6, epic:shared-search
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
approve

What I read: 14 `story:search-*` drafts, with `aep plan artifact show` on each. I also ran `aep plan artifact waves --kind story` (263 within-epic collision lines, 122 against other epics) and `aep plan artifact graph` (95 `depends_on` edges). I computed edge reachability for every colliding search pair. I read `<scratch>/search-revise-brief-5.md` and checked R1 to R9 against the bodies.

Parallel safety:
- **Within the epic:** only 7 colliding pairs have no edge path, and each is named in a body or is an append-only line.
  - `design/metaspec.md` between `story:search-orgspec` and each of `story:search-gate`, `story:search-dominance`, `story:search-hill-climb`, `story:search-beam` and `story:search-result-best`: orgspec's Shared files names all five, and each writes a different paragraph or row.
  - `CHANGELOG.md` alone, for `story:search-batch-scoring` with `story:search-result-page`, and `story:search-result-page` with `story:search-simulate-adapter`: an "Unreleased" line each.
- **Other epics:** every other-epic collision with a search story is either on an `implemented` story (`story:contain-in-and-per`, `story:quality-catalogue`, `story:kpi-quality-catalogue-shape`, `story:kpi-quality-page`, `story:ux-lane-budget`) or on a draft the search bodies already name. The named drafts are `story:kpi-quality-values`, `story:spec-quality-conformance`, `story:spec-quality-drift`, `story:spec-quality-page` and `story:spec-quality-self`.
- **Pass 5 decisions:** all nine hold.
  - **R1:** the stop-reason text in `story:search-hill-climb` and `story:search-beam` matches the owner text in `story:search-enumerate` verbatim.
  - **R2, R3, R7:** the derivations in hill-climb and beam match their stated rules.
  - **R4, R5:** I recomputed the seed 1 and seed 2 outputs mod 2 and mod 3 with `bc`. They give the stated swaps: `[p8, p2, p5]` in beam case 1 (keeps p8 and p2), `[4, 1]` in hill-climb case 2, and `[p2, p1]` in the enumerate `--budget 1` case. The beam arithmetic 2+3+4+2+2+2 = 15 variants and `runs` 16 holds.
  - **R6:** `story:search-gate` writes the five hand-written scores with values (g=5/h=2, 4/1, h-only 2, 5/2, 6/3) and lists them in scope and Surfaces. It names its order after enumerate, simulate-adapter and batch-scoring through `story:search-design`.
  - **R8:** both Shared-files bullets in `story:search-gate` say the earlier changes come through `story:search-design`, and "the planner orders them" is gone.
  - **R9:** `story:search-simulate-adapter` now says enumerate creates `run.rs` and `crates/metaspec/src/search.rs`, and score creates `search/mod.rs`.

What I could not establish:
- **Out of my lane, not findings:**
  - `story:search-gate` says the stubs move the gated KPI "the wrong way" for both fixture packs, and I did not check that against the catalogues.
  - `story:search-simulate-adapter` adds an optional `source` to each KPI. If it is also written for `check`, it would change the byte-equal golden of `story:search-enumerate`.
- **`CHANGELOG.md`:** it is touched by every story in the store. Only some bodies say "one line each". It is an append-only collision, so I did not count it as a finding.

Counts: surfaces cited in a body for 14 of 14 items, inferred 0, unplaceable 0. Acceptance reads traced to a producer in the set: 33. A direct `depends_on` edge records 31 of them. The other 2 are the gate re-record through the simulate and batch stubs, which the body states, ordered by transitive edges through `story:search-design`.

```findings
[]
```
