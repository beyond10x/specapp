---
format: aep.planning-md/3
id: story:search-batch-scoring
kind: story
status: active
title: 'The batch adapter: a tool that scores many candidates in one run'
summary: batch in the check and simulate adapters; names what a pack tool must accept and answer; the search uses it when declared; same result as one run per candidate.
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-enumerate
- depends_on: story:search-simulate-adapter
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/adapter.rs
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/batch.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/run.rs
- confidence: cited
  path: crates/metaspec-core/src/tool.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/mod.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/search_batch.rs
- confidence: inferred
  path: fixtures/search/stub-check-harbor.sh
- confidence: inferred
  path: fixtures/search/stub-check-orchard.sh
- confidence: inferred
  path: fixtures/search/stub-simulate-harbor.sh
- confidence: inferred
  path: fixtures/search/stub-simulate-orchard.sh
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: spec/domains/packs.yaml
revision: 22
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:11Z", actor: "human:operator", revision: 21}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:11Z", actor: "human:operator", revision: 22}
---
# The batch adapter: a tool that scores many candidates in one run

## Context

architecture-decision-record:shared-search: a search starts the tool of the pack once per candidate
unless the tool scores many variants in one run; a pack without a batch is scored one candidate
per run (an inferred cost in the record, not measured). A tool run has a start-up cost that a
search pays for every candidate, so the manifest may declare a batch for `check` and for `simulate`.
The pack that wants it files its side from this story; a pack that does not is unchanged.

## Scope

- New optional `batch` inside the adapter of `check` and inside the adapter of `simulate`:
  `{ args, limit, at }`.
  - `args`: the arguments after `tool.run` for a batch run (the single-document `args` are not used).
  - `limit`: the most documents of one run (an integer at least 2).
  - `at`: a JSON pointer, in the answer, to the list of results. Every other pointer of the adapter
    (`valid`, `findings`, `counts`, `kpis`, `per_part`, `error`) is then relative to one entry of
    that list, as `at` of the single adapter is relative to the answer.
- What a pack tool must accept and answer for a batch, so that the pack can file its side:
  1. It accepts `<tool.run> <batch.args> <path-1> ... <path-n>` with 1 to `limit` document paths
     as the last arguments, stdin closed, the pack directory as working directory, no network.
  2. It prints one JSON answer on stdout whose list at `at` has exactly one entry per path, in the
     order of the arguments, each entry in the shape the single-document answer has at its own `at`.
  3. A document it refuses is an entry that carries the text the adapter `error` points at; the
     other entries are still answered. An exit code the adapter lists in `exit` comes with an
     answer; any other exit, a missing list or an entry count other than n fails the whole run.
  4. The run is deterministic for one set of documents, whatever the order or the batch size, so
     that a candidate has one score.
- metaspec uses a batch only in a search (`story:search-enumerate`, with `--simulate` of
  `story:search-simulate-adapter`) and only when the manifest declares it: it groups the next candidates
  up to `limit` (never beyond the budget left), runs the tool once, and maps each entry like a single
  answer. The baseline is one single run. `--deadline` is the deadline of one run, a batch included:
  a batch over it is killed with everything it started, and every candidate of that batch is in the
  result with state `timed_out`. A failed batch marks its candidates `failed`; the search goes on
  with the next batch.
- `runs` already exists in `metaspec-result/1` (`story:search-result-format`, written by
  `story:search-enumerate` as one run per candidate plus the baseline); this story only changes its value
  to the number of batch runs plus the baseline, per verb: with only `check` batched, the batch runs of
  `check` plus its baseline run; with `--simulate` each verb counts its own batch runs plus its own
  baseline run (single runs of a verb with no batch count one per candidate). No schema change for the result.
- The result of a search is the same, byte for byte, with a batch and without one on the same
  document and seed, apart from `runs`; a test proves it.
- Schema revision: `schema/metaspec-pack-1.schema.json` gains `batch` in `check_adapter` and
  `simulate_adapter`, optional, the format string stays `metaspec-pack/1`; `CHANGELOG.md` names the file
  and the property under "Unreleased". The digest of `metaspec trust` covers it.
- The ESS model first: `spec/domains/packs.yaml` gains the batch of an adapter as a noun beside the
  adapters, zero or one per adapter, owned by it; `ess specify validate` accepts it (`task check:spec`).
  The source of that cardinality and ownership is architecture-decision-record:shared-search (a tool
  that scores many variants declares it, otherwise one run per candidate); no ess/1 document states it
  yet, and anything the record does not say is an `UNMAPPED:` marker.
- Not in this story: any change to the tool of a pack, any manifest of a known pack (they declare a
  batch when their tool has one), the text of `design/metaspec.md`.

## Surfaces

- new: `crates/metaspec-core/src/search/batch.rs`, `crates/metaspec-core/tests/cases/search_batch.rs`.
- changed: `fixtures/search/stub-check-harbor.sh`, `fixtures/search/stub-check-orchard.sh`,
  `fixtures/search/stub-simulate-harbor.sh`, `fixtures/search/stub-simulate-orchard.sh` (each gains a
  batch mode, selected by its arguments, and counts its own starts in a file the test names, to show the
  number of runs), `crates/metaspec-core/src/manifest.rs`, `crates/metaspec-core/src/adapter.rs`,
  `crates/metaspec-core/src/tool.rs`, `crates/metaspec-core/src/search/run.rs`,
  `crates/metaspec-core/tests/cases/mod.rs`, `schema/metaspec-pack-1.schema.json`,
  `spec/domains/packs.yaml`, `CHANGELOG.md`.

## Shared files

- `crates/metaspec-core/src/manifest.rs`, `adapter.rs`, `tool.rs` and `schema/metaspec-pack-1.schema.json`:
  also changed by `story:cloudspec-manifest` (reached through `story:search-score`), by
  `story:view-selectors`, `story:hub-badges`, `story:matrix-view`, `story:focus-pages` and
  `story:collapsed-regions` (the view stories run before or after the search stories; the order of the
  waves is the planner call), and by `story:search-simulate-adapter`, which this story depends on.
- `crates/metaspec-core/src/search/run.rs`: created by `story:search-enumerate`, changed by
  `story:search-simulate-adapter`; this story depends on both.
- `fixtures/search/stub-*.sh`: created by `story:search-enumerate` and `story:search-simulate-adapter`;
  extended here, after both.
- `crates/metaspec-core/src/manifest.rs`, `crates/metaspec-core/src/adapter.rs` and
  `schema/metaspec-pack-1.schema.json`: also written by draft `story:kpi-quality-values`, draft
  `story:spec-quality-conformance` and draft `story:spec-quality-drift` (drafts of other epics; the coordinator orders the waves when they are accepted; no edge to drafts).

## Domain relations

- Adapter → Batch, one-to-zero-or-one, the adapter owns its batch, a batch cannot exist without its
  adapter: inferable from architecture-decision-record:shared-search ("a batch adapter for `check`
  and `simulate` is part of the epic; a pack without it is scored one candidate per run"); no
  `ess/1` document declares it, and this story writes it there first. Cited this way, not by an
  `ess/1` document, on purpose, so that the first step of this story is the document.

## Constraints

- No word of a pack under `crates/`; metaspec implements no rule and no simulation.
- Tests: the batch mode of the stubs, a counter file for the number of starts; no real
  pack tool, no model call, no network. No test over 60 s, the test step within 120 s; a test over 5 s
  is named with its time. A slot job ends within 10 minutes. No Python.

## Acceptance

`cargo nextest run -p metaspec-core search_batch` passes for both fixture packs: on a scratch copy of the
marked document of the pack in which the test marks a second place so that the space has 12 points (11
candidates), `metaspec search` on a copy of the fixture manifest whose `check` declares a batch of `limit` 4
and the batch stub writes a result equal to the one without the batch apart from `runs`, and `runs` is 4
(the baseline and 3 batches) against 12 without the batch.
