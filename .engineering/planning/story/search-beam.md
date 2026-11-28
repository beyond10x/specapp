---
format: aep.planning-md/3
id: story:search-beam
kind: story
status: active
title: 'Beam search: a seeded search that keeps the front of each round as the next starting set'
summary: 'metaspec search --strategy beam --beam-width: rounds over the neighbours of the beam, cut by the seeded order, ended by search::run::stop_reason.'
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-design
- depends_on: story:search-hill-climb
- depends_on: story:search-dominance
- informed_by: decision-blocker:kpi-order
- informed_by: decision-blocker:search-edits
- depends_on: story:search-simulate-adapter
- depends_on: story:search-batch-scoring
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: inferred
  path: crates/metaspec-core/src/search/beam.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/mod.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/run.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/mod.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/search_beam.rs
- confidence: inferred
  path: crates/metaspec/src/search.rs
- confidence: cited
  path: design/metaspec.md
- confidence: inferred
  path: fixtures/expected/search/beam
revision: 13
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:12Z", actor: "human:operator", revision: 12}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:12Z", actor: "human:operator", revision: 13}
---
# Beam search: a seeded search that keeps the non-dominated candidates of each round as the next beam

## Context

Hill climbing (`story:search-hill-climb`) follows one candidate and stops at the first one with no
better neighbour. Beam search keeps several candidates at once, so that a search can leave a local top.
The order is dominance (`decision-blocker:kpi-order`, answer A; `story:search-dominance`) and the
candidates are `set_field` edits on marked places only (`decision-blocker:search-edits`, answer A). The
neighbour function, the grid of the tests and the stub are those of `story:search-hill-climb`.

## Scope

- `metaspec search ... --strategy beam [--beam-width N]` (N at least 1, default 4, printed by `--help` and
  recorded as `beam_width`; the flag `--strategy` takes `beam` beside `enumerate` and `hill_climb`, in
  `crates/metaspec/src/search.rs`, created by `story:search-enumerate`). Budget, `--time-limit`, `--deadline`,
  `--seed`, `--out`, the refusals and the exit codes are those of `story:search-enumerate`. `--simulate` is
  owned by `story:search-simulate-adapter` and a declared `batch` by `story:search-batch-scoring`, both inside
  `search::run::score_points`; this story depends on both and changes neither.
- The run: score the original (the baseline); the beam is the baseline. Each round scores, through
  `search::run::score_points` (`crates/metaspec-core/src/search/run.rs`, `story:search-enumerate`), the one
  scoring path of every strategy, the neighbours (`search::neighbours::of`, `story:search-hill-climb`) of the beam
  members that are not scored yet, in ascending point order and each point once, so that `--simulate` and a
  declared `batch` apply as in bounded enumeration (one batch run per verb per round where the round fits the
  limit). One rule makes the next beam. When the round
  has at least one feasible candidate, the next beam is `search::order::front` (`story:search-dominance`) of the
  candidates scored in this round, in the order scored, and states no filter of its own; when that front is
  larger than `--beam-width` it is put in the order of `search::rng::shuffle` (`story:search-enumerate`, the stream
  continuing from round to round) and the first `--beam-width` are kept. When the round has no feasible candidate,
  the next beam is the first `--beam-width` candidates ranked by `search::order::better`: the candidates are put in
  the order of `search::rng::shuffle`, then up to `--beam-width` times the first remaining candidate that no other
  remaining candidate beats is taken, so ties stay in the seeded order. The baseline and the members of earlier
  beams are not candidates again.
- A round adds a new member to the beam when it has at least one candidate; a round with none makes no move (`moved`
  false). After each round the search calls `search::run::stop_reason(deadline_hit, scored, budget, moved, unscored)`
  (`story:search-enumerate`) and goes on only while it returns `None`; it decides no stop itself. Stop reasons: `search::run::stop_reason` and `StopReason`
  (`story:search-enumerate`) are the code, and "Decided: shared search" (`story:search-design`) writes the four reasons
  and their precedence once; this story copies neither. The result carries `strategy` `beam`, `beam_width`, and `front` and `best` as `story:search-dominance` defines them,
  over every scored variant; `runs` counts tool runs; `stop.evaluated` is the number of scored candidates. One seed on one
  document gives one result, byte for byte.
- Not in this story: a change of the schema (`story:search-dominance` owns it), the neighbour function
  (`story:search-hill-climb`), weights, edits other than `set_field` on marked places.
- Its paragraph of `design/metaspec.md`: under "Decided: shared search" of `story:search-design`, one
  paragraph that states the beam, its width, the round that scores the whole neighbourhood and the rule for a round
  with no feasible candidate, and says how the beam ends by naming the reason (`exhausted`, `no_better_neighbour`,
  `budget` or `deadline`) and no more; the reasons and their precedence are written once, by `story:search-design`.

## The test cases

The grid, the table `k(a, b)`, the point numbers, the stub `fixtures/search/stub-check-grid.sh` and the stream outputs of
seeds 1 and 2 are those of "The test grid" in `story:search-hill-climb` (16 points, original p0 with value 0, local tops of 3 at
p1, p2, p4, p5 and p8, global top p15 with 9). Variants are numbered in the order scored.

### Case 1: `--strategy beam --beam-width 2 --seed 2` (default budget)

- Baseline p0 (0): 1 run.
- Round 1, beam `[p0]`: the unscored neighbours are 1 and 4; scored (v1 = p1 = 3, v2 = p4 = 3). The front is `[p1, p4]`,
  2 members, not above the width: the next beam is `[p1, p4]`.
- Round 2: neighbours of p1 are 0, 2, 5 and of p4 are 0, 5, 8; the unscored ones, each once, are 2, 5, 8; scored (v3 = p2
  = 3, v4 = p5 = 3, v5 = p8 = 3). The front is `[p2, p5, p8]`: 3 members, more than the width 2, so it is shuffled with
  the outputs of seed 2: `k = 2`, `r1 = 10905525725756348110`, `r1 mod 3 = 1`, swap items 2 and 1: `[p2, p8, p5]`; `k = 1`,
  `r2 = 13819372491320860226`, `r2 mod 2 = 0`, swap items 1 and 0: `[p8, p2, p5]`. The first 2 are kept: the next beam is
  `[p8, p2]` and p5 is dropped (not the identity cut).
- Round 3: neighbours of p8 are 4, 9, 12 and of p2 are 1, 3, 6; unscored 3, 6, 9, 12: v6 = p3 = 0, v7 = p6 = 2, v8 = p9 = 1,
  v9 = p12 = 0. The front is `[p6]` (2 is the highest): the next beam is `[p6]`, below the 3 of the beam it leaves.
- Round 4: neighbours of p6 are 2, 5, 7, 10; unscored 7, 10: v10 = p7 = 1, v11 = p10 = 4; the next beam is `[p10]`.
- Round 5: neighbours of p10 are 6, 9, 11, 14; unscored 11, 14: v12 = p11 = 5, v13 = p14 = 6; the next beam is `[p14]`.
- Round 6: neighbours of p14 are 10, 13, 15; unscored 13, 15: v14 = p13 = 2, v15 = p15 = 9; the next beam is `[p15]`. Now all
  16 points are scored (the baseline and 15 variants): `stop_reason(false, 15, 50, true, 0)` is `Some(Exhausted)` and the search
  stops; there is no round 7.
- Result: 15 variants (2 + 3 + 4 + 2 + 2 + 2), `runs` 16, `stop.evaluated` 15. All 16 points are scored (the baseline and 15
  variants), so `stop.reason` is `exhausted`. `front` is `v15` (p15 = 9 dominates every other point) and `best` is `v15`.
  `strategy` `beam`, `beam_width` 2. Hill climbing with the same seed (case 2 of `story:search-hill-climb`) stops on p4 with 3;
  the beam left the local tops and reached 9.

### Case 2: `--strategy beam --beam-width 2 --seed 2 --budget 9`

- Rounds 1 and 2 are as in case 1: 5 variants scored (p1, p4, p2, p5, p8), the next beam `[p8, p2]`.
- Round 3 has 4 unscored neighbours (3, 6, 9, 12) and 4 of the budget left: all are scored (v6 = p3, v7 = p6, v8 = p9,
  v9 = p12), the budget 9 is reached with unscored points remaining: the search stops.
- Result: variants v1..v9 = p1, p4, p2, p5, p8, p3, p6, p9, p12; `runs` 10; `stop.reason` `budget`; `stop.evaluated` 9. p12 is
  scored only because the cut kept p8 (a cut that kept p2 and p5, as seed 1 does, would not reach it). `front` is
  `v1, v2, v3, v4, v5` (3 is the highest, none dominates another); `best` is absent.

### Case 3: `--strategy beam --beam-width 1 --seed 2` with `fixtures/search/stub-check-grid-invalid.sh` (a round with no feasible candidate)

- The baseline p0 answers valid false with 2 gated findings (infeasible); 1 run. Beam width 1.
- Round 1, beam `[p0]`: unscored neighbours 1 and 4 are scored (v1 = p1, v2 = p4), each valid false with 1 gated finding: the round has
  no feasible candidate, so the next beam is ranked by `search::order::better`. Shuffle `[p1, p4]`: `k = 1`, `r1 = 10905525725756348110`,
  `r1 mod 2 = 0`, swap: `[p4, p1]`. The first remaining candidate that no other beats is p4 (p1 has the same gated findings and failed
  gates, a tie, so the seeded order decides): the next beam is `[p4]`.
- Round 2: neighbours of p4 are 0, 5, 8; unscored 5, 8; scored (v3 = p5 = 3, v4 = p8 = 3, both valid). The front is `[p5, p8]`, above the
  width 1: shuffle `[p5, p8]` with `r2 = 13819372491320860226`, `r2 mod 2 = 0`, swap: `[p8, p5]`; the next beam is `[p8]`.
- Round 3: neighbours of p8 are 4, 9, 12; unscored 9, 12: v5 = p9 = 1, v6 = p12 = 0; the front is `[p9]`.
- Round 4: neighbours of p9 are 5, 8, 10, 13; unscored 10, 13: v7 = p10 = 4, v8 = p13 = 2; the front is `[p10]`.
- Round 5: neighbours of p10 are 6, 9, 11, 14; unscored 6, 11, 14: v9 = p6 = 2, v10 = p11 = 5, v11 = p14 = 6; the front is `[p14]`.
- Round 6: neighbours of p14 are 10, 13, 15; unscored 15: v12 = p15 = 9; the front is `[p15]`; `stop_reason(false, 12, 50, true, 3)` is `None`
  (p2, p3 and p7 are not scored).
- Round 7: the neighbours of p15 are 11 and 14, both scored: no candidate, no move: `stop_reason(false, 12, 50, false, 3)` is
  `Some(NoBetterNeighbour)`.
- Result: variants v1..v12 = p1, p4, p5, p8, p9, p12, p10, p13, p6, p11, p14, p15; `runs` 13; `stop.reason` `no_better_neighbour`;
  `stop.evaluated` 12. `front` is `v12` (the baseline, v1 and v2 are infeasible and in no front; p15 = 9 dominates every other
  feasible variant) and `best` is `v12`: the baseline is infeasible, so it is never in the front, and `v12` is the one front variant and
  ranks above the baseline by `order::better` (feasible beats infeasible), which is the second way `best` is set in
  `story:search-dominance`. `beam_width` 1.

## Surfaces

- new: `crates/metaspec-core/src/search/beam.rs`, `crates/metaspec-core/tests/cases/search_beam.rs`,
  recorded results `fixtures/expected/search/beam/harbor-seed2.json` and `fixtures/expected/search/beam/orchard-seed2.json`, `harbor-invalid.json` and `orchard-invalid.json` in the same directory.
- changed: `crates/metaspec-core/src/search/run.rs` (the call of the strategy beside the others),
  `crates/metaspec-core/src/search/mod.rs`, `crates/metaspec/src/search.rs` (the value of `--strategy` and
  `--beam-width`), `design/metaspec.md` (one paragraph), `CHANGELOG.md`,
  `crates/metaspec-core/tests/cases/mod.rs` (one `mod` line).

## Shared files

- `crates/metaspec/src/search.rs`, `crates/metaspec-core/src/search/run.rs`: created by
  `story:search-enumerate`; also changed by `story:search-hill-climb`, `story:search-simulate-adapter` and
  `story:search-batch-scoring`, which this story depends on.
- `crates/metaspec-core/src/search/mod.rs`: created by `story:search-score`; one line here.
- `fixtures/search/stub-check-grid.sh` and `fixtures/search/stub-check-grid-invalid.sh`: created by `story:search-hill-climb`; this story
  reads them and rewrites none.
- `design/metaspec.md`: one paragraph after `story:search-design`, which it depends on.
- `crates/metaspec-core/tests/cases/mod.rs`, `CHANGELOG.md`: one line each.

## Domain relations

- Run → Variant, one-to-many, owned: inferable (`spec/domains/runs.yaml`, `metaspec.runs.Run`, relation
  `variants`, kind `owns`). The run carries a budget of at least 1 and a deadline: inferable
  (`metaspec.runs.Run`, fields `budget` and `deadline`, invariant `budget >= 1`).

## Constraints

- No word of a pack under `crates/` (the stub and the test hold the names of the fixture packs); every score is the
  answer of the tool; the original document is never changed.
- Tests: the stub of the grid in a copy of the fixture manifest; no real pack tool, no model call, no network. No test
  over 60 s, the test step within 120 s; a test over 5 s is named with its time. A slot job ends within 10 minutes.
  No Python.

## Acceptance

`cargo nextest run -p metaspec-core search_beam` passes for both fixture packs on the grid of `story:search-hill-climb`:
case 1 (`--beam-width 2 --seed 2`) has 15 variants, `runs` 16, the round-2 cut keeping p8 and p2 (so the beam of round 3 is
`[p8, p2]`), `front` and `best` the variant of p15 and `stop.reason` `exhausted`, and is byte-equal to
`fixtures/expected/search/beam/<pack>-seed2.json` on a second run; case 2 (`--budget 9`) has variants v1..v9 = p1, p4, p2,
p5, p8, p3, p6, p9, p12, `runs` 10 and `stop.reason` `budget`; case 3 (`--beam-width 1 --seed 2` on the invalid-grid stub) has
12 variants in the order p1, p4, p5, p8, p9, p12, p10, p13, p6, p11, p14, p15, the beam `[p4]` after round 1 (no feasible
candidate, ranked by `better`, the tie broken by the seed), `runs` 13, `front` and `best` the variant of p15 (the baseline is infeasible and the one feasible front variant ranks above it) and `stop.reason`
`no_better_neighbour`, and is byte-equal to `<pack>-invalid.json`.

The paragraph of this story adds no copy of the stop reasons to `design/metaspec.md`: the one line of the design story still
counts once after this story, so this prints `1` before and after it:

```sh
grep -c 'Stop reasons by precedence: `deadline`, `budget`, `no_better_neighbour`, `exhausted`' design/metaspec.md
```
