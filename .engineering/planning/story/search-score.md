---
format: aep.planning-md/3
id: story:search-score
kind: story
status: active
title: 'Score a document through check: validity, findings and KPI values in the direction of the catalogue'
summary: metaspec score prints a metaspec-score/1 of one document from the check adapter; first for agenticspec; adds kpis.directions to the manifest.
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:cloudspec-manifest
- depends_on: story:extends-inherits
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/lib.rs
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/mod.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/score.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/mod.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/search_score.rs
- confidence: cited
  path: crates/metaspec/src/main.rs
- confidence: inferred
  path: crates/metaspec/src/score.rs
- confidence: inferred
  path: design/metaspec.md
- confidence: cited
  path: fixtures/answers/agenticspec/architecture
- confidence: cited
  path: fixtures/manifests/harbor.yaml
- confidence: cited
  path: fixtures/manifests/orchard.yaml
- confidence: cited
  path: packs/agenticspec.yaml
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: inferred
  path: schema/metaspec-score-1.schema.json
- confidence: inferred
  path: scripts/acceptance.d/search-score.sh
- confidence: inferred
  path: scripts/replay-answers.sh
- confidence: inferred
  path: scripts/replay-answers.test.sh
revision: 26
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:10Z", actor: "human:operator", revision: 24}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:10Z", actor: "human:operator", revision: 25}
---
# Score a document through check: validity, findings and KPI values in the direction of the catalogue

## Context

The first gap of the first consumer of `epic:shared-search`: agenticspec main has a `check` adapter
(`packs/agenticspec.yaml`, `tool.verbs.check`: `valid`, `counts`, `kpis: /kpis`, the list
`/violations`) and its architectures mark values with `{value, vary}`, but nothing reads a check
result as a score. `metaspec_core::tool::check` (`crates/metaspec-core/src/tool.rs`) returns a
`Check` with `Findings { valid, findings, counts, kpis }`; only `metaspec render` and
`metaspec docs build` call it, to draw. No command checks one document alone. A search compares
candidates by score, so the score comes first and is shown on its own.

## Scope

- `metaspec score <NAME | --manifest FILE> --pack <DIR> <DOCUMENT> [--json] [--deadline SECONDS]`.
  The document is a path inside the pack (refused when it leaves it, as every manifest path is).
  The tool runs only for a trusted pack (`metaspec trust`); an untrusted pack, a refusal, a
  timeout or a failed run exits 1 and prints the state of the check, as `render` shows it.
  Exit 0 when the document is scored, 2 on bad usage.
- A score, `metaspec-score/1` (new, `schema/metaspec-score-1.schema.json`): `state` (the check
  states of `crates/metaspec-core/src/tool.rs`: `ran`, `timed_out`, `refused`, `failed`, ...),
  and for state `ran`: `valid`, `gated` (the count of gated findings), `findings` (as
  `metaspec-findings/1`), and `kpis`, one entry per KPI value the check answered: `id`, `value`,
  `direction` (`higher`, `lower` or `none`) and `gate` (Boolean, from the catalogue row; shown,
  not tested). A KPI id the catalogue does not have has direction `none`.
- Comparing two scores, `search::score::compare(reference, candidate)` in
  `crates/metaspec-core/src/search/score.rs`: one verdict per KPI present in both, `better`, `worse` or
  `same` in the direction of the KPI, and `not_ordered` for direction `none` or a KPI in one only; the
  verdicts of the candidate against the reference, and nothing else. The order of whole scores (a feasible
  score beats an infeasible one; among infeasible ones fewer gated findings, then fewer failed gates; how the
  verdicts combine) is `search::order` of `story:search-dominance`; the gate test of a KPI value is
  `story:search-gate`. This story builds neither.
- The words of a pack for a direction: new optional `kpis.directions` in the manifest,
  `{ higher: [words], lower: [words] }`. Absent, the words are `higher_is_better` and
  `lower_is_better` (the words of the catalogue of metaspec). A catalogue word in neither list is
  direction `none`; a word in both lists refuses the manifest (`manifest_invalid`).
  Schema revision: `schema/metaspec-pack-1.schema.json` gains the optional property
  `kpis.directions`; the format string stays `metaspec-pack/1`, as `source_file_url` did
  (`design/metaspec.md`, "Decided in wave 3"); the entry under "Unreleased" in `CHANGELOG.md`
  names the file and the property, so a pack repository can cite it. `packs/agenticspec.yaml`
  sets `directions: { higher: [higher], lower: [lower] }`; both fixture manifests set the same
  words. The row field read for a direction stays `direction`; a direction per mode is not used
  by a search.
- The real-checkout acceptance row this story adds, `scripts/acceptance.d/search-score.sh`: `metaspec
  score agenticspec --pack <agenticspec-dir> architectures/<each>.yaml --json` on a checkout of
  agenticspec main; each score is valid against its schema, `valid` and gated findings equal what
  `agenticspec sim check --json` reports, and each KPI value carries the direction `higher` or
  `lower` read from that catalogue. It runs outside `task check`.
- The verdict per KPI is computed once, in Rust, by `search::score::compare` (baseline, candidate);
  `story:search-result-format` carries it in the score of each variant, and no other code or page
  recomputes it.
- Not in this story: simulate (`story:search-simulate-adapter`), candidates (`story:search-candidate-space`),
  the page, any change to `design/metaspec.md` (`story:search-design` owns it).

## Surfaces

- new: `crates/metaspec-core/src/search/mod.rs`, `crates/metaspec-core/src/search/score.rs`,
  `crates/metaspec-core/tests/cases/search_score.rs` (and `mod search_score;` in
  `crates/metaspec-core/tests/cases/mod.rs`), `crates/metaspec/src/score.rs`,
  `schema/metaspec-score-1.schema.json`, `scripts/replay-answers.sh` (POSIX sh: prints
  `<case>.stdout.json` and exits with `<case>.exit` of a directory of recorded answers, for the
  case named by the document file name), `scripts/replay-answers.test.sh`,
  `scripts/acceptance.d/search-score.sh`, goldens `fixtures/answers/agenticspec/architecture/<case>.score.json`.
- changed: `crates/metaspec-core/src/lib.rs` (`pub mod search;`), `crates/metaspec-core/src/manifest.rs`
  (`Kpis.directions`), `schema/metaspec-pack-1.schema.json`, `crates/metaspec/src/main.rs`
  (`Command::Score`), `packs/agenticspec.yaml`, `fixtures/manifests/harbor.yaml`,
  `fixtures/manifests/orchard.yaml`, `CHANGELOG.md`.

## Shared files

- `crates/metaspec-core/src/manifest.rs`, and
  `schema/metaspec-pack-1.schema.json`: also changed by `story:cloudspec-manifest` (this story
  depends on it, so it lands first), and by `story:view-selectors`, `story:hub-badges`,
  `story:matrix-view`, `story:focus-pages` and `story:collapsed-regions`; the view stories run
  before or after this one, the order of the waves is the planner call.
- `crates/metaspec-core/src/search/mod.rs` and `crates/metaspec/src/main.rs`: later stories of the
  epic add modules and commands; each depends on this story.
- `crates/metaspec-core/tests/cases/mod.rs`: one `mod` line per test module.
- `fixtures/manifests/harbor.yaml` and `fixtures/manifests/orchard.yaml`: also changed by draft
  `story:kpi-quality-values` and draft `story:spec-quality-drift`, drafts of other epics; the coordinator
  orders the waves when they are accepted. No edge to drafts.
- `crates/metaspec-core/src/manifest.rs` and `schema/metaspec-pack-1.schema.json`: also written by draft
  `story:kpi-quality-values`, draft `story:spec-quality-conformance` and draft `story:spec-quality-drift` (drafts of other epics; the coordinator orders the waves when they are accepted; no edge to drafts).

## Domain relations

- KpiValue → Kpi (the catalogue row of an id), many-to-one, the row may be absent for an id the
  catalogue does not have: inferable (`spec/domains/runs.yaml`, `metaspec.runs.KpiValue`,
  relation `kpi`, field `kpi_id` of type `Optional<metaspec.blocks.KpiId>`).

## Constraints

- No word of a pack under `crates/` (`scripts/check-vocabulary.sh`); metaspec implements no rule
  and no simulation: every value comes from the tool, read by the adapter.
- Tests: harbor and orchard through stub tools, agenticspec through recorded answers
  (`fixtures/answers/agenticspec/architecture/`) replayed by `scripts/replay-answers.sh` in a
  copy of `packs/agenticspec.yaml` whose `tool.run` the test replaces. No real pack tool, no
  model call, no network. No test over 60 s, the test step within 120 s
  (`.config/nextest.toml`); a test over 5 s is named with its time. A slot job ends within 10
  minutes (`slot --timeout`, at most 30m). No Python: the replay is POSIX sh.
- The agenticspec catalogue is not committed here; the directions of agenticspec are shown by
  the acceptance row on a checkout of agenticspec main, outside `task check`.
- Public repository: nothing of a real pack beyond `packs/` and recorded answers of invented inputs.

## Acceptance

`cargo nextest run -p metaspec-core -p metaspec search_score` passes: its cases print a
`metaspec-score/1` valid against `schema/metaspec-score-1.schema.json` for both fixture packs and
for the recorded answers of agenticspec (a valid and a refused architecture), with the direction
`higher` or `lower` of each harbor KPI read through `kpis.directions`, without a real pack tool.
