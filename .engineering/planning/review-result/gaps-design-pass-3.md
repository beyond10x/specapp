---
format: aep.planning-md/3
id: review-result:gaps-design-pass-3
kind: review-result
status: active
title: aep:plan-critic-design, pass 3, cloudspec gap stories
relations:
- reviews: epic:cloudspec-pack
- reviews: story:cloudspec-warnings
- reviews: story:part-block-aliases
- reviews: story:extends-inherits
revision: 1
---
**approve**

K3, K4, K10 and K11 are fixed. The edits introduced nothing that needs revision. `aep plan artifact validate` reports `valid`; that is not a finding.

**Pass-2 findings**
- **K3:** story:cloudspec-warnings declares only `depends_on story:cloudspec-manifest` (no `story:hub-badges` edge). The Surfaces paragraph says it has no edge to hub-badges and gives D52 as the reason. The "in implementation" wording is gone.
- **K4:** story:part-block-aliases Scope and Surfaces both say `graph::read` keeps its signature and callers. They name `graph::read_with_aliases`, called only from `crates/metaspec-docs/src/document.rs:126`. The four tests carry the `alias_` prefix, one bullet each.
- **K10:** in story:extends-inherits, "Not in this story" says `kpis.directions` comes from story:search-score, which lands after this story. An extending pack declares its own `kpis`.
- **K11:** Surfaces under "Shared, by story" lists story:search-score with `manifest.rs`, the schema, `design/metaspec.md` and `crates/metaspec/src/main.rs`. It states the trade-off: the order is kept so the consumer's gaps land first, and the alternative was an additive merge on those files. The ordering edge exists: `story:search-score depends_on story:extends-inherits`.

**Cycle check**
- I walked 116 `depends_on` and `blocks` edges across the whole store, not only the three stories. Both mean "needs first"; I flipped `blocks` to match. `tsort` reports no cycle.
- The chain around the three stories is a DAG: cloudspec-manifest → cloudspec-warnings → part-block-aliases → {collapsed-regions, view-selectors, matrix-view, focus-pages} → extends-inherits → search-score → cloudspec-runs.
- Each ordering edge has its shared file named beside it in the Surfaces paragraphs, so I do not report it as a serialising chain. The trade-off is the one K11 states.

**Acceptance reads**
- I traced 6 acceptance reads to a producer in the set, and an edge records all 6.
  - The aliases rows read the `aliases` entry in `packs/cloudspec.yaml`, written by the same story.
  - The extends rows read the aliased graph from part-block-aliases, a direct edge.
  - The extends warnings row reads the finding list from cloudspec-warnings, recorded through part-block-aliases.
  - The warnings rows read the `packs/cloudspec.yaml` manifest from cloudspec-manifest, a direct edge.
- No acceptance line reads state from an unrecorded producer.

Out of my lane: the first extends row compares the aliased graph against `packs/cloudspec.yaml`; whether that comparison can be run is acceptance's lane. I did not re-check the three Outcome bullets the coordinator added to `epic:cloudspec-pack`. Its lines 31-37 are consistent with the three stories.

```findings
[
{"file":".engineering/planning/story/extends-inherits.md","line":0,"category":"design","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"Surfaces lists story:search-score as sharing `design/metaspec.md`, but story:search-score's scope lists no `design/metaspec.md` (it lists manifest.rs, the schema, crates/metaspec/src/main.rs, CHANGELOG.md and others). The ordering edge is real through manifest.rs, the schema and main.rs, so the order stands; only that one shared-file claim is unconfirmed. Either story:search-score adds `design/metaspec.md` to its scope (a manifest change is written there per AGENTS.md:30), or K11 drops it from the list. Not a blocker. — aep plan artifact show story:search-score (scope block)"}
]
```
