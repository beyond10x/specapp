---
format: aep.planning-md/3
id: review-result:gaps-design-pass-1
kind: review-result
status: active
title: aep:plan-critic-design, pass 1, cloudspec gap stories
relations:
- reviews: epic:cloudspec-pack
- reviews: story:cloudspec-warnings
- reviews: story:part-block-aliases
- reviews: story:extends-inherits
revision: 1
---
needs-revision

Findings, one line each. None is a cycle. All five are `introduced` and `warning`.

- story:cloudspec-runs — it edits `packs/cloudspec.yaml` (a `batch` entry on `check`, plus `kpis.directions`). It has no Surfaces section and no edge to the chain of drafts that serialises that file. `check` is the same verb story:cloudspec-warnings adds a finding list to. Add `depends_on story:cloudspec-warnings` (or the reverse) and name the file in both Surfaces sections. The alternative is to leave the two unordered and accept a hand merge inside one verb entry. — `.engineering/planning/story/cloudspec-runs.md:38-41`, `.engineering/planning/story/cloudspec-warnings.md:57-59`
- story:part-block-aliases — one alias lookup is specified at two sites, the graph and the layout style. `graph::read(class, path, doc)` takes no manifest (`crates/metaspec-core/src/graph.rs:182`), and its only caller is in `metaspec-docs` (`crates/metaspec-docs/src/document.rs:126`), which is not in the Surfaces. View selection compares the raw `part.block` (`crates/metaspec-layout/src/lib.rs:230-234`), and hub-badges `hubs.blocks` and matrix grouping read it too. The body should name one site (the graph) and say that view `blocks`, `hubs.blocks` and matrix grouping read the target block. — `.engineering/planning/story/part-block-aliases.md:67-68`
- story:extends-inherits — `blocks.aliases` lives inside `blocks`, and inheritance of `blocks` is all-or-nothing ("never both"). The body does not say that the aliases travel with an inherited `blocks`. Nor does it say that the alias target check runs against the base's blocks directory, whereas part-block-aliases refuses at "every command that loads the pack". An extending pack also cannot add one alias without copying all of `blocks`. — `.engineering/planning/story/extends-inherits.md:108`, `.engineering/planning/story/part-block-aliases.md:70-71`
- story:extends-inherits — it inherits `tool` but not `kpis` (line 81). story:cloudspec-runs declares `batch` under `tool.verbs.check` (inherited) and `kpis.directions` under `kpis` (not inherited). An extending pack therefore gets half of the search configuration. The body should say how `kpis.directions` reaches an extending pack. — `.engineering/planning/story/extends-inherits.md:81`, `.engineering/planning/story/cloudspec-runs.md:38-41`
- story:cloudspec-warnings — the edge to story:hub-badges records only `CHANGELOG.md`, which all five view stories share. The one real shared file, `scripts/acceptance.sh` with story:focus-pages, has no edge. The edge puts a four-deep chain, hub-badges → warnings → aliases → extends-inherits, behind an active layout story for a change to one key of a yaml file. The trade-off is to keep the edge, or to take `CHANGELOG.md` out of the ordering (append-only lines) and edge only on `scripts/acceptance.sh`. I am not asking for the edge to go. — `.engineering/planning/story/cloudspec-warnings.md:57-59`

I read 4 stories in full (the three drafts plus cloudspec-runs), the 5 active view stories, search-score and search-batch-scoring, and `packs/cloudspec.yaml`. Commands: `aep plan artifact show`, `relations`, `graph` and `validate` (valid), plus `git diff`.

I walked every `depends_on` edge in the store, not only the ids you gave me. The chain is hub-badges → warnings → aliases → extends-inherits (plus collapsed-regions), all ending at story:cloudspec-manifest → story:contain-in-and-per. Nothing depends back on the drafts, so there is no cycle. I traced 3 acceptance reads to a producer in the set. Two of them are from extends-inherits to the aliases and warnings entries in `packs/cloudspec.yaml`, one direct and one through the chain. The third is runs reading nothing from the three drafts. All three are recorded by an edge.

Out of my lane (parallel safety or acceptance):
- part-block-aliases claims shared files with view-selectors, matrix-view, focus-pages and collapsed-regions (`manifest.rs`, `layout/src/lib.rs`, the schema), but its only edge reaches hub-badges.
- extends-inherits has an edge to collapsed-regions only, though it shares the same files with hub-badges and view-selectors.
- The inherited tool's working directory is unstated. `tool::run` takes one `root` for both the program path and the working directory (`crates/metaspec-core/src/tool.rs:126`).
- `--base-pack` on `docs build` in multi-pack mode is unspecified. That command does not use `PackArgs`.
- `Kpis.path` is required (`crates/metaspec-core/src/manifest.rs`), and cloudspec has no catalogue, so `kpis.directions` in the runs story may not be declarable as written.

```findings
[
{"file":".engineering/planning/story/cloudspec-runs.md","line":38,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"it edits packs/cloudspec.yaml (a batch entry on the check verb, kpis.directions) with no Surfaces section and no edge to story:cloudspec-warnings, which adds a finding list to the same check verb; the warnings, aliases and extends-inherits chain was built to serialise that file and leaves this fourth owner outside it"},
{"file":".engineering/planning/story/part-block-aliases.md","line":67,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"one alias lookup is specified at two sites, the graph and the layout style, while graph::read takes no manifest and view block selection, hubs.blocks and matrix grouping read the raw part.block; the body does not name the one site or say those selectors read the target block"},
{"file":".engineering/planning/story/extends-inherits.md","line":108,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"blocks.aliases sits inside blocks and blocks is inherited all or nothing, but the body does not say the aliases travel with an inherited blocks section or that the alias target check runs against the base's blocks directory"},
{"file":".engineering/planning/story/extends-inherits.md","line":81,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"tool is inherited but kpis is not, while story:cloudspec-runs puts batch under tool and kpis.directions under kpis, so an extending pack gets half of the search configuration; the body does not say how kpis.directions reaches it"},
{"file":".engineering/planning/story/cloudspec-warnings.md","line":57,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the depends_on edge to story:hub-badges records only CHANGELOG.md, which every view story shares, while the real shared file scripts/acceptance.sh with story:focus-pages has no edge; the edge holds aliases and extends-inherits behind an active layout story for a one-key yaml change, so choose between keeping it and edging on acceptance.sh alone"}
]
```
