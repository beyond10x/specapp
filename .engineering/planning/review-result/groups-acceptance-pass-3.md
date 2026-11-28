---
format: aep.planning-md/3
id: review-result:groups-acceptance-pass-3
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 3, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
needs-revision

Both pass 2 findings are resolved. Item 15 of story:block-groups is new and has two defects, and the rest of both stories can be checked.

**Pass 2 findings**
- **Resolved:** block-groups item 14 now gives member counts per value. It expects exactly three boxes: `vessel=2m` with 3 parts, `vessel=3m` with 2 and `tug` with 3. It also says why the other harbor documents show none: cove and lagoon have one vessel and one tug each.
- **Resolved:** block-groups item 1 now lists three cases of the same block twice: no `by`, the same `by`, and mixed. Scope refuses the block twice whatever its `by`, so the two agree.

**Findings**

story:block-groups — item 15 is the test `group_named_edges_stays_plain` in `metaspec-layout`. It says `metaspec-core`'s `test(/::group_unknown/)` "also holds the warning" `view_region_edges`, but it names no test that does. Item 2's test `group_unknown_block_and_field_are_named` lists four other codes and not this one. A test named `group_named_edges_stays_plain` cannot match `::group_unknown`, so the warning outcome has no named test or exact command. Name a core test with a `group_unknown` prefix, or split the warning into its own item with its own test name and command, as the region analogue does in `fixtures.rs:244`. — `.engineering/planning/story/block-groups.md:288`

story:block-groups — item 15 says "a block `edges` with `by` makes the boxes `edges=<value>`" but gives no parts or values. A value held by one part stays a plain part (Scope: a group of one is plain), so which boxes to expect depends on a graph the item does not state. State the `by` values and counts, for example 2 and 2 under one `by` field, and the expected ids. — `.engineering/planning/story/block-groups.md:288`

The rest of both stories is checkable:
- **block-groups, items 1 to 14:** each names a graph or fixture, the state it turns on, and a runnable `cargo nextest` filter. Items 3 to 13 run under item 3's `-p metaspec-layout` `::group_` filter, and item 14 names its own `metaspec` filter.
- **band-wrap, items 1 to 11:** the same holds. The `view_wrap_nested` warning, the `(k/m)` titles and the `band-title` text at `svg.rs:141-150` all exist or are specified, and item 11 states the wrapped and unwrapped comparison.

**What I read:** 3 ids, all 3 read in full with `aep plan artifact show`: story:block-groups, story:band-wrap and review-result:groups-acceptance-pass-2. In the tree I read `fixtures/manifests/harbor.yaml`, the four harbor documents, `fixtures/expected/`, `manifest.rs`, `svg.rs`, `Taskfile.yml`, the `view_region_edges` code in `pack.rs`, and `fixtures.rs:244-290`.

**What I could not establish**
- `marina.yaml` does not exist yet, so I cannot confirm that band-wrap item 11 gets exactly one band of 8. Its links are not fixed by block-groups item 14. I did not flag this.
- Block-groups item 14 shows the `fleets` view as `collapse: {...}` only. `View.layout` is required (`manifest.rs:165`), and the item does not name it. Band-wrap item 11 does name its layout. I did not flag this.
- Out of my lane (scope critic): `GraphView.tsx` counting, `pages.test.tsx`, and the new `members` and `by` scene fields have no acceptance line. This is carried over from pass 2.

```findings
[
{"file":".engineering/planning/story/block-groups.md","line":288,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"item 15 says the `::group_unknown` command of metaspec-core 'also holds the warning' view_region_edges but names no test that does, and a test named group_named_edges_stays_plain cannot match that filter, so the warning outcome has no named test or exact command"},
{"file":".engineering/planning/story/block-groups.md","line":288,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"item 15 says a block `edges` with `by` makes the boxes `edges=<value>` but names no parts or values, so a value held by one part stays plain and the expected boxes are not stated"}
]
```
