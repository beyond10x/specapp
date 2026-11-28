---
format: aep.planning-md/3
id: story:quality-catalogue
kind: story
status: implemented
title: metaspec carries its own catalogue of quality KPIs, in the format the packs use
summary: One row per quality measure of both epics, with unit and direction; the runner reads the measures from it.
relations:
- decomposes: epic:kpi-quality
scope:
- confidence: cited
  path: crates/metaspec-core/build.rs
- confidence: cited
  path: crates/metaspec-core/src/lib.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/catalogue.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/mod.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/mod.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/catalogue.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/mod.rs
- confidence: cited
  path: kpis/catalogue.yaml
revision: 18
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T10:35:33Z", actor: "human:operator", revision: 16, decided_on: {"recorded":{"review_outcome":5}}}
- {from: "proposed", to: "active", at: "2026-11-28T10:35:33Z", actor: "human:operator", revision: 17, decided_on: {"recorded":{"review_outcome":5}}}
- {from: "active", to: "implemented", at: "2026-11-28T10:56:20Z", actor: "human:operator", revision: 18, decided_on: {"recorded":{"test_result":1,"review_outcome":6}}}
---
# metaspec carries its own catalogue of quality KPIs

## Context

`initiative:measure-quality`: the answers are KPIs themselves; metaspec carries them as its own KPI
catalogue in the same format the packs use. Every measure of `epic:kpi-quality` and
`epic:spec-quality` is one row. The quality runner reads the unit and direction of each measure
from this catalogue: a measure is data, not code (ESS `metaspec.quality` summary).

Repository: metaspec. Builds on branch `invert-pack-knowledge`
(manifest `metaspec-pack/1`, held manifests in `packs/` embedded by `crates/metaspec-core/build.rs`),
not yet on `main`.

## Scope

- `kpis/catalogue.yaml` at the root of metaspec, format `metaspec-kpis/1`, rows in a list `kpis`
  with the columns the packs share (`design/metaspec.md`, "What the packs share"): `id`, `block`, a
  measure kind, `unit`, `summary`, `how`, `direction`, a direction per mode (may be empty),
  `gate`, `reads`, `status`.
- One row per measure id the two epics fix under "Measures", and no other. KPI question (10):
  `coverage.blocks`, `coverage.rules`, `coverage.lifecycles`, `grounding`,
  `completeness.fields`, `alive`, `discrimination`, `gate_fidelity`, `redundancy`, `planned`.
  Specification question (30): `validity.refusals`, `validity.warnings`, `layering`,
  `ess_completeness.unscenarioed`, `ess_completeness.unanswered`, `ess_completeness.outside`,
  `conformance.synthesized`, `conformance.passed`, `conformance.known_failing`,
  `conformance.skipped`, `mutation.score`, one `mutation.<class>` row for each of the 14 classes
  `epic:spec-quality` names, `honesty`, `refusal_power`, `document_health`, `adoption_level`,
  `drift`. The `drift` row is in from the start (`decision-blocker:pack-drift-source` is cleared
  with option A) and is measured by `story:spec-quality-drift`.
- `direction` in metaspec words, `higher_is_better` or `lower_is_better` (ESS type
  `metaspec.quality.Direction`: "metaspec's own words; a pack's catalogue keeps its own words").
  Lower is better for `validity.*`, `layering`, `ess_completeness.*`, `conformance.known_failing`,
  `conformance.skipped`, `honesty`, `redundancy`, `planned` and `drift`; higher for the rest.
- `gate` absent on every row: no threshold is invented (`epic:kpi-quality`, Open).
- `status` is `planned` on every row at this story's landing. A row becomes `measured` only
  through `story:spec-quality-self`, the one later story that writes this file: it sets
  `measured` on every row whose measure is built, after every measure story has landed. No
  measure story changes `kpis/catalogue.yaml`. No test case of this story reads the `status`
  column, so none of them breaks when `story:spec-quality-self` changes it.
- `block` names the block of metaspec's own specification a measure belongs to (`quality`, the
  domain `metaspec.quality`), final from this story on. Whether that block is planned is not
  written in the catalogue: it follows from the pack's blocks (a planned block is one the pack
  does not have), so it stops being planned when `spec/` is in the tree, with no edit of the
  `block` column by any later story.
- A loader, the catalogue embedded in the binary the way `packs/` manifests are.
- Test cases (in `crates/metaspec-core/tests/cases/quality/catalogue.rs`): the committed catalogue
  holds exactly the 40 ids above, once each (reads the `id` column only); the loader refuses a
  duplicate id, a row without unit, a row without direction, and a direction outside the two
  words. The case that refuses a `measured` row with no built measure is not here: it reads the
  runner's registry and belongs to `story:kpi-quality-catalogue-shape`, which creates it.
- The test module tree of the initiative starts here: `tests/cases/mod.rs` gains `mod quality;`,
  `tests/cases/quality/mod.rs` declares `catalogue`; `story:kpi-quality-catalogue-shape` adds the
  module of every later story to it.

## Surfaces

- new: `kpis/catalogue.yaml`, `crates/metaspec-core/src/quality/mod.rs`,
  `crates/metaspec-core/src/quality/catalogue.rs`,
  `crates/metaspec-core/tests/cases/quality/mod.rs`,
  `crates/metaspec-core/tests/cases/quality/catalogue.rs`
- one line each: `crates/metaspec-core/src/lib.rs`, `crates/metaspec-core/build.rs`,
  `crates/metaspec-core/tests/cases/mod.rs`

## Shared files

- `kpis/catalogue.yaml`: also changed by `story:spec-quality-self` (status column only), which
  depends on this story through every measure story.
- `crates/metaspec-core/src/quality/mod.rs` and `crates/metaspec-core/tests/cases/quality/mod.rs`:
  also changed by `story:kpi-quality-catalogue-shape` (module declarations), which depends on this
  story.
- `crates/metaspec-core/build.rs`: also changed by `story:spec-quality-self` (a third held
  manifest), which depends on this story transitively.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- Measure → Kpi, one-to-one, required; the measure is a row of metaspec's catalogue —
  inferable (`quality.yaml`, entity `metaspec.quality.Measure`, relation `kpi`, references,
  cardinality one, via the required field `kpi_id`: "metaspec is a pack with a manifest and a KPI
  catalogue of its own").
- Kpi → Block, many-to-one, zero for a planned block — inferable (`blocks.yaml`,
  `metaspec.blocks.Kpi`, relation `block`, via Optional `block_id`).
- Kpi → Pack, many-to-one — inferable (`blocks.yaml`, `metaspec.blocks.Kpi`, relation `pack`).

## Constraints

Tests: a test is killed at 60 s, the test step ends within 120 s; a slot job ends within 10
minutes. No Python. No word of a pack under `crates/` (`scripts/check-vocabulary.sh`).

## Acceptance

At this story's landing, `cargo nextest run -p metaspec-core quality::catalogue` passes on the
committed `kpis/catalogue.yaml`, its id case showing exactly one row for each of the 40 measure
ids of the two epics (10 of the KPI question, 30 of the specification question, `drift` among
them); the case reads the `id` column only, so it still passes after `story:spec-quality-self`
sets `status: measured`.
