---
format: aep.planning-md/3
id: story:search-hill-climb
kind: story
status: active
title: 'Hill climbing: a seeded search that moves to the first neighbour search::order::better ranks above the current candidate'
summary: 'metaspec search --strategy hill_climb: neighbours of marked places, first better neighbour in the seeded order, stop through search::run::stop_reason, byte-equal on one seed.'
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-design
- depends_on: story:search-dominance
- depends_on: story:search-candidate-space
- depends_on: story:search-enumerate
- informed_by: decision-blocker:kpi-order
- informed_by: decision-blocker:search-edits
- depends_on: story:search-simulate-adapter
- depends_on: story:search-batch-scoring
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: inferred
  path: crates/metaspec-core/src/search/climb.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/mod.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/neighbours.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/run.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/mod.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/search_climb.rs
- confidence: inferred
  path: crates/metaspec/src/search.rs
- confidence: cited
  path: design/metaspec.md
- confidence: inferred
  path: fixtures/expected/search/climb
- confidence: inferred
  path: fixtures/search/stub-check-grid-invalid.sh
- confidence: inferred
  path: fixtures/search/stub-check-grid.sh
revision: 16
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:12Z", actor: "human:operator", revision: 15}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:12Z", actor: "human:operator", revision: 16}
---
# Hill climbing: a seeded search that moves to the first neighbour `search::order::better` ranks above the current candidate

## Context

Bounded enumeration (`story:search-enumerate`) scores points of the space in order and cannot cover a space
larger than the budget. The order of candidates is decided (`decision-blocker:kpi-order`, answer A,
dominance, built by `story:search-dominance`), and what a search may generate is decided
(`decision-blocker:search-edits`, answer A: `set_field` on marked places only; the space of
`story:search-candidate-space`). This story adds the first strategy that uses the order. agenticspec main
keeps its own search (`crates/agenticspec-sim/src/search.rs`, `best.rs`, `ladder.rs`); it stays and
metaspec neither calls nor replaces it.

## Scope

- `metaspec search ... --strategy hill_climb` (the command and the flag `--strategy`, with the one value
  `enumerate`, are created by `story:search-enumerate` in `crates/metaspec/src/search.rs`; this story adds the
  value). Budget, `--time-limit`, `--deadline`, `--seed`, `--out`, the refusals and the exit codes are those of
  `story:search-enumerate`. `--simulate` is owned by `story:search-simulate-adapter` and a declared `batch` by
  `story:search-batch-scoring`, both inside `search::run::score_points`; this story depends on both and
  changes neither.
- A neighbour of a point: the point in which exactly one marked place takes the next or the previous value
  of its range and every other place stays (`search::neighbours::of(space, point)` in
  `crates/metaspec-core/src/search/neighbours.rs`, on the `Space` of `story:search-candidate-space`, the points
  numbered as `Space::point(n)` does, 0 the original, in ascending order; a place of one value has no
  neighbour). A step has up to 2 neighbours per place.
- The run: score the original (the baseline) and make it the current candidate. Each step scores the whole
  neighbourhood of the current candidate that is not scored yet, in ascending point order, through
  `search::run::score_points` (`crates/metaspec-core/src/search/run.rs`, `story:search-enumerate`), the one
  scoring path of every strategy: a candidate is scored by `check`, by `simulate` too under `--simulate` and in
  one batch run per verb where the manifest declares a `batch` and the neighbourhood fits its limit, exactly as in
  bounded enumeration. The
  neighbours of the current candidate (scored now or before) are put in the order of `search::rng::shuffle`
  of `story:search-enumerate`, the stream continuing from step to step. One rule decides a move: the first
  neighbour in that order with `search::order::better(neighbour, current)` (`story:search-dominance`) becomes the
  current candidate. For feasible candidates `better` is dominance; from an infeasible current candidate (an
  original that fails `check` or a gate) the search moves to any candidate ranked above it. When no neighbour is
  better the step makes no move (`moved` false). After scoring and choosing, the step calls
  `search::run::stop_reason(deadline_hit, scored, budget, moved, unscored)` (`story:search-enumerate`) and stops only
  when it returns `Some`; it decides no stop itself, and a move is made only when it returns `None`. A point scored once is not scored again. Every scored point is a variant, in the
  order scored.
- Stop reasons: `search::run::stop_reason` and `StopReason` (`story:search-enumerate`) are the code, and "Decided: shared
  search" (`story:search-design`) writes the four reasons and their precedence once; this story copies neither. A climb
  that ends with every point scored ends `exhausted`; none of the four cases below does (they leave 11, 11, 12 and 9
  points unscored). The result carries
  `strategy` `hill_climb`, and `front` and `best` as `story:search-dominance` defines them, over every scored variant;
  `runs` counts tool runs; `stop.evaluated` is the number of scored candidates.
- Not in this story: beam search (`story:search-beam`), a change of the schema (`story:search-dominance` owns
  it), weights or a priority order, edits other than `set_field` on marked places.
- Its paragraph of `design/metaspec.md`: under "Decided: shared search" of `story:search-design`, one
  paragraph that states the neighbour, the step that scores the whole neighbourhood and the first-better rule
  (`search::order::better`), and says how the climb ends by naming the reason (`no_better_neighbour`, `budget`,
  `deadline` or `exhausted`) and no more; the reasons and their precedence are written once, by `story:search-design`.

## The test grid

The tests of this story and of `story:search-beam` run on one grid, built in a scratch copy of the marked
document of each fixture pack, with the stub `fixtures/search/stub-check-grid.sh <pack>` as the `check` tool of a
copy of the fixture manifest. The pack tool is not used.

- Two marked places, each written `{ value: 0, vary: { min: 0, max: 3, step: 1 } }`, so each place has the
  values 0, 1, 2, 3 and its original value is 0 (the range minimum). Place A is the first marked place in
  document order, place B the second. The test rewrites the marker of the fixture field to this form and marks
  one more numeric field the same way on an earlier part (harbor: `reach` on `lone_berth` is A and `draught` on
  `barge` is B; orchard: `age` on `apple` is A and `hands` on `picker` is B).
- The space has 16 points. Point `n` is `(a, b)` with `n = 4a + b` (the last place changes fastest), so point 0
  is the original `(0, 0)`. The neighbours of a point are one step of one place: the interior point 5 `(1, 1)` has
  4, namely 1, 4, 6 and 9; point 1 has 3, namely 0, 2 and 5; the corner 0 has 2, namely 1 and 4.
- The stub answers, for every point and independent of the pack tool, valid true, no findings and one KPI, the
  first KPI of the fixture catalogue whose direction is higher (harbor `crane_lift_rate`, orchard
  `tree_yield_kilograms`), with the value `k(a, b)` of this table (row `a`, column `b`, the point number in
  brackets):

  | a \ b | 0 | 1 | 2 | 3 |
  |--------|---|---|---|---|
  | 0 | 0 (p0) | 3 (p1) | 3 (p2) | 0 (p3) |
  | 1 | 3 (p4) | 3 (p5) | 2 (p6) | 1 (p7) |
  | 2 | 3 (p8) | 1 (p9) | 4 (p10) | 5 (p11) |
  | 3 | 0 (p12) | 2 (p13) | 6 (p14) | 9 (p15) |

  The original has 0. Points 1, 2, 4, 5 and 8 are plateau tops of 3 with no better neighbour (a local top); the
  global top is point 15 with 9. One KPI means point x dominates point y exactly when `k(x) > k(y)`; equal values
  dominate neither way.
- The invalid-grid stub `fixtures/search/stub-check-grid-invalid.sh <pack>` answers the same `k` table, except that p0 answers
  valid false with 2 gated findings, p1 and p4 answer valid false with 1 gated finding each, and none of the three answers a KPI;
  every other point answers valid true with no finding.
- The stream: seed 1 gives the outputs 10451216379200822465, 13757245211066428519, 17911839290282890590; seed 2
  gives 10905525725756348110, 13819372491320860226, 10987583248141275951.

### Case 1: `--strategy hill_climb --seed 1` (default budget)

- The baseline p0 (value 0) is scored: 1 run. Step 1, current p0: neighbours 1 and 4 are scored in that order
  (p1 = 3, p4 = 3; variants v1 = p1, v2 = p4). Shuffle `[1, 4]`: `k = 1`, `r1 mod 2 = 1`, `j = 1`, no swap: order
  `[1, 4]`. p1 dominates p0 (3 > 0): the current candidate is p1.
- Step 2, current p1 (3): neighbours 0, 2, 5; 2 and 5 are not scored yet and are scored (p2 = 3, p5 = 3; v3 = p2,
  v4 = p5). Shuffle `[0, 2, 5]`: `k = 2`, `r2 mod 3 = 1`, swap items 2 and 1: `[0, 5, 2]`; `k = 1`, `r3 mod 2 = 0`,
  swap items 1 and 0: `[5, 0, 2]`. p5 = 3 and p2 = 3 are not above 3 and p0 = 0 is below: none dominates, the search
  stops.
- Result: variants v1..v4 = p1, p4, p2, p5; `runs` 5 (the baseline and 4); `stop.reason` `no_better_neighbour` (11
  points are not scored); `stop.evaluated` 4; the current candidate at the end is p1. `front` is `v1, v2, v3, v4` (all
  3, none dominates another, the baseline 0 is dominated); `best` is absent. `strategy` `hill_climb`.

### Case 2: `--strategy hill_climb --seed 2` (default budget)

- Baseline p0: 1 run. Step 1: p1 = 3, p4 = 3 scored (v1 = p1, v2 = p4). Shuffle `[1, 4]`: `r1 mod 2 = 0`, swap items
  1 and 0: `[4, 1]`; p4 dominates p0: the current candidate is p4, a different first improvement from case 1.
- Step 2, current p4 (3): neighbours 0, 5, 8; 5 and 8 are scored (p5 = 3, p8 = 3; v3 = p5, v4 = p8). Shuffle
  `[0, 5, 8]`: `k = 2`, `r2 mod 3 = 2`, no swap; `k = 1`, `r3 mod 2 = 1`, no swap: `[0, 5, 8]`; none is above 3: the
  search stops.
- Result: variants v1..v4 = p1, p4, p5, p8; `runs` 5; `stop.reason` `no_better_neighbour`; the search ends on p4, not
  on p1; `front` `v1, v2, v3, v4`; `best` absent.

### Case 3: `--strategy hill_climb --seed 1 --budget 3`

- Baseline p0: 1 run. Step 1 scores p1 and p4 (v1, v2; 2 of 3). Shuffle as in case 1: the current candidate is p1.
  Step 2: the unscored neighbours of p1 are 2 and 5, but 1 of the budget is left, so only p2 is scored (v3 = p2 = 3);
  the step ends with `stop_reason(false, 3, 3, ...)` = `Some(Budget)`: the search stops before it moves.
- Result: variants v1..v3 = p1, p4, p2; `runs` 4; `stop.reason` `budget`; `stop.evaluated` 3; `front` `v1, v2, v3`.

### Case 4: `--strategy hill_climb --seed 2` with `fixtures/search/stub-check-grid-invalid.sh` (an infeasible original)

- Baseline p0: valid false with 2 gated findings: infeasible, 1 run. Step 1, current p0: neighbours 1 and 4 are scored
  (v1 = p1, v2 = p4), each valid false with 1 gated finding. Both are infeasible like p0 and rank above it (fewer gated
  findings, 1 < 2). Shuffle `[1, 4]`: `r1 mod 2 = 0`, swap: `[4, 1]`; the first neighbour ranked above p0 is p4: the move is to
  p4 (`scored` 2, `unscored` 13: `None`).
- Step 2, current p4 (infeasible, 1 gated finding): neighbours 0, 5, 8; 5 and 8 are scored (v3 = p5 = 3, v4 = p8 = 3, both
  valid). Shuffle `[0, 5, 8]`: `r2 mod 3 = 2`, no swap; `r3 mod 2 = 1`, no swap: `[0, 5, 8]`. p0 (2 gated findings) does not rank above
  p4; p5 is feasible and feasible beats infeasible: the move is to p5 (`None`).
- Step 3, current p5 (feasible, 3): neighbours 1, 4, 6, 9; 6 and 9 are scored (v5 = p6 = 2, v6 = p9 = 1). Shuffle `[1, 4, 6, 9]`
  with `k = 3`, `r4 mod 4 = 0`; `k = 2`, `r5 mod 3 = 1`; `k = 1`, `r6 mod 2 = 1`: `[9, 6, 4, 1]`. p9 and p6 are below 3 and p4 and
  p1 are infeasible: none is better, no move (`moved` false, `scored` 6, `unscored` 9): `Some(NoBetterNeighbour)`.
- Result: variants v1..v6 = p1, p4, p5, p8, p6, p9; `runs` 7; `stop.reason` `no_better_neighbour`; `stop.evaluated` 6; the climb ends on
  the feasible p5. `front` is `v3, v4` (the baseline is infeasible and not a member); `best` is absent.

## Surfaces

- new: `crates/metaspec-core/src/search/neighbours.rs`, `crates/metaspec-core/src/search/climb.rs`,
  `crates/metaspec-core/tests/cases/search_climb.rs`, `fixtures/search/stub-check-grid.sh` and `fixtures/search/stub-check-grid-invalid.sh` (POSIX sh, the pack
  named by the first argument, the table above), recorded results `fixtures/expected/search/climb/harbor-seed1.json`,
  `harbor-seed2.json`, `orchard-seed1.json`, `orchard-seed2.json`, `harbor-invalid.json` and `orchard-invalid.json` in the same
  directory.
- changed: `crates/metaspec-core/src/search/run.rs` (the call of the strategy beside enumeration),
  `crates/metaspec-core/src/search/mod.rs`, `crates/metaspec/src/search.rs` (the value of `--strategy`),
  `design/metaspec.md` (one paragraph), `CHANGELOG.md`, `crates/metaspec-core/tests/cases/mod.rs` (one `mod` line).

## Shared files

- `crates/metaspec/src/search.rs`, `crates/metaspec-core/src/search/run.rs`: created by
  `story:search-enumerate`, which this story depends on; also changed by `story:search-simulate-adapter` and
  `story:search-batch-scoring`, which this story depends on, and by `story:search-beam`, which depends on this one.
- `crates/metaspec-core/src/search/mod.rs`: created by `story:search-score`; one line here.
- `design/metaspec.md`: one paragraph after `story:search-design`, which it depends on.
- `crates/metaspec-core/tests/cases/mod.rs`, `CHANGELOG.md`: one line each.

## Domain relations

- Run → Variant, one-to-many, owned: inferable (`spec/domains/runs.yaml`, `metaspec.runs.Run`, relation
  `variants`, kind `owns`). The run carries a budget of at least 1 and a deadline: inferable
  (`metaspec.runs.Run`, fields `budget` and `deadline`, invariant `budget >= 1`).

## Constraints

- No word of a pack under `crates/` (the stub and the test hold the names of the fixture packs); every score is
  the answer of the tool; the original document is never changed.
- Tests: the stub of the grid in a copy of the fixture manifest; no real pack tool, no model call, no network. No
  test over 60 s, the test step within 120 s; a test over 5 s is named with its time. A slot job ends within 10
  minutes. No Python.

## Acceptance

`cargo nextest run -p metaspec-core search_climb` passes for both fixture packs on the grid above, with the variants as
derived: case 1 (`--seed 1`) has variants v1..v4 = p1, p4, p2, p5, `runs` 5 and `stop.reason` `no_better_neighbour`, and is
byte-equal to `fixtures/expected/search/climb/<pack>-seed1.json` on a second run; case 2 (`--seed 2`) has variants v1..v4
= p1, p4, p5, p8, `runs` 5 and `stop.reason` `no_better_neighbour`, and is byte-equal to
`<pack>-seed2.json`; case 3 (`--seed 1 --budget 3`) has variants v1..v3 = p1, p4, p2, `runs` 4 and `stop.reason` `budget`; case 4
(`--seed 2` on the invalid-grid stub, an infeasible original) has variants v1..v6 = p1, p4, p5, p8, p6, p9, the moves p0 to p4 to
p5, `runs` 7, `front` `v3, v4` and `stop.reason` `no_better_neighbour`, and is byte-equal to `<pack>-invalid.json`.

The paragraph of this story adds no copy of the stop reasons to `design/metaspec.md`: the one line of the design story still
counts once after this story, so this prints `1` before and after it:

```sh
grep -c 'Stop reasons by precedence: `deadline`, `budget`, `no_better_neighbour`, `exhausted`' design/metaspec.md
```
