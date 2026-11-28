---
format: aep.planning-md/3
id: story:kpi-quality-values
kind: story
status: draft
title: alive, discrimination and redundancy from the check of every document
summary: A check pass over every document through the adapter, broken-example classes in the manifest, and three measures of KPI values.
relations:
- decomposes: epic:kpi-quality
- depends_on: story:kpi-quality-catalogue-shape
- depends_on: story:quality-ratchet
scope:
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/check_pass.rs
- confidence: cited
  path: crates/metaspec-core/src/quality/kpis/values.rs
- confidence: cited
  path: crates/metaspec-core/src/report.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/quality/values.rs
- confidence: cited
  path: fixtures/expected/harbor.json
- confidence: cited
  path: fixtures/expected/harbor/pages/broken
- confidence: cited
  path: fixtures/expected/harbor/pages/index.html
- confidence: inferred
  path: fixtures/expected/harbor/pages/plan
- confidence: cited
  path: fixtures/expected/harbor/quality/alive.json
- confidence: cited
  path: fixtures/expected/harbor/quality/discrimination.json
- confidence: cited
  path: fixtures/expected/harbor/quality/redundancy.json
- confidence: cited
  path: fixtures/expected/harbor/scenes/broken
- confidence: cited
  path: fixtures/expected/orchard.json
- confidence: cited
  path: fixtures/expected/orchard/quality/alive.json
- confidence: cited
  path: fixtures/expected/orchard/quality/discrimination.json
- confidence: cited
  path: fixtures/expected/orchard/quality/redundancy.json
- confidence: cited
  path: fixtures/manifests/harbor.yaml
- confidence: cited
  path: fixtures/packs/harbor/broken
- confidence: cited
  path: fixtures/packs/harbor/tool/answers
- confidence: cited
  path: packs/orgspec.yaml
- confidence: cited
  path: quality/baselines/agenticspec/alive.json
- confidence: cited
  path: quality/baselines/agenticspec/discrimination.json
- confidence: cited
  path: quality/baselines/agenticspec/redundancy.json
- confidence: cited
  path: quality/baselines/harbor/alive.json
- confidence: cited
  path: quality/baselines/harbor/discrimination.json
- confidence: cited
  path: quality/baselines/harbor/redundancy.json
- confidence: cited
  path: quality/baselines/orchard/alive.json
- confidence: cited
  path: quality/baselines/orchard/discrimination.json
- confidence: cited
  path: quality/baselines/orchard/redundancy.json
- confidence: cited
  path: quality/baselines/orgspec/alive.json
- confidence: cited
  path: quality/baselines/orgspec/discrimination.json
- confidence: cited
  path: quality/baselines/orgspec/redundancy.json
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: schema/metaspec-pack-check-1.schema.json
- confidence: cited
  path: scripts/acceptance.d/kpi-quality-values.sh
- confidence: cited
  path: scripts/acceptance.sh
revision: 14
---
# alive, discrimination and redundancy from the check of every document

## Context

Three measures of `epic:kpi-quality` that read the value a pack's tool reports for each KPI on
each document. The values come from the check adapter of the manifest, so the same code serves
every pack that has one; a pack without one has gaps. This story also makes broken examples
documents of the pack (`expect: invalid`), which `story:kpi-quality-rules` and
`story:spec-quality-refusal` read.

## Scope

- A check pass, filling the stub `crates/metaspec-core/src/quality/check_pass.rs`: the check
  adapter of the manifest run on every document of every `parts-links/1` class, as
  `metaspec render` runs it (trust required, deadline, the adapter's mapping), once per run of
  the command.
- The value of a KPI on a document: the entry of the check's `counts` whose key equals the KPI id.
- alive: KPIs with a value on at least one document, divided by KPIs; behind: the others.
- discrimination: alive KPIs whose value differs between documents, divided by alive KPIs;
  behind: the flat KPIs.
- redundancy: the number of pairs of KPIs that are not flat and agree on every document; behind:
  both KPIs of each pair.
- Gaps, each naming the missing input: no check adapter (agenticspec today, its checker prints
  text); a check adapter without `counts` (orchard); a tool the user has not trusted (naming
  `metaspec trust`).
- Broken examples: an optional field of a document class in `metaspec-pack/1`, `expect: invalid`
  (the documents of the class are examples meant to fail a rule). `metaspec pack check --json`
  counts the documents of such classes as `counts.broken_documents`.
- `packs/orgspec.yaml` gains a class for `organizations/broken/*.yaml` with `expect: invalid`.
  `scripts/acceptance.sh` then reads the broken designs with `packs/orgspec.yaml` and drops the
  manifest it derives with `sed` for them.
- harbor: `fixtures/manifests/harbor.yaml` gains a class `broken` over `broken/*.yaml` with
  `expect: invalid`, and harbor gains one broken plan `fixtures/packs/harbor/broken/no_quay.yaml`
  with its tool answer `fixtures/packs/harbor/tool/answers/no_quay.json` (failing
  `berth_without_quay` alone). The recorded answers `inlet.json` and `reef.json` count the harbor
  KPI `vessel_unmoored_count` in their `tally`, so at least one harbor KPI has a value. The
  harbor catalogue is not changed here (it is `story:kpi-quality-grounding`'s).
- The baselines of the three measures, `quality/baselines/<pack>/{alive,discrimination,redundancy}.json`
  where the reading is a value, written by the acceptance run's ratchet and committed here.

## Surfaces

- filled (stubs of `story:kpi-quality-catalogue-shape`):
  `crates/metaspec-core/src/quality/check_pass.rs`,
  `crates/metaspec-core/src/quality/kpis/values.rs`,
  `crates/metaspec-core/tests/cases/quality/values.rs`
- changed: `crates/metaspec-core/src/manifest.rs` and `schema/metaspec-pack-1.schema.json` (field
  `expect`), `crates/metaspec-core/src/report.rs` and `schema/metaspec-pack-check-1.schema.json`
  (`counts.broken_documents`), `packs/orgspec.yaml` (the broken class), `scripts/acceptance.sh`
  (the `sed` manifest dropped), `fixtures/manifests/harbor.yaml`,
  `fixtures/packs/harbor/tool/answers/inlet.json`, `fixtures/packs/harbor/tool/answers/reef.json`,
  `fixtures/expected/harbor.json` and `fixtures/expected/orchard.json` (the new count),
  `fixtures/expected/harbor/pages/index.html` (the new class and plan),
  `fixtures/expected/harbor/pages/plan/` (inferred: the inlet and reef pages carry the tally)
- new: `fixtures/packs/harbor/broken/no_quay.yaml`,
  `fixtures/packs/harbor/tool/answers/no_quay.json`, `fixtures/expected/harbor/pages/broken/`,
  `fixtures/expected/harbor/scenes/broken/`,
  `fixtures/expected/{harbor,orchard}/quality/{alive,discrimination,redundancy}.json`,
  `quality/baselines/{harbor,orchard,orgspec,agenticspec}/{alive,discrimination,redundancy}.json`,
  `scripts/acceptance.d/kpi-quality-values.sh`

## Shared files

- `crates/metaspec-core/src/manifest.rs`, `schema/metaspec-pack-1.schema.json`, `packs/orgspec.yaml`:
  also changed by `story:spec-quality-conformance` (field `blocks.known_failing`), which depends on
  this story, and the first two by `story:spec-quality-drift` (verb `regenerate`), which depends on
  conformance.
- `fixtures/packs/harbor/broken/`, `fixtures/packs/harbor/tool/answers/`,
  `fixtures/expected/harbor.json`, `fixtures/expected/harbor/pages/` (`index.html`, `plan/`,
  `broken/`) and `fixtures/expected/harbor/scenes/broken/`: also changed by
  `story:kpi-quality-rules` (more broken plans), which depends on this story. Where those plans
  move the harbor values of this story's measures, rules also regenerates
  `fixtures/expected/harbor/quality/{alive,discrimination,redundancy}.json` and resets
  `quality/baselines/harbor/{alive,discrimination,redundancy}.json`.
- `scripts/acceptance.sh`: also changed by `story:kpi-quality-catalogue-shape` (sources the row
  files), which this story depends on.

## Domain relations

ESS spec: `spec/domains/` (lands with the spec pull request).

- CheckResult → Document, many-to-one — inferable (`checks.yaml`, `metaspec.checks.CheckResult`,
  relation `document`).
- CheckResult → Finding, one-to-many, result owns — inferable (`checks.yaml`,
  `metaspec.checks.CheckResult`, relation `findings`).
- Document → DocumentClass, many-to-one — inferable (`documents.yaml`,
  `metaspec.documents.Document`, relation `class`).
- Kpi → Rule, a KPI counts the rule whose count the tool reports under the KPI id, zero or one
  rule per KPI — inferable (inferred from orgspec: `orgspec check --json` reports `kpis` keyed by
  rule ids, 19 of its 35 keys equal ids of catalogue rows of status `checked`, and
  `scripts/check-kpis.sh:14` says such a KPI is the one "`orgspec check` counts from a design";
  no ess/1 document declares a relation between `metaspec.blocks.Kpi` and
  `metaspec.checks.Rule`).

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

`task acceptance -- <orgspec-dir> <agenticspec-dir>`, with no manifest of orgspec derived by the
script, exits 0 with rows showing `metaspec pack check orgspec --json` counting the 30 files of
`organizations/broken/*.yaml` as `broken_documents` of the class `packs/orgspec.yaml` marks
`expect: invalid`, harbor counting its one broken plan the same way, and `metaspec quality kpis`
over orgspec's 38 documents printing alive equal to the catalogue ids among the keys of
`orgspec check --json` `kpis` (19 of 111 on 2026-11-28), discrimination and redundancy as values
there and on harbor, and all three as gaps naming the missing counts on orchard and the missing
check adapter on agenticspec.
