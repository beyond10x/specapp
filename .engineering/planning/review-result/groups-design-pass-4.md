---
format: aep.planning-md/3
id: review-result:groups-design-pass-4
kind: review-result
status: active
title: aep:plan-critic-design, pass 4, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
needs-revision

- `story:block-groups` — its roll-up "generalises `Region` to a box id plus an optional part", but `rows.rs`, `nested.rs`, `hubs_of` and `elided` read a box only as an index into `graph.parts`. A group box has no part, so it would draw as a ghost (band "names no part has", no block colour, dropped by the `!b.ghost` filter) and could never be a hub. The body does not say where the box becomes a part of the folded graph. That decides whether `rows.rs` and `nested.rs` are edited, and band-wrap and badge-width share those files. — `.engineering/planning/story/block-groups.md:125`, `crates/metaspec-layout/src/rows.rs:204` and `crates/metaspec-layout/src/lib.rs:737`
- `story:block-groups` — the no-value box id `<block>=` is also the id of a part whose `by` field holds the empty text. `name_of` returns `Some("")` for that part, so the box has two sources and a wrong title `<block> (no <by>)`, or two boxes with one id. The body should say that empty text counts as no value, or that the two are distinct. — `.engineering/planning/story/block-groups.md:111` and `crates/metaspec-core/src/graph.rs:163`

**Pass 3 status**
- Ghost names vs group ids: resolved. Line 118 now gives the `#2` rule for a ghost name as well as for a part id. A ghost link end is not in the fold's owner map, so rolled ids stay unambiguous.
- The id of the no-value box: resolved. Line 111 gives `<block>=` with the title `<block> (no <by>)`. Acceptance 4 covers the six-part case.
- The new empty-text finding above is a leftover edge of the same box id, not a failure of the pass-3 fix.

**Shape**
- **Cycle:** none. `block-groups` depends on `collapsed-regions` and `extends-inherits`. `band-wrap` depends on `block-groups`. Six later stories depend on `band-wrap`. All `depends_on` edges mean "needs first", and none comes back to the set.
- **Chain:** the chain is serial, but each edge has its reason beside it (shared files, `marina.yaml`, order D61). That is the shared-surface trade-off, not a chain defect.
- **Ownership:** `block-groups` owns `ViewSpec.group`, the group box and `marina.yaml`. `band-wrap` owns `wrap` and the row cut. Neither describes half of the other.
- **Reuse:**
  - `fold`, `draw` and `views_of` share one box-id roll-up.
  - Group views inherit `wrap` through `..spec.clone()`.
  - The reuse stops short at the box node, which is the first finding.
- **Manifest syntax:** `collapse: { blocks?, groups: [{ block, by? }] }` extends `CollapseBlocks`. `wrap` sits flat beside `rows`. Both fit the manifest.

**What I read:** 2 stories, `review-result:groups-design-pass-3`, and the code in `lib.rs`, `rows.rs`, `manifest.rs` and `graph.rs`. I ran `aep plan artifact show`, `relations`, `graph` and `validate` (144 artifacts, valid). I walked 969 graph edges, including edges outside the set, and found no cycle. I traced 7 acceptance reads to producers in the set: `marina` and the goldens for block-groups 13 and 14 and band-wrap 10 and 11, and `ViewSpec` and `views_of` for band-wrap 9. A `depends_on` edge records all 7.

**What I could not establish**
- A `by` value containing `/` or spaces lands in the view id and the page anchor. I could not tell whether part ids already allow the same.
- Out of my lane: the group drill-down view says "the links between them", but `select` also keeps links to ghosts, as region views do today. Acceptance 11 does not say which holds.
- Out of my lane: band-wrap scope says group views inherit `wrap`, but acceptance 9 tests only `<id>/edges` and `<id>/<region>`.
- Out of my lane: scope bullet 2 says "no hub of the view" while the hub bullet judges `in_degree` hubs after the roll-up. This is wording.

```findings
[
{"file":".engineering/planning/story/block-groups.md","line":125,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the roll-up generalises Region to a box id plus an optional part, but rows.rs, nested.rs, hubs_of and elided read a box only as an index into graph.parts, so a group box with no part would draw as a ghost and could not be a hub; the body does not say where the box becomes a part of the folded graph, which decides whether rows.rs and nested.rs (shared with story:band-wrap and story:badge-width) are edited"},
{"file":".engineering/planning/story/block-groups.md","line":111,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the no-value box id <block>= is also the id of a part whose by field holds the empty text (name_of returns Some for an empty string), so one id has two sources and the title '<block> (no <by>)' is wrong for the second; the body should say that empty text counts as no value, or that the two are distinct"}
]
```
