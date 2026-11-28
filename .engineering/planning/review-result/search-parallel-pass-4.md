---
format: aep.planning-md/3
id: review-result:search-parallel-pass-4
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, pass 4, epic:shared-search
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

All four Pass 3 decisions hold (P4, P5, G1, G2). The only open items are four stories that don't name other-epic drafts on files they share.

- **P4:** Gate and dominance list `fixtures/expected/site` in Surfaces and scope. Each says it regenerates it through `task check:docs` after re-recording `fixtures/expected/search/<pack>.json`. Each names result-page, result-best and the five view stories (hub-badges, matrix-view, focus-pages, view-selectors, collapsed-regions) as co-writers. Gate's stale "simulate story is ordered by the planner" sentence is gone.
- **P5:** Result-best depends on beam, and beam on hill-climb. The reason is stated in its Shared files (`search-result-best.md:75`).
- **G1:** Orgspec writes its row of "Gaps per pack" and depends on design. Design names the six later writers, each with its edge, in Context and in Shared files (`search-design.md:90`). Design no longer reads a "Gaps found" section.
- **G2:** Hill-climb and beam name `search::run::score_points` in `run.rs`, and both list `run.rs` in Surfaces. Enumerate defines it. The ordering edges that make this safe are present: gate to dominance, dominance to hill-climb, hill-climb to beam, and result-best to all four.

Findings:

story:search-score — Shared files name only the five view stories and `story:cloudspec-manifest` as other writers of `crates/metaspec-core/src/manifest.rs` and `schema/metaspec-pack-1.schema.json`. They omit drafts `story:kpi-quality-values`, `story:spec-quality-conformance` and `story:spec-quality-drift`, which list both files. Cited, both bodies. Remedies: an ordering edge to those drafts with the shared file as its reason, or splitting the surface. Search-score already names other-epic drafts for `fixtures/manifests/*.yaml` with "No edge to drafts". — .engineering/planning/story/search-score.md:121

story:search-simulate-adapter — Shared files omit `story:kpi-quality-values`, `story:spec-quality-conformance` and `story:spec-quality-drift` as co-writers of `manifest.rs`, `schema/metaspec-pack-1.schema.json` and `crates/metaspec-core/src/adapter.rs`. The drift draft says no other story changes `adapter.rs`, which is wrong while this story exists. Cited, both bodies. — .engineering/planning/story/search-simulate-adapter.md:124

story:search-batch-scoring — Same omission for `manifest.rs`, `adapter.rs` and the pack-1 schema. `tool.rs` is also shared with the simulate adapter, but that is within the epic and ordered. Cited, both bodies. — .engineering/planning/story/search-batch-scoring.md:111

story:search-orgspec — "`packs/orgspec.yaml`: no other story of this epic changes it" leaves out drafts `story:kpi-quality-values` (adds a broken class) and `story:spec-quality-conformance` (names `blocks/known-failing.txt`), which also change the file. Cited, both bodies. — .engineering/planning/story/search-orgspec.md:77

What I read: 14 search stories, plus the shared sections of `kpi-quality-values`, `spec-quality-conformance` and `spec-quality-drift`. Commands: `aep plan artifact show` (for each), `aep plan artifact waves --kind story` (the full collision list), `aep plan artifact list`, and `git`/`rg` reads in the worktree.

Counts and notes for the operator:
- **Surfaces:** 14 of 14 items placed through cited paths (every body names its paths), 0 inferred-only, 0 unplaced.
- **Acceptance reads:** about 22 traced to a producer inside the set. Each has a `depends_on` edge, direct or through a chain (the most direct is hill-climb to dominance, beam to hill-climb, result-best to gate, dominance and the page).
- **Unordered pair, named, not a finding:** orgspec has no edge to gate, dominance, hill-climb, beam or result-best. It shares `design/metaspec.md` with them, in different sections (its row versus their paragraphs). Orgspec and design both name the pair.
- **Gate wording, note only:** gate's Shared files say `run.rs` is "created or changed by score, result-format, enumerate and simulate-adapter". That is loose, and it omits batch-scoring. Ordering holds through the gate-to-design edge, so this changes nothing.
- **Other-epic collisions, note only:** the implemented stories `contain-in-and-per`, `kpi-quality-catalogue-shape`, `kpi-quality-page`, `quality-catalogue` and `ux-lane-budget`, plus `fixtures/expected/site` shared with the view stories, are merged or already named.
- **Out of my lane:** nothing found that sets a verdict. Acceptance checkability, including the 12-point cases of A6 and A7, belongs to the acceptance critic.

```findings
[
{"file":".engineering/planning/story/search-score.md","line":121,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"Shared files omit drafts story:kpi-quality-values, story:spec-quality-conformance and story:spec-quality-drift as co-writers of crates/metaspec-core/src/manifest.rs and schema/metaspec-pack-1.schema.json (cited, both bodies); name them with an ordering edge or split the surface"},
{"file":".engineering/planning/story/search-simulate-adapter.md","line":124,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"Shared files omit drafts story:kpi-quality-values, story:spec-quality-conformance and story:spec-quality-drift as co-writers of manifest.rs, adapter.rs and schema/metaspec-pack-1.schema.json (cited, both bodies); name them with an ordering edge or split the surface"},
{"file":".engineering/planning/story/search-batch-scoring.md","line":111,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"Shared files omit drafts story:kpi-quality-values, story:spec-quality-conformance and story:spec-quality-drift as co-writers of manifest.rs, adapter.rs and schema/metaspec-pack-1.schema.json (cited, both bodies); name them with an ordering edge or split the surface"},
{"file":".engineering/planning/story/search-orgspec.md","line":77,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the claim that no other story of this epic changes packs/orgspec.yaml leaves out drafts story:kpi-quality-values and story:spec-quality-conformance, which also list it (cited, both bodies); name them with an ordering edge or split the surface"}
]
```
