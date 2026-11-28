---
format: aep.planning-md/3
id: epic:kpi-quality
kind: epic
status: draft
title: 'KPI quality: measures of a pack''s KPI catalogue'
summary: Coverage, grounding, aliveness, discrimination, gate fidelity and redundancy of every pack's KPIs, as KPIs of metaspec.
relations:
- decomposes: initiative:measure-quality
revision: 6
---
# KPI quality: measures of a pack's KPI catalogue

## Outcome

`metaspec quality kpis --pack <name> --pack-dir <dir>` reports, for every pack, the measures below
as numbers with the KPI ids behind each count, and a page per pack shows them (the page is an
addition, `initiative:measure-quality`, "Additions decided in planning"). The same measures run on
every pack through the manifest; a measure a pack cannot supply data for is reported as a gap of
that pack, never silently zero. Each value is held by the ratchet against a baseline: an
addition the planning session decided on 2026-11-28 under the owner's standing rule "decide the
reversible option", recorded in `initiative:measure-quality`, "Additions decided in planning"
(not part of the owner's ask).

## Measures (fixed; each is one row of metaspec's own catalogue, id in backticks)

- `coverage.blocks`: share of blocks with at least one KPI.
- `coverage.rules`: share of the rules the pack's tool reports that have a KPI counting them.
- `coverage.lifecycles`: share of entity lifecycles whose events some KPI reads.
- `grounding`: share of the KPIs with a non-empty `reads` whose `reads` name only events the
  compiled blocks declare; a KPI with an empty `reads` is left out of the share (generalises what
  each pack's own `check-kpis.sh` does).
- `completeness.fields`: share of KPIs with unit, direction, a direction per mode, and a gate
  threshold where `gate` is set (an addition, `initiative:measure-quality`, "Additions decided in
  planning"; the initiative's "complete" asks for coverage, not for the fields of a row).
- `alive`: share of KPIs the pack's tool reports a value for on at least one document.
- `discrimination`: share of the alive KPIs whose value differs across the pack's documents; an
  alive KPI with one value on every document is listed as flat. A KPI that is not alive is left
  out of the share (it is counted by `alive`).
- `gate_fidelity`: for each gating KPI with a broken example, the KPI moves on that example
  and on no valid example.
- `redundancy`: pairs of KPIs, neither of them flat, whose values agree on every document
  (candidates for merging); two flat KPIs agree trivially and are not counted.

These three definitions, narrowed from the first draft (all KPIs, all pairs), are the epic's
definitions; `story:kpi-quality-grounding` and `story:kpi-quality-values` build them as written
here.
- `planned`: KPIs that name a block the pack does not have yet.

## Acceptance

`task acceptance -- <orgspec-dir> <agenticspec-dir>` exits 0 with rows showing, for harbor,
orchard, orgspec and agenticspec, `metaspec quality kpis --json` printing a value or a gap naming
the missing input for each of the ten measures above (no other measure and none missing), every
value held by the ratchet against its committed `quality/baselines/<pack>/<measure>.json`, and a
`quality.html` naming all ten. The children that deliver it: `story:quality-catalogue`,
`story:kpi-quality-catalogue-shape`, `story:kpi-quality-grounding`, `story:kpi-quality-values`,
`story:kpi-quality-rules`, `story:kpi-quality-page`, `story:quality-ratchet` (which also keeps a
private pack's baselines under `$XDG_STATE_HOME/metaspec/quality/baselines/<pack>/`,
`decision-blocker:private-pack-baseline`, option A).

## Facts this rests on

- orgspec's `orgspec check --json` reports a `kpis` map per document (one count per rule) and a
  `measures` map; its 30 broken designs are one per rule (metaspec wave 2 acceptance: 30 of 30
  rule ids matched).
- agenticspec's catalogue has 125 rows, orgspec's 111 (wave 1 acceptance).
- The KPI catalogue's neutral fields are defined in `design/metaspec.md`, "Neutral formats",
  `metaspec-kpis/1`.

## Open

- Thresholds: which value of each measure is "good" is not known before the first run on both
  packs. The first run sets a baseline and a ratchet (a worse value fails); no absolute target
  is invented.
