---
format: aep.planning-md/3
id: story:spec-quality-self
kind: story
status: draft
title: metaspec measures its own spec/ with both questions
summary: 'packs/metaspec.yaml and task quality:self: both reports on metaspec''s own spec and catalogue, held by the ratchet.'
relations:
- decomposes: epic:spec-quality
- depends_on: story:spec-quality-static
- depends_on: story:kpi-quality-catalogue-shape
- depends_on: story:quality-ratchet
- depends_on: story:kpi-quality-grounding
- depends_on: story:kpi-quality-values
- depends_on: story:kpi-quality-rules
- depends_on: story:spec-quality-conformance
- depends_on: story:spec-quality-layering
- depends_on: story:spec-quality-mutation
- depends_on: story:spec-quality-refusal
- depends_on: story:spec-quality-drift
scope:
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/metaspec-core/build.rs
- confidence: cited
  path: crates/metaspec-core/src/known.rs
- confidence: cited
  path: kpis/catalogue.yaml
- confidence: cited
  path: packs/metaspec.yaml
- confidence: cited
  path: quality/baselines/metaspec
- confidence: inferred
  path: scripts/check-vocabulary.sh
- confidence: cited
  path: taskfiles/quality.yml
revision: 9
---
# metaspec measures its own spec/ with both questions

## Context

`initiative:measure-quality`: metaspec's own specification is the first pack the measures run
on, and its catalogue of quality KPIs is measured like any other ("KPIs for KPIs"). metaspec is a
pack with a manifest and a KPI catalogue of its own (ESS `metaspec.quality.Measure`). Its
specification, the ESS meta model in `spec/`, is written in another worktree and is not committed
yet; this story starts when it is in the tree.

It runs after every measure story of both questions (`depends_on` each: grounding, values,
rules, conformance, layering, mutation, refusal, drift, besides static, catalogue-shape and the
ratchet), because its acceptance reads a reading for every row of `kpis/catalogue.yaml`, and each
row's measure is built by one of them.

## Scope

- `packs/metaspec.yaml`, a held manifest like `packs/orgspec.yaml`: `blocks.path: spec`,
  `kpis.path: kpis/catalogue.yaml`, no document class and no tool (level L1); embedded in the
  binary and listed by `metaspec pack known`.
- `task quality:self` (task `self` in `taskfiles/quality.yml`, included by `Taskfile.yml` as
  `quality` since `story:kpi-quality-catalogue-shape`) runs
  `metaspec quality kpis metaspec --pack .` and `metaspec quality spec metaspec --pack .`, then
  the ratchet against `quality/baselines/metaspec/<measure>.json`, which its first run writes and
  which are committed.
- What metaspec cannot supply for itself (no documents, no tool: alive, discrimination, refusal
  power and the like) is a gap of the pack metaspec, listed like any pack's.
- `kpis/catalogue.yaml`: sets `status: measured` on every row whose measure the registry maps to a
  built function (no stub), which after the measure stories is every row. This is the only change
  of the catalogue after `story:quality-catalogue`, and the status column only; the `block` column
  is not touched (block `quality` stops being planned because `spec/` is in the tree, not by an
  edit). The registry check of `story:kpi-quality-catalogue-shape` holds the result.
- `task quality:self` joins `task check` only when it is timed within the step budget (then this
  story changes the `check` task of `Taskfile.yml`); otherwise it stays a task of its own and the
  report says so with the time.
- The vocabulary check does not take metaspec's own block names as pack words: they are the nouns
  of metaspec's code.

## Surfaces

- new: `packs/metaspec.yaml`, `taskfiles/quality.yml`, `quality/baselines/metaspec/`
- changed: `crates/metaspec-core/build.rs` and `crates/metaspec-core/src/known.rs` (a third held
  manifest), `scripts/check-vocabulary.sh` if it reads held manifests, `kpis/catalogue.yaml`
  (status column), `Taskfile.yml` (only if `quality:self` joins `task check`)
- reads, not written here: `spec/` (the ESS meta model)

## Shared files

- `kpis/catalogue.yaml`, `crates/metaspec-core/build.rs`: written by `story:quality-catalogue`,
  which this story depends on transitively.
- `Taskfile.yml`: written by `story:kpi-quality-catalogue-shape` (the includes), which this story
  depends on.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- Measure → Kpi, one-to-one, required: metaspec is a pack whose catalogue holds the measures —
  inferable (`quality.yaml`, `metaspec.quality.Measure`, relation `kpi`, via required `kpi_id`).
- Baseline → Pack and Baseline → Measure, one per measure and pack; metaspec is a public pack, so
  its baselines are committed in metaspec — inferable (`quality.yaml`,
  `metaspec.quality.Baseline`, relations `pack`, `measure`; `decision-blocker:private-pack-baseline`).
- Pack → Gap, one per pack and feature — inferable (`packs.yaml`, `metaspec.packs.Pack`, relation
  `gaps`). metaspec's manifest is held in `packs/` — inferable (`packs.yaml`, type
  `metaspec.packs.ManifestSource`, variant `held`).

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

`task quality:self` exits 0, printing for every row of `kpis/catalogue.yaml`, each of status
`measured`, a value or a listed gap from `metaspec quality kpis metaspec --pack .` or
`metaspec quality spec metaspec --pack .`, held by the ratchet to the committed
`quality/baselines/metaspec/`.
