---
format: aep.planning-md/3
id: story:kpi-quality-rules
kind: story
status: draft
title: coverage.rules and gate fidelity over the rules the tool reports and the broken examples
summary: Every rule with a KPI that counts it; every gating KPI moving on its broken example and on no valid one.
relations:
- decomposes: epic:kpi-quality
- depends_on: story:kpi-quality-values
scope:
- confidence: cited
  path: crates/metaspec-core/src/quality/kpis/rules.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/rules.rs
- confidence: cited
  path: fixtures/expected/harbor.json
- confidence: cited
  path: fixtures/expected/harbor/pages/broken
- confidence: cited
  path: fixtures/expected/harbor/pages/index.html
- confidence: inferred
  path: fixtures/expected/harbor/quality/alive.json
- confidence: cited
  path: fixtures/expected/harbor/quality/coverage.rules.json
- confidence: inferred
  path: fixtures/expected/harbor/quality/discrimination.json
- confidence: cited
  path: fixtures/expected/harbor/quality/gate_fidelity.json
- confidence: inferred
  path: fixtures/expected/harbor/quality/redundancy.json
- confidence: cited
  path: fixtures/expected/harbor/scenes/broken
- confidence: cited
  path: fixtures/expected/orchard/quality/coverage.rules.json
- confidence: cited
  path: fixtures/expected/orchard/quality/gate_fidelity.json
- confidence: cited
  path: fixtures/packs/harbor/broken
- confidence: cited
  path: fixtures/packs/harbor/tool/answers
- confidence: cited
  path: quality/baselines/agenticspec/coverage.rules.json
- confidence: cited
  path: quality/baselines/agenticspec/gate_fidelity.json
- confidence: inferred
  path: quality/baselines/harbor/alive.json
- confidence: cited
  path: quality/baselines/harbor/coverage.rules.json
- confidence: inferred
  path: quality/baselines/harbor/discrimination.json
- confidence: cited
  path: quality/baselines/harbor/gate_fidelity.json
- confidence: inferred
  path: quality/baselines/harbor/redundancy.json
- confidence: cited
  path: quality/baselines/orchard/coverage.rules.json
- confidence: cited
  path: quality/baselines/orchard/gate_fidelity.json
- confidence: cited
  path: quality/baselines/orgspec/coverage.rules.json
- confidence: cited
  path: quality/baselines/orgspec/gate_fidelity.json
- confidence: cited
  path: scripts/acceptance.d/kpi-quality-rules.sh
revision: 9
---
# coverage.rules and gate fidelity over the rules the tool reports and the broken examples

## Context

Two measures of `epic:kpi-quality` that tie KPIs to the rules of the pack's tool: does every
rule have a KPI that counts it, and does each gating KPI move on the broken example of its rule
and on no valid example. Reads the check pass and the broken-example classes of
`story:kpi-quality-values`.

## Scope

- The rules the tool reports: the union, over every document, of the keys of the check's
  `counts` and the rule ids of its findings.
- coverage.rules: rules with a KPI whose id equals the rule id, divided by rules; behind: rules
  without one.
- A broken example of a rule: a document of a class with `expect: invalid` whose gated findings
  name that rule and no other.
- gate fidelity (`gate_fidelity`): over the gating KPIs (gate column `true` or a number) whose
  rule has a broken example. A KPI is faithful when its value is the same on every valid document
  and its value on that broken example differs from it. Value: faithful KPIs divided by those
  KPIs; behind: the unfaithful ones. A gap when no gating KPI has a broken example.
- Gaps, each naming the missing input: no check adapter; no `counts`; no class with
  `expect: invalid` (gate fidelity only).
- harbor gains broken plans under `fixtures/packs/harbor/broken/`, one for each rule its tool
  counts that `story:kpi-quality-values`' `no_quay.yaml` does not already fail, each failing that
  rule alone, with a recorded answer each under `fixtures/packs/harbor/tool/answers/`. The class
  and the harbor catalogue are not changed here.
- The baselines of the two measures, `quality/baselines/<pack>/{coverage.rules,gate_fidelity}.json`
  where the reading is a value, written by the acceptance run's ratchet and committed here.

## Surfaces

- filled (stubs of `story:kpi-quality-catalogue-shape`):
  `crates/metaspec-core/src/quality/kpis/rules.rs`,
  `crates/metaspec-core/tests/cases/quality/rules.rs`
- new: broken plans in `fixtures/packs/harbor/broken/` and their answers in
  `fixtures/packs/harbor/tool/answers/`, their pages and scenes in
  `fixtures/expected/harbor/pages/broken/` and `fixtures/expected/harbor/scenes/broken/`,
  `fixtures/expected/{harbor,orchard}/quality/{coverage.rules,gate_fidelity}.json`,
  `quality/baselines/{harbor,orchard,orgspec,agenticspec}/{coverage.rules,gate_fidelity}.json`,
  `scripts/acceptance.d/kpi-quality-rules.sh`
- changed: `fixtures/expected/harbor.json` (document counts),
  `fixtures/expected/harbor/pages/index.html` (the new plans), and where the new plans move them
  `fixtures/expected/harbor/quality/{alive,discrimination,redundancy}.json` and
  `quality/baselines/harbor/{alive,discrimination,redundancy}.json`

## Shared files

- `fixtures/packs/harbor/broken/`, `fixtures/packs/harbor/tool/answers/`,
  `fixtures/expected/harbor.json`, `fixtures/expected/harbor/pages/index.html`,
  `fixtures/expected/harbor/pages/broken/`, `fixtures/expected/harbor/scenes/broken/`: also
  changed by `story:kpi-quality-values`, which this story depends on. The harbor broken plans of
  both stories are one set in one class: values adds the class and `no_quay.yaml`, this story the
  rest.
- `fixtures/expected/harbor/quality/{alive,discrimination,redundancy}.json` and
  `quality/baselines/harbor/{alive,discrimination,redundancy}.json`: written by
  `story:kpi-quality-values`, which this story depends on. The new plans are documents of harbor,
  so where they move those values this story regenerates the three goldens and resets the three
  harbor baselines (a fixture change, not a regression; named with old and new values in its
  report).
- `story:spec-quality-refusal` reads the final set of harbor broken plans and depends on this
  story.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- Kpi → Rule, zero or one rule per KPI, by equal id — inferable (inferred from orgspec:
  `orgspec check --json` `kpis` is keyed by rule ids, 19 of 35 equal to ids of catalogue rows of
  status `checked`; `scripts/check-kpis.sh:14`; no ess/1 document declares a relation between
  `metaspec.blocks.Kpi` and `metaspec.checks.Rule`).
- Finding → Rule, many-to-one, absent for a rule metaspec has no row of; this story reads the rule
  id as the tool writes it — inferable (`checks.yaml`, `metaspec.checks.Finding`, relation `rule`,
  via Optional `rule_id`).
- CheckResult → Document and CheckResult → Finding (owns) — inferable (`checks.yaml`,
  `metaspec.checks.CheckResult`, relations `document`, `findings`).
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
`metaspec quality kpis` for orgspec printing coverage.rules over every rule id its 38 documents
report and gate fidelity over its gating KPIs whose rule one of its 30 broken designs fails alone,
both as values on harbor, and as gaps naming the missing input on orchard and agenticspec.
