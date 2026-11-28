---
format: aep.planning-md/3
id: review-result:groups-design-pass-5
kind: review-result
status: active
title: aep:plan-critic-design, pass 5, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
approve

Both pass-4 findings are resolved:

- **Group box as a synthetic part (D64).** The scope now says where the box becomes a part: `fold` appends a synthetic `Part` to the folded graph, with the box id, the group's block and no fields, and the folded selection lists its index (`story:block-groups` scope, line 128 of the artifact file). That settles the finding.
  - It is consistent with the code. `rows.rs` takes the band and colour from `parts[i].block` (`rows.rs:204`), and `Region.part` is read only in `select`, `fold`, `draw` and `views_of`, which the story already lists as edited.
  - `rows.rs` and `nested.rs` stay untouched, so the files shared with `story:band-wrap` and `story:badge-width` do not grow.
- **Empty-text `by` counts as no value.** The scope says so in two places (the Domain/Scope text and the `<block>=` bullet), and acceptance 4 pins it with `draught: ""` joining the `vessel=` box. This matches `name_of` returning `Some("")` (`graph.rs:163`).

What I checked on the shape:

- **Cycles.** None. `block-groups` depends on `collapsed-regions` and `extends-inherits`. `band-wrap` depends on `block-groups`, and six later stories depend on `band-wrap`. A `tsort` over the `depends_on`, `blocks`, `decomposes` and `derived_from` edges found no loop.
- **Chain.** The order is serial, but each edge has its reason written beside it: shared files, `marina.yaml` and D61 order. That is the shared-surface trade-off, not a chain defect.
- **Ownership.** `block-groups` owns `Collapse.groups`, `ViewSpec.group`, the group box and `marina.yaml`. `band-wrap` owns `wrap` and the row cut, and only reads `marina.yaml`. Neither describes half of the other's abstraction.
- **Reuse of the collapsed-regions mechanism.** Region and group share one `fold`, one roll-up, one `<view>/<id>` drill-down, one finding count and `REGION_EDGES`. The group view differs only in `ViewSpec.group` carrying the member ids, because the drill-down spec is stripped of `collapse`. No second roll-up exists.
- **Manifest syntax.**
  - `collapse: { blocks?, groups: [{block, by?}] }` extends `CollapseBlocks`, which the `Collapse::Blocks` variant already holds, and `deny_unknown_fields` still rejects other keys.
  - `all` and the id list cannot combine with `groups`, and the parser refuses the same block twice.
  - `wrap` sits flat beside `rows`.
  - The schema's `oneOf` object branch needs `blocks` optional, and the body says so.
- **Box id namespace.** Block keys match `^[A-Za-z_][A-Za-z0-9_-]*$` (`schema:221`), so they hold no `=`, `/` or `#`. That makes `<block>` and `<block>=<value>` unique across blocks. The `#2` rule covers part ids and ghost names, and `edges` is handled by `REGION_EDGES`.

Part 3, what I read: 3 artifacts (both stories and `review-result:groups-design-pass-4`), plus `lib.rs`, `rows.rs`, `manifest.rs`, `graph.rs`, the pack schema and `harbor.yaml` in the tree. Commands: `aep plan artifact show`, `relations`, `graph` and `validate` (146 artifacts, valid). I walked the 973 edges of the whole graph, including edges outside the set, and found no cycle. I traced 6 acceptance reads to producers in the set, and a `depends_on` edge records all 6:
- `marina.yaml` in `block-groups` 14 and in `band-wrap` 11.
- The harbor goldens in `block-groups` 13 and `band-wrap` 10.
- `ViewSpec` and `views_of` in `band-wrap` 9.
- `harbor.yaml` in both stories.

Part 4, what I could not establish:
- Whether a `by` value containing `/` or spaces is safe in the view id and the page anchor. Part ids that name region views already carry the same exposure, so I did not treat it as a defect.
- Out of my lane: `band-wrap` scope says group views inherit `wrap`, but its acceptance 9 tests only `<id>/edges` and `<id>/<region>`.
- Out of my lane: the group drill-down says "the links between them", but `select` also keeps links to ghosts. Acceptance 11 does not say which holds.

```findings
[]
```
