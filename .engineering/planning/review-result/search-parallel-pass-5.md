---
format: aep.planning-md/3
id: review-result:search-parallel-pass-5
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, pass 5, epic:shared-search
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

- story:search-gate — Shared files still call the order of `story:search-simulate-adapter` and `story:search-batch-scoring` against this story "before or after" and "the planner orders them". The graph already puts both before it, because `depends_on` runs gate to `story:search-design` to simulate-adapter and batch-scoring. Decision G7 asked to "say which stories before it (by name) write run.rs, batch-scoring included". Both stories are named, but the sentence still leaves the order open. The same wording sits on the score-schema bullet, "which the planner orders against this story". Say "before it, through `story:search-design`" in both places. Cited. The order is settled by the `depends_on` edges shown by `aep plan artifact graph`. — .engineering/planning/story/search-gate.md:93, .engineering/planning/story/search-gate.md:91

**What I read.** All 14 `story:search-*` drafts in full, and `story:cloudspec-runs`. I ran `aep plan artifact show` on each, plus `aep plan artifact waves --kind story --status draft` and `aep plan artifact graph`. I also ran `aep plan artifact validate`, which prints `valid` with no search-related warnings, and grepped the story files and the tree. For the cross-epic pairs I filtered the waves collision list to pairs with exactly one search story.

**Pass 4 decisions.**
- P6 landed. score, simulate-adapter and batch-scoring each name `story:kpi-quality-values`, `story:spec-quality-conformance` and `story:spec-quality-drift` as co-writers of `manifest.rs`, `adapter.rs` and `schema/metaspec-pack-1.schema.json`. They say "no edge to drafts".
- P7 landed. `story:search-orgspec` names `story:kpi-quality-values` and `story:spec-quality-conformance` on `packs/orgspec.yaml`. The "no other story of this epic changes it" sentence is gone.
- All 25 cross-epic collision lines the tool prints for search stories are named in the owning search story's Shared files. These are the manifest, adapter and schema trio, `harbor.yaml`, `orchard.yaml`, `orgspec.yaml`, `crates/metaspec/tests/main.rs` and `Taskfile.yml`.
- G5 landed: hill-climb and beam name simulate-adapter and batch-scoring as owners inside `score_points` and depend on both.
- G6 landed: dominance owns `search::order::front` and beam uses it.
- Only the second half of G7 is not applied, which is the finding above. Enumerate is no longer named as a schema writer.

**Counts.**
- Surfaces cited: 14 of 14 stories. Inferred: 0. Not placed: 0. Every body has a Surfaces section with paths.
- Acceptance reads traced to a producer in the set: 35. All 35 are recorded by a `depends_on` edge, some directly and some through `story:search-design`.

**What I could not establish.**
- Out of my lane, for the acceptance critic: the `story:search-gate` acceptance case "a gated KPI the baseline answered and the candidate does not answer" names no stub or hand-built score that omits a KPI. Gate's Surfaces list no `fixtures/search/*` file. If gate adds a stub, it collides with the stubs written by enumerate, simulate-adapter and batch-scoring, but those stories all come before it by edge.
- The other epics' stories are drafts, and `story:view-selectors`, `story:hub-badges`, `story:matrix-view` and `story:focus-pages` declare no scope. I took their overlap only from what the search bodies name.

```findings
[
{"file":".engineering/planning/story/search-gate.md","line":93,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"Shared files says run.rs is changed 'before or after it' by story:search-simulate-adapter and story:search-batch-scoring, and the score schema is 'ordered against this story by the planner'; the graph already puts both before this story through story:search-design, so decision G7 (say which stories before it write run.rs, batch-scoring included) is not applied: state 'before it, through story:search-design' on both bullets (cited, both bodies and the depends_on edges)"}
]
```
