---
format: aep.planning-md/3
id: review-result:groups-scope-pass-2
kind: review-result
status: active
title: aep:plan-critic-scope, pass 2, block-groups and band-wrap
relations:
- reviews: story:block-groups
- reviews: story:band-wrap
revision: 1
---
approve

Both pass-1 findings are resolved. The epic's Outcome now has the two bullets pass 1 asked for. The first is at `.engineering/planning/epic/cloudspec-pack.md:33-35` and names story:block-groups. The second is at `:36-37` and names story:band-wrap. I checked that those bullets, not the stories alone, make each story traceable to a sentence of the parent.

Promises read from the parent before the stories: 10 Outcome bullets, of which 9 are not "Later". Of those 9, I traced the 2 new bullets to story:block-groups and story:band-wrap, and the other 7 to stories outside this set, which this pass does not judge.

- **Pass-1 finding on story:block-groups: resolved.** Its outcome (one box per block, split by `by`, `<n> parts`, member ids in the title and `collapsed.members`, links rolled up, drill-down `<view>/<box id>`) is now the epic's bullet at `:33-35`.
- **Pass-1 finding on story:band-wrap: resolved.** `wrap: <n>` cutting a `rows` band into rows of at most n boxes is now the epic's bullet at `:36-37`.
- **G6 is claimed once.** story:block-groups takes parts of one block in no region, drawn one per box, split by a field, with a count.
- **G7 is claimed once.** story:block-groups takes it through the same mechanism, with member names listed. The epic's "member names" is met by member ids in the box title and in `collapsed.members`.
- **G8 is claimed once.** story:band-wrap takes it. Wrapping appears in story:block-groups only under "Not in this story", pointing to story:band-wrap.
- **No other story claims groups or wrap.** `git grep` over `.engineering/planning/story` for wrap and group found only unrelated uses. `badge-width.md:34` mentions badges wrapping inside `nested`, which is a different outcome. Both stories carry `decomposes` to the epic, and story:band-wrap `depends_on` story:block-groups.
- **Nothing drafted outside the gaps and bullets.** The supporting items all serve the two bullets:
  - the scene and pack schemas, `spec/domains/documents.yaml`, `design/metaspec.md`, `CHANGELOG.md` and the harbor fixtures and goldens;
  - the `pack check` codes `view_unknown_group_field`, `view_group_hub` and `view_wrap_nested`;
  - the page sentence and the `GraphView` counting text.
- **Exclusions respected.** The "Not in this story" lists (wrap by width, two-field split, `nested` wrapping) match what the epic does not ask for. They are named omissions, not silent narrowing.

What I read: 4 artifacts, the epic, both stories and `review-result:groups-scope-pass-1`. I ran `aep plan artifact show` on each, plus `aep plan artifact list` and `graph`, and `git grep` over the story directory.

What I could not establish:
- The consumer's gap list (G6 to G8) is not in the store, so I could not check its wording against its source. This is the same limit as pass 1.
- Out of my lane: whether the acceptances are checkable, the shared-file ordering with later stories, and the roll-up generalisation in `fold`.

```findings
[]
```
