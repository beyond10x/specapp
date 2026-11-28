---
format: aep.planning-md/3
id: review-result:groups-design-pass-1
kind: review-result
status: active
title: aep:plan-critic-design, pass 1, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
needs-revision

block-groups reuses the collapsed-regions mechanism. It does not build a second roll-up. Two things in it need changing, and band-wrap has one smaller defect.

**What holds**
- **Reuse:** block-groups generalises `Region` to a box id plus an optional part. It makes `fold`, `draw` and `views_of` read that id and adds `ViewSpec.group` beside `region`. It states "A second roll-up is refused" (`block-groups.md:112-118`). Link roll-up, badges, findings and the `metaspec-scene/1` `collapsed` box are shared with regions.
- **Cycles:** none. The edges are band-wrap → block-groups → collapsed-regions → {cloudspec-manifest, part-block-aliases}. Every one is `depends_on`, which `aep plan artifact relations` defines as "needs first". collapsed-regions is already in code (`Region` and `fold` exist), so that edge is satisfied.
- **Chain:** the three-deep chain is not a finding. The band-wrap edge has its reason written beside it, the shared `manifest.rs`, `lib.rs`, pack schema, `harbor.yaml` and goldens (`band-wrap.md`, Surfaces). The trade-off is that the edge keeps the two apart on those files. Splitting `marina.yaml` and the harbor view entries would let them run apart, but they would still share `manifest.rs`, `lib.rs`, the pack schema and `documents.yaml`.
- **Split abstraction:** none. G6/G7 (grouping) and G8 (wrapping) are separate features. `wrap` reaches `<id>/<group>` through the existing `..spec.clone()`, with no code in either story for the other.
- **Syntax:**
  - `collapse: { blocks?, groups: [{ block, by? }] }` extends the object form that `blocks` already uses, so `block` mirrors `hubs.blocks`.
  - `wrap: <n>` sits flat beside `rows`, like the other layout options.
  - Both are consistent with the existing `collapse` and view options.

**Findings**
- story:block-groups, the drill-down view's membership cannot be derived (`block-groups.md:124-126`). The view is made `..spec.clone()` with `collapse` and `hubs` stripped, and says only "`ViewSpec` gains `group` beside `region`". A region view works because the region is a part id and membership comes from `in`. A group's membership needs `groups`, the region set and the hub set, and all three are gone from the drill-down spec. `<block>=<value>` does not even carry the `by` field. The body should say `ViewSpec.group` holds the member ids, or the whole definition, and that `select` filters to them.
- story:block-groups, hub exclusion and hub status are circular (`block-groups.md:139-141`). "A part the view makes a hub (… `in_degree`) is never a member" needs the hub set before the roll-up. "A group box is a hub by `in_degree` … counted after the roll-up" needs it after. `hubs_of` runs on the rolled-up selection, after `fold` (`lib.rs:735-755`, `layout` at `:474-486`). The body should state the order: `hubs.parts` and `hubs.blocks` exclude members before the roll-up, an `in_degree` hub is judged after it, and a part that is a hub only by `in_degree` joins its group (or is excluded on a pre-roll-up count). Acceptance 3 should then say which kind of hub it uses.
- story:band-wrap, the shared `marina.yaml` is claimed two ways (`band-wrap.md:108-109`). It lists "the document story:block-groups adds, or a document of this story's own if that has not landed". The `depends_on` edge means block-groups lands first, so the alternative is dead text. It also leaves open whether band-wrap adds parts to `marina` or only a view. Adding parts would change block-groups' box counts and its goldens (acceptance 13). The body should say it edits block-groups' `marina.yaml` by adding a view only. The view would use `rows: family` so vessels and tugs fall in one band of 8. Or it should use its own document.

**What you read:** 2 artifacts through `aep plan artifact show`, plus `relations`, `graph` (about 130 nodes; I walked the edges of both stories and of collapsed-regions, extends-inherits, focus-pages, view-selectors and matrix-view, outside the set) and `validate` (valid). I also read `manifest.rs:155-230`, `lib.rs:160-260, 295-520, 728-775`, the `rows.rs` row keys, `harbor.yaml`, and the "Decided: collapsed regions" section of `design/metaspec.md`. I traced 4 acceptance reads to producers in the set (block-groups 11-13 → collapsed-regions; band-wrap 11 → block-groups' `marina`), and every one has an edge.

**What I could not establish**
- Whether `marina` puts 7 or more boxes in one band for band-wrap's `wrapped` view. That depends on `rows: family` or layer, and the body does not say.
- Unease, not a finding: `collapse: all` and the id list cannot be combined with `groups`, and the body does not say why. A consumer that wants every region collapsed and the remaining parts grouped must list region blocks instead.
- Out of my lane (parallel-safety): block-groups and band-wrap both share `manifest.rs`, `lib.rs`, the pack schema and the harbor goldens with view-selectors, matrix-view, focus-pages and extends-inherits. No edge orders them against those four.

```findings
[
{"file":".engineering/planning/story/block-groups.md","line":124,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the drill-down view is made with collapse and hubs stripped, yet `ViewSpec.group` is not said to hold the member ids, so `select` cannot recompute a group's membership (regions can, from `in`); the body should name what `group` carries"},
{"file":".engineering/planning/story/block-groups.md","line":139,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"a hub by in_degree is never a member, but in_degree is counted after the roll-up that membership decides, so the two rules are circular; the body should state the order and which hubs exclude members before the roll-up"},
{"file":".engineering/planning/story/band-wrap.md","line":108,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"marina.yaml is claimed as block-groups' document 'or a document of this story's own if that has not landed', which the depends_on edge makes dead, and the body does not say whether band-wrap adds parts (changing block-groups' goldens) or only a view"}
]
```
