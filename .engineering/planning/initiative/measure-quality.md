---
format: aep.planning-md/3
id: initiative:measure-quality
kind: initiative
status: draft
title: Measure the quality of specification packs and of their KPIs
summary: metaspec answers, with numbers and the same way for every pack, how good a pack's KPI catalogue is and how good its specification is.
revision: 5
---
# Measure the quality of specification packs and of their KPIs

## Outcome

For every pack metaspec knows (orgspec, agenticspec, cloudspec once it has blocks), and for
metaspec's own specification, metaspec answers two questions with numbers, the same way for
every pack:

1. **How good is a pack's KPI catalogue?** Complete (every block, rule and lifecycle has a
   measure), grounded (every KPI reads events the blocks declare), alive (a pack's tool reports
   a value for it), discriminating (its value differs between documents, and each gating KPI
   moves on the broken example of its rule), not redundant (no two KPIs carry the same signal).
2. **How good is a specification?** Valid, layered, complete in ESS's own terms (constructs
   without a scenario, unanswered questions), held by its conformance suite (passed, known
   failing, mutation score), honest (open `UNMAPPED:` markers), able to refuse (each rule has a
   broken example that fails it), and at which adoption level (L0 to L5) metaspec can use it.

The answers are KPIs themselves. metaspec carries them as its own KPI catalogue in the same
format the packs use, so the measures of quality are themselves measured ("KPIs for KPIs"), and
metaspec's own specification is the first pack they run on.

## Why

- Today each pack checks its catalogue with its own script (orgspec `scripts/check-kpis.sh`,
  agenticspec the same name, different rules). Nothing compares packs, and nothing says whether
  a KPI ever moves.
- The owner made metaspec the top-level application over all packs (2026-11-28): whatever it
  does for one pack it offers for every pack. Quality is the first feature that only exists at
  that level.
- `design/metaspec.md` lists the neutral formats metaspec already reads (`metaspec-pack/1`,
  `metaspec-findings/1`, the KPI catalogue with neutral `measure` and `modes`); the measures are
  computed from those and from `ess` outputs, never from a pack's own code.

## Additions decided in planning

Four parts of this initiative are not in the owner's ask above. The planning session added all
four on 2026-11-28 under the owner's standing rule "decide the reversible option, report it":

- **drift**, a measure of the specification question (`epic:spec-quality`,
  `story:spec-quality-drift`): generated files of a pack that differ from a fresh generation,
  read through a new adapter verb of the manifest. Recorded in
  `decision-blocker:pack-drift-source` (option A, over B and "C: drop drift").
- **Baseline and ratchet** (`story:quality-ratchet`, used by both epics): the first run sets the
  baseline, a worse value fails, no invented targets (`epic:kpi-quality`, Open). Where a private
  pack's baseline lives is recorded in `decision-blocker:private-pack-baseline` (option A).
- **A quality page per pack** (`story:kpi-quality-page` and `story:spec-quality-page`, the
  `--quality` flag of `render`): the reports of both questions rendered as `quality.html`.
  The Outcome asks for numbers; the page is how they are read.
- **`completeness.fields`**, a measure of the KPI question (`epic:kpi-quality`,
  `story:kpi-quality-catalogue-shape`): the share of KPIs with unit, direction, a direction per
  mode and a gate threshold. "Complete" above asks that every block, rule and lifecycle has a
  measure; the fields of a KPI row are a further promise.

All four are reversible: drift is one catalogue row, one story and one manifest verb; the ratchet
is one verb and its baseline files; the page is two stories, one flag of `render` and its
goldens; `completeness.fields` is one catalogue row and one measure function.

## Name

All names are `metaspec` since the repository was renamed: the CLI, the crates (`crates/metaspec-core`, `crates/metaspec`), the formats and schemas (`metaspec-quality/1`, `schema/metaspec-quality-1.schema.json`) and the user's state directory `$XDG_STATE_HOME/metaspec/...`.

## Out of scope

- Judging whether a KPI measures the right thing for the business. metaspec measures the
  catalogue's structure and behaviour; relevance stays with the pack's owner.
- Changing any pack to raise its numbers. A low number is a finding for the pack's owner.

## Children

- `epic:kpi-quality`
- `epic:spec-quality`

## Acceptance

`task acceptance -- <orgspec-dir> <agenticspec-dir>` exits 0, its rows showing `metaspec quality
kpis --json` and `metaspec quality spec --json` for harbor, orchard, orgspec and agenticspec, each
printing for every measure of its question a value or a gap naming the missing input
(`epic:kpi-quality` and `epic:spec-quality` carry the measure lists and their own acceptance).
