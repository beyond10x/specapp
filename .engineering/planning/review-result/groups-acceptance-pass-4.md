---
format: aep.planning-md/3
id: review-result:groups-acceptance-pass-4
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 4, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
needs-revision

Both pass 3 findings are resolved, and items 15 and 16 each have one new defect. Item 4 and the rest of both stories are checkable.

**Pass 3 findings**
- **Resolved:** the `view_region_edges` warning now has its own item 16 with its own test and command. The test is `group_named_edges_is_named`, run with `cargo nextest run -p metaspec-core -E 'test(/::group_named_edges_is_named/)'`. The analogue it cites exists at `crates/metaspec-core/tests/cases/fixtures.rs:244`.
- **Resolved:** item 15 now states the graph for the `by` case. Four `edges` parts with `zone` `a`, `a`, `b`, `b` give exactly `edges=a` and `edges=b`, 2 parts each.
- **Item 4:** the 5-part and 6-part states are stated. The `vessel=` box and the title beginning `vessel (no draught)` agree with the scope. It is checkable.

**Findings**
- story:block-groups — item 15's first graph names no hub part and no link, but `views_of` only emits `<view>/edges` when the view elides a link (`crates/metaspec-layout/src/lib.rs:214-222`), so "`<view>/edges` only as the view of the hubs" holds trivially or not depending on a state the item does not give. Name the hub (a part in `hubs.parts`, or an `in_degree`) and the link from an `edges` part that makes the edges view exist. — `.engineering/planning/story/block-groups.md:292`
- story:block-groups — item 16's second half, "a view with `groups: [{ block: edges, by: zone }]` has none", does not say none of what. It also does not say whether the added document's parts carry `zone` or whether the pack has a block `edges`. `view_unknown_group_field` and `view_unknown_block` can fire, depending on how the fixture is built. Pin the expected codes in both halves, as the analogue does with `codes(&report.warnings) == [...]` at `fixtures.rs:287` and `:301`. — `.engineering/planning/story/block-groups.md:300`

**What I read:** 3 ids, all in full: story:block-groups, story:band-wrap and review-result:groups-acceptance-pass-3, with `aep plan artifact show <id>`. In the tree I read `lib.rs:205-262`, `pack.rs:336-362` and `:665-730`, `fixtures.rs:225-310`, `svg.rs:135-155`, `rows.rs:205-215` and `:625-645`, `harbor.yaml`, the four harbor documents, `fixtures/expected/`, and the `#view-` links in `cove.html` and `page.rs`.

**What I could not establish**
- band-wrap items 1 to 11 are checkable: each has a state and an observable result, and the filters `::wrap_`, the `-p metaspec` form and the `svg.rs:141-150` `band-title` text all resolve. `marina.yaml` does not exist yet, so I cannot confirm the single band of 8 boxes in item 11. Its links do not change the 3, 3, 2 split, and I did not flag this.
- Out of my lane (scope critic): band-wrap's scope says `wrap` reaches `<id>/<group>` views, but item 9 tests only `<id>/edges` and `<id>/<region>`. The new `members` and `by` scene fields and the `GraphView.tsx` counting have no acceptance line, which is carried over from passes 2 and 3.
- band-wrap item 8 holds two tests, `wrap_ghosts_wrap_too` and `wrap_hubs_wrap_too`. Each is separately named and checkable under `::wrap_`, so I did not flag it, as in pass 3.

```findings
[
{"file":".engineering/planning/story/block-groups.md","line":292,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"item 15's first graph names no hub part and no link, but views_of emits <view>/edges only when the view elides a link (lib.rs:214-222), so the check that <view>/edges is only the view of the hubs turns on a state the item does not give"},
{"file":".engineering/planning/story/block-groups.md","line":300,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"item 16 says a view with groups by zone 'has none' without saying none of what, and does not say whether the added document's parts carry zone or the pack has a block edges, so view_unknown_group_field and view_unknown_block can fire; pin the expected warning codes in both halves as fixtures.rs:287 and :301 do"}
]
```
