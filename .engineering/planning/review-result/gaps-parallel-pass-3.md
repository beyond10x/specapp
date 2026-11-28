---
format: aep.planning-md/3
id: review-result:gaps-parallel-pass-3
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, pass 3, cloudspec gap stories
relations:
- reviews: epic:cloudspec-pack
- reviews: story:cloudspec-warnings
- reviews: story:part-block-aliases
- reviews: story:extends-inherits
revision: 1
---
approve

**What I read:** 3 stories (`story:cloudspec-warnings`, `story:part-block-aliases`, `story:extends-inherits`), the 3 new Outcome bullets of `epic:cloudspec-pack`, and `gaps-revise-2.md` (K1-K11). Commands: `cat -n` on the three drafts, `aep plan artifact graph`, `aep plan artifact validate` (108 artifacts, valid), `git grep graph::read` and `git log`, and a path grep of every story's scope for the shared surfaces.

**Pass-2 finding (K4 / D51), fixed.** The drafts now keep `graph::read` unchanged.
- `part-block-aliases.md:71-75` and `:96-97` say `graph::read` keeps its signature and its callers. The new `graph::read_with_aliases` has one caller, `crates/metaspec-docs/src/document.rs:126`.
- I checked this against the tree. `graph::read` at `graph.rs:182` has the same signature as before. Its other callers are all test files: 8 in `metaspec-core/tests/cases` and 3 in `metaspec-layout/tests/cases`. `document.rs:126` is the only non-test caller. No other story names `graph::read` or `read_with_aliases` except `search-candidate-space`, covered below.

**D52 (dropped `hub-badges` edge), no shared file left unordered.**
- `cloudspec-warnings` now depends only on `story:cloudspec-manifest`.
- `hub-badges` is merged into this integration branch (HEAD `5d87235`). Its scope has neither `scripts/acceptance.sh` nor `packs/cloudspec.yaml`. The files it shares with `part-block-aliases` (`manifest.rs`, `pack.rs`, the schema, `design/metaspec.md`) are ordered by history, so no edge is needed.
- Every other story whose scope names `packs/cloudspec.yaml` or `scripts/acceptance.sh` inside the set is reached by `depends_on`:
  - `part-block-aliases` depends on `cloudspec-warnings`.
  - `focus-pages` depends on `part-block-aliases`.
  - `extends-inherits` depends on `part-block-aliases`.
  - `cloudspec-runs` depends on `cloudspec-warnings`.
  - `search-score` depends on `extends-inherits`, which orders `main.rs`, the schema and `manifest.rs`.
- `CHANGELOG.md` is shared by all of them; the draft already calls it append-only lines.
- The new Outcome bullets match the three stories.

**Edits introduced nothing new.**
- Acceptance reads traced to a producer in the set: 4 (warnings from `cloudspec-manifest`, aliases from warnings, extends from aliases, `search-score` from extends). All 4 have a `depends_on` edge.
- `extends-inherits` reaches warnings only through `part-block-aliases`. That chain is stated in `extends-inherits.md:119`.

**Surfaces:** 3 items placed as cited, 0 inferred, 0 unplaced.

**What I could not establish, or leave outside my lane (none of it sets the verdict):**
- Outside the set:
  - `story:search-candidate-space` changes `crates/metaspec-core/src/graph.rs` and the code path that takes a part's block in `graph::read` (`.engineering/planning/story/search-candidate-space.md:63`, `:96`). `part-block-aliases` changes `read` and adds `read_with_aliases` in the same file, and neither body names the other. `search-candidate-space.md:96` calls itself graph.rs's "only story of the epic", which is true only within the shared-search epic. The two surfaces are cited and the stories are unordered. Remedies: an edge recording the shared file, or a split. The coordinator should also decide whether an aliased block word applies to a value from a marked `block: {value, vary}`.
  - `story:kpi-quality-values` (draft) also lists `scripts/acceptance.sh`. It has no edge to the cloudspec chain and does not mention it (`kpi-quality-values.md:86`). Pre-existing.
- `hub-badges.md` still has `status: active` although its merge is in git. The store's status is stale, and the dropped edge does not depend on it.
- Out of my lane: `tool::check(pack, …, Some(&graph), …)` at `document.rs:127` now receives the aliased graph, so the pack tool's input changes. This is a design question and I did not check it.

```findings
[]
```
