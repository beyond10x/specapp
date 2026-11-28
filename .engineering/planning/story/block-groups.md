---
format: aep.planning-md/3
id: story:block-groups
kind: story
status: active
title: Parts outside a region collapse into one box per block, split by a field
summary: 'collapse gains groups: parts in no region, by block and optionally one field, become one box with a count and the member names; links roll up and drill down as for collapsed regions.'
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:collapsed-regions
- depends_on: story:extends-inherits
scope:
- confidence: inferred
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: cited
  path: crates/metaspec-core/src/pack.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/fixtures.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/manifest.rs
- confidence: cited
  path: crates/metaspec-docs/src/documents.rs
- confidence: cited
  path: crates/metaspec-layout/src/lib.rs
- confidence: cited
  path: crates/metaspec-layout/src/scene.rs
- confidence: cited
  path: crates/metaspec-layout/src/svg.rs
- confidence: inferred
  path: crates/metaspec-layout/tests/cases/groups.rs
- confidence: cited
  path: crates/metaspec/src/page.rs
- confidence: inferred
  path: crates/metaspec/tests/main.rs
- confidence: inferred
  path: design/metaspec.md
- confidence: inferred
  path: fixtures/expected/harbor/pages/plan
- confidence: inferred
  path: fixtures/expected/harbor/scenes/plan
- confidence: inferred
  path: fixtures/expected/site/harbor
- confidence: inferred
  path: fixtures/manifests/harbor.yaml
- confidence: inferred
  path: fixtures/packs/harbor/harbors/marina.yaml
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: schema/metaspec-scene-1.schema.json
- confidence: cited
  path: spec/domains/documents.yaml
- confidence: cited
  path: web/ui/src/GraphView.tsx
- confidence: inferred
  path: web/ui/test/pages.test.tsx
revision: 11
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T20:23:26Z", actor: "human:operator", revision: 10}
- {from: "proposed", to: "active", at: "2026-11-28T20:23:26Z", actor: "human:operator", revision: 11}
---
# Parts outside a region collapse into one box per block

## Context

A consumer that extends cloudspec draws architectures whose parts are not in a region: 25 parts of
one block (a store block, aliased from a pack word) and 9 parts of another (endpoints) are drawn
one per box. After story:collapsed-regions its dev overview measured 16077 to 6818 px, 65 boxes,
75 edges and 40 badges; its own interim page draws the same at 1427 pt with 21 nodes. Gaps G6 and
G7 of the consumer's list are what remains of that width and box count.

`collapse` reaches only regions: `regions()` takes the shown parts another shown part sits `in`
(`crates/metaspec-layout/src/lib.rs:344-385`, holders from `in_parents` at `:310-322`), so a part
in no region is never one box with its peers. `Collapse` has three forms, all of them regions
(`crates/metaspec-core/src/manifest.rs:186-193`, `CollapseBlocks` at `:210-212`, the `View` field at
`:175-177`).

Do part fields reach the graph? Yes, today. A part's every field but `block` is read into
`Part.fields` (`crates/metaspec-core/src/graph.rs:47`), a marked place `{value, vary}` as its value
(`graph.rs:207-214`, stored at `:230`), and the block is read after `blocks.aliases`
(`graph.rs:215-216`). The layout already reads `part.fields` (`crates/metaspec-layout/src/lib.rs:1110`).
Nothing is needed in `metaspec-core`'s graph module for a split by a field. Only a scalar field
(text, number, boolean) names a group, as `name_of` reads it (`graph.rs:163`); a part without
the field, with a list or mapping in it, or with an empty text in it, has no value.

What the roll-up and the drill-down need is the part of a box: today a collapsed box IS a part of
the graph (`Region.part`, `lib.rs:300-308`), and `fold` keeps `owner` by part id
(`lib.rs:503-516`). A group has no part in the document.

## Domain relations

- Group: new noun of a view's `collapse`, one per (block, value of `by`) of the parts the view
  groups; it is drawn as one box. Group to its members: one to many, the members are parts of the
  graph and exist without the group; a part is a member of at most one group of one view.
  `spec/domains/documents.yaml` already declares the view's nouns (`metaspec.documents.Collapse`,
  `:75-94`, with its `all`, parts and `blocks` forms; `metaspec.documents.View`, `:189`): the group
  is declared there as a new entity beside `Collapse`, with its `block` and `by`, and the
  relation `Collapse` to `Group` (one to many, owned), before the code; `ess specify validate`
  on that file is the first step of the story.

## Scope

- Manifest syntax, an extension of `collapse`'s object form:
  `collapse: { blocks: [<block>], groups: [{ block: <block>, by: <field> }, { block: <block> }] }`.
  `blocks` becomes optional in the object form (today it is required, `manifest.rs:210-212`, the
  schema's `required` at `schema/metaspec-pack-1.schema.json:299`); one of `blocks` and `groups` is
  required. `groups` names one entry or more; `block` is a block of the pack (after aliases, as
  `blocks` names it); `by` is optional and names one part field.
- A group is made from the shown parts that are in no region: a part of the group's block that sits
  `in` no shown part, that no shown part sits `in`, that has no shown `per` host (`rows` draws such
  a part inside its host, `crates/metaspec-layout/src/rows.rs:108-120`) and that is no part the view makes a hub by `hubs.parts` or
  `hubs.blocks` (a hub by `in_degree` is judged after the roll-up, see the hub bullet below). Such
  parts of one block become one box per value of `by` (one box per block without `by`). A part
  without a value of `by` (the field absent, a list or mapping, or an empty text) is in the box of
  its block with no value: id `<block>=` (empty value), title `<block> (no <by>)`; with one such
  part it stays plain, as every group of one.
- A box is one group of the scene: id `<block>` without `by`, `<block>=<value>` with it (a block
  word as `Part.block` holds it, after aliases); its label is that id, its text `<n> parts` (and
  `<m> findings inside` when it has them), colour and icon those of the block. Its title (hover,
  and the SVG `<title>`) lists the members' ids, one per line, in the document's order, and names
  the drill-down view. The id of a group that equals a part's id, or the name of a ghost box (a
  name a link gives that no part has), takes `#2` after it, as a repeated link id does
  (`graph.rs:62`, `Link.id`).
- Links roll up as for a collapsed region: a link from or to a member is drawn from or to the box,
  links with the same two drawn ends and one name are one edge with `count` and `links`; a link
  between two members of one box is drawn only in the group's view; a link to a hub is a badge with
  `count` and `links`. This is the one mechanism of `fold`: a region and a group are both a box
  with an id, a set of members and the links of its border. A second roll-up is refused.
- The box is a part after the fold (D64). `Region` (a `part: usize` into the graph,
  `lib.rs:300-308`) gains the box id, the group's block and the member ids, and its `part`
  becomes optional (none for a group, as `select` has no part to name). `fold` returns the folded
  graph in which each group box is a synthetic `Part` appended to the graph's parts, with the box
  id, the group's block (so the block's colour and band come from it), the title, and no fields;
  the folded selection lists its index in `parts`. `draw`, `views_of` and `fold` read the box id
  (`lib.rs:233-251`, `:503-516`, `:599-614`). Everything after the fold reads the box as a part:
  `rows.rs` and `nested.rs` (their `Node` and card code, `rows.rs:95-200`), `hubs_of` and
  `elided` (`lib.rs:737-795`) are read, not edited, by this story. A collapsed region already has
  its part in the graph, so nothing changes for regions.
- Drill-down: each group of a document adds the view `<view>/<box id>` after the view, its
  `<view>/edges` and its region views, the view without `collapse` and `hubs`, drawing the members
  and nothing else. The drill-down spec is made with `collapse` and `hubs` stripped
  (`lib.rs:233-249`), so membership cannot be recomputed in it: `ViewSpec` gains `group`
  beside `region` (`lib.rs:168-188`), and `group` carries the member part ids (decided when
  `views_of` makes the view, from the document's graph). `select` keeps exactly those parts and
  the links between them and takes no group of its own (`lib.rs:401-407`), as a region view keeps
  the region and the parts inside it.
- Findings: a finding on a member, or on a link between two members, is counted on the box and
  marked in the group's view, as for a region (`lib.rs:599-614`).
- `metaspec-scene/1`: the box's `collapsed` holds `parts`, `view` and `findings` as for a region,
  and gains `members` (ids) and `by` (`{ field, value }`, absent without `by`) for a group only
  (`crates/metaspec-layout/src/scene.rs:125-136`, schema `metaspec-scene-1.schema.json:63-73`).
  A scene of a view without groups keeps its bytes.
- Interaction with `collapse: all` and the lists: `all`, a list of ids and `blocks` collapse
  regions as today and cannot be written beside `groups` (they are other forms of `collapse`;
  `blocks` is the one form that can: `{ blocks: [<region block>], groups: [...] }`). A block named
  by both `blocks` and `groups` has its regions collapsed and its remaining parts grouped.
- Interaction with hubs, in this order (`hubs_of` runs after `fold`, on the rolled-up selection,
  `lib.rs:474-486`, `:737-760`): (1) a part that `hubs.parts` or `hubs.blocks` names is never a
  member; it stays a hub, and `metaspec pack check` names a block in both `hubs.blocks` and
  `groups` (`view_group_hub`); (2) the groups are made from the other parts and the links roll up;
  (3) an `in_degree` hub is judged after the roll-up, on the rolled-up selection, so a part that
  would be a hub only by `in_degree` joins its group, and a group box is a hub when its rolled-up
  incoming links reach `in_degree`, as a collapsed region is (`views_of`, `lib.rs:219-224`).
- A group of one part stays a plain part, drawn as today, with no drill-down view: a box that
  stands for one part says nothing a part does not. A region of one part still collapses
  (`region_lines`, `lib.rs:558-561`), unchanged.
- A group whose box id is `edges` (a block named `edges`, with no `by`) is never made: its view
  would take the id of the view `<view>/edges` of a view with hubs. Its members stay plain parts,
  as a region named `edges` stays expanded: the story reuses `REGION_EDGES`
  (`crates/metaspec-layout/src/lib.rs:342-352`, `crates/metaspec-core/src/manifest.rs`) and
  `metaspec pack check` names it with the same `view_region_edges`
  (`crates/metaspec-core/src/pack.rs:672-722`), with the documents where the view would group it.
  With `by`, the id is `edges=<value>` and clashes with nothing.
- `metaspec pack check`: a block `groups` names that the pack does not have is `view_unknown_block`
  (`crates/metaspec-core/src/pack.rs:342-361`); an alias word is
  `view_alias_block`; a `by` that no part of the block has in any document of the class is
  `view_unknown_group_field`. The same block twice in `groups` is refused by the parser
  whatever its `by` (the same or different, with or without): a part would be in two boxes, or
  one entry would be dead.
- The static page and the docs site: the sentence of a page with collapsed boxes
  (`crates/metaspec-docs/src/documents.rs:273-279`, `crates/metaspec/src/page.rs:150-154`) says
  groups in their own words, only for a scene that has one; `GraphView` counts and links them the
  same way (`web/ui/src/GraphView.tsx:52`, `:67`, `:135`).

## Surfaces

Files: `crates/metaspec-core/src/manifest.rs`, `crates/metaspec-core/src/pack.rs`,
`crates/metaspec-core/tests/cases/manifest.rs`, `crates/metaspec-core/tests/cases/fixtures.rs`,
`crates/metaspec-layout/src/lib.rs`, `crates/metaspec-layout/src/scene.rs`,
`crates/metaspec-layout/src/svg.rs`, `crates/metaspec-layout/tests/cases/groups.rs` (new),
`crates/metaspec-docs/src/documents.rs`, `crates/metaspec/src/page.rs`,
`crates/metaspec/tests/main.rs`, `web/ui/src/GraphView.tsx`, `web/ui/test/pages.test.tsx`,
`schema/metaspec-pack-1.schema.json`, `schema/metaspec-scene-1.schema.json`,
`spec/domains/documents.yaml`, `design/metaspec.md`, `CHANGELOG.md`,
`fixtures/manifests/harbor.yaml`, `fixtures/packs/harbor/harbors/marina.yaml` (new),
`fixtures/expected/` (the harbor pages and site, regenerated).

Shared with other planned stories, checked with `aep plan artifact show`:

Order, D61 (the consumer's gaps first): story:extends-inherits, then this story, then
story:band-wrap, then story:view-selectors, story:matrix-view, story:focus-pages,
story:search-score, story:badge-width and story:placement-overlay: each of the last six
`depends_on` story:band-wrap, as story:band-wrap words it, and none depends on another of the
six.

- story:extends-inherits (active): `manifest.rs`, `pack.rs`, the pack schema, `design/metaspec.md`,
  `CHANGELOG.md` and its test files. This story `depends_on` it, so it lands after it.
- story:band-wrap: `manifest.rs`, `pack.rs`, `lib.rs` (`ViewSpec`, `views_of`), the pack schema,
  `spec/domains/documents.yaml`, `harbor.yaml`, `marina.yaml` and the harbor goldens. It
  `depends_on` this story.
- story:view-selectors (`select` and `View`), story:matrix-view, story:focus-pages (`views_of`,
  `page.rs`): `manifest.rs`, `lib.rs`, the pack schema, `spec/domains/documents.yaml`,
  `harbor.yaml` and the harbor goldens; matrix-view also `scene.rs` and the scene schema,
  focus-pages also `page.rs` and `documents.rs`. Each `depends_on` story:band-wrap, so after this
  story.
- story:search-score: `manifest.rs`, the pack schema, `harbor.yaml`, `design/metaspec.md` and
  `CHANGELOG.md`. It `depends_on` story:band-wrap, so after this story.
- story:badge-width: `rows.rs`, `nested.rs` and `lib.rs` (badges). It `depends_on`
  story:band-wrap, so after this story.
- story:placement-overlay: `manifest.rs`, the pack schema and `documents.rs` (a manifest mapping
  and a view). It `depends_on` story:band-wrap, so after this story.
- story:graph-viewport: `web/ui/src/GraphView.tsx`, where this story edits the counting text
  (`:52`, `:67`, `:135`). It is ordered after this story through story:focus-pages, which
  `depends_on` story:band-wrap, which `depends_on` this story.
- story:nested-readable: `nested.rs` only; this story does not edit it. Not shared.

## Not in this story

- Parts inside a region that is not collapsed, and parts inside a collapsed region (hidden in its
  box already).
- A split by two fields, or by a computed value; one field, or none.
- A group whose members come from several blocks (`block` is one block).
- Wrapping a band (story:band-wrap).
- A name or title a pack gives a group box; the box is named by block and value.

## Acceptance

Tests live in `crates/metaspec-layout/tests/cases/groups.rs` and the cases of `metaspec-core` and
`metaspec` named below. Words are invented (harbor pack: blocks `vessel`, `tug`, `berth`, `quay`;
field `draught`). No word of a real pack is in a test.

1. `group_manifest_takes_blocks_and_groups`:
   `cargo nextest run -p metaspec-core -E 'test(/::group_manifest/)'`. `collapse: { groups: [{ block:
   vessel, by: draught }, { block: tug }] }` and `{ blocks: [quay], groups: [...] }` parse;
   `{ groups: [] }`, `{}`, a group without `block`, and the same block twice in `groups` whatever
   its `by` (`[{ block: vessel }, { block: vessel }]`, `[{ block: vessel, by: draught }, { block:
   vessel, by: draught }]` and `[{ block: vessel, by: draught }, { block: vessel }]`) are refused.
2. `group_unknown_block_and_field_are_named`:
   `cargo nextest run -p metaspec-core -E 'test(/::group_unknown/)'`. A group of a block the pack
   does not have is `view_unknown_block`, an alias word `view_alias_block`, a `by` no part of the
   block has `view_unknown_group_field`, a block in `hubs.blocks` and `groups` `view_group_hub`.
3. `group_parts_outside_a_region_make_one_box`:
   `cargo nextest run -p metaspec-layout -E 'test(/::group_/)'`. A graph of 5 `vessel` parts and one
   `quay` part `q`: `w1` and `w2` are `in: q`, `w3` is named in `hubs.parts`, `w4` and `w5` sit in
   no region. With `groups: [{ block: vessel }]` the view draws one box `vessel` with
   `collapsed.parts` 2 (`w4`, `w5`); `q`, `w1`, `w2` and `w3` keep their own boxes, and `w3` is
   a hub drawn apart. The hub here is one that `hubs.parts` names, not one by `in_degree`.
4. `group_split_by_field_makes_one_box_per_value`: 5 `vessel` parts with `draught` 2m, 2m, 3m, 3m
   and none, `groups: [{ block: vessel, by: draught }]`: the scene has two boxes, `vessel=2m` (2
   parts) and `vessel=3m` (2 parts), and one plain part for the part without `draught`, drawn as
   without groups with no `collapsed`: two boxes and one plain part. With two parts without
   `draught` in place of one (6 parts), the scene has a third box with id `vessel=` (empty value),
   `collapsed.parts` 2 and a title that begins `vessel (no draught)`; a part with `draught: ""`
   (empty text) counts as without a value and joins that box.
5. `group_box_lists_its_members_and_its_count`: the box's `collapsed.members` are the member ids in
   the document's order, its title holds one id per line and names the view, and its text says
   `<n> parts`.
6. `group_of_one_part_stays_a_plain_part`: a block with one part outside a region has no box with
   `collapsed`, no view `<view>/<id>`, and its scene equals the scene of a view without groups.
7. `group_links_roll_up_by_far_end_and_name`: two links from two members to one far part with one
   name are one edge `<box>-><far>:<name>` with `count` 2 and the two ids in `links`; a link between
   two members is in no edge of the view and is in the group's view.
8. `group_rolled_link_to_a_hub_is_a_badge_with_its_count`: two `vessel` parts `v1` and `v2` outside
   a region, a `dock` part named in `hubs.parts` (no `in_degree`), links `v1->dock:serves` and
   `v2->dock:serves`, `groups: [{ block: vessel }]`: the scene has no edge to `dock`, one badge on
   the box `vessel` with `count` 2, the two link ids in `links`, and a label ending `×2`.
9. `group_box_is_a_hub_by_in_degree_after_the_roll_up`: four `vessel` parts `v1` to `v4` outside a
   region, four `tug` parts `t1` to `t4`, links `t1->v1:guides`, `t2->v1:guides`, `t3->v2:guides`,
   `t4->v3:guides`, `hubs: { in_degree: 2 }`, `groups: [{ block: vessel }]`. Before the roll-up
   `v1` has 2 incoming links and would be a hub by itself; after it `v1` is a member of the
   box `vessel`, which has 4 rolled-up incoming links, so the box is the one hub of the scene
   (`hub: true`), no part `v1` to `v4` has a box of its own, and the four links are badges on
   `t1` to `t4`.
10. `group_finding_is_counted_on_the_box_and_drawn_in_its_view`: a finding on a member is on the
    box's `collapsed.findings`, and its mark is in the scene of `<view>/<box id>`.
11. `group_drill_down_view_holds_the_members_only`: `views_of` adds `<view>/<box id>` after the
    view, its edges view and its region views; its `ViewSpec.group` holds the member ids; its
    scene has the members and the links among them, no `collapsed` box, and the inherited `rows`.
12. `group_with_regions_collapses_both`:
    `{ blocks: [quay], groups: [{ block: vessel }] }` draws the quay regions as boxes and the
    ungrouped vessels as one box, and the two kinds share one roll-up (a link from a vessel to a
    berth in a quay is one edge between the two boxes).
13. `group_view_without_groups_keeps_its_bytes`: the scenes of the harbor views `layout`,
    `guidance` and `quays` equal `fixtures/expected/harbor` before and after.
14. `group_fixture_view_page_holds_the_boxes`:
    `cargo nextest run -p metaspec -E 'test(/::group_/)'`. The page of the harbor document
    `marina` (new, invented: 5 vessels, 3 with `draught` 2m and 2 with `draught` 3m, and 3 tugs,
    none in a quay) in the harbor view `fleets` (`layout: rows`, `collapse: { groups: [{ block:
    vessel, by: draught }, { block: tug }] }`) shows exactly three boxes: `vessel=2m` with 3 parts,
    `vessel=3m` with 2 parts and `tug` with 3 parts, each linking `#view-fleets/<id>`, and the
    sentence that names groups; the pages of the other harbor documents show no group box (cove
    and lagoon have one vessel and one tug each, which stay plain parts).
15. `group_named_edges_stays_plain`:
    `cargo nextest run -p metaspec-layout -E 'test(/::group_named_edges_stays_plain/)'`. A graph
    with 3 parts `e1`, `e2`, `e3` of a block `edges` outside a region, a part `h`, a link
    `e1->h:serves`, `groups: [{ block: edges }]` and `hubs: { parts: [h] }`: the scene of the view
    has no box `edges` with `collapsed`, `e1`, `e2` and `e3` are plain boxes, `h` is the hub and
    the link is a badge on `e1`; `views_of` gives the view and `<view>/edges` (the view of the
    hubs, because the link is elided) and no group view of that id; the scene of `<view>/edges`
    shows `e1`, `e2`, `e3` and `h` as plain boxes, no badge, and the link `e1->h:serves` as an
    edge. A second
    graph with four parts of block `edges` and a field `zone` of the values `a`, `a`, `b`, `b`,
    `groups: [{ block: edges, by: zone }]`, has exactly two group boxes, `edges=a` and `edges=b`,
    with 2 parts each.
16. `group_named_edges_is_named`:
    `cargo nextest run -p metaspec-core -E 'test(/::group_named_edges_is_named/)'`, in the style of
    the region case `region_collapse_of_no_region_and_of_a_region_named_edges_are_named`
    (`crates/metaspec-core/tests/cases/fixtures.rs:244`): on a copy of a fixture pack with one
    document more that holds 3 parts of a block `edges`, each with a field `zone`, in a pack that
    has the block `edges`, the codes of the warnings of `metaspec pack check` are pinned as at
    `fixtures.rs:287` and `:301`: a view with `groups: [{ block: edges }]` has exactly
    `["view_region_edges"]`, its message naming `edges` and that document (as `fixtures.rs:283-286`
    does for a region); a view with `groups: [{ block: edges, by: zone }]` has exactly `[]`.

`task check:docs` passes with the new harbor goldens regenerated and no change to the goldens of
the orchard pack and of every other harbor view. Each test runs under 5 s; the step under the
repository's 60 s and 120 s bounds.
