---
format: aep.planning-md/3
id: story:spec-quality-refusal
kind: story
status: draft
title: refusal power and document health from the check of every document
summary: Each rule refused alone by a broken example; the share of example documents that are valid.
relations:
- decomposes: epic:spec-quality
- depends_on: story:kpi-quality-values
- depends_on: story:spec-quality-static
- depends_on: story:quality-ratchet
- depends_on: story:kpi-quality-rules
scope:
- confidence: cited
  path: crates/metaspec-core/src/quality/spec/refusal.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/refusal.rs
- confidence: cited
  path: fixtures/expected/harbor/quality/document_health.json
- confidence: cited
  path: fixtures/expected/harbor/quality/refusal_power.json
- confidence: cited
  path: fixtures/expected/orchard/quality/document_health.json
- confidence: cited
  path: fixtures/expected/orchard/quality/refusal_power.json
- confidence: cited
  path: quality/baselines/agenticspec/document_health.json
- confidence: cited
  path: quality/baselines/agenticspec/refusal_power.json
- confidence: cited
  path: quality/baselines/harbor/document_health.json
- confidence: cited
  path: quality/baselines/harbor/refusal_power.json
- confidence: cited
  path: quality/baselines/orchard/document_health.json
- confidence: cited
  path: quality/baselines/orchard/refusal_power.json
- confidence: cited
  path: quality/baselines/orgspec/document_health.json
- confidence: cited
  path: quality/baselines/orgspec/refusal_power.json
- confidence: cited
  path: scripts/acceptance.d/spec-quality-refusal.sh
revision: 8
---
# refusal power and document health from the check of every document

## Context

Two measures of `epic:spec-quality`: can each rule of the tool refuse (a broken example fails
that rule alone), and how many example documents are valid. Reads the check pass and the
broken-example classes (`expect: invalid`) of `story:kpi-quality-values`, and the harbor broken
plans of `story:kpi-quality-values` and `story:kpi-quality-rules`.

## Scope

- refusal power (`refusal_power`): rules the tool reports (keys of `counts` and rule ids of
  findings, over every document) that have a broken example, divided by those rules. A broken
  example of a rule: a document of a class with `expect: invalid` whose gated findings name that
  rule and no other. behind: the rules without one.
- document health (`document_health`): valid documents divided by the documents of the classes
  without `expect: invalid`; behind: each invalid document and the rules of its findings.
- Gaps, each naming the missing input: no check adapter (both); no class with `expect: invalid`
  (refusal power).
- The baselines `quality/baselines/<pack>/{refusal_power,document_health}.json` where the reading
  is a value, written by the acceptance run's ratchet and committed here.

## Surfaces

- filled (stubs of `story:kpi-quality-catalogue-shape`):
  `crates/metaspec-core/src/quality/spec/refusal.rs`,
  `crates/metaspec-core/tests/cases/quality/refusal.rs`
- new: `fixtures/expected/{harbor,orchard}/quality/{refusal_power,document_health}.json`,
  `quality/baselines/{harbor,orchard,orgspec,agenticspec}/{refusal_power,document_health}.json`,
  `scripts/acceptance.d/spec-quality-refusal.sh`
- reads, not written here: `crates/metaspec-core/src/quality/check_pass.rs` and the harbor broken
  plans of `story:kpi-quality-values`

## Shared files

- No file is written by another story. The harbor golden and baseline of `refusal_power` read
  the harbor broken plans, which `story:kpi-quality-values` and `story:kpi-quality-rules` add;
  this story depends on both, so it renders them on the final set of plans and neither changes
  them afterwards.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- CheckResult → Document and CheckResult → Finding (owns) — inferable (`checks.yaml`,
  `metaspec.checks.CheckResult`, relations `document`, `findings`).
- Finding → Rule, many-to-one, absent for a rule metaspec has no row of; read here as the rule id
  the tool writes — inferable (`checks.yaml`, `metaspec.checks.Finding`, relation `rule`).
- Document → DocumentClass — inferable (`documents.yaml`, `metaspec.documents.Document`, relation
  `class`).

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
`metaspec quality spec` for orgspec printing refusal power as the share of the rule ids its
documents report that one of its 30 broken designs fails alone and document health over its 8
designs, both as values on harbor, refusal power as a gap on orchard (no broken examples), and
both as gaps on agenticspec (no check adapter).
