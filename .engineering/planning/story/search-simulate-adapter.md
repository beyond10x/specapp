---
format: aep.planning-md/3
id: story:search-simulate-adapter
kind: story
status: active
title: 'The simulate adapter: KPI values of a document from the tool simulation'
summary: tool.verbs.simulate in the manifest, score --simulate and search --simulate; names what a pack tool must accept and answer; agenticspec has no such verb yet.
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-score
- depends_on: story:search-result-format
- depends_on: story:search-enumerate
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/adapter.rs
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/mod.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/run.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/score.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/simulate.rs
- confidence: cited
  path: crates/metaspec-core/src/tool.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/mod.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/search_simulate.rs
- confidence: inferred
  path: crates/metaspec/src/score.rs
- confidence: inferred
  path: crates/metaspec/src/search.rs
- confidence: inferred
  path: fixtures/search/stub-simulate-harbor.sh
- confidence: inferred
  path: fixtures/search/stub-simulate-orchard.sh
- confidence: inferred
  path: fixtures/search/stub-simulate-slow-harbor.sh
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: inferred
  path: schema/metaspec-score-1.schema.json
- confidence: cited
  path: spec/domains/packs.yaml
revision: 30
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:11Z", actor: "human:operator", revision: 29}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:11Z", actor: "human:operator", revision: 30}
---
# The simulate adapter: KPI values of a document from the tool simulation

## Context

`metaspec-pack/1` declares one verb, `check`. `design/metaspec.md` ("The verbs to come") says
`simulate` takes the same form, and the epic scores a candidate with KPI values "from `check` or
`simulate`". agenticspec main simulates through `agenticspec sim run <experiment> --out <dir>`: it
takes an experiment file (an architecture, a profile, seeds, a search mode), not an architecture
document, and writes `result.json` (`agenticspec-sim-result/1`) into a directory, not to stdout.
The tool of the pack has no verb that simulates one architecture document and answers on stdout;
that is a gap of agenticspec, listed per pack by `story:search-design`, and closed in the pack
(its tool), never by code for one pack here.

## Scope

- New manifest verb `tool.verbs.simulate`, an adapter like `check`: metaspec runs
  `<tool.run> <args> <document>` in the pack directory, stdin closed, no network, under the
  deadline of a run, and reads stdout as JSON. Fields: `args`, `exit`, `at`, `error`, `valid` as in
  the check adapter (`crates/metaspec-core/src/manifest.rs`, `CheckAdapter`), and
  `kpis: { at, value }` (`at`: pointer to a mapping from a KPI id to a number or to an object;
  `value`: pointer inside an object entry to the number) and `per_part: { at, value }` (a mapping
  from a part name to such a mapping). A value that is no number is a failed run.
- What a pack tool must accept and answer for this verb: one document path as the last argument,
  any fixed arguments inside `args` (a profile path inside the pack, for example); a JSON answer on
  stdout (not a file); the KPI values of the document as simulated, by the KPI id of the pack
  catalogue, and optionally per part by the part name of the document; an exit code the adapter
  lists in `exit`, with `error` for a document it refuses. The simulation is deterministic for
  one document (a seed fixed in the arguments), because a search compares candidates by value.
- `metaspec score ... --simulate` (`story:search-score`): the KPI values of the score come from
  `simulate` instead of `check`; a KPI id both answered takes the simulated value. Each KPI of the
  score gains an optional `source` (`check` or `simulate`) and an optional `per_part` (part name to
  number); `schema/metaspec-score-1.schema.json` adds those optional properties and the format string
  stays `/1`. A run over the deadline is killed with everything it started and shown as `timed_out`.
- `metaspec search ... --simulate` (`story:search-enumerate`): every candidate and the baseline are
  scored with the simulated KPI values; validity and findings still come from `check`, so a manifest
  with `simulate` has `check`. Without the flag a search never runs `simulate`. A manifest that
  declares no `simulate` refuses the flag with exit 1.
- `runs` counts tool runs. A candidate scored with `--simulate` costs one `check` run and one `simulate` run (or
  its share of a batch run of each verb, `story:search-batch-scoring`); so does the baseline. A candidate
  whose `check` answered and whose `simulate` was killed at the deadline is listed with state `timed_out`,
  its validity and findings kept and no KPI values; it is never better than a candidate with values (the
  order itself is `story:search-dominance`).
- The level: a manifest with `simulate` is L4 (`Manifest::level`, now L3 at most); the
  `metaspec-pack-check/1` report names it (`schema/metaspec-pack-check-1.schema.json`
  already lists the levels; a test proves L4 is accepted). The digest that `metaspec trust` records
  covers the new verb, so a pack that adds it asks for trust again.
- Schema revision: `schema/metaspec-pack-1.schema.json` gains `tool.verbs.simulate`
  (`$defs/simulate_adapter`), optional, the format string stays `metaspec-pack/1`; a line under
  "Unreleased" in `CHANGELOG.md` names the file and the property.
- `packs/agenticspec.yaml` declares `simulate` only when its tool has a verb as described above;
  until then the manifest is unchanged and the acceptance row says "no simulate verb" for it.
- No standalone `metaspec simulate` command and no result of a simulate run: simulated values enter
  only through `score --simulate` and `search --simulate`.
- Not in this story: batches (`story:search-batch-scoring`), the text of `design/metaspec.md`
  (`story:search-design`).

## Surfaces

- new: `crates/metaspec-core/src/search/simulate.rs`, `crates/metaspec-core/tests/cases/search_simulate.rs`,
  `fixtures/search/stub-simulate-harbor.sh` and `fixtures/search/stub-simulate-orchard.sh` (POSIX sh
  stubs that answer a simulation in the shape of each fixture pack, from the values in the document) and
  `fixtures/search/stub-simulate-slow-harbor.sh` (sleeps 10 seconds, so a deadline of 1 second kills it).
- changed: `crates/metaspec-core/src/manifest.rs` (`Verbs.simulate`, `Verbs::names`,
  `Manifest::level`), `crates/metaspec-core/src/tool.rs`, `crates/metaspec-core/src/adapter.rs`,
  `crates/metaspec-core/src/search/mod.rs`, `crates/metaspec-core/src/search/score.rs`,
  `crates/metaspec-core/src/search/run.rs`, `crates/metaspec/src/score.rs`, `crates/metaspec/src/search.rs`,
  `schema/metaspec-pack-1.schema.json`, `schema/metaspec-score-1.schema.json`, `spec/domains/packs.yaml`,
  `crates/metaspec-core/tests/cases/mod.rs`, `CHANGELOG.md`.

## Shared files

- `crates/metaspec-core/src/manifest.rs`, `crates/metaspec-core/src/tool.rs`,
  `crates/metaspec-core/src/adapter.rs` and `schema/metaspec-pack-1.schema.json`: also changed by
  `story:cloudspec-manifest` (this story reaches it through `story:search-score`), by
  `story:view-selectors`, `story:hub-badges`, `story:matrix-view`, `story:focus-pages` and
  `story:collapsed-regions` (the view stories run before or after the search stories; the order of the
  waves is the planner call), and by `story:search-batch-scoring`, which depends on this story.
- `crates/metaspec-core/src/search/run.rs` and `crates/metaspec/src/search.rs`: created by `story:search-enumerate`,
  which this story depends on. `crates/metaspec-core/src/search/mod.rs`: created by `story:search-score`, with a line
  added by `story:search-candidate-space` and `story:search-enumerate`, both of which this story depends on.
- The `--simulate` flags live in the clap Args structs of `crates/metaspec/src/score.rs` and
  `crates/metaspec/src/search.rs`; `crates/metaspec/src/main.rs` is not changed by this story.
- `fixtures/search/`: the directory and the check stubs are created by `story:search-enumerate`; this
  story adds the simulate stubs and rewrites none.
- `crates/metaspec-core/src/manifest.rs`, `crates/metaspec-core/src/adapter.rs` and
  `schema/metaspec-pack-1.schema.json`: also written by draft `story:kpi-quality-values`, draft
  `story:spec-quality-conformance` and draft `story:spec-quality-drift` (drafts of other epics; the coordinator orders the waves when they are accepted; no edge to drafts).

## Domain relations

- Pack → Verb, one-to-many, a manifest declares the verbs of its tool: inferable
  (`spec/domains/packs.yaml`, `metaspec.packs.Verb` has the variants `graph`, `check`, `simulate`,
  `search`, `apply`; the pack field `tool_verbs` is `List<metaspec.packs.Verb>`). In that document
  the level of a manifest with `simulate` or `search` and no `apply` is `L4` (an invariant of
  `metaspec.packs.Pack`). The adapter of `simulate` is a new noun there: this story writes it
  beside `check_adapter` in `spec/domains/packs.yaml` and `ess specify validate` accepts it
  (`task check:spec`), anything not readable from the code left as an `UNMAPPED:` marker.
- Run → Document, many-to-one: inferable (`spec/domains/runs.yaml`, `metaspec.runs.Run`, relation `document`).

## Constraints

- No word of a pack under `crates/`; metaspec simulates nothing: the tool answers.
- Tests: the stub tools of the fixture packs only; no real pack tool, no model call, no network.
  No test over 60 s, the test step within 120 s; a test over 5 s is named with its time. A slot job
  ends within 10 minutes. No Python: the stubs are POSIX sh.
- Public repository: the stubs and answers are invented.

## Acceptance

`cargo nextest run -p metaspec-core -p metaspec search_simulate` passes for both fixture packs, on a copy of
the fixture manifest whose `tool` is the stubs: `metaspec score ... --simulate --json` prints KPI values
with `source` `simulate` equal to what the simulate stub answers; `metaspec search ... --simulate` writes a
result valid against `schema/metaspec-result-1.schema.json` whose variants carry those simulated values and
whose `runs` is twice the number of scored documents (one `check` and one `simulate` run each); with the slow
simulate stub and `--deadline 1` the candidate is listed `timed_out` with the validity and findings of `check`
and no KPI values (the test takes about 1 s).
