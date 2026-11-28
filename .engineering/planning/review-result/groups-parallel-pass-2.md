---
format: aep.planning-md/3
id: review-result:groups-parallel-pass-2
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, pass 2, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
needs-revision

Pass 1 is resolved: all 10 findings are closed by recorded edges, and I re-read the recorded order. Two wording and scope defects remain.

- `story:block-groups` — its order line says the six stories after `story:band-wrap` each `depends_on` "the one before it". The edges say otherwise. `story:view-selectors`, `story:matrix-view`, `story:focus-pages`, `story:search-score`, `story:badge-width` and `story:placement-overlay` each `depends_on` `story:band-wrap` only, and none depends on another of the six. `story:band-wrap` words it correctly. A worker reading `block-groups` would take the six as serialised, which the D61 braces and the edges do not say. — `.engineering/planning/story/block-groups.md:178-180`
- `story:block-groups` — its scope names the whole directory `fixtures/expected/harbor` (inferred), but its body says "the harbor pages and site, regenerated". That directory also holds `quality/` and `scenes/`. In `aep plan artifact waves --kind story` the scope collides with 20 or more quality goldens, and `story:band-wrap` does the same. The draft quality stories cannot be judged against a surface this wide. Narrow the entry to the subdirectories the regeneration changes, such as `pages/plan`. — `.engineering/planning/story/block-groups.md:40`
- `story:band-wrap` — the same wide scope entry, with the same effect. — `.engineering/planning/story/band-wrap.md:33`

**Pass 1 findings**

| # | Pass 1 finding | Now |
|---|---|---|
| 1 | `block-groups` vs `extends-inherits` (blocker) | Resolved: `block-groups` has `depends_on story:extends-inherits`, and its body names the shared files. |
| 2 | `block-groups` vs `view-selectors`, `matrix-view`, `focus-pages` | Resolved: all three `depends_on story:band-wrap`, which `depends_on story:block-groups`. The body names the files. |
| 3 | `block-groups` vs `search-score` | Resolved: `search-score` has `depends_on story:band-wrap`, and the body names it. |
| 4 | `band-wrap` vs `extends-inherits` | Resolved: the body names `manifest.rs`, `pack.rs` and the schema, and the order runs through `block-groups`. |
| 5 | `band-wrap` vs `search-score` | Resolved. |
| 6 | `band-wrap` vs the three view stories | Resolved. |
| 7 | `badge-width` had no scope | Resolved: it now has a scope (inferred) and `depends_on story:band-wrap`, and both bodies name it. |
| 8 | `nested-readable` unplaced | Resolved: scope is `nested.rs` (inferred). Neither new story edits that file, and the layout collision list shows none with either. |
| 9 | `graph-viewport` vs `GraphView.tsx` | Resolved: `graph-viewport` `depends_on story:focus-pages`, so it runs after both new stories, and the `block-groups` body names the file. |
| 10 | `placement-overlay` unplaced | Resolved: it has a scope (inferred) and `depends_on story:band-wrap`. |

**Other planned metaspec stories**

- **Edges match D61.** In the `waves` output `extends-inherits` is in wave 10, `block-groups` 12, `band-wrap` 14, then `badge-width` and `placement-overlay` 15, `focus-pages` 16, `matrix-view` 17, `search-score` 18 and `view-selectors` 19.
- **Search stories are ordered by the graph.** `search-batch-scoring` and `search-simulate-adapter` cite `manifest.rs` and the pack schema, `search-candidate-space` cites `documents.yaml`, and `search-result-page` cites `documents.rs`. All of them sit downstream of `search-score`, so they land after `band-wrap`, and no extra edge is needed. The two bodies name only `search-score`; that is acceptable because the path is transitive.
- **Draft stories of other epics need no edge now.** `kpi-quality-values`, `spec-quality-conformance` and `spec-quality-drift` collide with both new stories on `crates/metaspec-core/src/manifest.rs` and `schema/metaspec-pack-1.schema.json` (cited on both sides). `kpi-quality-values` also shares `harbor.yaml` (cited) and the `pages/plan` goldens (inferred). Their "Shared files" sections name only each other, and neither new story names them. They enter no wave, so nothing runs beside the new stories. Acceptance of any of the three should add an edge, or an ordering that records the shared files, against `story:band-wrap`.

**Notes outside the set**
- `badge-width` and `nested-readable` both land on `nested.rs` with no edge (pre-existing).
- `focus-pages` and `placement-overlay` share `documents.rs` and are unordered (pre-existing).
- `view-selectors`, `matrix-view` and `focus-pages` share `manifest.rs`, `lib.rs` and the schema and are not ordered among themselves (pre-existing).

**What I read:** 11 artifacts in the set, plus the relations of every other story (search, spec-quality, kpi-quality), `review-result:groups-parallel-pass-1`, and `nested.rs` and `lib.rs` in the tree. I ran `aep plan artifact show`, `list --kind story` and `waves --kind story`, plus `git grep`.

**Surface tally:** 7 items cited, 4 inferred (`badge-width`, `placement-overlay`, `nested-readable`, `graph-viewport`), 0 unplaced. I traced 4 acceptance reads to a producer in the set and all 4 have a `depends_on` edge:
- `band-wrap` reads `marina.yaml` from `block-groups`.
- `band-wrap` reads the regenerated goldens from `block-groups`.
- `block-groups` reads the harbor goldens from `collapsed-regions`.
- `block-groups` reads the inherited `rows` from `extends-inherits`.

**What I could not establish**
- Whether the hunks inside the shared files overlap. The collisions are file-level.
- Whether `block-groups` leaves `nested.rs` compiling. `nested.rs` imports `Selection`, `Elision`, `Card` and `grow` from `lib.rs`, and `block-groups` generalises `Region` and `fold` there. Its body says it does not edit `nested.rs`; only the build would show it.
- Out of my lane: whether the draft quality stories belong in the same epic wave listing. That is the design critic's call.

```findings
[
{"file":".engineering/planning/story/block-groups.md","line":180,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the order line says story:view-selectors, story:matrix-view, story:focus-pages, story:search-score, story:badge-width and story:placement-overlay each depends_on 'the one before it', but the edges are one depends_on story:band-wrap each and none between the six, so the body records an order the store does not hold; word it as 'each depends_on story:band-wrap' as band-wrap.md:124 does"},
{"file":".engineering/planning/story/block-groups.md","line":40,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"scope names the whole directory fixtures/expected/harbor (inferred surface) while the body says 'the harbor pages and site, regenerated', so the directory also takes quality/ and scenes/ and `aep plan artifact waves --kind story` reports collisions with 20 or more quality goldens of the draft kpi-quality and spec-quality stories that are not real; narrow the entry to the subdirectories the regeneration changes"},
{"file":".engineering/planning/story/band-wrap.md","line":33,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"scope names the whole directory fixtures/expected/harbor (inferred surface) while the body says 'the harbor pages and site, regenerated', so the directory also takes quality/ and scenes/ and `aep plan artifact waves --kind story` reports collisions with 20 or more quality goldens of the draft kpi-quality and spec-quality stories that are not real; narrow the entry to the subdirectories the regeneration changes"}
]
```
