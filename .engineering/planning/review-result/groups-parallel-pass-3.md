---
format: aep.planning-md/3
id: review-result:groups-parallel-pass-3
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, pass 3, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
approve

All three pass-2 findings are resolved, and I found no new collision between the two stories and the other planned metaspec stories.

**Pass 2 findings**

| # | Pass 2 finding | Now |
|---|---|---|
| 1 | The `block-groups` order line says the six later stories each `depends_on` "the one before it". | Resolved. The Surfaces section of `story:block-groups` now says "each of the last six `depends_on` story:band-wrap, as story:band-wrap words it, and none depends on another of the six" (`aep plan artifact show story:block-groups`, lines 161-165). The edges agree. |
| 2 | The `block-groups` scope names the whole directory `fixtures/expected/harbor`. | Resolved. The scope now lists `fixtures/expected/harbor/pages/plan`, `fixtures/expected/harbor/scenes/plan` and `fixtures/expected/site/harbor`. |
| 3 | `band-wrap` has the same wide scope entry. | Resolved. It now lists `pages/plan`, `scenes/plan` and `site/harbor`. In the `waves` output neither story collides on the harbor `quality/` goldens any more. |

**Shared files and order**

- **Shared files:** the two stories share files with each other and with the other planned stories. The shared files are `manifest.rs`, `pack.rs`, `lib.rs`, the pack schema, `documents.yaml`, `harbor.yaml`, `design/metaspec.md`, `CHANGELOG.md` and the harbor goldens. Every pair that is not already an edge is ordered by a `depends_on` chain.
- **Order of the D61 chain:**
  - `extends-inherits` (active, wave 10) comes before `block-groups` (wave 12), which comes before `band-wrap` (wave 14).
  - `block-groups` has `depends_on` on `extends-inherits` and `collapsed-regions`. `band-wrap` has `depends_on` on `block-groups`.
  - Each of `view-selectors`, `matrix-view`, `focus-pages`, `search-score`, `badge-width` and `placement-overlay` has `depends_on story:band-wrap`. None depends on another of the six. This matches the D61 braces.
  - `graph-viewport` has `depends_on story:focus-pages`, so it runs after both stories.
- **Search and cloudspec stories:**
  - Every active `search-*` story reaches `search-score` through `depends_on`. `search-score` depends on `band-wrap`, so none runs beside the two stories.
  - `cloudspec-simulate` and `cloudspec-runs` are ordered through `search-score` the same way.
  - These stories need no direct edge.
- **Bodies say so:**
  - Both bodies name the shared files and the edge for each of the six, for `graph-viewport` and for `extends-inherits`. `block-groups` also names `nested-readable` as not shared.
  - The two bodies agree on who edits `marina.yaml`: `block-groups` adds it, and `band-wrap` says it does not edit it.
- **No collision with `nested-readable`:** it lands only on `nested.rs`, and neither story edits that file.

**What I read:** 12 artifacts in the set, plus the relations of every other story. They are the two stories, the D61 stories, `graph-viewport`, `nested-readable` and `collapsed-regions`, plus `review-result:groups-parallel-pass-2`. I ran `aep plan artifact show`, `list --kind story` and `waves --kind story`, plus `git grep` on `fixtures/packs/harbor`.

**Surface tally:** 5 items cited and 6 inferred, 0 unplaced. `block-groups`, `band-wrap`, `extends-inherits`, `view-selectors` and `matrix-view` have cited surfaces. `focus-pages`, `search-score`, `badge-width`, `placement-overlay`, `graph-viewport` and `nested-readable` have inferred ones. I traced 4 acceptance reads to a producer in the set, and 4 have a `depends_on` edge:
- `band-wrap` reads `marina.yaml` from `block-groups`.
- `band-wrap` reads the regenerated goldens from `block-groups`.
- `block-groups` reads the harbor goldens from `collapsed-regions` (implemented, so already done).
- `block-groups` reads view fields from `extends-inherits`.

**What I could not establish**
- Whether the edits inside the shared files overlap. The collisions are file-level.
- Whether `nested.rs` still compiles after `block-groups` generalises `Region` and `fold` in `lib.rs`. Only a build shows it.
- Notes outside the two stories, not findings:
  - `view-selectors`, `matrix-view` and `focus-pages` share `manifest.rs`, `lib.rs` and the schema and have no edge among them (pre-existing).
  - `badge-width` and `nested-readable` share `nested.rs` with no edge (pre-existing).
  - `focus-pages` and `placement-overlay` share `documents.rs` with no edge (pre-existing).
  - The draft stories of other epics, `kpi-quality-values`, `spec-quality-conformance`, `spec-quality-drift` and `spec-quality-page`, share `manifest.rs`, the schema or the harbor goldens with the two stories. They enter no wave until accepted, so their acceptance should add an edge or an ordering against `story:band-wrap`.

```findings
[]
```
