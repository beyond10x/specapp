---
format: aep.planning-md/3
id: review-result:gaps-parallel-pass-2
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, pass 2, cloudspec gap stories
relations:
- reviews: epic:cloudspec-pack
- reviews: story:cloudspec-warnings
- reviews: story:part-block-aliases
- reviews: story:extends-inherits
revision: 1
---
needs-revision

story:part-block-aliases — handing `graph::read` the alias map changes a signature with 10 call sites (cited, `git grep "graph::read"`). Only 3 of them (`document.rs:126`, `graph.rs`, `tests/cases/graph.rs`) are in the body's scope or Surfaces, and the other 7 are not. The unlisted files are core tests `tools.rs:62,142`, `contain_edges.rs:12`, `exclude_edges.rs:144` and `answers.rs:91`, and layout tests `common.rs:121`, `hubs.rs:308` and `contain_edges.rs:14`. Merged hub-badges already edited `hubs.rs` and `layout/tests/cases/contain_edges.rs` (commit 650a085). Later stories that read graphs, such as search-candidate-space on `graph.rs`, will land on the same files. The body should name these files, or say `read` keeps its signature and a second function takes the map. The remedy is the author's choice. — `.engineering/planning/story/part-block-aliases.md:70-71`, scope at `:25`, Surfaces at `:91-94`

Checks that held:
- I ran `aep plan artifact waves --kind story` for every shared file of the three drafts.
- I computed transitive `depends_on` ancestry between each draft and every colliding story (54 pairs).
- Every pair with an active or other in-set story is ordered by an edge:
  - The chain holds: hub-badges, then cloudspec-warnings, then part-block-aliases, then the four view stories, then extends-inherits, then search-score.
  - Every search story reaches extends-inherits through search-score or search-enumerate.
  - cloudspec-runs sits after warnings and search-score.
  - `CHANGELOG.md` is the only unordered shared file among the in-set stories, as D46 states, and I did not count it.
- 12 colliding pairs have no edge:
  - Six are with the implemented kpi-quality-catalogue-shape and kpi-quality-page (acceptance.sh, `main.rs`, `tests/main.rs`). Implemented stories need no edge.
  - The other six are with the draft quality epics (kpi-quality-values; spec-quality-conformance, -drift and -page; the remaining pair is not pinned here). They are outside the set and ordered when accepted, per your brief. Each of those drafts will need an edge to part-block-aliases, extends-inherits or cloudspec-warnings when accepted. The shared files are `manifest.rs`, the schema and `acceptance.sh`.
- D43, D44, D46, D47 and D49 hold in my lane. The edges D46 lists are present in the store and the graph has no cycle.

What I read: 3 drafts, plus the scopes and edges of 40 stories. The commands were `aep plan artifact waves --kind story`, `list`, `show` and `relations`, `git grep` for `graph::read` callers, and `git show --stat 650a085`. I did not run the cloudspec file reads for the alias counts (acceptance lane).

Part 3, surfaces:
- cloudspec-warnings: 2 of 3 scope paths cited, 1 inferred (`CHANGELOG.md`), 0 unplaced.
- part-block-aliases: 7 cited, 4 inferred, 0 unplaced.
- extends-inherits: 6 cited, 8 inferred (including `fixtures/extends`, which does not exist yet), 0 unplaced.
- I traced 7 acceptance reads to a producer in the set, and `depends_on` records 7:
  - aliases (c) reads `packs/cloudspec.yaml` from warnings.
  - extends (b) reads each part's block from aliases, and the finding lists from warnings (transitive).
  - extends (b) reads `packs/cloudspec.yaml` from hub-badges (transitive; hub-badges' only cloudspec change is the `check` finding list, which it does not touch).
  - view-selectors, collapsed-regions, matrix-view and focus-pages read the graph's target block from aliases.
  - cloudspec-runs reads warnings.

Part 4, not established or out of my lane:
- Whether `tool.rs:126` and `pack.rs:716-731` behave as the extends body describes (feasibility, not concurrency).
- Whether the "The same run passes…" bullets in extends-inherits are one checkable acceptance each (acceptance lane).
- Hub-badges is merged but still has status `active`, and the warnings body says "in implementation". I did not weigh this.
- The part-block-aliases body says it shares `document.rs` or `documents.rs` with hub-badges and collapsed-regions. Their scopes list `documents.rs`, a different file from the one aliases lists. It is ordered either way, so I did not count it as a finding.

```findings
[
{"file":".engineering/planning/story/part-block-aliases.md","line":70,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the alias map is passed to `graph::read`, a signature with 10 call sites (cited by git grep); scope and Surfaces name only document.rs:126, graph.rs and tests/cases/graph.rs, and omit core tests tools.rs:62,142, contain_edges.rs:12, exclude_edges.rs:144, answers.rs:91 and layout tests common.rs:121, hubs.rs:308, contain_edges.rs:14, so later stories that touch these files are unnamed collisions; name them, or keep the signature and add a second function. The two remedies are not chosen here"}
]
```
