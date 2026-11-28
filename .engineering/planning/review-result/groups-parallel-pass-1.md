---
format: aep.planning-md/3
id: review-result:groups-parallel-pass-1
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, pass 1, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
needs-revision

Neither story can be worked beside the stories that share its files, because the plan records no order for them. The collisions below come from `aep plan artifact waves --kind story`; `story:extends-inherits` is already in implementation on the same files.

**Where they land**
- `story:block-groups` and `story:band-wrap` both edit `manifest.rs`, `pack.rs`, `crates/metaspec-layout/src/lib.rs`, the pack schema, `spec/domains/documents.yaml`, `design/metaspec.md`, `CHANGELOG.md`, `harbor.yaml` and the harbor goldens.
- `story:band-wrap` does depend on `story:block-groups`, and both bodies say so. Both depend on `story:collapsed-regions`, which is implemented.
- Neither has a `depends_on` edge to `story:extends-inherits`, `story:view-selectors`, `story:matrix-view`, `story:focus-pages` or `story:search-score`.

**What the plan says**
- `story:block-groups` names `story:extends-inherits`, the three view stories, `story:badge-width` and the other drafts in prose (`block-groups.md:168-181`), but orders none of them. It does not name `story:search-score`.
- `story:extends-inherits` says it "lands before story:view-selectors, story:matrix-view and story:focus-pages" (`extends-inherits.md:127`) and does not name either new story.
- `story:band-wrap` names the three view stories. It only calls `story:extends-inherits` one of "the only other stories with a layout or a manifest section of their own" (`band-wrap.md:121-123`). That does not say the two share `manifest.rs`, `pack.rs` and the pack schema. It does not name `story:search-score` either.
- The wave listing puts `story:block-groups` in wave 10 and `story:extends-inherits` in wave 11, though the latter is already active. Nothing in the store records that order.

**Which name no surface**
- `story:badge-width`, `story:nested-readable`, `story:graph-viewport` and `story:placement-overlay` have no `scope`, so `waves` does not list them and runs no collision check on them.
- `story:badge-width`:
  - Its body cites no path.
  - Both new stories claim to share `rows.rs` and `lib.rs` with it, and neither orders it.
- `story:nested-readable`: its body cites no path. Both new stories rely on `nested.rs` for "not shared", but that file is not in its body.
- `story:graph-viewport`: it names only the symbols `GraphView` and `GraphSwitch`. `story:block-groups` edits `web/ui/src/GraphView.tsx` at lines 52, 67 and 135, so the two collide in one file.
- `story:placement-overlay`: it names neither a file nor a symbol, and neither new story mentions it.


**What I read:** 11 artifacts (`story:block-groups`, `story:band-wrap`, `story:extends-inherits`, `story:view-selectors`, `story:matrix-view`, `story:focus-pages`, `story:search-score`, `story:nested-readable`, `story:graph-viewport`, `story:badge-width`, `story:placement-overlay`). I used `aep plan artifact show`, `graph`, `relations` and `waves --kind story`, plus `git grep` on the tree.

**Surface tally:**
- Cited: 7 items.
- Inferred: 3 items (`story:badge-width`, `story:nested-readable`, `story:graph-viewport`).
- Unplaced: 1 item (`story:placement-overlay`).
- Acceptance reads traced to a producer in the set: 3 (the existing harbor goldens from `story:collapsed-regions`; `marina.yaml` and the regenerated goldens from `story:block-groups`, read by `story:band-wrap`). All 3 have a `depends_on` edge: `story:block-groups` to `story:collapsed-regions`, and `story:band-wrap` to `story:block-groups` for the other two.

**What I could not establish**
- Which story edits which lines. The collisions are file-level, and several are inferred from the `scope` field. I did not check whether the hunks overlap.
- Whether the draft stories `story:nested-readable`, `story:graph-viewport` and `story:placement-overlay` will touch `rows.rs`, `lib.rs` or `manifest.rs`, because their scopes are not yet written.
- Out of my lane and not counted in the verdict: the body of `story:band-wrap` (acceptance 11) leaves open whether it creates `marina.yaml` itself or reuses the one from `story:block-groups`. That is for the acceptance critic.

```findings
[
{"file":".engineering/planning/story/block-groups.md","line":172,"category":"parallel-safety","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:block-groups and story:extends-inherits (active, in implementation) both land on crates/metaspec-core/src/manifest.rs, crates/metaspec-core/src/pack.rs and schema/metaspec-pack-1.schema.json (cited, both bodies; design/metaspec.md, CHANGELOG.md and the test files are inferred); block-groups names the sharing in prose but has no depends_on edge to story:extends-inherits and gives no order, and extends-inherits.md:127 names neither new story; either add the edge recording the shared files as its reason, or split the surface so the two no longer share it"},
{"file":".engineering/planning/story/block-groups.md","line":170,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:block-groups and the active stories story:view-selectors, story:matrix-view and story:focus-pages share manifest.rs, crates/metaspec-layout/src/lib.rs, the pack schema, fixtures/manifests/harbor.yaml and the regenerated harbor pages/plan goldens (cited in block-groups; matrix-view also scene.rs and the scene schema, focus-pages also page.rs and documents.rs); the body names them but records no depends_on edge and none of those three bodies names block-groups; either add edges naming the shared files as the reason or split the surface so they no longer share it"},
{"file":".engineering/planning/story/block-groups.md","line":168,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:block-groups collides with story:search-score on manifest.rs and schema/metaspec-pack-1.schema.json (cited, both bodies) and on fixtures/manifests/harbor.yaml, design/metaspec.md and CHANGELOG.md (inferred), and the 'Shared with other planned stories' list does not name story:search-score; add the edge or split the surface, and name it"},
{"file":".engineering/planning/story/band-wrap.md","line":121,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:band-wrap and story:extends-inherits share manifest.rs, pack.rs and the pack schema (cited, both bodies), but the line only calls extends-inherits one of the stories with 'a manifest section of their own', without saying the files are shared, and no depends_on edge orders them; name the shared files and add the edge (or split the surface); an edge from block-groups to extends-inherits would order it transitively"},
{"file":".engineering/planning/story/band-wrap.md","line":112,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:band-wrap collides with story:search-score on manifest.rs and the pack schema (cited, both bodies) and on harbor.yaml, design/metaspec.md and CHANGELOG.md (inferred), and the 'Shared with other planned stories' list does not name story:search-score; add the edge or split the surface, and name it"},
{"file":".engineering/planning/story/band-wrap.md","line":121,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:band-wrap and the active stories story:view-selectors, story:matrix-view and story:focus-pages share manifest.rs, lib.rs, the pack schema, harbor.yaml and the harbor pages/plan goldens (cited in band-wrap); the body names them but records no depends_on edge and none of the three bodies names band-wrap; add edges naming the shared files as the reason or split the surface"},
{"file":".engineering/planning/story/badge-width.md","line":23,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:badge-width has no scope and its Scope section cites no path, so it is absent from `aep plan artifact waves --kind story` and unassessed; block-groups.md:172 and band-wrap.md:118 both claim it shares lib.rs/rows.rs (inferred surface: rows.rs, nested.rs, lib.rs by the nouns 'rows', 'nested', 'badges') and no depends_on edge orders it against either; establish the surface and the order, or leave it out of any concurrent set"},
{"file":".engineering/planning/story/nested-readable.md","line":18,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:nested-readable has no scope and its Scope section cites no path, so it is unassessed (inferred surface: crates/metaspec-layout/src/nested.rs by the noun 'nested'); block-groups.md:179 and band-wrap.md:124 call it 'not shared' on a surface its own body never names; establish the surface or leave it out of any concurrent set"},
{"file":".engineering/planning/story/graph-viewport.md","line":22,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:graph-viewport has no scope and cites no path, only the symbols GraphView and GraphSwitch (inferred surface: web/ui/src/GraphView.tsx), and block-groups.md:179 edits that same file at :52, :67 and :135; its only edge is to story:focus-pages, none to block-groups; establish the surface and record the order or the split"},
{"file":".engineering/planning/story/placement-overlay.md","line":21,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:placement-overlay has no scope, cites no path or symbol, has no relations, and neither block-groups nor band-wrap names it; it cannot be placed (it plausibly adds a manifest mapping, so manifest.rs and the pack schema are likely, which is inferred); establish the surface or leave it out of any concurrent set"}
]
```
