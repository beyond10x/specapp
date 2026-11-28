---
format: aep.planning-md/3
id: story:quality-ratchet
kind: story
status: draft
title: 'Baseline and ratchet: the first run sets a baseline, a worse value fails'
summary: 'One baseline file per pack and measure: committed in metaspec for public and fixture packs, in the user''s state directory for a private pack; better or equal is taken, worse exits 1; shared by both epics.'
relations:
- decomposes: epic:kpi-quality
- depends_on: story:kpi-quality-catalogue-shape
- depends_on: story:spec-quality-static
scope:
- confidence: cited
  path: crates/metaspec-core/src/quality/ratchet.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/ratchet.rs
- confidence: cited
  path: crates/metaspec/src/quality/ratchet.rs
- confidence: cited
  path: fixtures/ratchet/harbor-kpis.json
- confidence: cited
  path: fixtures/ratchet/harbor-spec.json
- confidence: cited
  path: quality/baselines/agenticspec/adoption_level.json
- confidence: cited
  path: quality/baselines/agenticspec/completeness.fields.json
- confidence: cited
  path: quality/baselines/agenticspec/coverage.blocks.json
- confidence: cited
  path: quality/baselines/agenticspec/ess_completeness.outside.json
- confidence: cited
  path: quality/baselines/agenticspec/ess_completeness.unanswered.json
- confidence: cited
  path: quality/baselines/agenticspec/ess_completeness.unscenarioed.json
- confidence: cited
  path: quality/baselines/agenticspec/honesty.json
- confidence: cited
  path: quality/baselines/agenticspec/planned.json
- confidence: cited
  path: quality/baselines/agenticspec/validity.refusals.json
- confidence: cited
  path: quality/baselines/agenticspec/validity.warnings.json
- confidence: cited
  path: quality/baselines/harbor/adoption_level.json
- confidence: cited
  path: quality/baselines/harbor/completeness.fields.json
- confidence: cited
  path: quality/baselines/harbor/coverage.blocks.json
- confidence: cited
  path: quality/baselines/harbor/ess_completeness.outside.json
- confidence: cited
  path: quality/baselines/harbor/ess_completeness.unanswered.json
- confidence: cited
  path: quality/baselines/harbor/ess_completeness.unscenarioed.json
- confidence: cited
  path: quality/baselines/harbor/honesty.json
- confidence: cited
  path: quality/baselines/harbor/planned.json
- confidence: cited
  path: quality/baselines/harbor/validity.refusals.json
- confidence: cited
  path: quality/baselines/harbor/validity.warnings.json
- confidence: cited
  path: quality/baselines/orchard/adoption_level.json
- confidence: cited
  path: quality/baselines/orchard/completeness.fields.json
- confidence: cited
  path: quality/baselines/orchard/coverage.blocks.json
- confidence: cited
  path: quality/baselines/orchard/ess_completeness.outside.json
- confidence: cited
  path: quality/baselines/orchard/ess_completeness.unanswered.json
- confidence: cited
  path: quality/baselines/orchard/ess_completeness.unscenarioed.json
- confidence: cited
  path: quality/baselines/orchard/honesty.json
- confidence: cited
  path: quality/baselines/orchard/planned.json
- confidence: cited
  path: quality/baselines/orchard/validity.refusals.json
- confidence: cited
  path: quality/baselines/orchard/validity.warnings.json
- confidence: cited
  path: quality/baselines/orgspec/adoption_level.json
- confidence: cited
  path: quality/baselines/orgspec/completeness.fields.json
- confidence: cited
  path: quality/baselines/orgspec/coverage.blocks.json
- confidence: cited
  path: quality/baselines/orgspec/ess_completeness.outside.json
- confidence: cited
  path: quality/baselines/orgspec/ess_completeness.unanswered.json
- confidence: cited
  path: quality/baselines/orgspec/ess_completeness.unscenarioed.json
- confidence: cited
  path: quality/baselines/orgspec/honesty.json
- confidence: cited
  path: quality/baselines/orgspec/planned.json
- confidence: cited
  path: quality/baselines/orgspec/validity.refusals.json
- confidence: cited
  path: quality/baselines/orgspec/validity.warnings.json
- confidence: cited
  path: schema/metaspec-quality-baseline-1.schema.json
- confidence: cited
  path: scripts/acceptance.d/zz-quality-ratchet.sh
revision: 15
---
# Baseline and ratchet: the first run sets a baseline, a worse value fails

## Context

Shared by `epic:kpi-quality` and `epic:spec-quality`: which value of a measure is good is not
known before the first run, so the first run sets a baseline, a better or equal value is taken
into it, and a worse value fails. No absolute target is invented (`epic:kpi-quality`, Open).
It works on any `metaspec-quality/1` report, of either question.

## Scope

- `metaspec quality ratchet <report.json>... [--baselines <dir>]`; fills the stubs
  `crates/metaspec/src/quality/ratchet.rs` and `crates/metaspec-core/src/quality/ratchet.rs` (the
  verb is wired in `main.rs` by `story:kpi-quality-catalogue-shape`).
- One file per pack and measure, `<dir>/<pack>/<measure>.json`, format
  `metaspec-quality-baseline/1` with a JSON Schema, holding the value, the direction it was taken
  with and the source digest of the report.
- Where the files live, by the manifest's `visibility`
  (`decision-blocker:private-pack-baseline`, option A):
  - public and fixture packs: `quality/baselines/<pack>/<measure>.json` in the metaspec checkout,
    committed;
  - a private pack: `$XDG_STATE_HOME/metaspec/quality/baselines/<pack>/<measure>.json` (default
    `~/.local/state/metaspec`, the state directory that holds `trust.json`), keyed by the pack
    name, never in the checkout. Nothing of a private pack is written under the checkout.
  - `--baselines <dir>` overrides both (tests of the ratchet's arithmetic only; the location case
    below runs without it).
- A measure with no baseline: its value becomes the baseline, with the direction of its row in
  metaspec's catalogue. A better or equal value is taken and written. A worse value exits 1 naming
  the measure, the baseline and the value, and leaves the file unchanged. Better and worse follow
  the row's direction: for `higher_is_better` a lower value is worse, for `lower_is_better` a
  higher value is worse.
- A gap reading has no value: the baseline is kept, the gap is printed, and it does not fail (the
  ratchet takes a value only).
- Acceptance row file `scripts/acceptance.d/zz-quality-ratchet.sh`: runs last (name order) over
  every report under `target/acceptance/quality/`. Its first run writes the baselines of the
  measures built by then (those of `story:kpi-quality-catalogue-shape` and
  `story:spec-quality-static`) for harbor, orchard, orgspec and agenticspec, and this story
  commits them; after that, `task acceptance -- <orgspec-dir> <agenticspec-dir>` exits 0 against
  the committed baselines with `git status --porcelain quality/baselines/` empty. A measure story
  that lands later commits the baseline files of its own measures that its acceptance run writes;
  it depends on this story for that.
- Test cases (in `crates/metaspec-core/tests/cases/quality/ratchet.rs`, on the recorded harbor
  reports of both questions):
  - `first_run_writes_baseline`: a first run writes one baseline file per measure; an equal run
    exits 0 with the files unchanged;
  - `better_value_taken`, `worse_value_exits_1`, for `coverage.blocks` (`higher_is_better`) and
    `honesty` (`lower_is_better`) each: a better value (higher, respectively lower) is taken into
    its file, and a worse value (lower, respectively higher) exits 1 with the measure and both
    values named and the file unchanged;
  - `gap_keeps_baseline`: a gap reading keeps the baseline and exits 0;
  - `private_pack_baseline`: a manifest with `visibility: private` (written by the test),
    `XDG_STATE_HOME` set to a scratch directory and no `--baselines`: the first run writes one
    file per measure under `$XDG_STATE_HOME/metaspec/quality/baselines/<pack>/` and nothing under
    the checkout's `quality/baselines/`.
  The acceptance runs the first, the worse-value and the location case.
- The committed baselines of the four packs (a check of this story's scope, not its acceptance):
  after `task acceptance -- <orgspec-dir> <agenticspec-dir>`, `git status --porcelain
  quality/baselines/` is empty and the task exits 0, once the first run's baselines are
  committed.
- The ESS spec's `metaspec.quality.Baseline` summary gets the same split (in the
  `ess-meta-model` worktree, not a surface of this repository).

## Surfaces

- new: `schema/metaspec-quality-baseline-1.schema.json`, recorded reports
  `fixtures/ratchet/harbor-kpis.json` and `fixtures/ratchet/harbor-spec.json`,
  `quality/baselines/<pack>/<measure>.json` for harbor, orchard, orgspec and agenticspec and the
  measures `coverage.blocks`, `completeness.fields`, `planned`, `validity.refusals`,
  `validity.warnings`, `ess_completeness.unscenarioed`, `ess_completeness.unanswered`,
  `ess_completeness.outside`, `honesty`, `adoption_level` (where the reading is a value),
  `scripts/acceptance.d/zz-quality-ratchet.sh`
- filled (stubs of `story:kpi-quality-catalogue-shape`): `crates/metaspec/src/quality/ratchet.rs`,
  `crates/metaspec-core/src/quality/ratchet.rs`, `crates/metaspec-core/tests/cases/quality/ratchet.rs`

## Shared files

- The three stub files: created by `story:kpi-quality-catalogue-shape`, which this story depends
  on.
- `quality/baselines/`: every later measure story adds the files of its own measures there
  (`story:kpi-quality-grounding`, `-values`, `-rules`, `story:spec-quality-conformance`,
  `-layering`, `-mutation`, `-refusal`, `-drift`), and `story:spec-quality-self` adds
  `quality/baselines/metaspec/`; each depends on this story. No file is written by two stories.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- Baseline → Pack, many-to-one, and Baseline → Measure, many-to-one; one baseline per measure and
  pack — inferable (`quality.yaml`, `metaspec.quality.Baseline`, relations `pack` and `measure`).
  Where a private pack's baseline lives: `decision-blocker:private-pack-baseline` (cleared, A).
- The ratchet: a better or equal value is taken, a worse one fails and the baseline is kept, and
  the first value sets it — inferable (`quality.yaml`, commands `metaspec.quality.SetBaseline` and
  `metaspec.quality.Ratchet`, error `metaspec.quality.Regressed`).
- Measure → Kpi, the row that carries the direction — inferable (`quality.yaml`,
  `metaspec.quality.Measure`, relation `kpi`).

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

`cargo nextest run -p metaspec-core quality::ratchet` passes; its cases `first_run_writes_baseline`
(a first run writes the baseline), `worse_value_exits_1` (a worse value exits 1 with the baseline
file unchanged) and `private_pack_baseline` (a private pack writes under
`$XDG_STATE_HOME/metaspec/quality/baselines/<pack>/` and nothing under the checkout's
`quality/baselines/`) are named in Scope.
