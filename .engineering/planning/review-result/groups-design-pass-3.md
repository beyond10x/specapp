---
format: aep.planning-md/3
id: review-result:groups-design-pass-3
kind: review-result
status: active
title: aep:plan-critic-design, pass 3, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
needs-revision

Pass 2's `edges` finding is resolved. The body now has a paragraph on a group whose box id is `edges` (`.engineering/planning/story/block-groups.md`, the "A group whose box id is `edges`" bullet). It reuses `REGION_EDGES` and `view_region_edges`. The members stay plain parts and `pack check` names the view. The `by` case is covered: the id is `edges=<value>`, which clashes with nothing. Acceptance 15 tests both sides. I checked the claim against the code: `REGION_EDGES` is at `lib.rs:342-352` and `manifest.rs:198`, and `view_region_edges` is at `pack.rs:672-722`. The pass-1 findings stay resolved.

I found two new warnings on `story:block-groups`, both about the new box id namespace:

- `story:block-groups` — a group id `<block>` is checked only against part ids (the `#2` rule), but a ghost box (a name a link uses that no part has) takes its box id from that name (`ghost_card` / `Selection.ghosts`, `lib.rs:416-452`). A link to a name equal to a block word therefore gives the scene two boxes with one id, and the rolled links `vessel->…` become ambiguous. The body should say the `#2` rule also covers ghost names. — `.engineering/planning/story/block-groups.md:117`
- `story:block-groups` — a part without a value of `by` goes "in the box of its block with no value, titled so" (line 112), but no id is given for that box. The id is the drill-down view id `<view>/<box id>`, and the id forms `<block>` and `<block>=<value>` have no slot for it. Acceptance 4 shows only the one-part case, which stays plain. The body should name the id of the no-value box. — `.engineering/planning/story/block-groups.md:112`

Shape of the set:
- **Cycles and chain:** there is no cycle. I walked every `depends_on` edge touching the two stories, and all of them mean "needs first". The chain `extends-inherits` → `block-groups` → `band-wrap` → six later stories is serial. Each edge has its reason written beside it: shared files, `marina.yaml`, and D61's order (consumer gaps first). That is the shared-surface trade-off, not a chain defect.
- **Ownership:** `block-groups` owns `ViewSpec.group`, the group box, the group view and `marina.yaml`. `band-wrap` owns `wrap` and the row cut. Neither describes half of the other. `band-wrap` acceptance 11 reads the `marina` document that `block-groups` creates, and `depends_on` records it.
- **Reuse:**
  - `fold`, `draw` and `views_of` take one box-id mechanism, and the body says "A second roll-up is refused".
  - `ViewSpec.group` sits beside `region` as a second optional field. It is not a second mechanism, and I do not count it as a finding.
  - Group views are copied by `..spec.clone()`, so `wrap` reaches them with no extra code.
- **Syntax:** `collapse: { blocks?, groups: [{ block, by? }] }` extends the existing object form (`CollapseBlocks`, `manifest.rs:210`). `wrap: <n>` sits flat beside `rows`. Both are consistent with the manifest.
- **Slice:** `block-groups` crosses core, layout, scene, docs, page and UI, but it can be shown end to end through the harbor `fleets` page. It is a vertical slice.

What I read: 2 stories, `review-result:groups-design-pass-2`, and the pass 2 findings. I ran `aep plan artifact show`, `relations`, `graph` and `validate` (141 artifacts, valid). I checked the story claims in the worktree code: `lib.rs`, `pack.rs`, `manifest.rs`, `rows.rs`, `nested.rs`, `harbor.yaml` and the harbor documents. I walked about 50 edges in the graph, including edges outside the set, and found no cycle. I traced 7 acceptance reads to producers in the set (`marina` and the harbor goldens for `block-groups` 13 and 14 and `band-wrap` 10 and 11; `ViewSpec` and `views_of` for `band-wrap` 9), and a `depends_on` edge records all 7.

What I could not establish:
- A `by` value can hold characters such as `/` or spaces, and it lands in the view id and the page anchor. The pass-2 unease still stands, and I could not tell whether part ids already allow the same.
- The `nested` layout and groups: the story is written for `rows` (the `per` exclusion cites `rows.rs`), and the acceptance names no layout for `fleets`. I think that is acceptance or scope, not design, so it is out of my lane.
- `band-wrap` scope still says "once story:block-groups has landed" although the `depends_on` edge makes that unconditional. That is wording and not mine.

```findings
[
{"file":".engineering/planning/story/block-groups.md","line":117,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"a group id is checked only against part ids (the #2 rule), but a ghost box takes its id from the name a link uses that no part has, so a name equal to a block word gives two boxes of one id and an ambiguous rolled link; the body should say what a group id equal to a ghost name becomes"},
{"file":".engineering/planning/story/block-groups.md","line":112,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the parts of a block without a value of by go in a box of their block 'titled so', but the body gives that box no id, and its id is the drill-down view id <view>/<box id>; the id forms <block> and <block>=<value> leave it undefined"}
]
```
