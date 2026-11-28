---
format: aep.planning-md/3
id: review-result:adversary-hub-badges-pass-1
kind: review-result
status: active
title: aep:adversary, pass 1, story:hub-badges
relations:
- reviews: story:hub-badges
revision: 1
---
# aep:adversary, pass 1, story:hub-badges

unit: story:hub-badges, commit 650a085 on branch hub-badges (worktree cloudspec-hub-badges, base 56a8656), plus my 3 new test cases, not committed
verdict: NEEDS-CHANGE
cases: executed 174→177, red 3
origin: introduced 4 / pre-existing 0 / undecided 0
wrote-outside-worktree: 3 (test scratch directories the metaspec test binary created under its cargo target tmpdir)
needs-coordinator: none

**1. Diff stat.** Only test files changed.
```
 crates/metaspec-core/tests/cases/manifest.rs | 20 +++++++++++++++
 crates/metaspec-layout/tests/cases/hubs.rs   | 37 ++++++++++++++++++++++++++++
 crates/metaspec/tests/main.rs                | 37 ++++++++++++++++++++++++++++
 3 files changed, 94 insertions(+)
```

**2. New cases.** I wrote all three before running anything, then ran each alone. All three are red now.

- `crates/metaspec-layout/tests/cases/hubs.rs`, `hub_view_whose_document_has_no_hub_adds_no_second_identical_drawing`
  - **Setup:** a view with `hubs: {in_degree: 3}` on a graph whose most-linked part has 2 incoming links.
  - **Asserts:** the view elides nothing, and its `/edges` twin has the same boxes, edges and rows (both pass). It then asserts that `views_of` returns only `["one"]`, which fails.
  ```
  assertion `left == right` failed: a view that elides nothing in this document needs no `/edges` twin: the page would draw the same drawing twice
    left: ["one", "one/edges"]
   right: ["one"]
  ```
- `crates/metaspec-core/tests/cases/manifest.rs`, `hub_view_refuses_hubs_that_name_no_hub`
  - **Asserts:** `hubs: { parts: [] }`, `{ blocks: [] }` and `{ parts: [], blocks: [] }` are refused.
  ```
  panicked at crates/metaspec-core/tests/cases/manifest.rs:321:9:
  , hubs: { parts: [] } has a key and names no hub; it is refused like `hubs: {}`
  ```
- `crates/metaspec/tests/main.rs`, `hub_edges_view_svg_ids_stay_apart_from_a_declared_view_of_the_same_prefix`
  - **Setup:** the orchard manifest copied with a second view `work-edges` added next to `work` (the view with hubs), then rendered.
  - **Asserts:** in `layout/hillside.html`, the SVG ids `v-work-edges-arrow`, `-t` and `-d` each occur once.
  ```
  assertion `left == right` failed: the SVG id v-work-edges-arrow is one element of the page; the views work/edges and work-edges share it
    left: 2
   right: 1
  ```

**3. Suite run**, made after the cases existed: `cargo nextest run --workspace`, exit 100.
```
    Starting 177 tests across 3 binaries
     Summary [   6.249s] 177 tests run: 174 passed, 3 failed, 0 skipped
        FAIL ( 16/177) metaspec::main hub_edges_view_svg_ids_stay_apart_from_a_declared_view_of_the_same_prefix
        FAIL ( 83/177) metaspec-core::main cases::manifest::hub_view_refuses_hubs_that_name_no_hub
        FAIL (159/177) metaspec-layout::main cases::hubs::hub_view_whose_document_has_no_hub_adds_no_second_identical_drawing
```
All 12 existing `hub_` cases pass. Every test ran under 1 s.

**4. Findings** (all against 650a085)

| # | file:line | verdict | origin | what was measured / what reaches it |
|---|---|---|---|---|
| F1 | crates/metaspec-layout/src/lib.rs:204 | NEEDS-CHANGE | introduced | **Measured:** `views_of` adds `<id>/edges` whenever `hubs` is set, even when no part of the document becomes a hub. That view's drawing is identical to the first. The static page then shows the same drawing twice, and the docs site shows a switch between two identical views. **Reaches it:** any class with `hubs: {in_degree: N}` that has a document whose parts all stay below N. That is the normal case for a class with documents of different sizes, which is the consumer's situation. A `hubs.blocks` naming an unknown block reaches it too, alongside the warning. **Fix:** `views_of` already gets the graph, so it can add the twin only when `hubs_of` finds a hub (or only when the scene has badges). |
| F2 | schema/metaspec-pack-1.schema.json:281 | NEEDS-CHANGE | introduced | **Measured:** `minProperties: 1` lets `hubs: {parts: []}` and `{blocks: []}` through. The view then names no hub and still gets an `/edges` twin (F1). The spec domain says the schema refuses a `hubs` that names nothing, and it does not. **Reaches it:** a pack author writing an empty list. **Fix:** `minItems: 1` on `parts` and `blocks`. |
| F3 | crates/metaspec/src/page.rs:154 | INFEASIBLE | introduced | **Measured:** the SVG prefix turns `/` into `-`, so the generated view `work/edges` and a declared view `work-edges` both get the prefix `v-work-edges`. Their marker, title and desc ids are duplicated on one page. **Reaches it:** nothing found. I built the second view myself; neither fixture pack nor the three real packs declare such a view id. **Fix:** a separator a view key cannot contain, e.g. `.`. |
| F4 | crates/metaspec-docs/src/documents.rs:314 | CONFIRMED | introduced | **Measured:** with one badge the docs page reads "1 more link go to 1 hub". This is read from the code; I wrote no case for it. **Reaches it:** any docs view with exactly one badge. |

**5. Attacked and not broken**
- `in_degree` counts only the links the view shows: links filtered out by `carries` or `blocks` are not counted, ghost callers are.
- A hub that is also a caller: hub-to-hub links and links out of a hub stay edges.
- A finding on an elided link: one mark, on the badge, never on an edge too. It still shows in `drawn_in` for both views, and on a child of a host in `rows`.
- Badge geometry: a host's badges and its children in `rows`, a container wider than its card in `nested`, a ghost caller, and a link with no `from`.
- In `rows` a hub with `per` is drawn in the hubs band, not inside its host. No test catches a mutant that drops this. I could not prove that with a mutated copy because no scratch directory was assigned.
- Scenes and pages without hubs: `badges` and `hub` are skipped when empty, the prefix only changes for ids containing `/`, and docs pairing only triggers on an exact `<id>/edges` match.
- Schema: `in_degree: 0`, negative values and `{}` are refused. Giving both forms at once is allowed by the story.
- An unknown block in `hubs.blocks` gives the `view_unknown_block` warning.

**Judgement, not tested:** in `nested`, a hub inside a container stays there rather than going to a band. The commit and the design doc say so on purpose, but the story's Scope says "drawn once, apart (a band of hubs)". Severity note; not in the block below.

**6. Paths written outside the worktree.** The CLI test's `scratch()` creates these under the build host's cargo target tmpdir, relative to that target directory:
- `tmp/metaspec-cli/hub-prefix-manifest`
- `tmp/metaspec-cli/hub-prefix-state`
- `tmp/metaspec-cli/hub-prefix-pages`

**Failing tests left in the tree**
- `cases::hubs::hub_view_whose_document_has_no_hub_adds_no_second_identical_drawing` (metaspec-layout)
- `cases::manifest::hub_view_refuses_hubs_that_name_no_hub` (metaspec-core)
- `hub_edges_view_svg_ids_stay_apart_from_a_declared_view_of_the_same_prefix` (metaspec)

**7. Findings block**

```findings
[
{"file":"crates/metaspec-layout/src/lib.rs","line":204,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"views_of adds `<id>/edges` for every view with hubs even when the document has no hub, so the page draws the same drawing twice and the docs site switches between two identical views"},
{"file":"schema/metaspec-pack-1.schema.json","line":281,"category":"boundary","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"minProperties 1 accepts `hubs: {parts: []}` and `{blocks: []}`, which name no hub and still produce an `/edges` twin; parts and blocks need minItems 1"},
{"file":"crates/metaspec/src/page.rs","line":154,"category":"boundary","severity":"note","verdict":"INFEASIBLE","origin":"introduced","message":"the SVG prefix maps `work/edges` and a declared view `work-edges` both to `v-work-edges`, duplicating marker, title and desc ids on one page; no pack shown to declare such an id"},
{"file":"crates/metaspec-docs/src/documents.rs","line":314,"category":"judgement","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"with one badge the docs meta reads `1 more link go to 1 hub`"}
]
```
