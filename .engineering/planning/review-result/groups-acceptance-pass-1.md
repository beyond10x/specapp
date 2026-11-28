---
format: aep.planning-md/3
id: review-result:groups-acceptance-pass-1
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 1, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
needs-revision

story:block-groups — item 4 says "three boxes", but the third is "a box titled with no value for the one part (stays a plain part: the group of one)", so a run can't tell whether to expect 3 boxes or 2 boxes plus a plain part — `.engineering/planning/story/block-groups.md:210`
story:block-groups — item 8 joins two independent outcomes (a rolled-up link to a hub is a badge, and a group box is a hub by `in_degree`) and names no `in_degree` value or graph for the second — `.engineering/planning/story/block-groups.md:221`
story:band-wrap — item 8 gives a count for ghosts ("5 names … are 3 rows") but "hubs likewise" has no setup or number, so that half cannot be checked — `.engineering/planning/story/band-wrap.md:162`
story:band-wrap — item 11 does not say which harbor document holds "a band of at least 7 boxes". It does not say where "the same view without `wrap`" comes from, since `fixtures/manifests/harbor.yaml` has no unwrapped twin of `wrapped`. It does not say what in the SVG counts as "separate bands". Those are the `.band-title` texts `<title> (k/m)` written at `crates/metaspec-layout/src/svg.rs:141-150` — `.engineering/planning/story/band-wrap.md:167`

**What I read:** 2 of 2 ids (story:block-groups, story:band-wrap). I ran `aep plan artifact show` on each and `aep plan artifact kinds`. I also read the tree: `fixtures/manifests/harbor.yaml`, the four harbor documents, `fixtures/expected/{harbor,orchard}`, `crates/metaspec-layout/tests/cases/{common,scenes}.rs`, `Taskfile.yml` `check:docs`, and `svg.rs` and `page.rs` for the `#view-` links.

**Checkable as written**
- block-groups: items 1–3, 5–7 and 9–13 each name a test or a fixture outcome and are checkable.
  - Items 3 and 5–12 have no command of their own and run under item 3's filter `test(/::group_/)` in `metaspec-layout`.
  - Items 12 and 13 name their fixtures. The golden helper in `common.rs` compares bytes, so item 12 can run in the layout crate.
  - Item 13's claims match the harbor documents: cove, lagoon, inlet and reef each have one vessel.
- band-wrap: items 1–7, 9 and 10 are checkable as written. Item 10 is a byte comparison; the orchard scenes goldens exist.

**What I could not establish**
- The drafter's `marina` content (5 vessels, 3 tugs) is not in the tree yet. Whether it gives one band of at least 7 boxes in `wrapped` depends on how `rows.rs` keys the `craft` family.
- Several items are one test with many assertions (block-groups 1, 2 and 5; band-wrap 3 and 5). I did not flag them. They run as one command, and the list of cases in each is checkable.
- Out of my lane, for the scope critic: block-groups' scope lists the `GraphView` counting (`web/ui/src/GraphView.tsx`, `pages.test.tsx`) and the scene-schema additions `members` and `by`. No acceptance line names a check for either. The schema check may be implicit in the scene schema validation in `scenes.rs`.

```findings
[
{"file":".engineering/planning/story/block-groups.md","line":210,"category":"acceptance","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"item 4 says the 5 parts make 'three boxes' but its third is 'a box titled with no value for the one part (stays a plain part: the group of one)', so the expected scene is stated two ways and cannot be checked; state whether the unvalued part is a box or a plain part"},
{"file":".engineering/planning/story/block-groups.md","line":221,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"item 8 joins two independent outcomes (a rolled-up link to a hub is a badge with count and links; a group box with in_degree links is a hub) and gives no in_degree value or graph for the second, so one can pass while the other fails"},
{"file":".engineering/planning/story/band-wrap.md","line":162,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"item 8 gives a count for ghosts ('5 names no part has with wrap: 2 are 3 rows') but 'hubs likewise' names no setup and no row count, so that half is not observable"},
{"file":".engineering/planning/story/band-wrap.md","line":167,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"item 11 names no document for 'a band of at least 7 boxes', gives no source for 'the same view without wrap' (harbor.yaml has no unwrapped twin of `wrapped`), and does not say what in the SVG counts as separate bands (the `<title> (k/m)` band-title texts)"}
]
```
