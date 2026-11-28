---
format: aep.planning-md/3
id: review-result:groups-acceptance-pass-2
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 2, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
needs-revision

Pass 1: all four findings are resolved. I found two new warnings on story:block-groups, both with a fix I can name.

story:block-groups — item 14 says "5 vessels with two `draught` values" and then "shows the vessel boxes", so it gives no member count per value; a 4-and-1 split gives one box and one plain part (a group of one stays plain, Scope), and nothing says how many boxes to expect — `.engineering/planning/story/block-groups.md:269`
story:block-groups — item 1 refuses "`vessel` twice", but Scope refuses a block twice only "with different `by`", so the acceptance leaves unsaid what the same `by` twice does — `.engineering/planning/story/block-groups.md:222`

**Pass 1 findings**
- **Resolved:** block-groups item 4 (old line 210) now reads "two boxes (`vessel=2m`, `vessel=3m`) and one plain part, drawn as without groups with no `collapsed`". The expected scene is stated one way.
- **Resolved:** the `in_degree` half of old item 8 is now item 9. It has a graph (`v1` to `v4`, `t1` to `t4`, four links, `in_degree: 2`), a before and after (`v1` would be a hub alone, the box is the hub after the roll-up), and its own assertions (`hub: true`, no part boxes, four badges).
- **Resolved:** band-wrap item 8 now has a setup for hubs (`c1`, `h1` to `h5`, `wrap: 2`). It gives rows of 2, 2 and 1 with the titles `hubs (1/3)`, `(2/3)` and `(3/3)`. The `hubs` band title exists at `crates/metaspec-layout/src/rows.rs:208`.
- **Resolved:** band-wrap item 11 now names the document (`marina`, 5 `vessel` and 3 `tug` parts, all in family `craft` per `fixtures/manifests/harbor.yaml`). It also names the unwrapped comparison (the same `ViewSpec` with `wrap` absent, in the same test) and the SVG texts (the `band-title` text of `svg.rs:141-150`). It adds no twin view.

**What I read:** 2 of 2 ids (story:block-groups, story:band-wrap) plus `review-result:groups-acceptance-pass-1`, with `aep plan artifact show` on each. I also ran `aep plan artifact kinds` and `lifecycle story`. In the tree I read `fixtures/manifests/harbor.yaml`, the harbor pack, and `rows.rs`, `svg.rs`, `page.rs` and `lib.rs` where the items name them.

**What I could not establish**
- `marina.yaml` does not exist yet. Whether band-wrap item 11 gets exactly one band of 8 depends on its links and names, which block-groups item 14 does not fix.
- Block-groups item 14 says "the sentence that names groups" and gives no wording. A check can only look for the word, and I did not flag it.
- Band-wrap item 8 holds two named tests (`wrap_ghosts_wrap_too`, `wrap_hubs_wrap_too`). Each can pass or fail alone under `::wrap_`, so I did not flag it.
- Out of my lane (scope critic): block-groups' `GraphView.tsx` counting, `pages.test.tsx` and the scene-schema additions `members` and `by` have no acceptance line.

```findings
[
{"file":".engineering/planning/story/block-groups.md","line":269,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"item 14 names '5 vessels with two draught values' and 'the vessel boxes' but no member count per value, so a 4-and-1 split gives one box and one plain part (a group of one stays plain) and the number of boxes to expect is not stated"},
{"file":".engineering/planning/story/block-groups.md","line":222,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"item 1 refuses 'vessel twice' but Scope (line 156) refuses a block twice only 'with different by', so the acceptance does not say whether the same by twice is refused"}
]
```
