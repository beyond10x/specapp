---
format: aep.planning-md/3
id: story:spec-quality-drift
kind: story
status: draft
title: 'drift: generated files of a pack that differ from a fresh generation, through a new adapter verb'
summary: A manifest verb regenerate names the pack's own generation check, read like check; drift counts the files that differ, a pack without the verb has the gap drift.
relations:
- decomposes: epic:spec-quality
- depends_on: story:spec-quality-static
- depends_on: story:spec-quality-conformance
- depends_on: story:quality-ratchet
scope:
- confidence: cited
  path: crates/metaspec-core/src/adapter.rs
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/drift.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/drift.rs
- confidence: cited
  path: fixtures/expected/harbor/quality/drift.json
- confidence: cited
  path: fixtures/expected/orchard/quality/drift.json
- confidence: cited
  path: fixtures/manifests/orchard.yaml
- confidence: cited
  path: fixtures/packs/orchard/tool/answers/regenerate.json
- confidence: cited
  path: fixtures/packs/orchard/tool/orchard-tool
- confidence: cited
  path: quality/baselines/orchard/drift.json
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: scripts/acceptance.d/spec-quality-drift.sh
revision: 9
---
# drift: generated files of a pack that differ from a fresh generation, through a new adapter verb

## Context

A measure of `epic:spec-quality`: where a pack commits generated output with a check, how many
generated files differ from a fresh generation. metaspec knows a pack only through its manifest
and runs the pack's own tool only through an adapter of a declared verb.
`decision-blocker:pack-drift-source` is cleared with option A: a new adapter verb in the manifest
names the pack's own generation check, mapped like `check`; a pack without it has the listed gap
"drift".

## Scope

- A new verb `regenerate` under `tool.verbs` of `metaspec-pack/1` (field in
  `crates/metaspec-core/src/manifest.rs`, `Verbs::names` lists it, JSON Schema
  `schema/metaspec-pack-1.schema.json`): `args`, `exit`, `at`, `error` as the check adapter has
  them, and `differs`, a JSON pointer to the list of generated files that differ (each a path
  inside the pack). The adapter in `crates/metaspec-core/src/adapter.rs` runs it once per run, with
  trust and deadline like `check`; the pack's check must not write files in the pack (the verb is
  a check, the name follows the blocker's option A).
- drift: the number of files the answer lists; behind: those files (kind `file`). Direction
  `lower_is_better`.
- Gaps, each naming the missing input: no `regenerate` verb in the manifest (gap "drift"); a tool
  the user has not trusted (naming `metaspec trust`); an answer the mapping cannot read (naming
  the pointer).
- No level needs `regenerate`, so `metaspec pack check` levels, the `fixtures/expected/<pack>.json`
  goldens and the verb gaps of `story:spec-quality-static` (verbs a level needs) do not change; a
  pack without the verb shows it as the gap reading of `drift` only.
- The real packs: neither `packs/orgspec.yaml` nor `packs/agenticspec.yaml` gains the verb here.
  agenticspec's `agenticspec-docs --check` and orgspec's `scripts/blocks-digest.sh` print no JSON
  (`decision-blocker:pack-drift-source`, "Why it is open"), and metaspec does not change a pack to
  raise its numbers (`initiative:measure-quality`, Out of scope): both show the gap "drift" until
  their owners give their checks a JSON answer.
- Fixtures: `fixtures/manifests/orchard.yaml` declares `regenerate` on a recorded answer
  `fixtures/packs/orchard/tool/answers/regenerate.json` listing one differing file; harbor
  declares none and shows the gap.
- The ESS model's `metaspec.packs.Verb` gains the variant `regenerate` in the `ess-meta-model`
  worktree (not a surface of this repository; an existing type gains a variant, no new noun).
- The baseline `quality/baselines/orchard/drift.json`, written by the acceptance run's ratchet and
  committed here.

## Surfaces

- filled (stubs of `story:kpi-quality-catalogue-shape`):
  `crates/metaspec-core/src/quality/spec/drift.rs`,
  `crates/metaspec-core/tests/cases/quality/drift.rs`
- changed: `crates/metaspec-core/src/manifest.rs`, `schema/metaspec-pack-1.schema.json`,
  `crates/metaspec-core/src/adapter.rs`, `fixtures/manifests/orchard.yaml`,
  `fixtures/packs/orchard/tool/orchard-tool` (answers `regenerate` from the recorded file)
- new: `fixtures/packs/orchard/tool/answers/regenerate.json`,
  `fixtures/expected/{harbor,orchard}/quality/drift.json`, `quality/baselines/orchard/drift.json`,
  `scripts/acceptance.d/spec-quality-drift.sh`

## Shared files

- `crates/metaspec-core/src/manifest.rs`, `schema/metaspec-pack-1.schema.json`: also changed by
  `story:kpi-quality-values` (field `expect`) and `story:spec-quality-conformance` (field
  `blocks.known_failing`); this story depends on conformance, which depends on values.
- `crates/metaspec-core/src/adapter.rs`: no other story of the initiative changes it.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- Pack → Gap, one-to-many, pack owns, one gap per pack and feature — inferable (`packs.yaml`,
  `metaspec.packs.Pack`, relation `gaps`; `metaspec.packs.Gap` fields `kind`, `feature`).
- Measurement → Evidence (owns), a file named as the pack writes it — inferable (`quality.yaml`,
  `metaspec.quality.Measurement`, relation `evidence`; `metaspec.quality.BehindKind` variant
  `file`).
- The verbs of a manifest — inferable (`packs.yaml`, type `metaspec.packs.Verb`: `graph`, `check`,
  `simulate`, `search`, `apply`; `regenerate` is added there, see Scope).

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
`metaspec quality spec` printing drift 1 for orchard, naming the one file its recorded
`regenerate` answer lists, and the gap "drift" naming the missing `regenerate` verb for harbor,
orgspec and agenticspec.
