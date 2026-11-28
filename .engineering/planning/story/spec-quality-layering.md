---
format: aep.planning-md/3
id: story:spec-quality-layering
kind: story
status: draft
title: 'layering: dependency edges that point to a higher layer or form a cycle inside one'
summary: Edges between blocks from the compiled model, measured against the layers for every pack.
relations:
- decomposes: epic:spec-quality
- depends_on: story:spec-quality-static
- depends_on: story:quality-ratchet
scope:
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/layering.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/layering.rs
- confidence: cited
  path: fixtures/ess/layering/compile.json
- confidence: cited
  path: fixtures/expected/harbor/quality/layering.json
- confidence: cited
  path: fixtures/expected/orchard/quality/layering.json
- confidence: cited
  path: quality/baselines/agenticspec/layering.json
- confidence: cited
  path: quality/baselines/harbor/layering.json
- confidence: cited
  path: quality/baselines/orchard/layering.json
- confidence: cited
  path: quality/baselines/orgspec/layering.json
- confidence: cited
  path: scripts/acceptance.d/spec-quality-layering.sh
revision: 10
---
# layering: dependency edges that point to a higher layer or form a cycle inside one

## Context

A measure of `epic:spec-quality`. Each pack holds its layering with a script of its own
(`scripts/check-layers.sh` in orgspec and agenticspec); metaspec measures it the same way for
every pack from the compiled model.

## Scope

- The dependencies between blocks from `ess specify compile --format json`: a declared type of B
  used in A, the identity type of an entity of B, a relation of an entity of A to an entity of B;
  one edge per pair of blocks with the number of references behind it.
- layering: edges that point to a higher layer, plus edges on a cycle inside one layer; behind:
  those edges (kind `edge`, `<from>-><to>`).
- A block in no numbered directory: a gap naming it.
- A recorded compile answer for the test, `fixtures/ess/layering/compile.json`, with one upward
  edge and one cycle of two blocks inside one layer; layering on it is 3 (the upward edge and the
  two edges of the cycle).
- Acceptance row file `scripts/acceptance.d/spec-quality-layering.sh` (a test case, not the
  acceptance): its rows show `metaspec quality spec` printing layering 0 for harbor, orchard,
  orgspec and agenticspec, whose own `scripts/check-layers.sh` pass.
- The baseline `quality/baselines/<pack>/layering.json` for the four packs, written by the
  acceptance run's ratchet and committed here.

## Surfaces

- filled (stubs of `story:kpi-quality-catalogue-shape`):
  `crates/metaspec-core/src/quality/spec/layering.rs`,
  `crates/metaspec-core/tests/cases/quality/layering.rs`
- new: `fixtures/ess/layering/compile.json`, `fixtures/expected/{harbor,orchard}/quality/layering.json`,
  `quality/baselines/{harbor,orchard,orgspec,agenticspec}/layering.json`,
  `scripts/acceptance.d/spec-quality-layering.sh`

## Shared files

- None with another measure story: the registry entry and the test module are stubs of
  `story:kpi-quality-catalogue-shape`, the golden is one file per measure, and
  `fixtures/ess/layering/` is this story's alone.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- Dependency → Block, from and to, each many-to-one, with a count of at least 1 — inferable
  (`blocks.yaml`, `metaspec.blocks.Dependency`, relations `from` and `to`, field `count`).
- Block → Layer, many-to-one, absent for a block in no numbered directory — inferable
  (`blocks.yaml`, `metaspec.blocks.Block`, relation `layer`).
- What counts as a reference between two blocks — inferable (inferred from orgspec
  `scripts/check-layers.sh:5-11`; the ESS spec declares the edge, not what makes one).

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

`cargo nextest run -p metaspec-core quality::layering` passes with its case showing layering 3 on
the recorded compile answer `fixtures/ess/layering/compile.json`, naming its one upward edge and
the two edges of its cycle inside one layer.
