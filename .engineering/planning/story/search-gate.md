---
format: aep.planning-md/3
id: story:search-gate
kind: story
status: active
title: 'The gate test of a KPI value: a candidate worse than the original on a gated KPI fails'
summary: 'search::score::gate_failed and feasible: the original is the floor; optional gate_failed in metaspec-score/1.'
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-design
- depends_on: story:search-score
- depends_on: story:search-result-format
- depends_on: story:search-enumerate
- informed_by: decision-blocker:gate-threshold
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: inferred
  path: crates/metaspec-core/src/search/run.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/score.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/mod.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/search_gate.rs
- confidence: cited
  path: design/metaspec.md
- confidence: inferred
  path: fixtures/expected/search/gate
- confidence: inferred
  path: fixtures/expected/search/harbor.json
- confidence: inferred
  path: fixtures/expected/search/orchard.json
- confidence: cited
  path: fixtures/expected/site
- confidence: inferred
  path: fixtures/search/stub-check-gate.sh
- confidence: inferred
  path: schema/metaspec-score-1.schema.json
revision: 13
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:11Z", actor: "human:operator", revision: 12}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:11Z", actor: "human:operator", revision: 13}
---
# The gate test of a KPI value: a candidate worse than the original on a gated KPI fails

## Context

`epic:shared-search`: a candidate that fails `check` or a gate is never better than a valid one.
Failing `check` is a state of the score (`story:search-score`). A catalogue row has `gate` as a Boolean
and no threshold, so nothing said when a gated KPI value fails. `decision-blocker:gate-threshold` is
answered with option A: a candidate fails the gate of a gated KPI when its value is worse, in the
direction of the KPI, than the value of the original document; the original is the floor. No new format
of a pack and no threshold are needed. This story builds that test and the word "feasible" the order of
candidates reads.

## Scope

- `search::score::gate_failed(baseline, candidate)` in `crates/metaspec-core/src/search/score.rs`: the KPI
  ids with `gate` true (the field of each KPI entry of a score) whose verdict of `search::score::compare`
  against the baseline is `worse`, and the gated KPIs the baseline answered and the candidate does not answer. The last case is the
  coordinator completion of answer A: a missing value cannot show that the candidate is no worse than the
  original, so it fails the gate.
  A KPI with direction `none` never fails: nothing is worse without a direction. Computed once, in Rust.
- `search::score::feasible(score)`: state `ran`, `valid` true and `gate_failed` empty. The score of a
  variant carries it as the optional Boolean `feasible` (`metaspec-score/1`), written by the search run beside
  `gate_failed`; no other code or page restates the predicate. A candidate that is not feasible stays in the
  result with its score; the order in which it ranks is `story:search-dominance`, which reads `feasible` and
  never ranks an infeasible candidate above a feasible one. A candidate whose
  `simulate` timed out has no `ran` state and is not feasible.
- The score gains the optional properties `gate_failed` (the failed KPI ids, omitted when empty) and
  `feasible` (Boolean, written for every scored variant):
  `schema/metaspec-score-1.schema.json`, the format string stays `metaspec-score/1`; the score of the
  baseline has none. A page reads it and computes nothing. `search::run` writes it into the score of each
  variant beside `verdicts`; the recorded results `fixtures/expected/search/<pack>.json` are re-recorded
  with it (the stubs compute their values from the marked values, so a candidate moves the gated KPI of
  each fixture pack the wrong way).
- The test scores, hand-written `metaspec-score/1` files in `fixtures/expected/search/gate/`, each with state `ran`,
  `valid` true, `gated` 0 and no findings, and two KPIs: `g` (direction `higher`, `gate` true) and `h` (direction
  `lower`, `gate` false). `baseline.json`: g = 5, h = 2. `worse.json`: g = 4, h = 1 (verdict of `g` `worse`, of `h`
  `better`): `gate_failed` `[g]`, `feasible` false. `missing.json`: only h = 2, no entry for `g`: `gate_failed` `[g]`,
  `feasible` false. `equal.json`: g = 5, h = 2: no `gate_failed`, `feasible` true. `better.json`: g = 6, h = 3 (`g` `better`,
  `h` `worse` but not gated): no `gate_failed`, `feasible` true.
- The stub of this story, `fixtures/search/stub-check-gate.sh <pack>`, on the one marked place of the fixture document (not the grid
  of `story:search-hill-climb`, which this story does not depend on): harbor `draught` of `barge` (original 3m, range 2m, 3m, 4m)
  and orchard `hands` of `picker` (original 2, range 1, 2, 3). It answers valid true, no finding and one KPI, the gated
  lower-is-better KPI of the pack (harbor `vessel_unmoored_count`, orchard `basket_overflow_count`), with the value 2 at the
  original, 1 at the first other value (harbor 2m, orchard 1; point 1) and 3 at the second (harbor 4m, orchard 3; point 2).
  `metaspec search ... --seed 7` over it scores 2 candidates and ends `exhausted` with `runs` 3: v1 = point 1 has the value 1 (verdict
  `better`), no `gate_failed`, `feasible` true; v2 = point 2 has the value 3 (verdict `worse`), `gate_failed` the KPI id,
  `feasible` false.
- The re-record of `fixtures/expected/search/<pack>.json` runs the stubs of `fixtures/search/` created and extended by
  `story:search-enumerate`, `story:search-simulate-adapter` and `story:search-batch-scoring`; this story depends on those three through
  `story:search-design`, which depends on all of them; the only stub it creates is `stub-check-gate.sh`.
- Not in this story: the order of candidates, `best` and `front` (`story:search-dominance`), a threshold
  per KPI or a Boolean gate at 0 (options B and C of the blocker, not chosen), the page.
- Its paragraph of `design/metaspec.md`: under "Decided: shared search" of `story:search-design`, one
  paragraph that states the gate test (the original is the floor) and the word feasible.

## Surfaces

- changed: `crates/metaspec-core/src/search/score.rs`, `crates/metaspec-core/src/search/run.rs`,
  `schema/metaspec-score-1.schema.json`, `fixtures/expected/search/harbor.json`,
  `fixtures/expected/search/orchard.json`, `fixtures/expected/site` (regenerated through `task check:docs`
  after the results are re-recorded), `design/metaspec.md` (one paragraph), `CHANGELOG.md`,
  `crates/metaspec-core/tests/cases/mod.rs` (one `mod` line).
- new: `crates/metaspec-core/tests/cases/search_gate.rs`, `fixtures/search/stub-check-gate.sh`,
  recorded results `fixtures/expected/search/gate/harbor-search.json` and `orchard-search.json`, `fixtures/expected/search/gate/baseline.json`, `worse.json`,
  `missing.json`, `equal.json` and `better.json` in the same directory.

## Shared files

- `crates/metaspec-core/src/search/score.rs`: created by `story:search-score`, which this story depends on.
- `schema/metaspec-score-1.schema.json`: created by `story:search-score` and extended by `story:search-result-format`
  (`verdicts`) and `story:search-simulate-adapter` (`source`, `per_part`), all of which come before this story: this
  story depends on the first two and reaches the third through `story:search-design`.
- `crates/metaspec-core/src/search/run.rs`: created by `story:search-enumerate` and changed before this story by
  `story:search-simulate-adapter` and `story:search-batch-scoring`, which this story reaches through `story:search-design`;
  changed after it by the strategy stories.
- `fixtures/expected/search/<pack>.json` and `fixtures/expected/site`: this story re-records the results and
  regenerates the site through `task check:docs`; also written by `story:search-dominance`,
  `story:search-result-page` and `story:search-result-best`, and the site by `story:hub-badges`,
  `story:matrix-view`, `story:focus-pages`, `story:view-selectors` and `story:collapsed-regions`; the story
  that merges later regenerates it, and the order of the waves is the planner call.
- `design/metaspec.md`: this story adds one paragraph after `story:search-design`, which it depends on;
  the other later stories of the epic each add their own.
- `crates/metaspec-core/tests/cases/mod.rs`, `CHANGELOG.md`: one line each.

## Domain relations

- KpiValue → Kpi, many-to-one: inferable (`spec/domains/runs.yaml`, `metaspec.runs.KpiValue`, relation
  `kpi`). The Boolean `gate` of a catalogue row is read from `metaspec.blocks.Kpi`; no relation is added.

## Constraints

- No word of a pack under `crates/`; metaspec implements no rule: every KPI value is the answer of the tool.
- Tests: stub tools of the fixture packs only, in a copy of the fixture manifest; no real pack tool,
  no model call, no network. No test over 60 s, the test step within 120 s; a test over 5 s is named with
  its time. A slot job ends within 10 minutes. No Python.

## Acceptance

`cargo nextest run -p metaspec-core search_gate` passes:
- on the hand-written scores against `baseline.json`: `worse.json` has `g` in `gate_failed` and `feasible` false;
  `missing.json` has `g` in `gate_failed` (the baseline answered it, the candidate does not) and `feasible` false;
  `equal.json` and `better.json` have no `gate_failed` and `feasible` true;
- for both fixture packs, `metaspec search ... --seed 7` with `stub-check-gate.sh` writes `fixtures/expected/search/gate/<pack>-search.json`
  (byte-equal on a second run) in which v1 (value 1) has `feasible` true and no `gate_failed` and v2 (value 3) has
  `gate_failed` the KPI id and `feasible` false; and the re-recorded `fixtures/expected/search/<pack>.json` carry `feasible` on
  every variant and `gate_failed` on every variant that fails a gate, and all validate against `schema/metaspec-score-1.schema.json` and
  `schema/metaspec-result-1.schema.json`.
