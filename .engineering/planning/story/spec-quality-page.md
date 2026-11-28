---
format: aep.planning-md/3
id: story:spec-quality-page
kind: story
status: draft
title: The quality page shows the specification report beside the KPI report
summary: quality.html gets a section for the specification report.
relations:
- decomposes: epic:spec-quality
- depends_on: story:kpi-quality-page
- depends_on: story:spec-quality-static
scope:
- confidence: cited
  path: crates/metaspec/src/quality_page.rs
- confidence: cited
  path: crates/metaspec/tests/main.rs
- confidence: cited
  path: fixtures/expected/harbor/pages/quality.html
- confidence: cited
  path: fixtures/expected/orchard/pages/quality.html
- confidence: cited
  path: fixtures/quality-reports/harbor/spec.json
- confidence: cited
  path: fixtures/quality-reports/orchard/spec.json
- confidence: cited
  path: scripts/acceptance.d/spec-quality-page.sh
revision: 10
---
# The quality page shows the specification report beside the KPI report

## Context

`epic:spec-quality` asks for a page per pack. The quality page of `story:kpi-quality-page` shows
the KPI report; this story adds the specification report to the same page, so a pack has one
quality page with both questions.

## Scope

- `metaspec render ... --quality <report.json>` accepts a report of question `specification` too;
  `quality.html` has one section per question, each measure with its value or gap, unit,
  direction and the ids behind it.
- The golden pages of harbor and orchard are rendered from two recorded reports, the KPI report
  `fixtures/quality-reports/<pack>/kpis.json` of `story:kpi-quality-page` and a specification
  report `fixtures/quality-reports/<pack>/spec.json` committed here, never from the live output of
  `quality spec`: the measure stories of the specification question change neither.
- Acceptance row file `scripts/acceptance.d/spec-quality-page.sh` (a test case, not the
  acceptance): `task acceptance -- <orgspec-dir> <agenticspec-dir>` writes quality pages for
  orgspec and agenticspec that name every measure of both their reports.
- Not here: the table over all packs on a landing page. It is out of this initiative, deferred to
  the single metaspec site of the rename step (`epic:spec-quality`, "Not in this initiative").

## Surfaces

- new: `fixtures/quality-reports/harbor/spec.json`, `fixtures/quality-reports/orchard/spec.json`,
  `scripts/acceptance.d/spec-quality-page.sh`
- changed: `crates/metaspec/src/quality_page.rs`, `fixtures/expected/harbor/pages/quality.html`,
  `fixtures/expected/orchard/pages/quality.html`, `crates/metaspec/tests/main.rs`

## Shared files

- `crates/metaspec/src/quality_page.rs`, `crates/metaspec/tests/main.rs` and the two
  `quality.html` goldens: written by `story:kpi-quality-page`, which this story depends on.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- QualityReport → Pack, one report per pack and question — inferable (`quality.yaml`,
  `metaspec.quality.QualityReport`, relation `pack`, field `question`).
- Measurement → Measure and Measurement → Evidence (owns) — inferable (`quality.yaml`,
  `metaspec.quality.Measurement`, relations `measure`, `evidence`).

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

`task check` passes with its case comparing the quality page of harbor and orchard, rendered
from `fixtures/quality-reports/<pack>/kpis.json` and `fixtures/quality-reports/<pack>/spec.json`,
byte for byte with `fixtures/expected/<pack>/pages/quality.html`.
