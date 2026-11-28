---
format: aep.planning-md/3
id: story:search-result-format
kind: story
status: active
title: 'metaspec-result/1 and metaspec-patch/1: the formats a search writes'
summary: Two JSON Schemas and a typed writer, tested against recorded files; a variant carries a patch from the original and a score with its verdicts.
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-score
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: inferred
  path: crates/metaspec-core/src/search/mod.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/result.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/score.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/mod.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/search_result.rs
- confidence: inferred
  path: fixtures/result
- confidence: inferred
  path: schema/metaspec-patch-1.schema.json
- confidence: inferred
  path: schema/metaspec-result-1.schema.json
- confidence: inferred
  path: schema/metaspec-score-1.schema.json
revision: 18
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:10Z", actor: "human:operator", revision: 17}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:10Z", actor: "human:operator", revision: 18}
---
# metaspec-result/1 and metaspec-patch/1: the formats a search writes

## Context

`design/metaspec.md` ("Neutral formats") names `metaspec-result/1` (a run: baseline, variants, a
trace) and `metaspec-patch/1` (an ordered list of edits), and `spec/domains/runs.yaml` and
`spec/domains/drafts.yaml` model the Run, the Variant and the Edit union. Neither format has a JSON
Schema in `schema/`, so no producer can write one and no page can read one.
`story:search-score` defines the score each variant carries (`metaspec-score/1`).

## Scope

- `schema/metaspec-patch-1.schema.json`: `{ format, edits[] }`, an edit tagged by `op`, one
  variant per edit of `metaspec.drafts.Edit` in `spec/domains/drafts.yaml` with the members
  given there (`add_part`, `remove_part`, `set_field`, `add_link`, `remove_link`, `move`, `mark`,
  `unmark`, `set_border`, `pack_verb`). A subject is `part:<name>`, `link:<id>` or `document`; a
  path joins keys with `.` and writes list positions as numbers, as `metaspec-graph/1` markers do.
  Pinned here: a `set_field` whose path is a marked place sets the `value` of the marker and keeps
  its `vary` (as agenticspec main writes the best variant back). This story applies no patch.
- `schema/metaspec-result-1.schema.json`, the result of a search: `format`, `pack`, `document`
  (path inside the pack), `digest` (`sha256:` of the document as read), `verb` (`search`; the enum
  is widened when another run is written), `baseline` (`{ score }`, a `metaspec-score/1`), `seed`
  (integer or null), `budget`, `deadline_seconds`, `runs` (the number of tool runs the search
  used), `variants[{ id, edits, score }]` where `edits` is a `metaspec-patch/1` from the original
  document to the variant and `score` a `metaspec-score/1` (an unscored candidate has only its
  `state`), `trace[{ eval, variant }]` in evaluation order, and `stop` `{ reason, evaluated }` with
  `reason` one of `deadline`, `budget`, `no_better_neighbour`, `exhausted` (the schema owns the four strings, all four from the start; `search::run::StopReason` of `story:search-enumerate` serialises to them). `runs` is required from the start and counts tool runs: one per
  candidate plus one for the baseline when only `check` runs (a search with `--simulate` also counts its
  `simulate` runs, `story:search-simulate-adapter`); `story:search-batch-scoring` only changes its value. The
  members `best`, `front`, `strategy` and `beam_width` are added, as optional properties,
  by `story:search-dominance`, the one story that changes `schema/metaspec-result-1.schema.json` after this one, and it adds no stop reason; the format string stays `/1` because a
  property is added. Variant ids are `v1`, `v2`, ... in evaluation order; the baseline is not a variant.
- The verdict per KPI against the baseline: the score of a variant carries `verdicts`, a mapping from
  a KPI id to `better`, `worse`, `same` or `not_ordered`, computed once in Rust by
  `search::score::compare` of `story:search-score`. Schema revision:
  `schema/metaspec-score-1.schema.json` gains the optional property `verdicts`; the format string
  stays `metaspec-score/1`. The score of the baseline has none. A page reads it and computes nothing.
- A typed reader and writer in `crates/metaspec-core/src/search/result.rs` that validate against
  the schemas before writing, as `metaspec quality` does for its report. No command is added: the
  tests validate the recorded files against the schemas.
  Schema revision: two new files, `schema/metaspec-result-1.schema.json` and
  `schema/metaspec-patch-1.schema.json`, and the optional `verdicts` above, each described in
  `CHANGELOG.md` under "Unreleased".
- Not in this story: a producer of a search result (`story:search-enumerate`), the page
  (`story:search-result-page`), the text of `design/metaspec.md` (`story:search-design`).

## Surfaces

- new: `schema/metaspec-result-1.schema.json`, `schema/metaspec-patch-1.schema.json`,
  `crates/metaspec-core/src/search/result.rs`, `crates/metaspec-core/tests/cases/search_result.rs`,
  recorded files `fixtures/result/valid-search.json`, `fixtures/result/refused-*.json` (one per
  refusal).
- changed: `crates/metaspec-core/src/search/mod.rs` (from `story:search-score`),
  `crates/metaspec-core/src/search/score.rs`, `schema/metaspec-score-1.schema.json`,
  `crates/metaspec-core/tests/cases/mod.rs`, `CHANGELOG.md`.

## Shared files

- `crates/metaspec-core/src/search/mod.rs`, `crates/metaspec-core/src/search/score.rs`,
  `schema/metaspec-score-1.schema.json`: created by `story:search-score`, which this story depends on.
- `crates/metaspec-core/tests/cases/mod.rs`: one line, as in `story:search-score`.

## Domain relations

- Run → Variant, one-to-many, the run owns its variants: inferable (`spec/domains/runs.yaml`,
  entity `metaspec.runs.Run`, relation `variants`, kind `owns`, cardinality `many`).
- Run → Document, many-to-one, a run references the document it ran on: inferable
  (`spec/domains/runs.yaml`, `metaspec.runs.Run`, relation `document`).
- Variant → Patch, one patch per variant: inferable (`spec/domains/runs.yaml`,
  `metaspec.runs.Variant`, field `patch` of type `metaspec.drafts.Patch`; a field, not a
  `relations:` entry).
- Patch → Edit, one-to-many, ordered: inferable (`spec/domains/drafts.yaml`,
  `metaspec.drafts.Patch`, field `edits` of type `List<metaspec.drafts.Edit>`).

## Constraints

- No word of a pack in the schemas or the code. A test reads only the recorded files and the
  schemas: no tool starts, no model call, no network. No test over 60 s, the test step within
  120 s; a test over 5 s is named with its time. A slot job ends within 10 minutes. No Python.
- Public repository: the recorded files are invented.

## Acceptance

`cargo nextest run -p metaspec-core search_result` passes: `fixtures/result/valid-search.json` is valid
against `schema/metaspec-result-1.schema.json` and every patch in it against
`schema/metaspec-patch-1.schema.json`, and each `fixtures/result/refused-*.json` is refused with the
member the schema names.
