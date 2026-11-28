---
format: aep.planning-md/3
id: story:spec-quality-conformance
kind: story
status: draft
title: 'conformance: scenarios synthesized, passed, known failing and skipped on the interpreted target'
summary: The conformance suite of a pack's blocks run once per digest, with the known-failing list its manifest names.
relations:
- decomposes: epic:spec-quality
- depends_on: story:spec-quality-static
- depends_on: story:quality-ratchet
- depends_on: story:kpi-quality-values
scope:
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/conformance.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/conformance.rs
- confidence: cited
  path: fixtures/ess/harbor/run.json
- confidence: cited
  path: fixtures/ess/orchard/run.json
- confidence: cited
  path: fixtures/expected/harbor/quality/conformance.known_failing.json
- confidence: cited
  path: fixtures/expected/harbor/quality/conformance.passed.json
- confidence: cited
  path: fixtures/expected/harbor/quality/conformance.skipped.json
- confidence: cited
  path: fixtures/expected/harbor/quality/conformance.synthesized.json
- confidence: cited
  path: fixtures/expected/orchard/quality/conformance.known_failing.json
- confidence: cited
  path: fixtures/expected/orchard/quality/conformance.passed.json
- confidence: cited
  path: fixtures/expected/orchard/quality/conformance.skipped.json
- confidence: cited
  path: fixtures/expected/orchard/quality/conformance.synthesized.json
- confidence: cited
  path: packs/orgspec.yaml
- confidence: cited
  path: quality/baselines/agenticspec/conformance.known_failing.json
- confidence: cited
  path: quality/baselines/agenticspec/conformance.passed.json
- confidence: cited
  path: quality/baselines/agenticspec/conformance.skipped.json
- confidence: cited
  path: quality/baselines/agenticspec/conformance.synthesized.json
- confidence: cited
  path: quality/baselines/harbor/conformance.known_failing.json
- confidence: cited
  path: quality/baselines/harbor/conformance.passed.json
- confidence: cited
  path: quality/baselines/harbor/conformance.skipped.json
- confidence: cited
  path: quality/baselines/harbor/conformance.synthesized.json
- confidence: cited
  path: quality/baselines/orchard/conformance.known_failing.json
- confidence: cited
  path: quality/baselines/orchard/conformance.passed.json
- confidence: cited
  path: quality/baselines/orchard/conformance.skipped.json
- confidence: cited
  path: quality/baselines/orchard/conformance.synthesized.json
- confidence: cited
  path: quality/baselines/orgspec/conformance.known_failing.json
- confidence: cited
  path: quality/baselines/orgspec/conformance.passed.json
- confidence: cited
  path: quality/baselines/orgspec/conformance.skipped.json
- confidence: cited
  path: quality/baselines/orgspec/conformance.synthesized.json
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: scripts/acceptance.d/spec-quality-conformance.sh
- confidence: inferred
  path: taskfiles/slow.yml
revision: 11
---
# conformance: scenarios synthesized, passed, known failing and skipped on the interpreted target

## Context

A measure of `epic:spec-quality`: how far the conformance suite of a pack's blocks holds on the
interpreted reference target. orgspec, at its first public commit: 892 scenarios, 867 passed, 25
known failing, named in `blocks/known-failing.txt`.

## Scope

- `ess verify conform synthesize` and `ess verify conform run --target interpreted` on the blocks,
  with the known-failing list of the pack (`--known-failing`).
- An optional manifest field `blocks.known_failing` (a path inside the pack) in
  `metaspec-pack/1`; `packs/orgspec.yaml` names `blocks/known-failing.txt`. Without it no scenario
  is known failing.
- Four rows: `conformance.synthesized`, `.passed`, `.known_failing`, `.skipped`; behind: the
  failing and the skipped scenarios (kind `scenario`).
- Run once per digest of the pack's blocks, cached in the user's cache directory through the
  cache of `story:spec-quality-static` (called, not changed), never in the metaspec checkout. The
  time of one run on orgspec is measured and named in the report of this story. A run over 10
  minutes moves this measure into the `slow` lane: this story then adds `taskfiles/slow.yml` with
  task `conformance`, and `story:spec-quality-mutation`, which depends on this story, adds its
  task to that file.
- Blocks that do not validate, or a suite `ess` refuses: gaps naming the refusal.
- Recorded answers for the stub `ess`: `fixtures/ess/harbor/run.json`,
  `fixtures/ess/orchard/run.json` (new files; the stub is not changed).
- The baselines of the four rows, `quality/baselines/<pack>/conformance.*.json` where the reading
  is a value, written by the acceptance run's ratchet and committed here.

## Surfaces

- filled (stubs of `story:kpi-quality-catalogue-shape`):
  `crates/metaspec-core/src/quality/spec/conformance.rs`,
  `crates/metaspec-core/tests/cases/quality/conformance.rs`
- new: `fixtures/ess/harbor/run.json`, `fixtures/ess/orchard/run.json`,
  `fixtures/expected/{harbor,orchard}/quality/conformance.{synthesized,passed,known_failing,skipped}.json`,
  `quality/baselines/{harbor,orchard,orgspec,agenticspec}/conformance.{synthesized,passed,known_failing,skipped}.json`,
  `scripts/acceptance.d/spec-quality-conformance.sh`
- changed: `crates/metaspec-core/src/manifest.rs` and `schema/metaspec-pack-1.schema.json` (field
  `blocks.known_failing`), `packs/orgspec.yaml`

## Shared files

- `crates/metaspec-core/src/manifest.rs`, `schema/metaspec-pack-1.schema.json` and
  `packs/orgspec.yaml`: also changed by `story:kpi-quality-values` (field `expect`, the broken
  class), which this story depends on; the first two also by `story:spec-quality-drift` (verb
  `regenerate`), which depends on this story.
- `taskfiles/slow.yml`, only if the run is moved to the slow lane: also by
  `story:spec-quality-mutation`, which depends on this story.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- Measurement → Evidence (owns), a scenario named as `ess` writes it — inferable (`quality.yaml`,
  `metaspec.quality.Measurement`, relation `evidence`; `metaspec.quality.BehindKind` variant
  `scenario`).
- QualityReport → Pack — inferable (`quality.yaml`, `metaspec.quality.QualityReport`, relation
  `pack`).

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
`metaspec quality spec` for orgspec printing the synthesized, passed, known-failing and skipped
counts that `ess verify conform run --target interpreted` prints on its blocks with
`blocks/known-failing.txt`, and a value or a gap naming the missing input for each of the four
rows on harbor, orchard and agenticspec.
