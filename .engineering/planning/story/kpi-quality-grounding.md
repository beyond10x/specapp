---
format: aep.planning-md/3
id: story:kpi-quality-grounding
kind: story
status: draft
title: grounding and coverage.lifecycles from the events of the compiled blocks
summary: KPIs that read only declared events, and lifecycles whose events some KPI reads.
relations:
- decomposes: epic:kpi-quality
- depends_on: story:kpi-quality-catalogue-shape
- depends_on: story:quality-ratchet
scope:
- confidence: cited
  path: crates/metaspec-core/src/quality/kpis/grounding
- confidence: cited
  path: crates/metaspec-core/src/quality/kpis/grounding.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/grounding.rs
- confidence: cited
  path: fixtures/expected/harbor/quality/coverage.lifecycles.json
- confidence: cited
  path: fixtures/expected/harbor/quality/grounding.json
- confidence: cited
  path: fixtures/expected/orchard/quality/coverage.lifecycles.json
- confidence: cited
  path: fixtures/expected/orchard/quality/grounding.json
- confidence: cited
  path: fixtures/packs/harbor/kpis/catalogue.yaml
- confidence: cited
  path: fixtures/packs/orchard/kpis/catalogue.yaml
- confidence: cited
  path: quality/baselines/agenticspec/coverage.lifecycles.json
- confidence: cited
  path: quality/baselines/agenticspec/grounding.json
- confidence: cited
  path: quality/baselines/harbor/coverage.lifecycles.json
- confidence: cited
  path: quality/baselines/harbor/grounding.json
- confidence: cited
  path: quality/baselines/orchard/coverage.lifecycles.json
- confidence: cited
  path: quality/baselines/orchard/grounding.json
- confidence: cited
  path: quality/baselines/orgspec/coverage.lifecycles.json
- confidence: cited
  path: quality/baselines/orgspec/grounding.json
- confidence: cited
  path: scripts/acceptance.d/kpi-quality-grounding.sh
revision: 9
---
# grounding and coverage.lifecycles from the events of the compiled blocks

## Context

Two measures of `epic:kpi-quality` that compare the KPI catalogue with the events and lifecycles
of the compiled blocks. grounding generalises what each pack's own `scripts/check-kpis.sh` does
(orgspec rule 6: every event in `reads` is declared by a block).

## Scope

- Read the events and the entity lifecycles of the pack from `ess specify compile --format json`
  (the call pack loading already makes), in a child module of this story's file
  (`quality/kpis/grounding/model.rs`).
- grounding: KPIs whose `reads` name only events the compiled blocks declare, divided by the KPIs
  with a non-empty `reads`; a KPI with an empty `reads` is left out of the share. behind: the
  KPIs that name an undeclared event, and each such event (kind `event`).
- coverage.lifecycles: entity lifecycles whose events some KPI reads, divided by lifecycles. The
  events of a lifecycle are those emitted by the outcomes that create or update its entity.
  behind: lifecycles no KPI reads (kind `lifecycle`).
- A pack whose catalogue has no `reads` column on any row: both are gaps naming that column.
- The fixture catalogues (`fixtures/packs/harbor/kpis/catalogue.yaml`,
  `fixtures/packs/orchard/kpis/catalogue.yaml`) gain `reads` on some rows, naming events their
  blocks declare, and one harbor row names an event no block declares. No other story changes
  these two files: `story:kpi-quality-values` and `story:kpi-quality-rules` change harbor's tool
  answers and broken plans, not its catalogue. `pack check` and `render` do not read `reads`, so
  their goldens do not change (inferred from `crates/metaspec-core/src/pack.rs`, which keeps the
  rows as they are).
- The baselines of the two measures, `quality/baselines/<pack>/{grounding,coverage.lifecycles}.json`
  for the four packs where the reading is a value, written by the acceptance run's ratchet and
  committed here.

## Surfaces

- new: `crates/metaspec-core/src/quality/kpis/grounding/model.rs`,
  `fixtures/expected/{harbor,orchard}/quality/{grounding,coverage.lifecycles}.json`,
  `quality/baselines/{harbor,orchard,orgspec,agenticspec}/{grounding,coverage.lifecycles}.json`,
  `scripts/acceptance.d/kpi-quality-grounding.sh`
- filled (stubs of `story:kpi-quality-catalogue-shape`):
  `crates/metaspec-core/src/quality/kpis/grounding.rs`,
  `crates/metaspec-core/tests/cases/quality/grounding.rs`
- changed: `fixtures/packs/harbor/kpis/catalogue.yaml`, `fixtures/packs/orchard/kpis/catalogue.yaml`

## Shared files

- None with another measure story. The stub files come from `story:kpi-quality-catalogue-shape`
  and the baseline format from `story:quality-ratchet`; this story depends on both.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- Kpi → Event, many-to-many, by name as the catalogue writes it — inferable (inferred from orgspec
  `scripts/check-kpis.sh:14`, rule 6, which matches `reads` to the `events:` of the blocks; no
  ess/1 document declares this relation: `blocks.yaml` `metaspec.blocks.Kpi` holds `reads` as a
  list of names, with no `relations:` entry).
- Entity lifecycle → Event, one-to-many, through the outcomes that create or update the entity
  and emit the event — inferable (inferred from the ess/1 outcome form, e.g. `quality.yaml`
  command `metaspec.quality.Ratchet`, outcome `taken`: `updates: metaspec.quality.Baseline`,
  `emits: [metaspec.quality.BaselineMoved]`; no `relations:` entry declares it).
- Kpi → Block, zero for a planned block — inferable (`blocks.yaml`, `metaspec.blocks.Kpi`,
  relation `block`).

## Constraints

- No feature for one pack only: every measure runs through the manifest for every pack; what a
  pack cannot supply is a gap reading naming the missing input, never a silent zero.
- Tests run on the fixture packs harbor and orchard with recorded tool and `ess` answers: no
  `ess` synthesis, no real pack and no network per test. A test is killed at 60 s, the test step
  ends within 120 s; a slot job ends within 10 minutes (`slot --timeout`, at most 30m). A test
  over 5 s is named with its time in the report.
- No Python. No word of a pack under `crates/` (`scripts/check-vocabulary.sh`).
- Builds on branch `invert-pack-knowledge` (manifest `metaspec-pack/1`, the check adapter in
  `crates/metaspec-core/src/adapter.rs`, held manifests in `packs/`, CLI form
  `<NAME | --manifest FILE> --pack <DIR>`, `task acceptance -- <orgspec-dir> <agenticspec-dir>`
  with two arguments), not yet on `main`. Commands and crate paths are written as in the renamed repository.
- The layout of the initiative (`story:kpi-quality-catalogue-shape`): one module file, one test
  module, one acceptance row file `scripts/acceptance.d/<story>.sh`, one golden per pack and
  measure `fixtures/expected/<pack>/quality/<measure>.json` and one baseline per pack and measure
  `quality/baselines/<pack>/<measure>.json` belong to one story each, unless a body names the
  one later story that regenerates it.

## Acceptance

`task acceptance -- <orgspec-dir> <agenticspec-dir>` exits 0 with rows showing
`metaspec quality kpis` printing grounding and coverage.lifecycles as values for harbor, orchard,
orgspec and agenticspec, with grounding 1 for orgspec, whose own `scripts/check-kpis.sh` refuses
an event no block declares, and below 1 for harbor, naming its undeclared event.
