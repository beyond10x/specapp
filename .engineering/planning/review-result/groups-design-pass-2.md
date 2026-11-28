---
format: aep.planning-md/3
id: review-result:groups-design-pass-2
kind: review-result
status: active
title: aep:plan-critic-design, pass 2, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
needs-revision

Two of the three pass-1 findings are resolved. The third is resolved in substance. I found one new finding on `story:block-groups`.

**Pass-1 findings (`review-result:groups-design-pass-1`)**
- **`ViewSpec.group` membership: resolved.** `block-groups.md:125-131` says `ViewSpec.group` carries the member part ids, decided in `views_of` from the document's graph. It also says `select` keeps exactly those parts and the links between them. Acceptance 11 checks it.
- **Hub order: resolved.** The body now states the order (`hubs.parts` and `hubs.blocks` first, then the roll-up, then `in_degree` judged on the rolled-up selection). This matches the code, where `hubs_of` runs after `fold` (`lib.rs:474-486`). Acceptance 3 names the kind of hub it uses. Acceptance 9 covers the `in_degree` case.
- **`marina.yaml` ownership: resolved.**
  - `block-groups.md` owns the new `marina.yaml`.
  - `band-wrap.md:108-110` says it adds no part and no document and does not edit it. It adds only the `wrapped` view to `harbor.yaml`.
  - The dead "or a document of this story's own" text is gone.
  - Its acceptance 11 claim of one band of 8 craft boxes holds: `harbor.yaml:69-71` puts `vessel` and `tug` in family `craft`, and `rows.rs:212` keys a part by its family under `rows: family`.

**Shape**
- **Reuse:** a group is a second kind of box on the collapsed-regions mechanism. `Region` becomes a box id plus an optional part, one `fold` serves both, and the story says "A second roll-up is refused". The rows band and `Band` (in `rows.rs`) stay untouched by block-groups, and band-wrap reaches group views through `..spec.clone()`. I see no second mechanism.
- **Ownership:** block-groups owns `ViewSpec.group`, the box and the group view. band-wrap owns `wrap` and the row cut. Neither describes half of the other. The two share files, and `band-wrap depends_on block-groups` has its reason written beside it. That edge is the trade-off the rubric names, not a chain defect.
- **Syntax:** `collapse: { blocks?, groups: [{ block, by? }] }` extends the existing object form. `wrap: <n>` sits flat beside `rows`. Both are consistent.

**New finding (reuse gap)**
- `story:block-groups` takes the view id `<view>/<box id>` from the region views but not their collision rule. Regions already leave a part named `edges` uncollapsed (`REGION_EDGES`, `lib.rs:342-352`) and `metaspec pack check` names it as `view_region_edges` (`pack.rs:672-722`). A group of a block named `edges` without `by` would take the id of `<view>/edges`. The story names only the `#2` rule for a group id equal to a part id (`block-groups.md:115`). It should say what a group view id is when the box id is `edges`. Either the group stays plain and `pack check` names it, or the story reuses `REGION_EDGES` and `view_region_edges`.

**Edges and reads**
- I walked 19 edges of the two stories' neighbourhood in `aep plan artifact graph`, including edges outside the set.
- I found no cycle. Every edge in the chain is `depends_on`, meaning "needs first".
- I traced 6 acceptance reads to producers in the set: block-groups 13 and 14 and band-wrap 11 read `marina` or the harbor goldens, and block-groups 11 and 12 and band-wrap 9 read collapsed-regions or block-groups code. An edge records every one.
- `aep plan artifact validate` reports valid with 136 artifacts.

**What I could not establish**
- Unease, not a finding: a `by` value is any scalar, so a group id like `vessel=eu west/1` lands in a view id and an HTML anchor (`page.rs:110-120`). The story does not state any character rule. Part ids already pass through the same anchors, so I could not tell whether this is new.
- Out of my lane (parallel-safety): none new. Block-groups and band-wrap order against view-selectors, matrix-view and focus-pages through recorded edges.

```findings
[
{"file":".engineering/planning/story/block-groups.md","line":125,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the group view id <view>/<box id> reuses the region view namespace but not its REGION_EDGES rule, so a group of a block named edges without by takes the id of <view>/edges; the body should say what happens then (stay plain and name it in pack check, as view_region_edges does for regions)"}
]
```
