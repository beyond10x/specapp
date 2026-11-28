---
format: aep.planning-md/3
id: story:kpi-quality-page
kind: story
status: active
title: A quality page per pack shows the KPI report
summary: metaspec render writes quality.html from a KPI report.
relations:
- decomposes: epic:kpi-quality
- depends_on: story:kpi-quality-catalogue-shape
scope:
- confidence: cited
  path: crates/metaspec/src/page.rs
- confidence: cited
  path: crates/metaspec/src/quality_page.rs
- confidence: cited
  path: crates/metaspec/src/render.rs
- confidence: cited
  path: crates/metaspec/tests/main.rs
- confidence: cited
  path: fixtures/expected/harbor/pages/quality.html
- confidence: cited
  path: fixtures/expected/orchard/pages/quality.html
- confidence: cited
  path: fixtures/quality-reports/harbor/kpis.json
- confidence: cited
  path: fixtures/quality-reports/orchard/kpis.json
- confidence: cited
  path: scripts/acceptance.d/kpi-quality-page.sh
revision: 14
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T10:35:33Z", actor: "human:operator", revision: 13, decided_on: {"recorded":{"review_outcome":3}}}
- {from: "proposed", to: "active", at: "2026-11-28T10:35:33Z", actor: "human:operator", revision: 14, decided_on: {"recorded":{"review_outcome":3}}}
---
# A quality page per pack shows the KPI report

## Context

`epic:kpi-quality` asks for a page per pack beside the command. The docs site (wave 3, `docs
build`) does not exist yet; the pages metaspec writes today are those of `metaspec render`. The
quality page joins them and moves into the docs site with the other pages when it is built.

## Scope

- `metaspec render ... --quality <report.json>` (repeatable; the flag is wired in `main.rs` by
  `story:kpi-quality-catalogue-shape`) writes `quality.html` beside `index.html`: each measure
  with its value or gap, unit, direction, and the ids behind it. The report is a file
  `metaspec quality kpis --json` wrote; render runs no measure itself. This story fills the stub
  `crates/metaspec/src/quality_page.rs`.
- The index links `quality.html` only when a report is given, so every existing index golden
  stays as it is; the link is checked by a test assertion, not by a golden.
- The page rules of `render` hold: one file, CSS inline, no script that runs, no request, the data
  as JSON in a `<script type="application/json">` element.
- Every measure the report holds is shown, so later measures need no change of the page.
- The golden pages read a recorded report committed by this story,
  `fixtures/quality-reports/<pack>/kpis.json` for harbor and orchard, never the live output of
  `quality kpis`: a later measure story changes neither the recorded report nor the golden.
- Acceptance row file `scripts/acceptance.d/kpi-quality-page.sh` (a test case, not the
  acceptance): `task acceptance -- <orgspec-dir> <agenticspec-dir>` writes a quality page for
  orgspec and agenticspec that names every measure of their KPI report.

## Surfaces

- new: `fixtures/quality-reports/harbor/kpis.json`, `fixtures/quality-reports/orchard/kpis.json`,
  `fixtures/expected/harbor/pages/quality.html`, `fixtures/expected/orchard/pages/quality.html`,
  `scripts/acceptance.d/kpi-quality-page.sh`
- filled: `crates/metaspec/src/quality_page.rs` (stub of `story:kpi-quality-catalogue-shape`)
- changed: `crates/metaspec/src/render.rs`, `crates/metaspec/src/page.rs` (the link from the index
  when a report is given), `crates/metaspec/tests/main.rs`

## Shared files

- `crates/metaspec/src/quality_page.rs`, `crates/metaspec/tests/main.rs`,
  `fixtures/expected/{harbor,orchard}/pages/quality.html`: also changed by
  `story:spec-quality-page`, which depends on this story.
- `crates/metaspec/src/quality_page.rs`: created as a stub by `story:kpi-quality-catalogue-shape`,
  which this story depends on.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- QualityReport → Pack, Measurement → Measure, Measurement → Evidence (owns) — inferable
  (`quality.yaml`, relations `metaspec.quality.QualityReport.pack`,
  `metaspec.quality.Measurement.measure`, `metaspec.quality.Measurement.evidence`).

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

`task check` passes with its case comparing the `quality.html` of harbor and orchard, rendered
from `fixtures/quality-reports/<pack>/kpis.json`, byte for byte with
`fixtures/expected/<pack>/pages/quality.html`.
