---
format: aep.planning-md/3
id: review-result:search-acceptance-pass-7
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 7, epic:shared-search
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

- story:search-beam — case 3 and the acceptance say `best` is the variant of p15 (v12), but the baseline p0 answers no KPI, so p15 does not dominate it and `best` is absent under the dominance rule — `aep plan artifact show story:search-beam` ("`front` is `v12` ... and `best` is `v12`" under Case 3; "`front` and `best` the variant of p15" in Acceptance), against `aep plan artifact show story:search-dominance` ("`best`: the id of the one variant when the front is exactly that one variant and it dominates the baseline, otherwise absent")
- story:search-hill-climb — the Stop reasons bullet says "none of the three cases below does" end `exhausted`, but four cases follow (case 4 included) — `aep plan artifact show story:search-hill-climb` (Scope, Stop reasons bullet, "A climb that ends with every point scored ends `exhausted`; none of the three cases below does")
- story:search-design — V5 puts the four stop reasons and the precedence into "Decided: shared search", but the acceptance only counts eight greps (`0 0 0 1 1 1 1 1`) and none of them observes that text, so the story can pass without writing it — `aep plan artifact show story:search-design` (Acceptance; Scope, "Build order" bullet)
- story:search-hill-climb — its design paragraph must "quote no stop-reason text" and name only the reason, but no acceptance check can show that — `aep plan artifact show story:search-hill-climb` (Scope, "Its paragraph of `design/metaspec.md`" bullet; Acceptance)
- story:search-beam — same gap: the design paragraph must name the reason and "no more", and the acceptance does not check it — `aep plan artifact show story:search-beam` (Scope, "Its paragraph of `design/metaspec.md`" bullet; Acceptance)

For the last three, one added check would fix each: for example `grep -c 'Precedence: `deadline`' design/metaspec.md` prints 1 after the design story, and the same count must still print 1 after each strategy story.

Hand-derived numbers: everything except the `best` claim above holds. I ran a temporary probe in `<scratch>/p7`. It covers SplitMix64 seeds 1 and 2, the 4x4 grid, `better`, hill-climb cases 1 to 4 and beam cases 1 to 3, including the seed-1 beam cut. The `stop_reason` rows, the gate stub values and the fixture facts also check out against the tree.

**What I read:** 14 story drafts through `aep plan artifact show`. I read all of search-gate, enumerate, hill-climb, beam, design, dominance and score. For the other seven (batch-scoring, candidate-space, orgspec, result-best, result-format, result-page, simulate-adapter) I read only the Acceptance section, because they were not revised this pass. I also checked the fixture catalogues and marked documents under `fixtures/packs`.

**What I could not establish:**
- Whether `reach` on `lone_berth` and `age` on `apple` exist as numeric fields of the fixture blocks. The grid needs them, and that is the design critic's and scope critic's lane.
- Whether the re-recorded `fixtures/expected/search/<pack>.json` fail a gate on any variant. The enumerate stubs' values are not named, so I treated the gate story's bullet as checkable by recomputation only.

```findings
[
{"file":"aep plan artifact show story:search-beam","line":null,"category":"acceptance","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"case 3 and the acceptance say `best` is the variant of p15, but the baseline p0 answers no KPI so p15 does not dominate it and `best` is absent under story:search-dominance (\"it dominates the baseline, otherwise absent\")"},
{"file":"aep plan artifact show story:search-hill-climb","line":null,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the Stop reasons bullet says \"none of the three cases below does\" end exhausted, but four cases follow, case 4 included"},
{"file":"aep plan artifact show story:search-design","line":null,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the scope writes the four stop reasons and the precedence into \"Decided: shared search\", but the acceptance greps (`0 0 0 1 1 1 1 1`) observe none of that text, so the story can pass without it"},
{"file":"aep plan artifact show story:search-hill-climb","line":null,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the design paragraph must name the reason and \"no more\" (quote no stop-reason text), but the acceptance has no check that shows it"},
{"file":"aep plan artifact show story:search-beam","line":null,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the design paragraph must name the reason and \"no more\" (quote no stop-reason text), but the acceptance has no check that shows it"}
]
```
