---
format: aep.planning-md/3
id: review-result:search-design-pass-3
kind: review-result
status: active
title: aep:plan-critic-design, pass 3, epic:shared-search
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

D1 to D4 of pass 2 hold:
- **D1:** `runs` reads the same in enumerate, result-format, simulate-adapter and batch-scoring.
- **D2:** result-page's "Not in this story" names result-best as the owner of the drawing.
- **D3:** orgspec's Scope and design's "Gaps per pack" both name the "Gaps found" section.
- **D4:** orgspec's Surfaces put its cases into `search_score.rs`.

No cycle: `tsort` on all 90 `depends_on` edges in the store is clean. The five new stories also keep the dominance, gate, strategy and page order consistent with the three cleared blockers: dominance with a front, the original as the gate floor, `set_field` on marked places only.

Findings:

- story:search-design — Context and Shared files still say the stories of the epic "write nowhere in" `design/metaspec.md` and that this story "merges after them". Yet its own Scope and the graph make gate, dominance, hill-climb, beam and result-best each depend on it and add a paragraph, and Context names only "hill climbing, beam search, the studio page" as later owners. Rewrite both passages to name the five stories as later owners with the edges. — `.engineering/planning/story/search-design.md:36-37`, `:78`, `:89-90`
- story:search-design — the five edges from gate, dominance, hill-climb, beam and result-best to design exist only so each can add a paragraph, but design depends on orgspec, which "stops and files a dependency-blocker" when the orgspec tool has no KPI values. That blocks the whole chain (waves 13 to 18, one story per wave) behind an external pack gap. Trade-off, shared file `design/metaspec.md`: keep the order and accept the stall, or give the five paragraphs to design (design then depends on the five and no edge points at it) or to a section file of their own. State the choice and its reason in design's Shared files. — `aep plan artifact waves` (waves 13 to 18), `.engineering/planning/story/search-orgspec.md:54`
- story:search-hill-climb — it says nothing of how a second strategy obtains a scored candidate. `search/run.rs` holds the loop, tool run, budget, deadline, stop and result writing. Batch-scoring groups candidates "in a search (`story:search-enumerate`...)" and simulate-adapter scores through `--simulate`. Hill-climb and beam lack `run.rs` in Surfaces and never say whether they honour `--simulate` and `batch`. A first-improvement scan scores one neighbour at a time, so batching it has a cost the batch story does not address. Say which functions of `run.rs` the strategies call (or that this story extracts them and lists `run.rs`), and say what `--simulate` and `batch` do under `hill_climb` and `beam`. — `.engineering/planning/story/search-hill-climb.md:55`, `.engineering/planning/story/search-batch-scoring.md:73-74`
- story:search-result-best — the component restates feasibility as "`gate_failed` not empty, or a state other than `ran`". Gate defines feasible as state `ran`, `valid` true and `gate_failed` empty, and says a page "computes nothing". The two predicates differ on `valid` false. Cite the predicate of `story:search-gate` verbatim, or have gate write `feasible` into the score so the page reads it. — `.engineering/planning/story/search-result-best.md:49-50`, `.engineering/planning/story/search-gate.md:61`
- story:search-score — `search::score::compare` states a validity order ("a valid score is better than an invalid one", "two invalid scores compare by fewer gated findings"). Dominance states a second rule for the same candidates ("an infeasible candidate ... is never better than a feasible one and is in no front") and nothing about order among infeasible ones. Two stories own "valid beats invalid". Drop the validity order from `compare`, or say that `order` reads it and how infeasible candidates rank. — `.engineering/planning/story/search-score.md:76-80`, `.engineering/planning/story/search-dominance.md:61-62`

What I read: 14 `story:search-*` bodies, the three cleared decision-blockers, `epic:shared-search`, and the pass-2 brief. I ran `relations`, `graph` (379 edges, 90 `depends_on`, walked outside the set), `waves` and `validate` (valid). I traced 14 acceptance reads to a producer in the set, and an edge records every one.

What I could not establish:
- **Out of lane, parallel-safety:** result-best and hill-climb or beam both edit `design/metaspec.md` with no edge between them. Simulate-adapter and the strategy stories both edit `search.rs` and `run.rs`. The planner's collision waves currently order both pairs.
- **Out of lane, acceptance and scope:** `best` is the sole front member, but the baseline is not in the front. A lone variant that is worse than the original on ungated KPIs becomes `best`. Gate only guards gated KPIs.
- **Out of lane, acceptance:** the result-best acceptance does not observe `strategy` and `beam_width`, which its Scope says it draws.

```findings
[
{"file":".engineering/planning/story/search-design.md","line":89,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"Context and Shared files say the stories of the epic write nowhere in design/metaspec.md and that this story merges after them, while gate, dominance, hill-climb, beam and result-best depend on it and each add a paragraph; name the five as later owners with their edges"},
{"file":"aep plan artifact waves","line":null,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:search-design — the five edges from gate, dominance, hill-climb, beam and result-best to design exist only for a paragraph of design/metaspec.md, and design depends on orgspec, which can stop on a pack gap, so waves 13 to 18 queue behind it; trade-off: keep the order and accept the stall, or give the five paragraphs to design (design depends on the five) or to a section file of their own, and state the choice in Shared files"},
{"file":".engineering/planning/story/search-hill-climb.md","line":55,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the body does not say how a second strategy gets a scored candidate from search/run.rs (tool run, budget, deadline, stop, result writing) and lists no run.rs in Surfaces, nor what --simulate and batch do under hill_climb and beam; name the shared functions or the extraction and the --simulate and batch behaviour"},
{"file":".engineering/planning/story/search-result-best.md","line":49,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the component defines an infeasible variant as gate_failed not empty or a state other than ran, which differs from search::score::feasible (ran, valid, gate_failed empty) of story:search-gate on valid false; cite gate's predicate or have the score carry feasible"},
{"file":".engineering/planning/story/search-score.md","line":76,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"compare states a validity order (valid beats invalid, invalid by fewer gated findings) that story:search-dominance restates as infeasible never better than feasible and silent on order among infeasible; one story owns it, so drop it from compare or say that order reads it"}
]
```
