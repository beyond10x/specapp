---
format: aep.planning-md/3
id: review-result:search-design-pass-1
kind: review-result
status: active
title: aep:plan-critic-design, pass 1, epic:shared-search
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

- story:search-enumerate — the simulate abstraction is split across three stories. `story:search-simulate-adapter` owns the verb and `score --simulate`. This story owns `search --simulate` and the check answers inside the simulate stubs `fixtures/search/stub-*.sh`. `story:search-batch-scoring` extends the same stubs again. The depends_on edge to `story:search-simulate-adapter` serialises the first consumer's path behind L4, but agenticspec has no simulate verb. The fix is to give the check answers their own stub files (or create them here) and move `--simulate` of `search` into `story:search-simulate-adapter`, with that story depending on this one. Alternatively, merge the stubs into one owner and state the trade-off. — .engineering/planning/story/search-enumerate.md:14, :34, :70, :76; .engineering/planning/story/search-simulate-adapter.md:36, :66; `aep plan artifact graph`
- story:search-batch-scoring — `runs` is half of one result abstraction. `story:search-enumerate` writes the result without it. This story adds it as an optional property and defines it as "candidates / limit, rounded up, plus the baseline". That formula implies `runs` is present without a batch too, so the committed `fixtures/expected/search/<pack>.json` golden and the page fixtures change here. The surfaces list names neither. The fix is for `story:search-result-format` and `story:search-enumerate` to own `runs` from the start (one run per candidate plus the baseline), so this story only changes its value. — .engineering/planning/story/search-batch-scoring.md:49-53, :101; .engineering/planning/story/search-enumerate.md:54-58
- story:search-design — two owners of `design/metaspec.md` over time. The body says "no other story edits the file". It also says the story that clears a blocker "extends this story scope for its paragraph" (hill climbing, beam search, the studio page). Those stories land after this one merges, so the same sections get a second owner. It lists those items as "open, with its blocker". The fix is to drop the extension clause. The later stories should own their own paragraphs in an explicit `depends_on` edge, or this story should write no text for them. — .engineering/planning/story/search-design.md:64-67
- story:search-design — the `check-design-search.sh` rule "every `schema/metaspec-*.schema.json` has an entry in Neutral formats" couples this story to every other epic. At the current tree, "Neutral formats" has no entry for `metaspec-site/1` or `metaspec-index/1`. Both schemas exist in `schema/`, and the design names them only in the wave sections. The acceptance "exits 0" is unreachable without documenting two schemas this epic did not author. Any schema a quality or cloudspec story adds later turns `task check` red. The fix is to scope the check to the schemas of this epic (a listed set) or to add a shared owner for the missing entries. — .engineering/planning/story/search-design.md:68-70; design/metaspec.md:241-298 (no `metaspec-site` or `metaspec-index` bullet there; they appear only at :787 and :1025)
- story:search-score — the "verdict per KPI" comparison (`better`/`worse`/`same`/`not_ordered`) is built in `search/score.rs` with no consumer in the set. `story:search-enumerate` lists candidates without an order, and the best/front consumer is behind `decision-blocker:kpi-order`. `story:search-result-page` computes the same better/worse-by-direction mark independently in TypeScript. One abstraction is half-built in Rust and re-derived in the page. The fix is either to carry the per-KPI verdict against the baseline in `metaspec-score/1` or the variant (in `story:search-result-format`) and have the page read it, or to defer the Rust comparison to the story that follows `decision-blocker:kpi-order`. — .engineering/planning/story/search-score.md:38-43; .engineering/planning/story/search-result-page.md:39-41

What I read: 9 stories, the epic, `architecture-decision-record:shared-search`, the 3 decision-blockers, `story:cloudspec-runs` (outside the set) and `design/metaspec.md` (formats, tool adapter, levels, gaps, build order). Commands run: `aep plan artifact list/show/relations/graph/validate/waves`. I walked 39 edges, including the three `blocks` edges and the edges to the ADR, `story:cloudspec-runs` and `story:cloudspec-manifest`.
- There is no cycle. The `depends_on` edges form a DAG, with `story:search-design` as the sink of 8.
- I traced 26 acceptance reads of state a sibling produces. Each has a `depends_on` edge, direct or transitive, so no hidden dependency is missing.
- The score then result-format order is a real ordering (the format embeds the score), so it is not a chain defect.
- `aep plan artifact validate` reports "valid". It lists only the 5 active cloudspec view stories without a scope, which are outside this set.
- The edge from `story:search-result-page` to `story:search-enumerate` is a recorded-fixture dependency. I left it alone.

What I could not establish:
- Whether `story:search-score`, `story:search-simulate-adapter` and `story:search-batch-scoring` editing the same `manifest.rs`, `adapter.rs`, `tool.rs` and `metaspec-pack-1.schema.json` can run at once. That is out of my lane (parallel-safety). Their edges record the order.
- `story:cloudspec-runs` (outside the set) still says it declares `simulate` and `search` and expects a "best variant". The ADR says it needs `simulate` only. It has no `depends_on` edge to `story:search-simulate-adapter` or `story:search-enumerate`. That is out of my lane and outside the set.
- The three decision-blockers have `blocks` edges only to `epic:shared-search`, not to a story. The stories they stop (hill climbing, beam search, best/front) are not drafted, so the epic outcome "finds better variants" is not covered by this set. That is a scope-critic question and I did not let it set the verdict.
- Whether the acceptance lines are checkable. That is out of my lane (acceptance).

```findings
- file: .engineering/planning/story/search-enumerate.md
  line: 76
  category: design
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the simulate abstraction is split across search-simulate-adapter (verb, score --simulate), this story (search --simulate, check answers in the simulate stubs) and search-batch-scoring (stubs again); the depends_on edge to search-simulate-adapter serialises the first consumer's path behind L4, so give the check answers their own stub files and move search --simulate into search-simulate-adapter, which then depends on this story"
- file: .engineering/planning/story/search-batch-scoring.md
  line: 49
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "runs is half of one result abstraction: enumerate writes the result without it and this story adds it with a formula that implies it is present without a batch, changing goldens it does not list; result-format and enumerate should own runs from the start"
- file: .engineering/planning/story/search-design.md
  line: 67
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "no other story edits the file, yet the story that clears a blocker extends this story's scope for its paragraph after this story merges, giving design/metaspec.md a second owner; drop the extension clause or record the later edit as an explicit depends_on edge"
- file: .engineering/planning/story/search-design.md
  line: 69
  category: design
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the check that every schema/metaspec-*.schema.json has a Neutral formats entry fails on metaspec-site/1 and metaspec-index/1 (no entry at design/metaspec.md:241-298) and on any schema another epic adds later, so its acceptance is unreachable and it couples this story to every other epic; scope the check to this epic's schemas or give the missing entries an owner"
- file: .engineering/planning/story/search-score.md
  line: 38
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the per-KPI verdict comparison in search/score.rs has no consumer in the set while search-result-page re-derives better/worse by direction in TypeScript; carry the verdict in the score or variant via result-format, or defer the comparison to the story after decision-blocker:kpi-order"
```
