---
format: aep.planning-md/3
id: story:kpi-quality-catalogue-shape
kind: story
status: implemented
title: metaspec quality kpis reports coverage.blocks, completeness.fields and planned for every pack
summary: The KPI command, the shared report format with gaps as readings, and the measures read from the catalogue and the block list.
relations:
- decomposes: epic:kpi-quality
- depends_on: story:quality-catalogue
scope:
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/metaspec-core/src/quality/check_pass.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/kpis/catalogue_measures.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/kpis/grounding.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/kpis/mod.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/kpis/rules.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/kpis/values.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/mod.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/ratchet.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/registry.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/report.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/conformance.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/drift.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/layering.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/mod.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/mutation.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/refusal.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/static_measures.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/conformance.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/drift.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/grounding.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/kpis.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/layering.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/mod.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/mutation.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/ratchet.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/refusal.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/rules.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/spec.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/values.rs
- confidence: cited
  path: crates/metaspec/src/main.rs
- confidence: cited
  path: crates/metaspec/src/quality/kpis.rs
- confidence: cited
  path: crates/metaspec/src/quality/mod.rs
- confidence: cited
  path: crates/metaspec/src/quality/ratchet.rs
- confidence: cited
  path: crates/metaspec/src/quality/spec.rs
- confidence: cited
  path: crates/metaspec/src/quality_page.rs
- confidence: cited
  path: crates/metaspec/tests/main.rs
- confidence: cited
  path: fixtures/expected/harbor/quality/completeness.fields.json
- confidence: cited
  path: fixtures/expected/harbor/quality/coverage.blocks.json
- confidence: cited
  path: fixtures/expected/harbor/quality/planned.json
- confidence: cited
  path: fixtures/expected/orchard/quality/completeness.fields.json
- confidence: cited
  path: fixtures/expected/orchard/quality/coverage.blocks.json
- confidence: cited
  path: fixtures/expected/orchard/quality/planned.json
- confidence: cited
  path: schema/metaspec-quality-1.schema.json
- confidence: cited
  path: scripts/acceptance.d/kpi-quality-catalogue-shape.sh
- confidence: cited
  path: scripts/acceptance.sh
revision: 53
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T10:35:33Z", actor: "human:operator", revision: 51, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "proposed", to: "active", at: "2026-11-28T10:35:33Z", actor: "human:operator", revision: 52, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "active", to: "implemented", at: "2026-11-28T11:37:23Z", actor: "human:operator", revision: 53, decided_on: {"recorded":{"test_result":1,"review_outcome":3}}}
---
# `metaspec quality kpis` reports coverage.blocks, completeness.fields and planned for every pack

## Context

The tracer bullet of the KPI question (`epic:kpi-quality`): the command, the report format both
questions share, gaps as readings, and the three measures that read only the KPI catalogue and
the block list. It also lays out every file of the initiative that more than one story would
otherwise change, so that each later story adds or fills files of its own only.

## Scope

- `metaspec quality kpis <NAME | --manifest FILE> --pack <DIR> [--json]`, in the CLI form every
  command of the repository takes (the epic writes `--pack <name> --pack-dir <dir>`). It executes
  nothing of the pack but `ess specify compile` (levels L0 and L1, `design/metaspec.md`).
- Report format `metaspec-quality/1` (named like the other neutral formats), JSON Schema
  `schema/metaspec-quality-1.schema.json`: pack, question (`kpi_catalogue` | `specification`),
  source digest, `measurements[{measure, reading, behind}]` where a reading is
  `{kind: value, value}` or `{kind: gap, gap}` and `behind` is `[{kind, name}]` as the pack writes
  the names, and `gaps[{kind, feature, needs, summary}]`. A gap reading is also a gap of kind
  `measure` with the measure id as its feature.
- coverage.blocks: blocks with at least one KPI, divided by blocks; behind: blocks without one.
- completeness.fields: KPIs with unit, direction, a direction per mode, and a threshold where the
  gate column sets a gate, divided by KPIs. The gate column as written: a number is a threshold;
  `true` is a gate without a threshold and counts as incomplete; `false` or absent is no gate.
  behind: the incomplete KPIs.
- planned: KPIs that name a block the pack does not have; behind: those KPIs.
- Field names through the manifest (`kpis.fields` for measure and modes), the shared column
  names otherwise. A pack without a `kpis` section: each measure is a gap.
- Unit and direction of each measure from metaspec's catalogue. The catalogue's `status` column
  is not changed here (`story:spec-quality-self` sets it).

### The layout every later story fills (decided here, once)

- CLI wiring, the only change of `crates/metaspec/src/main.rs` in the initiative: subcommand
  `quality` with verbs `kpis`, `spec` and `ratchet`, and the flag `--quality <report.json>`
  (repeatable) of `render`, which main passes to `quality_page::write` after render. Dispatch in
  `crates/metaspec/src/quality/mod.rs`; `quality/kpis.rs` is this story's; `quality/spec.rs`
  (filled by `story:spec-quality-static`), `quality/ratchet.rs` (`story:quality-ratchet`) and
  `crates/metaspec/src/quality_page.rs` (`story:kpi-quality-page`) are stubs that exit 2 with
  "not built yet".
- Core modules: `crates/metaspec-core/src/quality/mod.rs` declares `report`, `registry`, `kpis`,
  `spec`, `check_pass` and `ratchet`. `quality/registry.rs` maps every measure id of metaspec's
  catalogue to a function in the module file of the story that builds it:
  `kpis/catalogue_measures.rs` (this story), `kpis/grounding.rs`, `kpis/values.rs`,
  `kpis/rules.rs`, `spec/static_measures.rs`, `spec/layering.rs`, `spec/conformance.rs`,
  `spec/mutation.rs`, `spec/refusal.rs`, `spec/drift.rs`. Each file not built here is a stub whose
  measures return a gap reading "not built yet"; `check_pass.rs` and `ratchet.rs` are stubs too.
  `kpis/mod.rs` and `spec/mod.rs` only declare those files and are not changed again. A story
  that needs a helper adds it as a child module of its own file.
- Test modules: `crates/metaspec-core/tests/cases/quality/mod.rs` gains one module per later
  story, each an empty file the story fills: `kpis`, `grounding`, `values`, `rules`, `spec`,
  `layering`, `conformance`, `mutation`, `refusal`, `drift`, `ratchet`. Filter names are
  `quality::<module>`.
- Acceptance rows: `scripts/acceptance.sh` sources every `scripts/acceptance.d/*.sh` in name
  order after its wave rows; each story adds one file named after it. Reports land under
  `target/acceptance/quality/`. `story:quality-ratchet`'s file is named
  `scripts/acceptance.d/zz-quality-ratchet.sh` so it runs last, over every report.
- Goldens: one file per pack and measure, `fixtures/expected/<pack>/quality/<measure>.json`, the
  measurement of that measure; a story writes only those of its measures.
- Taskfile: `Taskfile.yml` gains optional includes `slow: ./taskfiles/slow.yml` and
  `quality: ./taskfiles/quality.yml` (go-task `optional: true`; 3.54.0 on the host). The files
  are added by `story:spec-quality-mutation` and `story:spec-quality-self`.
- Registry check: a catalogue with a row of status `measured` whose registry entry is a stub or
  absent is refused, naming the row; a measure the registry maps with no row in the catalogue is
  refused too.
- Test cases of the layout, each named `layout_*` so one filter runs them in both crates:
  - `crates/metaspec-core/tests/cases/quality/kpis.rs`: the registry maps each of the 40 ids of
    metaspec's catalogue to a function or a stub, and each stub measure returns the gap reading
    "not built yet"; a catalogue (written by the test) with a `measured` row whose registry entry
    is a stub is refused, naming the row; a registry measure with no catalogue row is refused;
  - `crates/metaspec/tests/main.rs`: `metaspec render ... --quality <report.json>` reaches the
    stub `quality_page::write` and exits 2 with "not built yet", and `metaspec quality spec` and
    `metaspec quality ratchet` reach their stubs the same way.
- Test cases of the three measures, each named `golden_*` (`golden_coverage_blocks`,
  `golden_completeness_fields`, `golden_planned`) so that the filter
  `test(layout_) | test(golden_)` runs them with the layout cases in one command, the
  acceptance: the goldens
  `fixtures/expected/{harbor,orchard}/quality/{coverage.blocks,completeness.fields,planned}.json`
  in `quality::kpis`. Beside them, not the acceptance, the acceptance row file
  `scripts/acceptance.d/kpi-quality-catalogue-shape.sh`, whose rows show
  `metaspec quality kpis --json` for harbor, orchard, orgspec and agenticspec valid against
  `schema/metaspec-quality-1.schema.json`, coverage.blocks, completeness.fields and planned as
  values, and under planned exactly the KPIs whose block is among the planned blocks
  `metaspec pack check` counts for that pack.

## Surfaces

- new: `crates/metaspec-core/src/quality/report.rs`, `crates/metaspec-core/src/quality/registry.rs`,
  `crates/metaspec-core/src/quality/kpis/mod.rs`,
  `crates/metaspec-core/src/quality/kpis/catalogue_measures.rs`, the stub files
  `crates/metaspec-core/src/quality/{check_pass,ratchet}.rs`,
  `crates/metaspec-core/src/quality/kpis/{grounding,values,rules}.rs`,
  `crates/metaspec-core/src/quality/spec/mod.rs`,
  `crates/metaspec-core/src/quality/spec/{static_measures,layering,conformance,mutation,refusal,drift}.rs`,
  `crates/metaspec/src/quality/{mod,kpis,spec,ratchet}.rs`, `crates/metaspec/src/quality_page.rs`,
  `schema/metaspec-quality-1.schema.json`,
  `fixtures/expected/{harbor,orchard}/quality/{coverage.blocks,completeness.fields,planned}.json`,
  `crates/metaspec-core/tests/cases/quality/kpis.rs` and the empty test modules named above,
  `scripts/acceptance.d/kpi-quality-catalogue-shape.sh`
- changed: `crates/metaspec/src/main.rs`, `crates/metaspec-core/src/quality/mod.rs`,
  `crates/metaspec-core/tests/cases/quality/mod.rs`, `scripts/acceptance.sh` (sources the row
  files), `Taskfile.yml` (two optional includes), `crates/metaspec/tests/main.rs` (the `layout_*`
  CLI cases)

## Shared files

- `crates/metaspec-core/src/quality/mod.rs`, `crates/metaspec-core/tests/cases/quality/mod.rs`:
  created by `story:quality-catalogue`, which this story depends on.
- Each stub file is filled by exactly one later story, and every later story depends on this one
  (directly, or through `story:spec-quality-static` or `story:kpi-quality-values`).
- `scripts/acceptance.sh`: also changed by `story:kpi-quality-values` (drops the manifest the
  script derives with `sed`), which depends on this story.
- `crates/metaspec/tests/main.rs`: this story adds the `layout_*` CLI cases; also changed by
  `story:kpi-quality-page` and `story:spec-quality-page`, which depend on this story.
- `Taskfile.yml`: also changed by `story:spec-quality-self` (`task check`, only if
  `quality:self` is timed within the budget), which depends on this story.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- Kpi → Block, many-to-one, zero for a planned block — inferable (`blocks.yaml`,
  `metaspec.blocks.Kpi`, relation `block`, via Optional `block_id`).
- Block → Pack, many-to-one — inferable (`blocks.yaml`, `metaspec.blocks.Block`, relation `pack`).
- QualityReport → Pack, many-to-one — inferable (`quality.yaml`, `metaspec.quality.QualityReport`,
  relation `pack`).
- QualityReport → Measurement, one-to-many, report owns — inferable (`quality.yaml`,
  `metaspec.quality.QualityReport`, relation `measurements`, owns).
- Measurement → Measure, many-to-one — inferable (`quality.yaml`, `metaspec.quality.Measurement`,
  relation `measure`).
- Measurement → Evidence, one-to-many, measurement owns; each evidence names its id as the pack
  writes it — inferable (`quality.yaml`, `metaspec.quality.Measurement`, relation `evidence`;
  `metaspec.quality.Evidence` fields `kind`, `name`).
- Pack → Gap, one-to-many, pack owns, one gap per pack and feature — inferable (`packs.yaml`,
  `metaspec.packs.Pack`, relation `gaps`; `metaspec.packs.Gap` fields `kind`, `feature`).

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

`cargo nextest run -p metaspec-core -p metaspec -E 'test(layout_) | test(golden_)'` passes; its
`layout_` cases show the layout every later story fills in place (each of the 40 ids of metaspec's
catalogue reaches a function or a stub that returns the gap reading "not built yet",
`metaspec render ... --quality <report.json>` reaches the stub `quality_page::write` and exits 2
with "not built yet", a catalogue with a `measured` row whose registry entry is a stub is refused,
naming the row) and its `golden_` cases match the goldens of `coverage.blocks`,
`completeness.fields` and `planned` on harbor and orchard.
