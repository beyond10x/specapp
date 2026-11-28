---
format: aep.planning-md/3
id: story:search-dominance
kind: story
status: active
title: 'Dominance: the better candidate, and best and front in metaspec-result/1'
summary: 'search::order dominates and front over feasible scores; the one story that changes metaspec-result-1.schema.json after the format story: best, front, strategy, beam_width.'
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-design
- depends_on: story:search-gate
- depends_on: story:search-score
- depends_on: story:search-result-format
- depends_on: story:search-enumerate
- informed_by: decision-blocker:kpi-order
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: inferred
  path: crates/metaspec-core/src/search/mod.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/order.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/run.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/mod.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/search_order.rs
- confidence: cited
  path: design/metaspec.md
- confidence: inferred
  path: fixtures/expected/search/harbor.json
- confidence: inferred
  path: fixtures/expected/search/orchard.json
- confidence: cited
  path: fixtures/expected/site
- confidence: inferred
  path: schema/metaspec-result-1.schema.json
- confidence: cited
  path: spec/domains/runs.yaml
revision: 12
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:12Z", actor: "human:operator", revision: 11}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:12Z", actor: "human:operator", revision: 12}
---
# Dominance: the better candidate, and best and front in metaspec-result/1

## Context

A score is "ordered by the catalogue direction per KPI and its gate". `decision-blocker:kpi-order` is
answered with option A, dominance: a candidate B is better than a candidate A when every KPI with a
direction is at least as good in B and one is better; candidates of which neither dominates the other are
all kept as the front. No weight and no priority are invented. Bounded enumeration
(`story:search-enumerate`) lists every scored candidate and picks none. This story gives the order and
writes it into the result; it owns every change of `schema/metaspec-result-1.schema.json` after
`story:search-result-format`, so that the strategy stories change no schema.

## Scope

- `search::order` in `crates/metaspec-core/src/search/order.rs`, on the per-KPI verdicts of
  `search::score::compare(reference, candidate)` (`story:search-score`, which gives verdicts only: the
  verdicts of the candidate against the reference). `search::order::dominates(a, b)` is true when
  `compare(b, a)` has no verdict `worse` and at least one `better`; a KPI with direction `none` or one
  answered by only one of the two (`not_ordered`) is left out. `search::order::better(a, b)` is the whole
  order: a feasible score (`search::score::feasible` of `story:search-gate`) beats an infeasible one; two
  feasible scores compare by `dominates`; among infeasible scores one that did not run (state other than
  `ran`, a `simulate` that timed out included) ranks below one that ran, then fewer gated findings win,
  then fewer failed gates (`gate_failed`); anything else is no order. The order among infeasible candidates is
  the coordinator completion under the accepted epic (answer A of `decision-blocker:kpi-order` orders valid
  candidates only), as the gate rule for a missing value is in `story:search-gate`. An infeasible candidate is
  never better than a feasible one and is in no front.
- `search::order::front(candidates)`: the non-dominated set of a list of scored candidates, the feasible ones only,
  in list order; it is the one function of the front. The `front` of the result is `search::order::front` of the
  baseline followed by every variant, and `story:search-beam` calls it on the candidates of each round and states
  no filter of its own. In the result, the set is the non-dominated set over the baseline and every feasible
  variant, in evaluation order, the baseline first. A member that is a variant is written as its id; the baseline, when no variant dominates
  it and it is feasible, stays a member and is written with the id `baseline` (no variant id clashes with it). Two members with
  equal values are both in it. `best`: the id of the one variant when the front is exactly one variant and either it dominates the baseline or the
  baseline is infeasible (an infeasible baseline is never in the front, and a lone feasible variant ranks above it by
  `order::better`); otherwise absent. No weight, no priority list and no hidden tie-break
  (options B and C of the blocker are not built).
- Schema revision, `schema/metaspec-result-1.schema.json`, the format string stays `metaspec-result/1`:
  optional `best` (a variant id), optional `front` (a list of variant ids, or `baseline`), optional `strategy`
  (`enumerate`, `hill_climb`, `beam`; absent means `enumerate`), optional `beam_width` (integer, at least
  1); `stop.reason` is not touched, as `story:search-result-format` lists all four reasons; the strategy stories write
  `strategy` and `beam_width` and change no schema. A line under "Unreleased" in `CHANGELOG.md` names the file
  and the properties.
- `search::run` computes `front` and `best` once at the end of any strategy and writes them; the
  recorded results `fixtures/expected/search/<pack>.json` are re-recorded with them.
- The ESS model: `spec/domains/runs.yaml` types the run members `best` (references a variant,
  optional) and `front` (references variants), which the file today leaves untyped until the result has
  a schema; `ess specify validate` accepts it (`task check:spec`); the source is the answer of
  `decision-blocker:kpi-order`, and what it does not say is an `UNMAPPED:` marker.
- Not in this story: hill climbing and beam search (`story:search-hill-climb`, `story:search-beam`), the
  drawing of `best` and `front` (`story:search-result-best`), a priority order or weights.
- Its paragraph of `design/metaspec.md`: under "Decided: shared search" of `story:search-design`, one
  paragraph that states dominance, the front, `best` and the properties added to `metaspec-result/1`.

## Surfaces

- new: `crates/metaspec-core/src/search/order.rs`, `crates/metaspec-core/tests/cases/search_order.rs`.
- changed: `crates/metaspec-core/src/search/mod.rs`, `crates/metaspec-core/src/search/run.rs`,
  `schema/metaspec-result-1.schema.json`, `spec/domains/runs.yaml`, `fixtures/expected/search/harbor.json`,
  `fixtures/expected/search/orchard.json`, `fixtures/expected/site` (regenerated through `task check:docs`
  after the results are re-recorded), `design/metaspec.md` (one paragraph), `CHANGELOG.md`,
  `crates/metaspec-core/tests/cases/mod.rs` (one `mod` line).

## Shared files

- `schema/metaspec-result-1.schema.json`: created by `story:search-result-format`, which this story depends on;
  after this story no story of the epic changes it.
- `crates/metaspec-core/src/search/run.rs`: created by `story:search-enumerate`, which this story depends on;
  also changed by `story:search-gate`, which it depends on, and by the strategy stories, which depend on this one.
- `crates/metaspec-core/src/search/mod.rs`: created by `story:search-score`; a line here, as in the stories it depends on.
- `fixtures/expected/search/<pack>.json` and `fixtures/expected/site`: this story re-records the results and
  regenerates the site through `task check:docs`; also written by `story:search-gate`,
  `story:search-result-page` and `story:search-result-best`, and the site by `story:hub-badges`,
  `story:matrix-view`, `story:focus-pages`, `story:view-selectors` and `story:collapsed-regions`; the story
  that merges later regenerates it, and the order of the waves is the planner call.
- `design/metaspec.md`: one paragraph after `story:search-design`, which it depends on.
- `spec/domains/runs.yaml`: no other story of this epic changes it.

## Domain relations

- Run → Variant `front`, one-to-many, a reference and not an ownership (the run owns its variants through
  `variants`; the member `baseline` is the baseline of the run, not a variant): not stated in `spec/domains/runs.yaml` today; the source is `decision-blocker:kpi-order`
  (answer A keeps the non-dominated candidates as the front), and this story writes it there.
- Run → Variant `best`, zero or one, a reference: the same source, written there.

## Constraints

- No word of a pack under `crates/`; the order is computed from the KPI values and directions of the
  score, nothing else.
- Tests: the recorded scores of both fixture packs and hand-written scores (a dominating, a dominated, an
  incomparable pair, an infeasible one); no tool run, no model call, no network. No test over 60 s, the
  test step within 120 s; a test over 5 s is named with its time. A slot job ends within 10 minutes.
  No Python.

## Acceptance

`cargo nextest run -p metaspec-core search_order` passes, on hand-written scores of one baseline:
- `front`: an incomparable pair (one variant better on the first KPI and worse on the second, the other the reverse) has
  both variants in `front`; a dominated variant and an infeasible variant (a failed gate) are in no front; a variant
  that dominates the baseline leaves it out of `front`, and one that no variant dominates leaves it in as `baseline`;
  `best` is set only for a front of exactly one variant that dominates a feasible baseline or stands beside an infeasible
  one (two cases, each with a hand-written score), and absent for a front of two or more variants; the same function gives the front of a list that
  holds no baseline.
- `order::better`: a feasible score beats an infeasible one; among infeasible scores one that ran beats one that did
  not run, then fewer gated findings beat more, then fewer failed gates beat more; two feasible scores that are
  incomparable have no order.
- The re-recorded `fixtures/expected/search/<pack>.json` validate against `schema/metaspec-result-1.schema.json` with
  the same `front` the test computes.
