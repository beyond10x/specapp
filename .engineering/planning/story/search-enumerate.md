---
format: aep.planning-md/3
id: story:search-enumerate
kind: story
status: active
title: 'metaspec search: bounded enumeration of the marked values, scored by the pack tool'
summary: 'The search command with the first strategy: seeded, budget and deadline, stop reasons, metaspec-result/1 of every scored candidate; no best until the order is decided.'
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-score
- depends_on: story:search-candidate-space
- depends_on: story:search-result-format
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: inferred
  path: crates/metaspec-core/src/search/mod.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/rng.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/run.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/mod.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/search_run.rs
- confidence: cited
  path: crates/metaspec/src/main.rs
- confidence: inferred
  path: crates/metaspec/src/search.rs
- confidence: inferred
  path: fixtures/expected/search
- confidence: inferred
  path: fixtures/search/stub-check-harbor.sh
- confidence: inferred
  path: fixtures/search/stub-check-orchard.sh
- confidence: inferred
  path: fixtures/search/stub-check-slow-harbor.sh
revision: 27
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:10Z", actor: "human:operator", revision: 26}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:10Z", actor: "human:operator", revision: 27}
---
# metaspec search: bounded enumeration of the marked values, scored by the pack tool, written as metaspec-result/1

## Context

architecture-decision-record:shared-search: metaspec runs the search once for every pack and asks
the tool of the pack to score each candidate. The pieces exist as stories: a score
(`story:search-score`), the candidate space (`story:search-candidate-space`) and the result format
(`story:search-result-format`). This story joins them into the command and the first strategy,
scored through `check`; simulated KPI values come with `story:search-simulate-adapter`, which
depends on this story. Bounded enumeration needs no order of candidates, so it lists every scored
candidate without picking a best: the best and the front come with `story:search-dominance`, the gate test of a KPI value
with `story:search-gate`, and hill climbing and beam search with `story:search-hill-climb` and
`story:search-beam`. agenticspec main has its own search
(`crates/agenticspec-sim/src/search.rs`); it stays and metaspec neither calls nor replaces it.

## Scope

- `metaspec search <NAME | --manifest FILE> --pack <DIR> <DOCUMENT> [--strategy enumerate]
  [--budget N] [--time-limit SECONDS] [--deadline SECONDS] [--seed N] [--out FILE]`.
  Refused with exit 1 and a message when: the pack is not trusted, the manifest has no `check`
  adapter, the document is not a `parts-links/1` document of the pack, the document has no marked
  place. Exit 0 when a result is written, 2 on bad usage. The result goes to `--out` or stdout.
- The run: score the original document (the baseline), then the candidates of the space in
  order, each one a tool run (`check`) in the pack directory on a candidate document in the scratch
  directory of `story:search-candidate-space`. `--budget` is the number of scored candidates
  (default 50, at least 1); `--time-limit` the wall clock of the whole search in seconds (default 300);
  `--deadline` one tool run (default 30 seconds, as `render`). A tool run never runs past the time
  limit: its deadline is the smaller of `--deadline` and the time left. The defaults are printed by
  `--help` and recorded in the result.
- Order: the points of the space from 1 up (point 0 is the baseline, numbered as `Space::point(n)` of
  `story:search-candidate-space`). When the space is larger than the budget, the points are shuffled and the
  first `--budget` are scored; when the space fits, the seed changes nothing and the order is the odometer
  order. The stream, `search::rng` in `crates/metaspec-core/src/search/rng.rs`: SplitMix64 with the state set to
  `--seed` (default 0); each output is `state += 0x9E3779B97F4A7C15`, `z = state`,
  `z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9`, `z = (z ^ (z >> 27)) * 0x94D049BB133111EB`, `z ^ (z >> 31)`, all
  arithmetic modulo 2^64 (the first outputs of seed 1 are 10451216379200822465, 13757245211066428519 and
  17911839290282890590, which a test pins). `search::rng::shuffle(list)` is Fisher-Yates from the end: for `k` from
  `len - 1` down to 1, draw `r`, `j = r mod (k + 1)` (unsigned), swap the items `k` and `j`. One stream serves a
  whole run and continues from step to step, so one seed on one document gives one result byte for byte.
- A tool run over its deadline is killed with everything it started; that candidate stays in the
  result with state `timed_out`. A candidate the tool refuses or whose answer is unreadable stays
  with state `refused` or `failed`. Neither stops the search.
- The scoring path, `search::run::score_points(run, points)` in `crates/metaspec-core/src/search/run.rs`: scores
  a list of points in the given order, one `check` run per point here, under the budget and the deadline, counts
  `runs`, and returns the scores; when less budget remains than there are points, the first points of the list up to the
  budget are scored. Bounded enumeration calls it with every point; the strategy stories
  (`story:search-hill-climb`, `story:search-beam`) call it for the neighbourhood of each step, so that
  `--simulate` (`story:search-simulate-adapter`) and a declared `batch` (`story:search-batch-scoring`),
  which extend the inside of this one function, apply to every strategy.
- Stop reasons, the member `stop` of `metaspec-result/1`: the enum `StopReason` (`Deadline`, `Budget`,
  `NoBetterNeighbour`, `Exhausted`, written as `deadline`, `budget`, `no_better_neighbour`, `exhausted`) and the function
  below are code in `crates/metaspec-core/src/search/run.rs`; the unit tests of the acceptance say what each reason
  means. Bounded enumeration never writes `no_better_neighbour`; the schema of `story:search-result-format` lists it
  from the start. A budget equal to the number of candidates gives `budget`.
- One function applies the precedence: `search::run::stop_reason(deadline_hit: bool, scored: usize, budget: usize,
  moved: bool, unscored: usize) -> Option<StopReason>` in `crates/metaspec-core/src/search/run.rs`. It returns
  `Some(Deadline)` when `deadline_hit`; else `Some(Budget)` when `scored >= budget`; else `Some(NoBetterNeighbour)`
  when `!moved && unscored > 0`; else `Some(Exhausted)` when `unscored == 0`; else `None`. `scored` is the number of
  scored candidates (not the baseline), `unscored` the points not scored (the space size, less the baseline, less
  `scored`), `moved` whether the step made a move (bounded enumeration has no step and passes true, so it never writes
  `no_better_neighbour`). Bounded enumeration, hill climbing and beam call it after each step and decide no stop
  themselves: a search goes on exactly while it returns `None`.
- Output: `metaspec-result/1` of `story:search-result-format`: the baseline, `variants` (id, edits as a
  `metaspec-patch/1` from the original, score with its `verdicts` against the baseline), `trace`,
  `seed`, `budget`, `deadline_seconds`, `stop`, and `runs` (the tool runs: one `check` run per candidate plus one for
  the baseline, as no `--simulate` is given here); validated against the schema before it is written. No `best` and no `front` member.
  The original document is never changed.
- The check answers of the stubs are this story files: `fixtures/search/stub-check-harbor.sh` and
  `fixtures/search/stub-check-orchard.sh` (POSIX sh, answer in the shape of each fixture pack from the
  values in the document), and `fixtures/search/stub-check-slow-harbor.sh` (sleeps 10 seconds when a
  marked value is at its top, so a time limit of 1 second kills it).
- Not in this story: simulated values and `--simulate` (`story:search-simulate-adapter`), the batch of
  candidates per tool run (`story:search-batch-scoring`: here each candidate is one run), the page,
  the text of `design/metaspec.md`.

## Surfaces

- new: `crates/metaspec-core/src/search/run.rs` (the loop, `score_points`, the stream, the stop reasons),
  `crates/metaspec-core/src/search/rng.rs`, `crates/metaspec-core/tests/cases/search_run.rs`,
  `crates/metaspec/src/search.rs`, `fixtures/search/stub-check-harbor.sh`,
  `fixtures/search/stub-check-orchard.sh`, `fixtures/search/stub-check-slow-harbor.sh`, recorded
  results `fixtures/expected/search/harbor.json` and `fixtures/expected/search/orchard.json`.
- changed: `crates/metaspec-core/src/search/mod.rs`, `crates/metaspec-core/tests/cases/mod.rs`,
  `crates/metaspec/src/main.rs`, `CHANGELOG.md`.

## Shared files

- `crates/metaspec/src/main.rs`, `crates/metaspec-core/src/search/mod.rs`: one line each; also
  changed by `story:search-score`, `story:search-candidate-space` and `story:search-result-format`,
  which this story depends on, and by `story:search-simulate-adapter`, which depends on this one.
- `fixtures/search/`: this story creates the directory and the check stubs; `story:search-simulate-adapter`
  adds the simulate stubs and `story:search-batch-scoring` extends the stubs, after both.

## Domain relations

- Run → Variant, one-to-many, the run owns its variants: inferable (`spec/domains/runs.yaml`,
  `metaspec.runs.Run`, relation `variants`, kind `owns`, cardinality `many`).
- Run → Document, many-to-one: inferable (`spec/domains/runs.yaml`, `metaspec.runs.Run`, relation `document`).
- The run carries a budget of at least 1 and a deadline: inferable (`spec/domains/runs.yaml`,
  `metaspec.runs.Run`, fields `budget` and `deadline`, invariant `budget >= 1`). A stop reason is
  not modelled there (`RunEnd` has `finished`, `killed`, `refused`, `failed`); the studio that will
  hold runs is not built (`dependency-blocker:metaspec-studio`), so this story does not change the
  ESS model.

## Constraints

- No word of a pack under `crates/`; metaspec implements no rule: every score is the answer of the tool.
- Tests run on harbor and orchard with the stub tools of `fixtures/search/`, in a copy of the fixture
  manifest whose `tool` the test replaces: no real pack tool, no model call, no network. The time
  limit test uses 1 second and the slow stub is killed at it, so the test takes about 1 second. No test
  over 60 s, the test step within 120 s; a test over 5 s is named with its time. A slot job ends within
  10 minutes (`slot --timeout`). No Python.
- Public repository: stubs and recorded results are invented.

## Acceptance

`cargo nextest run -p metaspec-core search_run` passes, for both fixture packs, with three cases (and two unit tests, the last two bullets: the stop function and the `StopReason` serialisation) on a copy of the
fixture manifest with the stub tool, over the marked document of the pack, whose space has 2 candidates (points 1 and 2, the
values of the range other than the original, in order: harbor `draught` 2m and 4m, orchard `hands` 1 and 3):
- `metaspec search ... --seed 7`: the result has one variant per point but the original, a `set_field` patch for each,
  `stop.reason` `exhausted` and `runs` 3 (the baseline and the 2 points), and is byte-equal to
  `fixtures/expected/search/<pack>.json` on a second run with the same seed;
- `metaspec search ... --seed 2 --budget 1`: the 2 candidates exceed the budget, so points 1 and 2 are shuffled; the first
  output of seed 2, 10905525725756348110, is even, so `j = 0` and the two swap to `[p2, p1]`; the first 1 is scored: one
  variant, v1 = p2 (harbor `draught` 3m to 4m, orchard `hands` 2 to 3), `runs` 2, `stop.evaluated` 1, `stop.reason` `budget`;
- `metaspec search ... --time-limit 1` with the slow stub: the killed candidate is listed as `timed_out` and `stop.reason`
  is `deadline`.
- `stop_reason` on one input per row of the precedence: `(true, 1, 50, true, 5)` is `Some(Deadline)`; `(false, 50, 50, true, 0)` is
  `Some(Budget)` (the budget equals the candidates); `(false, 3, 50, false, 9)` is `Some(NoBetterNeighbour)`;
  `(false, 3, 50, false, 0)` is `Some(Exhausted)`; `(false, 3, 50, true, 9)` is `None`.
- `StopReason` serialises: a unit test serialises each of the four variants and compares the string with the matching entry of
  the `stop.reason` enum read from `schema/metaspec-result-1.schema.json` (`deadline`, `budget`, `no_better_neighbour`,
  `exhausted`), and compares the set of the four with the whole enum, so a fifth entry on either side fails it.
