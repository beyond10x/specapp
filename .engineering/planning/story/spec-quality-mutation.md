---
format: aep.planning-md/3
id: story:spec-quality-mutation
kind: story
status: draft
title: mutation score in a slow lane, once per digest of a pack's blocks
summary: task slow:mutation runs ess mutate per class under slot; quality spec reads the result or prints a gap; never in task check.
relations:
- decomposes: epic:spec-quality
- depends_on: story:spec-quality-conformance
scope:
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/mutation.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/mutation.rs
- confidence: cited
  path: fixtures/ess/harbor/mutate.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.emit-drop.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.emit-swap.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.error-swap.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.from-drop.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.grant-add.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.grant-drop.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.guard-boundary.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.guard-connective.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.guard-negate.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.order-flip.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.precedence-swap.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.score.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.sets-drop.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.sets-retarget.json
- confidence: cited
  path: fixtures/expected/harbor/quality/mutation.transition-to.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.emit-drop.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.emit-swap.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.error-swap.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.from-drop.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.grant-add.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.grant-drop.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.guard-boundary.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.guard-connective.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.guard-negate.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.order-flip.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.precedence-swap.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.score.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.sets-drop.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.sets-retarget.json
- confidence: cited
  path: quality/baselines/agenticspec/mutation.transition-to.json
- confidence: cited
  path: quality/baselines/harbor/mutation.emit-drop.json
- confidence: cited
  path: quality/baselines/harbor/mutation.emit-swap.json
- confidence: cited
  path: quality/baselines/harbor/mutation.error-swap.json
- confidence: cited
  path: quality/baselines/harbor/mutation.from-drop.json
- confidence: cited
  path: quality/baselines/harbor/mutation.grant-add.json
- confidence: cited
  path: quality/baselines/harbor/mutation.grant-drop.json
- confidence: cited
  path: quality/baselines/harbor/mutation.guard-boundary.json
- confidence: cited
  path: quality/baselines/harbor/mutation.guard-connective.json
- confidence: cited
  path: quality/baselines/harbor/mutation.guard-negate.json
- confidence: cited
  path: quality/baselines/harbor/mutation.order-flip.json
- confidence: cited
  path: quality/baselines/harbor/mutation.precedence-swap.json
- confidence: cited
  path: quality/baselines/harbor/mutation.score.json
- confidence: cited
  path: quality/baselines/harbor/mutation.sets-drop.json
- confidence: cited
  path: quality/baselines/harbor/mutation.sets-retarget.json
- confidence: cited
  path: quality/baselines/harbor/mutation.transition-to.json
- confidence: cited
  path: quality/baselines/orchard/mutation.emit-drop.json
- confidence: cited
  path: quality/baselines/orchard/mutation.emit-swap.json
- confidence: cited
  path: quality/baselines/orchard/mutation.error-swap.json
- confidence: cited
  path: quality/baselines/orchard/mutation.from-drop.json
- confidence: cited
  path: quality/baselines/orchard/mutation.grant-add.json
- confidence: cited
  path: quality/baselines/orchard/mutation.grant-drop.json
- confidence: cited
  path: quality/baselines/orchard/mutation.guard-boundary.json
- confidence: cited
  path: quality/baselines/orchard/mutation.guard-connective.json
- confidence: cited
  path: quality/baselines/orchard/mutation.guard-negate.json
- confidence: cited
  path: quality/baselines/orchard/mutation.order-flip.json
- confidence: cited
  path: quality/baselines/orchard/mutation.precedence-swap.json
- confidence: cited
  path: quality/baselines/orchard/mutation.score.json
- confidence: cited
  path: quality/baselines/orchard/mutation.sets-drop.json
- confidence: cited
  path: quality/baselines/orchard/mutation.sets-retarget.json
- confidence: cited
  path: quality/baselines/orchard/mutation.transition-to.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.emit-drop.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.emit-swap.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.error-swap.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.from-drop.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.grant-add.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.grant-drop.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.guard-boundary.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.guard-connective.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.guard-negate.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.order-flip.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.precedence-swap.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.score.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.sets-drop.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.sets-retarget.json
- confidence: cited
  path: quality/baselines/orgspec/mutation.transition-to.json
- confidence: cited
  path: scripts/acceptance.d/spec-quality-mutation.sh
- confidence: cited
  path: scripts/slow-mutation.sh
- confidence: cited
  path: scripts/slow-mutation.test.sh
- confidence: cited
  path: taskfiles/slow.yml
revision: 12
---
# mutation score in a slow lane, once per digest of a pack's blocks

## Context

A measure of `epic:spec-quality`: the share of `ess verify conform mutate` mutants the suite
kills, per mutant class, with the survivors listed. A mutation run synthesizes a suite per
mutant, far beyond the test budget, so it runs in a lane named `slow`, once per digest of the
pack's blocks, and never in `task check` (test killed at 60 s, step within 120 s).

## Scope

- `task slow:mutation -- <NAME | MANIFEST> <DIR>` (task `mutation` in `taskfiles/slow.yml`,
  included by `Taskfile.yml` as `slow` since `story:kpi-quality-catalogue-shape`) runs
  `ess verify conform mutate --target interpreted` on the blocks, with the known-failing list of
  the pack when its manifest names one. One slot job per mutant class (`--class`), each named with
  `slot --timeout` of at most 30m; a class that runs longer is split. The script waits and
  collects, so it starts without the `slot` prefix and puts the prefix on each `ess` call. POSIX
  sh or Rust, no Python.
- The result is written once per digest of the blocks into the user's cache directory (never the
  metaspec checkout); a digest with a result is skipped: the script starts no `ess` job for it and
  exits 0.
- `metaspec quality spec` reads the result for the current digest: `mutation.score` and one
  `mutation.<class>` row per class; behind: the surviving mutants (kind `mutant`).
- Test cases (in scope, not acceptance):
  - no result for the digest: `quality spec` prints every mutation row as a gap naming
    `task slow:mutation` (`crates/metaspec-core/tests/cases/quality/mutation.rs`, on harbor);
  - the reader on a recorded mutation report, `fixtures/ess/harbor/mutate.json`: score, one row
    per class, survivors behind;
  - the script on a stub `ess` (`scripts/slow-mutation.test.sh`, run by `task check:scripts`;
    the acceptance): a first run logs one job per class and writes the result for the digest, and
    the score read back from it equals the stub's; a second run over the same blocks starts no
    `ess` and exits 0;
  - `task check` runs no `ess verify conform mutate`: the stub `ess` of the tests refuses
    `mutate` outside `slow-mutation.test.sh`;
  - on a real pack, in the slow lane and outside the test step: after
    `task slow:mutation -- orgspec <orgspec-dir>`, `metaspec quality spec orgspec --pack
    <orgspec-dir>` prints `mutation.score` and one `mutation.<class>` row per class as values,
    each with its surviving mutants behind (row file `scripts/acceptance.d/spec-quality-mutation.sh`
    reads the cached result when one exists, and prints the gap otherwise).
- The baselines `quality/baselines/<pack>/mutation.*.json` where a result exists, written by the
  ratchet and committed here.

## Surfaces

- new: `taskfiles/slow.yml` (unless `story:spec-quality-conformance` created it),
  `scripts/slow-mutation.sh`, `scripts/slow-mutation.test.sh`, `fixtures/ess/harbor/mutate.json`,
  `fixtures/expected/harbor/quality/mutation.score.json` and one per class,
  `quality/baselines/{harbor,orchard,orgspec,agenticspec}/mutation.score.json` and one per class
  (where a result exists), `scripts/acceptance.d/spec-quality-mutation.sh`
- filled (stubs of `story:kpi-quality-catalogue-shape`):
  `crates/metaspec-core/src/quality/spec/mutation.rs`,
  `crates/metaspec-core/tests/cases/quality/mutation.rs`

## Shared files

- `taskfiles/slow.yml`: shared with `story:spec-quality-conformance` only if that story moves its
  run into the slow lane; this story depends on it.
- `Taskfile.yml` is not changed here: the include is `story:kpi-quality-catalogue-shape`'s.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- Measurement → Evidence (owns), a mutant named as `ess` writes it — inferable (`quality.yaml`,
  `metaspec.quality.Measurement`, relation `evidence`; `metaspec.quality.BehindKind` variant
  `mutant`).
- QualityReport → Pack, with the digest of the sources the measures read — inferable
  (`quality.yaml`, `metaspec.quality.QualityReport`, relation `pack`, field `source_digest`).

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

`task check:scripts` passes with `scripts/slow-mutation.test.sh` covering, on a stub `ess` that
logs each call, a first `scripts/slow-mutation.sh` run over blocks with no result for their digest
(one stub `ess` call per class logged, a result written for the digest, the score read back from
it equal to the stub's) and a second run over the same blocks (no new line in the call log, exit
0).
