---
format: aep.planning-md/3
id: review-result:adversary-collapsed-regions-pass-1
kind: review-result
status: active
title: aep:adversary, pass 1, story:collapsed-regions
relations:
- reviews: story:collapsed-regions
revision: 1
---
# aep:adversary, pass 1, story:collapsed-regions

unit: story:collapsed-regions, commit ea80669 (diff against 48fb83d), working tree <worktree>
verdict: CONFIRMED
cases: executed 190→194, red 4
origin: introduced 7 / pre-existing 0 / undecided 0
wrote-outside-worktree: none
needs-coordinator: none

The change breaks when a view sets both `hubs` and `collapse`. The schema allows that, and the implementor's own test uses it. No real pack sets both today, so none of the findings blocks the merge. All 4 new tests fail, each for the reason it names.

**1. What I changed**
```
 crates/metaspec-layout/tests/cases/mod.rs | 1 +
 1 file changed, 1 insertion(+)
?? crates/metaspec-layout/tests/cases/regions_hubs.rs   (new, untracked)
```
Both are test files. I changed no implementation file and committed nothing.

**2. New tests** in `<worktree>/crates/metaspec-layout/tests/cases/regions_hubs.rs`. Each one runs on both `rows` and `nested`, and all 4 are red now. Command: `cargo nextest run -p metaspec-layout -E 'test(regions_hubs::)'`. Output of the run of these 4 alone:
- `region_rolled_link_drawn_as_a_hub_badge_keeps_its_count`. Links a->f:x and b->f:x leave region r, and f is a hub. The two roll up into one badge, and that badge shows no count:
  `Rows: the badge of a link standing for two says its count: "→ f · x"`
- `region_rolled_link_drawn_as_a_hub_badge_carries_the_findings_of_its_links`. A finding on a->f:x is lost from the drawing:
  `Rows: the badge stands for a->f:x and carries its finding  left: []  right: [1]`
- `region_hub_by_in_degree_is_the_same_hub_where_the_edges_view_is_made_and_where_it_is_drawn`. With `in_degree: 2`, the list of views decides on the links before roll-up: there are 2 links to f, so it adds `v/edges`. The drawing decides on the links after roll-up: there is 1, so it draws no badge. Result: `v/edges` is generated as an exact copy of `v`:
  `Rows: views ["v", "v/edges", "v/r"]; the view draws 0 badges ...  left: true  right: false`
- `region_named_edges_gets_a_view_of_its_own_beside_the_edges_view`. Two generated views get the same id:
  `Rows: view ids are unique: ["v", "v/edges", "v/edges"]  left: 2  right: 3`

**3. Suite run** (after the tests above existed)
`cargo nextest run -p metaspec-core -p metaspec-layout -p metaspec --no-fail-fast`
`Summary [5.576s] 194 tests run: 190 passed, 4 failed, 0 skipped`, exit 100. The 4 failures are the 4 tests above. 190 matches the count the implementor reported.

**4. Findings**

| # | file:line | finding | verdict | severity | what reaches it |
|---|---|---|---|---|---|
| F1 | crates/metaspec-layout/src/lib.rs:873 | A rolled-up link that becomes a hub badge gets its label from `badge_label` without `×n`. `Badge` has no `count` or `links` field. | CONFIRMED | warning | A view with `hubs` and `collapse` where the far end is a hub, or where the region itself is a hub. No pack does this today. |
| F2 | crates/metaspec-layout/src/lib.rs:983 | `attach` looks up a badge's findings by `badge.id`. That is the made-up rolled id `r->f:x`, which no finding names, so the findings of the links it stands for give no mark. They also drop out of the "Drawn in" column for that view. Fix: use the rolled members, as is already done for edges (lib.rs:942). | CONFIRMED | warning | Same as F1. |
| F3 | crates/metaspec-layout/src/lib.rs:219 | `views_of` decides whether the view elides links before roll-up; `layout` decides after it. When they disagree, an extra `<id>/edges` view is generated. | CONFIRMED | note | `hubs.in_degree` together with `collapse`. |
| F4 | crates/metaspec-layout/src/lib.rs:233 | A region with the part id `edges` in a view with hubs gets the id `v/edges`, which the edges view already has. The static page then has two sections `id="view-v/edges"` and two SVG prefixes `v-v.edges`, and the region's box links to the edges view. Only CHANGELOG.md:27 mentions this; design/metaspec.md does not, and nothing guards against it. | CONFIRMED | warning | Any pack with a container part named `edges`. Part ids are free strings. |
| F5 | crates/metaspec-docs/src/documents.rs:315 | The docs site pairs any scene `<id>/edges` that follows view `<id>` into a GraphSwitch. A region named `edges` in a view without hubs would therefore be shown as "the same view with every link an edge". Found by reading the code; not run, because the fixture packs have no part named `edges`. | CONFIRMED | note | Same as F4, but without hubs. |
| F6 | crates/metaspec-layout/tests/cases/scenes.rs:85 | The check `edges + badges == shown.links` is false as soon as two links roll into one. It passes only because no fixture scene has `count` ≥ 2, so the page and site code for a count above 1 never runs on a fixture. | CONFIRMED | note | Will go red once a fixture rolls two links together. |
| F7 | design/metaspec.md:1266 | `per` making no region is stated. But `collapse: [x]`, where x is no region (a host only through `per`, or an unknown id), does nothing and gives no warning. `hubs.parts` behaves the same way. | CONFIRMED | note | A typo or a `per` host named in `collapse`. |

**5. What I attacked and could not break**
- **Collapse by alias word:** the region's block is resolved through `blocks.aliases`. A listed alias word matches no region and warns `view_unknown_block`, the same as `view.blocks`.
- **Collapse by full block name:** a full domain name such as `harbor.quay` is refused by the schema's key pattern.
- **Nested and odd ids:** a collapsed region inside a collapsed region goes into the outer box, and its parts and findings count once there. An id that is not a container collapses nothing (F7). An `in` cycle ends without hanging.
- **Findings on the box:** no double count (dedup is in place). A finding on the region itself stays on the box's own `findings`. Findings on parts in a nested region are counted on the outer box.
- **Link roll-up:**
  - The same name and far end in opposite directions stay two edges.
  - A link inside one region is drawn only in that region's view.
  - Region-to-region links roll up to `r1->r2:name`.
  - The rolled id cannot collide with a link that was not rolled.
- **Byte equality:** `collapsed`, `count` and `links` are skipped when absent. `ViewSpec.of` and `region` are never serialized. In the harbor pages, the existing views did not change; the only changes are the added views.
- **Schemas:** `collapse` in `metaspec-pack/1` matches what the code accepts. `collapsed.parts` (minimum 1), `count` and `links` in `metaspec-scene/1` match what the code writes.
- **Not checked:** I did not run the three real packs for byte equality.

**6. Paths written outside the worktree:** none. Command output went to the harness's own task files.

```findings
[
{"file":"crates/metaspec-layout/src/lib.rs","line":873,"category":"boundary","severity":"warning","verdict":"CONFIRMED","origin":"introduced","message":"a rolled-up border link drawn as a hub badge loses its ×n count because place_badges labels it with badge_label and Badge has no count or links"},
{"file":"crates/metaspec-layout/src/lib.rs","line":983,"category":"boundary","severity":"warning","verdict":"CONFIRMED","origin":"introduced","message":"attach looks up a badge's findings by the synthetic rolled id, so findings on the links a rolled-up badge stands for get no mark and drop out of the view's Drawn in"},
{"file":"crates/metaspec-layout/src/lib.rs","line":219,"category":"property","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"views_of decides elision on the selection before roll-up and layout after it, so hubs.in_degree with collapse generates a <id>/edges view identical to the view"},
{"file":"crates/metaspec-layout/src/lib.rs","line":233,"category":"boundary","severity":"warning","verdict":"CONFIRMED","origin":"introduced","message":"a region whose part id is edges in a view with hubs gets the view id <id>/edges already taken by the edges view, giving duplicate page sections and SVG ids and a box that links the wrong view"},
{"file":"crates/metaspec-docs/src/documents.rs","line":315,"category":"judgement","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"the docs site pairs any <id>/edges scene after <id> into a GraphSwitch, so a region named edges in a view without hubs is shown as the every-link-an-edge switch"},
{"file":"crates/metaspec-layout/tests/cases/scenes.rs","line":85,"category":"judgement","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"the edges plus badges equals shown links check is false once two links roll up and holds only because no fixture scene has a count of 2 or more"},
{"file":"design/metaspec.md","line":1266,"category":"judgement","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"collapse naming a part that is no region (a per host or an unknown id) silently collapses nothing with no warning"}
]
```
