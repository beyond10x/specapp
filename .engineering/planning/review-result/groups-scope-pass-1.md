---
format: aep.planning-md/3
id: review-result:groups-scope-pass-1
kind: review-result
status: active
title: aep:plan-critic-scope, pass 1, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
needs-revision

Both stories are traceable to the three gaps, and all three gaps are claimed. The problem is that the epic does not mention G6, G7 or G8, so neither story traces to a sentence of the parent.

- **Findings:**
  - story:block-groups — its outcome (one box per block, split by a field, with a count and member names) is in no sentence of the epic, so it reaches beyond the parent. The epic's collapse sentence says only "A region (`in:`) can be drawn collapsed…" (`.engineering/planning/epic/cloudspec-pack.md:31-32`). Parts in no region are not a region, and the epic's "Found" section names no such gap. Fix: add an Outcome bullet to the epic for groups, as was done for the three earlier gap stories (`approval-record:accept-cloudspec-gap-stories`). Cited: `.engineering/planning/story/block-groups.md:65`.
  - story:band-wrap — wrapping a `rows` band is in no sentence of the epic either. The only layout-related promises are the views by blocks, field and link pattern (`cloudspec-pack.md:29-30`) and the collapsed region (`:31-32`). Fix: add an Outcome bullet for the wrap. Cited: `.engineering/planning/story/band-wrap.md:51`.
- **Coverage of the three gaps:**
  - G6 is claimed by block-groups: one box per block, split by `by`, with the count as `<n> parts`.
  - G7 is claimed by block-groups: the same mechanism for the endpoints block, with member names in the box title and in `collapsed.members`.
  - G8 is claimed by band-wrap: `wrap: <n>`, inherited by the generated drill-down views.
  - Nothing is claimed twice. Wrapping appears in block-groups only under "Not in this story", pointing to band-wrap.
- **Nothing drafted outside the gaps:** the supporting edits (scene schema, `GraphView` counting text, `design/metaspec.md`, the harbor fixtures) all serve those gaps. No other story in the store claims groups or wrap.
- **Graph:** both stories carry a `decomposes` edge to the epic, and band-wrap `depends_on` block-groups.

What I read: 4 artifacts (the epic, story:block-groups, story:band-wrap and `approval-record:accept-cloudspec-gap-stories`). I ran `aep plan artifact show` on the first three, plus `aep plan artifact graph` and `aep plan artifact list`. I also ran `git grep` on `.engineering/planning/story` for wrap, group and G6 to G8, and on `.engineering` for gap numbers. Of 8 promises extracted from the epic's Outcome, 8 trace to an item. Neither draft traces to a promise: 0 of 2.

What I could not establish: how the consumer's gap list is held. It is cited only inside the stories and not in the store, so I could not check the wording of G6 to G8 against its source. Out of my lane: whether the acceptances are checkable, whether the shared-file edges hold (`manifest.rs`, `lib.rs`, the harbor goldens), and whether the roll-up generalisation in `fold` is sound.

```findings
[
{"file":".engineering/planning/story/block-groups.md","line":65,"category":"scope","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the outcome (one box per block, split by a field, with a count and member names, gaps G6 and G7) is not in any sentence of epic:cloudspec-pack, whose only collapse promise is 'A region (`in:`) can be drawn collapsed' (.engineering/planning/epic/cloudspec-pack.md:31); add an Outcome bullet to the epic for groups of parts outside a region, as was done for the earlier gap stories"},
{"file":".engineering/planning/story/band-wrap.md","line":51,"category":"scope","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the outcome (wrap a `rows` band at a set count, gap G8) is not in any sentence of epic:cloudspec-pack, whose view promises are by blocks, field and link pattern (.engineering/planning/epic/cloudspec-pack.md:29-30) and the collapsed region (:31-32); add an Outcome bullet to the epic for wrapping a band"}
]
```
