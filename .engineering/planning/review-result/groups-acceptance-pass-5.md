---
format: aep.planning-md/3
id: review-result:groups-acceptance-pass-5
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 5, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
approve

**Pass 4 findings**
- **Resolved:** item 15's first graph now gives a hub and a link, so the existence of `<view>/edges` no longer depends on an unstated state.
  - It names the part `h`, `hubs: { parts: [h] }`, the link `e1->h:serves`, and the three plain `edges` parts.
  - It states what turns on that state: `h` is the hub, the link is a badge on `e1`, `views_of` returns the view plus `<view>/edges`, and `<view>/edges` draws the link as an edge with no badge.
  - `views_of` emits `<view>/edges` only when it elides a link (`crates/metaspec-layout/src/lib.rs:214-222`), and the item now supplies that link.
- **Resolved:** item 16 now says what "none" means, and both halves pin their codes. `groups: [{ block: edges }]` gives exactly `["view_region_edges"]`, and `groups: [{ block: edges, by: zone }]` gives exactly `[]`.
  - It also fixes the state: three `edges` parts each carrying `zone`, in a pack that has the block `edges`. That rules out `view_unknown_block` and `view_unknown_group_field`.
  - The pinning style matches the analogue, which asserts `codes(&report.warnings)` at `crates/metaspec-core/tests/cases/fixtures.rs:287` and `:301`. The analogue test starts at `:244`.

I read both stories again and found nothing new. All 16 acceptance items of block-groups and all 11 of band-wrap have one observable outcome, the state it turns on, and a command or a named test that shows it. The filters `::group_manifest`, `::group_unknown`, `::group_`, `::group_named_edges_*`, `::wrap_` and `-p metaspec` all resolve to the named tests.

**What I read:** 3 ids, all in full, with `aep plan artifact show story:block-groups`, `story:band-wrap` and `review-result:groups-acceptance-pass-4`. In the tree I read `crates/metaspec-layout/src/lib.rs:205-262`, `crates/metaspec-core/tests/cases/fixtures.rs:225-310`, `crates/metaspec-core/src/pack.rs:665-730`, and `ls fixtures/packs/harbor/harbors`.

**What I could not establish**
- Item 16 cites `fixtures.rs:283-286` for the message-names-the-region check. Those lines are the code vector, and the message assertion is at `:294-298`. The check itself is still observable, so this is a citation slip and I did not raise it as a finding.
- Items 4 and 15 each put two states in one named test. Pass 4 accepted that shape for item 4, and the name filter runs both, so I did not flag it.
- `fixtures/packs/harbor/harbors/marina.yaml` does not exist yet. I cannot confirm the single band of 8 boxes in band-wrap item 11 or the 3 and 2 box counts in block-groups item 14. Both depend on the new document.
- Out of my lane (scope critic): band-wrap's scope says `wrap` reaches `<id>/<group>` views, but item 9 tests only `<id>/edges` and `<id>/<region>`. The scene fields `members` and `by` and the `GraphView.tsx` counting still have no acceptance line. Both points carry over from earlier passes.

```findings
[]
```
