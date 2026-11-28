---
format: aep.planning-md/3
id: review-result:search-design-pass-8
kind: review-result
status: active
title: aep:plan-critic-design, pass 8, epic:shared-search
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

Pass 7's decisions hold. I found one place where two stories own the same drawing, and one reverse pointer from a schema story to the symbol of a later story.

Checks on the decisions:
- **W1:** `story:search-enumerate` defines `stop_reason` and `StopReason` in `crates/metaspec-core/src/search/run.rs` and names the file in Surfaces. Its unit-test row covers every precedence row. It no longer says it is the one owner or that strategy stories quote it.
- **W2:** `story:search-design` writes "Stop reasons by precedence: ..." once. Its acceptance has the ninth grep, and the expected output `0 0 0 1 1 1 1 1 1` has the right number of fields.
- **W3:** hill-climb and beam drop the quoted block and point to `stop_reason` and "Decided: shared search". Both add the same grep, which prints 1 before and after, and beam's summary is fixed. Both still list the four reason names in their paragraphs, but not the precedence. That is naming, not copying, so it is not a finding.
- **W4:** result-format lists all four reasons from the start, and dominance says `stop.reason` is not touched.
- **W5:** the `best` rule is the same in dominance, beam cases 1 and 3, and the hill-climb cases. Hill-climb cases 1 to 4 and beam case 2 have fronts of two or more, so `best` is absent. Beam cases 1 and 3 have a lone variant that dominates the baseline or stands beside an infeasible one.
- **W6:** hill-climb now says four cases, and its 11, 11, 12 and 9 unscored counts check out.
- **Cycles:** there is none. The 14 stories form a layered graph that ends at `story:search-result-best`. The longest chain runs score, result-format, candidate-space, enumerate, simulate-adapter, batch-scoring, design, gate, dominance, hill-climb, beam, result-best. Each edge has a shared-file or produced-state reason written beside it, so it is a trade-off and not a serialising chain.

Findings:

story:search-result-best — its scope and acceptance say `ResultSections` shows "the stop reason `no_better_neighbour` in words", but `story:search-result-page` already draws `stop.reason` in words, and with all four reasons in the schema from the start (W4) that covers `no_better_neighbour` too; two stories own one drawing. Drop the stop reason from result-best and keep `strategy` and `beam_width`, or make result-page name all four wordings and leave result-best only the fixture read. — .engineering/planning/story/search-result-best.md:51 (also :110), .engineering/planning/story/search-result-page.md:69-70, .engineering/planning/story/search-result-format.md:61

story:search-result-format — its schema paragraph says "the enum is `search::run::StopReason` of `story:search-enumerate`", naming a symbol of a story that depends on it and so cannot take an edge back. The four strings are also written out in the schema, so the strings have two sources and the sentence says which one wins. Say that the schema owns the four strings and that enumerate's enum must serialise to them, or drop the parenthetical. — .engineering/planning/story/search-result-format.md:61

What I read: 14 `story:search-*` artifacts in full, plus `story:cloudspec-runs`, `story:cloudspec-manifest`, `epic:shared-search` relations, `aep plan artifact relations`, `graph` and `validate` (valid). I walked about 55 `depends_on` edges, including those to `story:cloudspec-manifest` and `story:cloudspec-runs` outside the set, and found no cycle. I traced about 20 acceptance reads to a producer in the set, and an edge records every one.

What I could not establish:
- Whether an acceptance catches the schema's four strings drifting from `StopReason`: no acceptance serialises `no_better_neighbour` until hill-climb. That is the acceptance critic's lane and does not set my verdict.
- Whether the seed arithmetic in the hill-climb and beam cases is correct: that is the acceptance critic's lane, and I did not recompute it.

```findings
[
{"file":".engineering/planning/story/search-result-best.md","line":51,"category":"design","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"its scope and acceptance say ResultSections shows the stop reason `no_better_neighbour` in words, but story:search-result-page already draws `stop.reason` in words with all four reasons in the schema from the start, so two stories own one drawing; drop the stop reason from result-best (keep strategy and beam_width) or have result-page name all four wordings"},
{"file":".engineering/planning/story/search-result-format.md","line":61,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the schema paragraph says the enum is `search::run::StopReason` of story:search-enumerate, naming a symbol of a story that depends on it while also writing the four strings itself; say that the schema owns the four strings and enumerate's enum must serialise to them, or drop the parenthetical"}
]
```
