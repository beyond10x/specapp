---
format: aep.planning-md/3
id: review-result:search-parallel-pass-1
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, pass 1, epic:shared-search
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

Cross-epic collisions on `manifest.rs` and the pack schema are the main defects. I used these artifact-store paths, all under `.engineering/planning/story/` in the metaspec worktree. Citations are `path:line` in those files.

- story:search-score — it changes `crates/metaspec-core/src/manifest.rs` (`Kpis.directions`), `schema/metaspec-pack-1.schema.json`, `fixtures/manifests/harbor.yaml` and `orchard.yaml`. These collide with story:cloudspec-manifest (cited scope `manifest.rs` and the pack schema, which it lands after contain-in-and-per) and story:view-selectors (cited `View` in `manifest.rs`). They also collide with story:matrix-view, story:hub-badges, story:focus-pages and story:collapsed-regions, which add view or class properties and a fixture view (inferred, their bodies cite no files). "Shared files" names none of them — search-score.md:67-70, :72-76; cloudspec-manifest.md:19,29,61-62; view-selectors.md:22.
- story:search-simulate-adapter — same surface (`manifest.rs`, `tool.rs`, the pack schema, cited), and "Shared files" says only search-score and search-batch-scoring change them, so it omits cloudspec-manifest and the view stories. Remedy is an ordering edge that records the shared file as its reason, or splitting the surface — search-simulate-adapter.md:69-71, :77-79.
- story:search-simulate-adapter — it and story:search-candidate-space share `crates/metaspec/src/main.rs` and `crates/metaspec-core/src/search/mod.rs` (cited, both bodies). Neither depends on the other, since both depend on result-format and neither on the other. This body lists `main.rs` but not `search/mod.rs`, and names no collision — search-simulate-adapter.md:69-72, search-candidate-space.md:68-69, :75-76.
- story:search-batch-scoring — it changes `manifest.rs`, `adapter.rs`, `tool.rs` and the pack schema (cited). That collides with cloudspec-manifest and the cloudspec view stories exactly as search-score does (view-selectors cited, the other four inferred), and "Shared files" names only search stories — search-batch-scoring.md:71-73, :78-79.
- story:search-batch-scoring — the body does not say whether `runs` is written when no batch is declared. If it is, this story rewrites enumerate's goldens `fixtures/expected/search/<pack>.json`, which story:search-result-page reads and builds into `fixtures/expected/site`. This story and search-result-page have no ordering edge. The surface is inferred, from the unstated `runs` case — search-batch-scoring.md:49-53, :101; search-result-page.md:56-58.
- story:search-candidate-space — it changes `spec/domains/documents.yaml` (cited). story:cloudspec-manifest also lists it, as inferred scope. Neither body names the other — search-candidate-space.md:57, :69; cloudspec-manifest.md:33.
- story:search-result-page — "Shared files" names hub-badges, matrix-view and focus-pages for `fixtures/expected/site`, `documents.rs` and `types.ts`. It omits story:view-selectors (a fixture view page) and story:collapsed-regions (drill-down views in the docs site). The surface is inferred for those two, because their bodies cite no files — search-result-page.md:63-65.
- story:search-design — it writes the cloudspec row of "Gaps per pack" in `design/metaspec.md`, and story:cloudspec-manifest writes the same section ("gains cloudspec"). The body says it "merges after them", but names neither story and has no `depends_on` edge, so `waves` cannot order them — search-design.md:55-58, :80-82; cloudspec-manifest.md:68.
- story:search-score — no `scope` is recorded, so `waves` lists the story as `unassessed` and never places it. Cross-epic collisions with cloudspec-manifest, whose scope is recorded, cannot show. Record the Surfaces with `aep plan artifact scope` — search-score.md:8, and the `waves` output line `unassessed: story:search-score`.
- story:search-result-format — no `scope` is recorded, same as search-score — search-result-format.md:8, and the `waves` output line `unassessed: story:search-result-format`.
- story:search-simulate-adapter — no `scope` is recorded, same as search-score — search-simulate-adapter.md:8, and the `waves` output line `unassessed: story:search-simulate-adapter`.
- story:search-candidate-space — no `scope` is recorded, same as search-score — search-candidate-space.md:8, and the `waves` output line `unassessed: story:search-candidate-space`.
- story:search-enumerate — no `scope` is recorded, same as search-score — search-enumerate.md:8, and the `waves` output line `unassessed: story:search-enumerate`.
- story:search-batch-scoring — no `scope` is recorded, same as search-score — search-batch-scoring.md:8, and the `waves` output line `unassessed: story:search-batch-scoring`.
- story:search-result-page — no `scope` is recorded, same as search-score — search-result-page.md:8, and the `waves` output line `unassessed: story:search-result-page`.
- story:search-orgspec — no `scope` is recorded, same as search-score — search-orgspec.md:8, and the `waves` output line `unassessed: story:search-orgspec`.
- story:search-design — no `scope` is recorded, same as search-score — search-design.md:8, and the `waves` output line `unassessed: story:search-design`.

**What I read:**
- 9 search drafts, story:cloudspec-manifest, story:view-selectors, story:contain-in-and-per and the bodies of matrix-view, hub-badges, focus-pages and collapsed-regions.
- epic:shared-search and the graph.
- Commands: `aep plan artifact list/show/waves/graph/relations`, plus `git grep` and `ls` of the tree.
- Surfaces by item: 9 cited (from their Surfaces sections), 0 inferred, 0 unplaceable. The store records no scope for any of the 9.
- Acceptance reads traced to a producer in the set: 24. A `depends_on` edge records 22 directly; the other 2 (batch to the result schema, orgspec to the result schema) are covered transitively through search-enumerate.
- Within the set, only search-simulate-adapter and search-candidate-space form an unordered pair that shares files. The other unordered pairs (batch/page, batch/orgspec, page/orgspec) share only `CHANGELOG.md`, apart from the `runs` question above.

**What I could not establish:**
- The five cloudspec view stories cite no files or scope, so their collisions with the search stories are inferred from their bodies.
- Whether `contain-in-and-per` (merged at dd30514, status still `active`) already covers search-candidate-space's `graph.rs` claim ("its only story here"). I did not treat it as a collision.
- Out of the set and not counted: view-selectors, matrix-view, hub-badges, focus-pages and collapsed-regions each declare no `depends_on` on one another, yet all edit `manifest.rs` and the pack schema.
- Out of the set and not counted: `waves` shows kpi-quality-values and spec-quality-conformance also changing `packs/orgspec.yaml`, which search-orgspec changes.
- `CHANGELOG.md` is shared by nearly every story; I did not file it as a finding.
- Out of my lane: whether `check-design-search.sh` is brittle when other epics add schema files (design critic).

```findings
- file: .engineering/planning/story/search-score.md
  line: 67
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "changes manifest.rs, metaspec-pack-1 schema and fixtures/manifests/harbor.yaml, orchard.yaml, which story:cloudspec-manifest and story:view-selectors also change (cited) and matrix-view, hub-badges, focus-pages, collapsed-regions probably change (inferred); Shared files names none and no edge or split records it"
- file: .engineering/planning/story/search-simulate-adapter.md
  line: 77
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "Shared files lists only search-score and search-batch-scoring for manifest.rs, tool.rs and the pack schema, omitting story:cloudspec-manifest and story:view-selectors (cited) and the other four view stories (inferred)"
- file: .engineering/planning/story/search-simulate-adapter.md
  line: 69
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "shares crates/metaspec/src/main.rs and crates/metaspec-core/src/search/mod.rs with story:search-candidate-space, which has no ordering edge to it, and does not say so"
- file: .engineering/planning/story/search-batch-scoring.md
  line: 78
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "changes manifest.rs, adapter.rs, tool.rs and the pack schema alongside story:cloudspec-manifest and story:view-selectors (cited) and the other four view stories (inferred), and Shared files names only search stories"
- file: .engineering/planning/story/search-batch-scoring.md
  line: 49
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "does not say whether runs is written when no batch is declared; if it is, this story rewrites fixtures/expected/search/<pack>.json that story:search-result-page reads, with no edge between them (inferred)"
- file: .engineering/planning/story/search-candidate-space.md
  line: 69
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "changes spec/domains/documents.yaml, which story:cloudspec-manifest lists as inferred scope, and neither body names the other"
- file: .engineering/planning/story/search-result-page.md
  line: 63
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "Shared files names hub-badges, matrix-view and focus-pages for fixtures/expected/site, documents.rs and types.ts but omits story:view-selectors and story:collapsed-regions, which also regenerate fixture pages (inferred)"
- file: .engineering/planning/story/search-design.md
  line: 80
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "writes the cloudspec row of Gaps per pack in design/metaspec.md that story:cloudspec-manifest also writes, and says it merges after without naming it or carrying a depends_on edge"
- file: .engineering/planning/story/search-score.md
  line: 8
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "no scope is recorded, so waves leaves the story unassessed and cannot show its collisions with story:cloudspec-manifest; record the Surfaces with aep plan artifact scope"
- file: .engineering/planning/story/search-result-format.md
  line: 8
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "no scope is recorded, so waves leaves the story unassessed; record the Surfaces with aep plan artifact scope"
- file: .engineering/planning/story/search-simulate-adapter.md
  line: 8
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "no scope is recorded, so waves leaves the story unassessed; record the Surfaces with aep plan artifact scope"
- file: .engineering/planning/story/search-candidate-space.md
  line: 8
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "no scope is recorded, so waves leaves the story unassessed; record the Surfaces with aep plan artifact scope"
- file: .engineering/planning/story/search-enumerate.md
  line: 8
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "no scope is recorded, so waves leaves the story unassessed; record the Surfaces with aep plan artifact scope"
- file: .engineering/planning/story/search-batch-scoring.md
  line: 8
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "no scope is recorded, so waves leaves the story unassessed; record the Surfaces with aep plan artifact scope"
- file: .engineering/planning/story/search-result-page.md
  line: 8
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "no scope is recorded, so waves leaves the story unassessed; record the Surfaces with aep plan artifact scope"
- file: .engineering/planning/story/search-orgspec.md
  line: 8
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "no scope is recorded, so waves leaves the story unassessed; record the Surfaces with aep plan artifact scope"
- file: .engineering/planning/story/search-design.md
  line: 8
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "no scope is recorded, so waves leaves the story unassessed; record the Surfaces with aep plan artifact scope"
```
