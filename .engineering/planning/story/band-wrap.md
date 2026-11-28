---
format: aep.planning-md/3
id: story:band-wrap
kind: story
status: active
title: A band of the layout rows wraps after a set number of boxes
summary: 'A view''s wrap: <n> cuts each band of rows into rows of at most n boxes, in a deterministic order; generated views inherit it.'
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:block-groups
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
  path: crates/metaspec-layout/src/lib.rs
- confidence: cited
  path: crates/metaspec-layout/src/rows.rs
- confidence: inferred
  path: crates/metaspec-layout/tests/cases/wrap.rs
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
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: spec/domains/documents.yaml
revision: 10
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T20:23:26Z", actor: "human:operator", revision: 9}
- {from: "proposed", to: "active", at: "2026-11-28T20:23:26Z", actor: "human:operator", revision: 10}
---
# A band of the layout rows wraps after a set number of boxes

## Context

A consumer that extends cloudspec drew one drill-down view (`<view>/<region>`) that is 9221 px wide
for 22 boxes: `rows` puts every box of a band in one row. Gap G8 of the consumer's list asks for a
view option that wraps a band, deterministic.

How `rows` lays out today (`crates/metaspec-layout/src/rows.rs`): every node (a box with the parts
drawn inside it) gets a row key from its block's layer or family, hubs and names no part has
included (`:203-220`); the keys, highest first, are the bands (`:221-223`); a band is exactly one
layer of the layered drawing, `2 * keys.len() + 1` layers with a label layer between every two
bands (`:231`, `:233-246`). The width of the scene is the widest layer (`:473-478`). The bands of
the scene are one `Band` per key (`:630-642`, `crates/metaspec-layout/src/scene.rs:45-51`). Nothing
splits a band.

The drill-down view of a collapsed region is made from its view by `..spec.clone()`
(`crates/metaspec-layout/src/lib.rs:233-249`), as is `<id>/edges` (`:225-230`), so an option on
`ViewSpec` (`lib.rs:168-188`, built from `View` at `:189-205`) reaches them without more code. The
implicit view "All parts and links" is built field by field (`lib.rs:262-278`).

## Scope

- Manifest syntax: `wrap: <n>` on a view, an integer of 2 or more (`View`,
  `crates/metaspec-core/src/manifest.rs:162-178`, beside `rows` at `:167`; schema
  `schema/metaspec-pack-1.schema.json`, beside `rows` at `:274-277`, `"type": "integer",
  "minimum": 2`). `ViewSpec` gains `wrap: Option<usize>`; `views_of` copies it to
  `<id>/edges`, to every `<id>/<region>` and to every `<id>/<group>` of story:block-groups through the existing `..spec.clone()`; the implicit view has none.
- Meaning, for `layout: rows` only: a band that holds more than `n` boxes is drawn in rows of at
  most `n` boxes, top to bottom. A box is a node of `rows.rs` (a part with the parts drawn inside
  it by `per` is one box; a ghost is one box; a collapsed box is one box). Every band wraps,
  hubs and ghosts included.
- Which box lands in which row, deterministic: the nodes of a band in the order `rows.rs` already
  makes (the order links first name them, then the document's order, `rows.rs:125-141`), cut into
  consecutive chunks of `n`. The cut is made before the barycentre sweeps (`:331-401`), which
  then order the boxes inside each row, so the same input gives the same bytes, with no hash
  order and no measuring of text.
- Each row of a wrapped band is a layer of its own, with a label layer between it and the next
  (the `keys`, `row_of` and `n_layers` of `rows.rs:221-231` count rows, not bands). Edges, labels and
  ports are routed by the machinery that exists: an edge between two rows of one band is an edge
  across rows (its label in the layer between, `:289-327`), an edge inside one row goes up to its
  label in the layer above (`:270-292`), and the ports spread as before (`:504-538`).
- Bands in the scene: one `Band` per row. A band that wrapped has the title
  `<title> (<k>/<m>)` on each of its rows; a band that fits keeps its title, so the scene of an
  unwrapped band is unchanged. The scene's format does not change (`Band` has `title`, `y`, `h`).
- `layout: nested` is not wrapped (it wraps by width already, `nested.rs`); `wrap` on a `nested`
  view is a `metaspec pack check` warning `view_wrap_nested`
  (`crates/metaspec-core/src/pack.rs`, with the view warnings at `:342-361`), and the view is
  drawn as without it.
- A view without `wrap` keeps its scenes, pages and sites byte for byte (the code path is the
  one that exists when `wrap` is absent, held by a golden test).

## Surfaces

Files: `crates/metaspec-core/src/manifest.rs`, `crates/metaspec-core/src/pack.rs`,
`crates/metaspec-core/tests/cases/manifest.rs`, `crates/metaspec-core/tests/cases/fixtures.rs`,
`crates/metaspec-layout/src/rows.rs`, `crates/metaspec-layout/src/lib.rs` (`ViewSpec`, `From`,
`views_of`, the implicit view),
`crates/metaspec-layout/tests/cases/wrap.rs` (new), `crates/metaspec/tests/main.rs`,
`schema/metaspec-pack-1.schema.json`, `spec/domains/documents.yaml` (the entity `View` gains
`wrap`, `:189`), `design/metaspec.md` (the `rows` decision at `:731-740`), `CHANGELOG.md`,
`fixtures/manifests/harbor.yaml` (one view, `wrapped`: `layout: rows`, `rows: family`, `wrap: 3`),
`fixtures/expected/` (the harbor pages and site, regenerated). The scene schema does not change.
The view reads the document `fixtures/packs/harbor/harbors/marina.yaml` that story:block-groups
adds (5 `vessel` and 3 `tug` parts: with `rows: family` the family `craft` holds all 8 in one
band). This story adds no part and no document and does not edit `marina.yaml`.

Shared with other planned stories, checked with `aep plan artifact show`:

- story:block-groups: both touch `manifest.rs`, `pack.rs`, `lib.rs` (`ViewSpec`, `views_of`), the
  pack schema, `spec/domains/documents.yaml`, `harbor.yaml` and the harbor goldens. Edge: this
  story `depends_on` story:block-groups, so block-groups lands first and this story starts from
  its `ViewSpec` and `views_of`.
- story:extends-inherits (active): `manifest.rs`, `pack.rs`, the pack schema, `design/metaspec.md`
  and `CHANGELOG.md`. This story lands after it through story:block-groups, which `depends_on`
  story:extends-inherits.

Order, D61 (the consumer's gaps first): story:extends-inherits, story:block-groups, this story,
then story:view-selectors, story:matrix-view, story:focus-pages, story:search-score,
story:badge-width and story:placement-overlay, each of which `depends_on` this story.

- story:view-selectors, story:matrix-view, story:focus-pages: `manifest.rs`, `lib.rs`, the pack
  schema, `spec/domains/documents.yaml`, `harbor.yaml` and the harbor goldens; matrix-view also
  `scene.rs` and the scene schema, focus-pages also `page.rs` and `documents.rs`. Each lands
  after this story.
- story:search-score: `manifest.rs`, the pack schema, `harbor.yaml`, `design/metaspec.md` and
  `CHANGELOG.md`. It lands after this story.
- story:badge-width: `rows.rs` (badges grow the box through `grow`, `rows.rs:154` and `:185`),
  `nested.rs` and `lib.rs`; the cut of this story is by count, so it does not depend on badge
  widths. It lands after this story; the conflict is a merge in `rows.rs`.
- story:placement-overlay: `manifest.rs`, the pack schema and `documents.rs`. It lands after this
  story.
- story:graph-viewport: `web/ui/src/GraphView.tsx`; this story does not edit it. It is ordered
  after this story through story:focus-pages.
- Not shared: story:nested-readable (`nested.rs`).

## Not in this story

- A wrap by width (a pixel bound): box widths follow their text, and a width bound is a second
  rule beside the count for a result the count already gives deterministically.
- A different `wrap` for a drill-down view than for its view; they share the number.
- A cut that keeps linked boxes in one row; the cut is by order, and the sweeps order each row.
- Wrapping in `nested`.
- Keeping the number of crossings the same as without `wrap`: more rows is more label layers; a
  view reads better, not with fewer crossings.

## Acceptance

Tests live in `crates/metaspec-layout/tests/cases/wrap.rs` and the cases of `metaspec-core` and
`metaspec` named below. Words are invented; no word of a real pack is in a test.

1. `wrap_manifest_takes_an_integer_of_two_or_more`:
   `cargo nextest run -p metaspec-core -E 'test(/::wrap_/)'`. `wrap: 6` parses on a `rows` view;
   `wrap: 1`, `wrap: 0`, `wrap: "6"` and `wrap: 2.5` are refused; a view without it is `None`.
2. `wrap_on_nested_is_named`: the same command. A `nested` view with `wrap` is the warning
   `view_wrap_nested` of `metaspec pack check`, naming the view; a `rows` view has none.
3. `wrap_band_splits_after_n_boxes`:
   `cargo nextest run -p metaspec-layout -E 'test(/::wrap_/)'`. A graph of 7 parts of one layer
   with `wrap: 3`: the scene has 3 rows for that band (3, 3 and 1 boxes), their `y` rise, no two
   boxes of the scene overlap, and the scene is narrower than the view without `wrap`.
4. `wrap_band_that_fits_keeps_its_title_and_its_row`: a band of 3 parts with `wrap: 3` has one
   row titled as without `wrap`, and a band of 4 has the titles `<title> (1/2)` and
   `<title> (2/2)`.
5. `wrap_order_is_deterministic_and_kept`: two layouts of one view give equal JSON, and the boxes
   of each row are the next `n` nodes of the order the links first name them, then the
   document's (a test that moves a part in the document changes the row it lands in as the
   order says).
6. `wrap_edge_between_rows_of_one_band_is_labelled_between_them`: a link from a box of the first
   row to a box of the second row of one band has points that start at the bottom of the upper
   box and end at the top of the lower one, and its label lies between the two rows.
7. `wrap_edge_inside_one_row_goes_up_to_the_label_above`: two boxes of one row linked: the
   label is in the gap above that row, which is the gap below the row before it.
8. `wrap_ghosts_wrap_too`: a part `p` with links to 5 names no part has, `wrap: 2`: the band
   `names no part has` is 3 rows of 2, 2 and 1 boxes, titled `names no part has (1/3)`, `(2/3)`
   and `(3/3)`; the band of `p` stays one row with its plain title.
   `wrap_hubs_wrap_too`: a part `c1` and five parts `h1` to `h5`, `hubs: { parts: [h1, h2, h3, h4,
   h5] }`, links `c1->h1:serves` to `c1->h5:serves`, `wrap: 2`: the five hubs are drawn once in the
   band `hubs`, which is 3 rows of 2, 2 and 1 boxes titled `hubs (1/3)`, `(2/3)` and `(3/3)`, the
   five links are badges on `c1`, and the band of `c1` stays one row.
9. `wrap_generated_views_inherit_it`: `views_of` gives `<id>/edges` and `<id>/<region>` of a view
   with `wrap: 2` the same `wrap`, and the implicit view has none.
10. `wrap_view_without_wrap_keeps_its_bytes`: the scenes of every existing harbor and orchard view
    equal the committed `fixtures/expected/harbor` and `fixtures/expected/orchard` before and after.
11. `wrap_fixture_view_page_holds_the_rows`:
    `cargo nextest run -p metaspec -E 'test(/::wrap_/)'`. The harbor view `wrapped` (`layout:
    rows`, `rows: family`, `wrap: 3`) on the document `marina` (added by story:block-groups: 5
    `vessel` and 3 `tug` parts, all in the family `craft`, so one band of 8 boxes) renders a page
    whose SVG has exactly three `band-title` texts for that band, `craft (1/3)`, `craft (2/3)` and
    `craft (3/3)` (the band titles `<title> (k/m)` written at
    `crates/metaspec-layout/src/svg.rs:141-150`), holding 3, 3 and 2 boxes. The same test lays out
    the same `ViewSpec` with `wrap` absent and states the comparison there: that scene has one
    `craft` band of 8 boxes and no `(k/m)` title, and the wrapped scene is narrower than it. No
    twin view is added to the manifest.

`task check:docs` passes with the new harbor goldens regenerated and no change to the goldens of
the orchard pack and of every other harbor view. Each test runs under 5 s; the step under the
repository's 60 s and 120 s bounds.
