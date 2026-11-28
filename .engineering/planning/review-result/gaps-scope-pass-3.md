---
format: aep.planning-md/3
id: review-result:gaps-scope-pass-3
kind: review-result
status: active
title: aep:plan-critic-scope, pass 3, cloudspec gap stories
relations:
- reviews: epic:cloudspec-pack
- reviews: story:cloudspec-warnings
- reviews: story:part-block-aliases
- reviews: story:extends-inherits
revision: 1
---
**approve**

Pass-2's finding is fixed, and the edits introduced nothing new. This is a read-only check of the three drafts (`story:cloudspec-warnings`, `story:part-block-aliases`, `story:extends-inherits`) against `epic:cloudspec-pack` at revision 3.

**Fixed.** The epic's Outcome (`.engineering/planning/epic/cloudspec-pack.md:33-39`) now holds the three promises, and each is claimed by exactly one story:
- "The page of a document shows the warnings of cloudspec's check as findings that do not gate" is claimed by `story:cloudspec-warnings`. Its second finding list at `/warnings` with `gate: false` and its two acceptance rows match the bullet.
- "A part whose block word names an entity inside a cloudspec block (for example `group`, `node_pool`, `endpoints`) is drawn in that block, through an alias map in the manifest" is claimed by `story:part-block-aliases`. The `blocks.aliases` field, `graph::read_with_aliases`, the `alias_` tests and the two cloudspec rows match it.
- "A pack that `extends: cloudspec` inherits its blocks and tool from the base pack's directory given at run time" is claimed by `story:extends-inherits`. It claims it through `--base-pack`, inherited `blocks` and `tool`, and the two cloudspec rows.

**Coverage numbers.** I extracted 8 promises from the epic's Outcome and traced all 8 to an item:
- the `cloudspec-manifest` bullet to `story:cloudspec-manifest`;
- the `in`/`per` bullet to `story:contain-in-and-per`;
- the views bullet to `story:view-selectors`;
- the collapsed-regions bullet to `story:collapsed-regions`;
- the three new bullets to the three stories above;
- "Later: runs" to `story:cloudspec-runs`.

**Reach beyond the parent.**
- Each of the three stories traces to an Outcome sentence, and the graph shows each with a `decomposes: epic:cloudspec-pack` edge.
- The extra scope in `story:extends-inherits` (trust digest of the base tool, base-extends refusal, `fixtures/extends`) follows from "inherits its tool … from the base pack's directory", so it is not reach.
- No other story in the graph claims warnings, aliases or inheritance, so no outcome is claimed twice.
- The edits of K1–K11 narrowed no promise. The "Not in this story" lists record the exclusions (the `resilience` block, `kpis` and `documents` inheritance, merging blocks).

**Out of my lane.** I left these to the other critics:
- whether the pass-2 acceptance bullets (K1, K5–K9) are checkable;
- the ordering edges, including D46, D51 and D52 and the shared-file lists;
- whether `story:search-score` landing after `story:extends-inherits` is the right trade-off.

```findings
[]
```
