---
format: aep.planning-md/3
id: review-result:gaps-scope-pass-2
kind: review-result
status: active
title: aep:plan-critic-scope, pass 2, cloudspec gap stories
relations:
- reviews: epic:cloudspec-pack
- reviews: story:cloudspec-warnings
- reviews: story:part-block-aliases
- reviews: story:extends-inherits
revision: 1
---
needs-revision

All three stories cover what the brief and D43-D50 ask, and I found nothing missing in them or promised twice. One thing is wrong, and it sits in the parent epic.

- epic:cloudspec-pack — the epic's Outcome has no sentence that warnings, block aliases or an inheriting `extends` could trace to, yet all three stories decompose it. The only nearby lines are Context ("`extends: cloudspec` from its own repository"), which is a fact about the consumer, and Found ("drawn as blocks the pack does not have"), which describes today's behaviour. The authority for the three stories is D41/D42 and the coordinator's decisions, which the epic does not record. A reader checking the epic against its stories cannot tell these three apart from unrequested work. The fix is to add Outcome bullets to the epic for warnings of the check on the page, aliases for part block words, and an `extends` manifest that inherits `blocks` and `tool`. Citation: `.engineering/planning/epic/cloudspec-pack.md:24-34`, quoting "`packs/cloudspec.yaml` at levels L0, L2 and L3 (blocks, documents, check)", the nearest promise. It covers the warnings story only partly and the other two not at all.

What I read: 1 parent epic, 3 stories, and cloudspec-runs, hub-badges, view-selectors and search-score. I ran `aep plan artifact show` on each, `graph`, `relations`, and `git diff` of the coordinator's edge edits. I also read `graph.rs`, `document.rs`, `pack.rs`, `design/metaspec.md` and `packs/cloudspec.yaml`, and counted block words in cloudspec's `origin/main` files.

- **Promises:** the epic makes 5 Outcome promises. I traced all 5 to items in the graph, but only 1 reaches the set, and only in part: the L3 check bullet to warnings. The other 4 sit in stories outside the set (cloudspec-manifest, contain-in-and-per, the view stories, collapsed-regions, cloudspec-runs).
- **Set items:** 1 of 3 traces to an epic sentence, in part. The other 2 trace only to the decisions.
- **Decisions D43-D50:** each is present in the bodies and none is claimed twice. The counts cited for cloudspec hold: 6 `group` parts in `groups.yaml`, 3 `node_pool`, 1 `endpoints`, and 1 each of `payment_api` and `mail_relay`.
- **Edges:** the D46 edges are in place. `view-selectors`, `collapsed-regions`, `matrix-view`, `focus-pages` and `extends-inherits` depend on `part-block-aliases`. `search-score` depends on `extends-inherits`. `cloudspec-runs` depends on `cloudspec-warnings`.

What I could not establish: nothing in my lane. Not run: `aep plan artifact validate`, since the rubric says to leave the validator's output to the caller.

Out of my lane, for the other critics:
- **Alias-map callers:** `graph::read` has 8 call sites. The aliases scope omits the six test callers: `metaspec-layout/tests/cases/{hubs,contain_edges,common}.rs` and `metaspec-core/tests/cases/{answers,exclude_edges,tools}.rs`. `hubs.rs` is also in hub-badges' scope. The scope critic for parallel-safety should check this, along with the signature change at `crates/metaspec-docs/src/document.rs:126`.
- **Edges with no stated reason:** neither `story:search-score` nor `story:extends-inherits` (Surfaces) says why `search-score` depends on it. `story:cloudspec-runs` ("Waits on") does not name `story:cloudspec-warnings` either.
- **Scope claims with no acceptance bullet:**
  - In `extends-inherits`: "A refusal names which pack the path was in", and "the alias target check runs against the base's blocks".
  - In `part-block-aliases`: "a word with neither alias nor block keeps its word in the graph".

```findings
[
{"file":".engineering/planning/epic/cloudspec-pack.md","line":24,"category":"scope","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the epic's Outcome (\"`packs/cloudspec.yaml` at levels L0, L2 and L3 (blocks, documents, check)\") names no warnings on the page, no block aliases and no inheriting `extends`, so story:cloudspec-warnings, story:part-block-aliases and story:extends-inherits (the last two most clearly) claim outcomes the parent never promised; add the three outcomes to the epic or record them as the coordinator's extension"}
]
```
