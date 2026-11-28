---
format: aep.planning-md/3
id: story:spec-quality-static
kind: story
status: implemented
title: metaspec quality spec reports validity, ESS completeness, honesty and adoption level for every pack
summary: The specification command on the shared format, with the measures read from ess outputs, the block sources and the manifest.
relations:
- decomposes: epic:spec-quality
- depends_on: story:kpi-quality-catalogue-shape
- depends_on: story:quality-catalogue
scope:
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/static_measures
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/static_measures.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/spec.rs
- confidence: cited
  path: crates/metaspec/src/quality/spec.rs
- confidence: cited
  path: fixtures/ess/ess
- confidence: cited
  path: fixtures/ess/harbor/validate.json
- confidence: cited
  path: fixtures/ess/orchard/validate.json
- confidence: cited
  path: fixtures/expected/harbor/quality/adoption_level.json
- confidence: cited
  path: fixtures/expected/harbor/quality/ess_completeness.outside.json
- confidence: cited
  path: fixtures/expected/harbor/quality/ess_completeness.unanswered.json
- confidence: cited
  path: fixtures/expected/harbor/quality/ess_completeness.unscenarioed.json
- confidence: cited
  path: fixtures/expected/harbor/quality/honesty.json
- confidence: cited
  path: fixtures/expected/harbor/quality/validity.refusals.json
- confidence: cited
  path: fixtures/expected/harbor/quality/validity.warnings.json
- confidence: cited
  path: fixtures/expected/orchard/quality/adoption_level.json
- confidence: cited
  path: fixtures/expected/orchard/quality/ess_completeness.outside.json
- confidence: cited
  path: fixtures/expected/orchard/quality/ess_completeness.unanswered.json
- confidence: cited
  path: fixtures/expected/orchard/quality/ess_completeness.unscenarioed.json
- confidence: cited
  path: fixtures/expected/orchard/quality/honesty.json
- confidence: cited
  path: fixtures/expected/orchard/quality/validity.refusals.json
- confidence: cited
  path: fixtures/expected/orchard/quality/validity.warnings.json
- confidence: cited
  path: scripts/acceptance.d/spec-quality-static.sh
revision: 14
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T10:35:33Z", actor: "human:operator", revision: 11, decided_on: {"recorded":{"review_outcome":1}}}
- {from: "proposed", to: "active", at: "2026-11-28T10:35:33Z", actor: "human:operator", revision: 12, decided_on: {"recorded":{"review_outcome":1}}}
- {from: "active", to: "implemented", at: "2026-11-28T14:22:06Z", actor: "human:operator", revision: 14, decided_on: {"recorded":{"test_result":1,"review_outcome":3}}}
---
# `metaspec quality spec` reports validity, ESS completeness, honesty and adoption level for every pack

## Context

The tracer bullet of the specification question (`epic:spec-quality`): the command, on the
report format of `story:kpi-quality-catalogue-shape`, with the measures that read only `ess`
outputs, the sources of the blocks and the manifest.

## Scope

- `metaspec quality spec <NAME | --manifest FILE> --pack <DIR> [--json]`, report
  `metaspec-quality/1` with question `specification`; fills the stub
  `crates/metaspec/src/quality/spec.rs` (the subcommand is wired in `main.rs` by
  `story:kpi-quality-catalogue-shape`).
- validity: `validity.refusals` and `validity.warnings`, the refusals and warnings of
  `ess specify validate --format json` on the blocks; behind: the refusal and warning codes with
  their subjects.
- ESS completeness: `completeness.counts` of the same answer, `ess_completeness.unscenarioed`,
  `.unanswered` and `.outside`; behind: the constructs. The JSON answer runs synthesis, so it is
  cached by the digest of the pack's blocks in the user's cache directory, never in the metaspec
  checkout (private packs, `design/metaspec.md`). The cache is a child module of this story's
  file; later stories call it and do not change it.
- honesty: open `UNMAPPED:` markers in the files under `blocks.path`; behind: `file:line` (kind
  `marker`).
- adoption level: the level `metaspec pack check` derives, 0 to 5; and the gaps of kind `verb` of
  the pack: each verb metaspec offers that the manifest does not declare, with the level that
  needs it (agenticspec: `check`, L3).
- Blocks that do not validate: the completeness rows are gaps naming the refusal.
- A stub `ess` for the tests, named by an environment variable: it prints
  `fixtures/ess/<pack>/<verb>.json` for `ess <...> <verb>` on that pack, so a later story adds an
  answer as a new file and never changes the stub.
- Test cases on harbor and orchard: a recorded validate answer with one refusal, one warning and
  one `UNMAPPED:` marker in a block file of the test's own copy, each counted and named behind.

## Surfaces

- new: `crates/metaspec-core/src/quality/spec/static_measures/` (child modules: the cached `ess`
  answer), `fixtures/ess/ess` (the stub), `fixtures/ess/harbor/validate.json`,
  `fixtures/ess/orchard/validate.json`,
  `fixtures/expected/{harbor,orchard}/quality/{validity.refusals,validity.warnings,ess_completeness.unscenarioed,ess_completeness.unanswered,ess_completeness.outside,honesty,adoption_level}.json`,
  `scripts/acceptance.d/spec-quality-static.sh`
- filled (stubs of `story:kpi-quality-catalogue-shape`): `crates/metaspec/src/quality/spec.rs`,
  `crates/metaspec-core/src/quality/spec/static_measures.rs`,
  `crates/metaspec-core/tests/cases/quality/spec.rs`

## Shared files

- The three stub files: created by `story:kpi-quality-catalogue-shape`, which this story depends
  on; no other story changes them.
- `fixtures/ess/`: this story adds the stub and the validate answers; `story:spec-quality-layering`,
  `story:spec-quality-conformance` and `story:spec-quality-mutation` add answer files of their own
  there, and all three depend on this story.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- QualityReport → Pack; QualityReport → Measurement (owns); Measurement → Evidence (owns) —
  inferable (`quality.yaml`, relations `metaspec.quality.QualityReport.pack`,
  `metaspec.quality.QualityReport.measurements`, `metaspec.quality.Measurement.evidence`).
- Pack → Gap, one-to-many, pack owns, one gap per pack and feature, kind `verb` for a verb —
  inferable (`packs.yaml`, `metaspec.packs.Pack`, relation `gaps`; `metaspec.packs.Gap` fields
  `kind`, `feature`, `needs`).
- The adoption level of a pack is derived from its manifest — inferable (`packs.yaml`,
  `metaspec.packs.Pack` field `level` and its invariants).

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
`metaspec quality spec --json` for harbor, orchard, orgspec and agenticspec valid against
`schema/metaspec-quality-1.schema.json`, its `validity.refusals` and `validity.warnings` equal to
the refusals and warnings of `ess specify validate --format json` on the same blocks and its ESS
completeness counts equal to that answer's `completeness.counts`, its `honesty` equal to the
number of `UNMAPPED:` lines `grep` finds under the pack's `blocks.path`, its adoption level equal
to `metaspec pack check` `level`, and agenticspec listing `check` as a verb gap.

Correction (2026-11-28, wave 3, by the coordinator): the last clause no longer holds for the real
pack. `packs/agenticspec.yaml` declares the verb `check` since b788cc3 (2026-11-28), so
agenticspec has no verb gap and is at level 3. The row compares the verbs metaspec offers minus
the verbs the manifest declares (agenticspec: none, equal), and an extra row runs agenticspec with
a manifest without `tool`, which lists `check` (needs L3) as its verb gap and level 2. That row
stands for the clause.
