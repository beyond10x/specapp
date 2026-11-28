---
format: aep.planning-md/3
id: review-result:search-design-pass-7
kind: review-result
status: active
title: aep:plan-critic-design, pass 7, epic:shared-search
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

pass 6's V4 holds: `search::run::stop_reason(...)` is named once, in `story:search-enumerate`, and hill climbing and beam both call it. V5 half-holds. The design paragraphs name the reason and no more, but enumerate, hill climbing and beam each still carry a stop-reason owner or copy that contradicts "written once by `story:search-design`".

- **Quoted stop-reason block:** `story:search-hill-climb` quotes the whole stop-reason block in its Scope and calls enumerate "their one owner". `story:search-beam` does the same.
- **Stale ownership sentence:** enumerate still says "the strategy stories quote this text". `design/metaspec.md` is meant to hold the text once, so this sentence is stale. There are now four copies of the text and three claims of ownership.
- **`StopReason` has no home:** its fourth variant `NoBetterNeighbour` is required by enumerate's own unit test. `reason` has three values in `story:search-result-format`, and the fourth reaches the schema only in `story:search-dominance`.

The other pass-6 items hold:
- V1: the gate stub, its values and its place in Surfaces are present.
- V2: the invalid-grid beam case is present.
- V3: the infeasible-original climb case is present.
- The hill-climb title now says "ranks above" instead of "dominates".

I found no cycle, no chain without a written reason, and no new horizontal slice.

Findings:
- story:search-hill-climb — its Scope quotes the stop-reason block as a second copy and names `story:search-enumerate` "their one owner", while `story:search-design` writes the reasons once. Replace the quote with a pointer to `stop_reason` in enumerate and to the "Decided: shared search" section — `.engineering/planning/story/search-hill-climb.md:86-92` against `.engineering/planning/story/search-design.md:245-247`
- story:search-beam — the same quoted block and "their one owner" in Scope, and its summary says "stop reasons as hill climbing", which points at a third place. Drop the quote and fix the summary — `.engineering/planning/story/search-beam.md:73-80`, `.engineering/planning/story/search-beam.md:7`
- story:search-enumerate — it declares itself "the one owner of the stop reasons" and says "the strategy stories quote this text", while design writes the text once from it and says the strategy paragraphs restate none. Change it to say enumerate defines `stop_reason` and the semantics, `story:search-design` writes the text once, and nobody quotes it — `.engineering/planning/story/search-enumerate.md:90-91`, `.engineering/planning/story/search-design.md:246-247`
- story:search-enumerate — `StopReason` is named with four variants and its unit test needs `Some(NoBetterNeighbour)`, but no scope or surface names the file that defines the enum. The typed writer in `result.rs` belongs to `story:search-result-format`, whose `reason` has three values, and the schema's fourth value comes only from `story:search-dominance`, which depends on enumerate. Name the file, and say the variant exists in Rust here but is not serialized until dominance lands — `.engineering/planning/story/search-enumerate.md:97-98`, `.engineering/planning/story/search-enumerate.md:170-172`, `.engineering/planning/story/search-result-format.md:173`, `.engineering/planning/story/search-dominance.md:82`

1. **What I read:** 14 search stories whole (`aep plan artifact show` content read from the store files), the brief, plus `aep plan artifact relations`, `graph` (385 edges, 103 `depends_on` across the store), `waves` and `validate` (valid). I fed the 103 `depends_on` edges to `tsort`: acyclic, including edges to `story:cloudspec-manifest`, `story:cloudspec-runs` and other epics. I traced about 16 acceptance reads to a producer in the set, and an edge records every one. The six later design-paragraph writers all carry the `depends_on` edge to `story:search-design` that its text promises.
2. **Could not establish:** nothing blocking. One unease, left out of the findings: `story:search-design` writes four stop reasons before `story:search-dominance` adds the fourth to the schema, so the design text runs ahead of the schema until dominance lands. Both are ordered by edges, so this is not a defect.
3. **Out of my lane:**
   - `story:search-hill-climb` says "none of the three cases below does" while it has four cases (`.engineering/planning/story/search-hill-climb.md:93`). This is an acceptance-wording matter.
   - `fixtures/expected/search/<pack>.json` is written by five stories, and the shared-file lists do not all name each other. This is parallel safety.

```findings
[
{"file":".engineering/planning/story/search-hill-climb.md","line":86,"category":"design","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"its Scope quotes the stop-reason block as a second copy and names story:search-enumerate 'their one owner', while story:search-design writes the reasons once; replace the quote with a pointer to stop_reason in enumerate and to the 'Decided: shared search' section"},
{"file":".engineering/planning/story/search-beam.md","line":73,"category":"design","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"its Scope quotes the stop-reason block as a copy and names story:search-enumerate 'their one owner', and its summary says 'stop reasons as hill climbing', a third place; drop the quote and fix the summary so the reasons are written once by story:search-design"},
{"file":".engineering/planning/story/search-enumerate.md","line":90,"category":"design","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"it declares itself 'the one owner of the stop reasons' and says 'the strategy stories quote this text', while story:search-design writes the text once and says the strategy paragraphs restate none; say enumerate defines stop_reason and the semantics, story:search-design writes the text once, and nobody quotes it"},
{"file":".engineering/planning/story/search-enumerate.md","line":97,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"StopReason is named with four variants and its unit test needs Some(NoBetterNeighbour), but no scope or surface names the file that defines the enum, and result-format's typed writer lists three reasons while the fourth reaches the schema only in story:search-dominance; name the file and say the variant exists in Rust here and is not serialized until dominance lands"}
]
```
