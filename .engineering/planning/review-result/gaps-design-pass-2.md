---
format: aep.planning-md/3
id: review-result:gaps-design-pass-2
kind: review-result
status: active
title: aep:plan-critic-design, pass 2, cloudspec gap stories
relations:
- reviews: epic:cloudspec-pack
- reviews: story:cloudspec-warnings
- reviews: story:part-block-aliases
- reviews: story:extends-inherits
revision: 1
---
needs-revision

I found no cycle. Four findings need a change: the extends-inherits ordering edge that holds the search chain, the `graph::read` signature ripple, the `directions` forward reference, and the `hub-badges` edge. All four are warnings. G1 to G11 and D43 to D50 hold in my lane. Nothing blocks the plan.

**Findings**
- story:extends-inherits — the new edge `story:search-score depends_on story:extends-inherits` is a bare relation line (search-score.md:12). The body of search-score and the Surfaces of extends-inherits give no reason for it, and Surfaces does not list search-score under "Shared, by story". The edge puts the whole search chain (14 stories, `cloudspec-runs` among them) behind hub-badges → warnings → aliases → the four view stories → extends-inherits. The only visible shared surface is `manifest.rs`, the schema and `design/metaspec.md` (search-score adds `kpis.directions`).
  - Fix: write that reason beside the edge, as a trade-off. Either keep the order for those files, or drop the edge and accept an additive merge on them. Cited: `extends-inherits.md:106-116`, `search-score.md:12`.
- story:part-block-aliases — D47 changes the signature of `graph::read`. It has 11 call sites. Besides `document.rs:126`, ten are in tests: `tests/cases/tools.rs:62,142`, `answers.rs:91`, `graph.rs:24,127,150`, `contain_edges.rs:12`, `exclude_edges.rs:144` in core, and layout `hubs.rs:308`, `contain_edges.rs:14`, `common.rs:121`. Scope lists only `tests/cases/graph.rs` and `manifest.rs`.
  - Fix: name the call sites in scope, or keep `read` as it is and apply the alias map by a method called from `document.rs` alone. Cited: `part-block-aliases.md:70`, `graph.rs:182`.
- story:extends-inherits — "Not in this story" tells an extending pack to declare `kpis` "(with `directions`)". `kpis.directions` is added by story:search-score, which now lands after this story, so the field does not exist yet. Fix: reword it to name story:search-score as the source. Cited: `extends-inherits.md:122`.
- story:cloudspec-warnings — G3's reason for `depends_on story:hub-badges` does not hold. Surfaces cites `acceptance.sh` and `packs/cloudspec.yaml`, but hub-badges edits neither. Its scope has only `CHANGELOG.md`, and commit `650a085` touches only `CHANGELOG.md` and the orchard fixture. The same paragraph says CHANGELOG is no ordering reason. Fix: name the real shared file, or drop the edge; `cloudspec-manifest` is already an edge. Cited: `cloudspec-warnings.md:56-61`.

**Cycle walk.** The dependency graph has no cycle. The topological sort (`tsort`) over all 110 edges ends cleanly with 0 errors.

**Decisions that hold**
- The refusal when an alias key is itself a block (D43) is consistent across the three stories.
- The single alias site (D47) is sound. The graph is the only reader of the written word, so style, `blocks` selection (`lib.rs:230`), `hubs.blocks`, labels, `blocks.rs:808` and `page.rs:236` all read the target.
- D44, D45, D46 and D49 hold. D50 holds: acceptance is one check per bullet. D48 holds too: the counts for cloudspec main `6259dce` are 6 `group` in `groups.yaml`, 3 `node_pool` in `node-pools.yaml`, 1 `endpoints` in `l4-media.yaml`, and 1 each of `payment_api` and `mail_relay` in `groups.yaml`. The targets `cloud.architecture`, `cloud.infrastructure` and `cloud.edge` exist, and none of `group`, `node_pool` or `endpoints` is a domain.

**What I read.** Three stories in full and the five view and hub stories, plus `search-score` and `cloudspec-runs`. I ran `aep plan artifact relations`, `graph` and `validate`; validate reported 108 artifacts, valid. I also read the code at `graph.rs:182`, `document.rs:126`, `lib.rs:230` and `adapter.rs`, and the cloudspec files at `origin/main`.

**Edges walked.** 110 `depends_on` edges across 40 stories, including every edge outside the three drafts. I traced 8 acceptance reads to a producer in the set, and an edge records all 8. Examples: extends (b) reads `packs/cloudspec.yaml`'s aliases and warnings (via the aliases edge), and the aliases row reads the warnings list.

**What I could not establish**
- Out of my lane: view-selectors, collapsed-regions, matrix-view and focus-pages are `active` (as are search-score and the search chain) but now depend on draft stories. Their own bodies carry the aliases edge with no reason; it is written only in part-block-aliases Surfaces. Whether in-flight work can wait on a draft is for the coordinator and parallel-safety.
- Out of my lane: the four view stories and aliases all edit `manifest.rs`, the schema and `design/metaspec.md`. Whether they can run at once is for parallel-safety.

```findings
[
{"file":".engineering/planning/story/extends-inherits.md","line":106,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:search-score depends_on story:extends-inherits (search-score.md:12) with no reason written beside the edge in either body, and Surfaces does not list story:search-score as sharing manifest.rs, the schema or design/metaspec.md; the edge holds the 14-story search chain and story:cloudspec-runs behind the chain hub-badges, cloudspec-warnings, part-block-aliases, the four view stories, extends-inherits; name the shared files and the trade-off (keep the order, or drop the edge and accept an additive merge) beside the edge"},
{"file":".engineering/planning/story/part-block-aliases.md","line":70,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"D47 changes the signature of graph::read (graph.rs:182), which has 11 call sites, 10 of them in tests of metaspec-core (tools.rs:62,142; answers.rs:91; graph.rs:24,127,150; contain_edges.rs:12; exclude_edges.rs:144) and metaspec-layout (hubs.rs:308; contain_edges.rs:14; common.rs:121), while scope names only tests/cases/graph.rs and manifest.rs; name the call sites in scope, or keep read unchanged and apply the alias map by a method called from document.rs alone"},
{"file":".engineering/planning/story/extends-inherits.md","line":122,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"'An extending pack declares its own `kpis` (with `directions`)' names a field that story:search-score adds, and search-score now lands after this story, so the field does not exist when this story lands; reword to name story:search-score as the source of `directions`"},
{"file":".engineering/planning/story/cloudspec-warnings.md","line":61,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"depends_on story:hub-badges has no reason that holds: Surfaces cites scripts/acceptance.sh and packs/cloudspec.yaml, but story:hub-badges scope lists neither (only CHANGELOG.md, which the same paragraph calls no ordering reason; commit 650a085 touches only CHANGELOG.md and the orchard fixture); name the shared file the edge records, or drop the edge, since story:cloudspec-manifest is already an edge"}
]
```
