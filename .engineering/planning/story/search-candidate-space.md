---
format: aep.planning-md/3
id: story:search-candidate-space
kind: story
status: active
title: 'The candidate space: the marked places of a document, in both forms of vary, and the candidates they make'
summary: Pins vary as a list or {min,max,step} in the graph schema, reads a marked block, builds candidates as set_field patches; shown by tests, no command.
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-result-format
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/graph.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/mod.rs
- confidence: inferred
  path: crates/metaspec-core/src/search/space.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/mod.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/search_space.rs
- confidence: inferred
  path: fixtures/expected/space
- confidence: cited
  path: schema/metaspec-graph-1.schema.json
- confidence: cited
  path: spec/domains/documents.yaml
revision: 16
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:10Z", actor: "human:operator", revision: 15}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:10Z", actor: "human:operator", revision: 16}
---
# The candidate space: the marked places of a document, in both forms of vary, and the candidates they make

## Context

A search changes only what the author marked. `metaspec_core::graph` already reads `{value, vary}`
(`crates/metaspec-core/src/graph.rs`: `marked`, `unmark`, `Marker { path, value, vary }`), and
`schema/metaspec-graph-1.schema.json` types `vary` as any value (`$defs/marker`, `"vary": {}`), so
the forms a search can read are not pinned. agenticspec main writes `vary` as a list of values
(`count: { value: 8, vary: [4, 8, 12, 16] }`) or as `{ min, max, step }` over numbers or numbers with a
unit (`{ min: 2, max: 24, step: 2 }`), and its own search over them stays as it is.
cloudspec plans marked places whose values are block names. Nothing here calls the search of any
pack. What a search may generate is decided (`decision-blocker:search-edits`, answer A): `set_field`
edits on marked places only, a marked block included; this story builds exactly that and no other edit.

## Scope

- Pinned forms of `vary`: a non-empty list of values of any JSON type, or a mapping of exactly
  `min`, `max` and `step`, each a number, or each a text of a number and the same unit suffix
  (`2`, `0.5`, `3s`); `step` above 0 and `min` not above `max`, at most 100000 values. The range of
  a mapping is `min`, `min + step`, ... up to `max`. The original `value` is always in the range:
  inserted in numeric order when every entry is a number, else appended. A mapping with a compound
  unit (two suffixes) is not read.
- Schema revision: `schema/metaspec-graph-1.schema.json`, `$defs/marker.vary`, becomes
  `oneOf` the list form and the range form above; the format string stays `metaspec-graph/1`. A
  mapping of exactly `value` and `vary` whose `vary` is neither form is no marker: the reader leaves
  it as the field it is, and the space lists it as `ignored` with its path and the reason.
- A marked `block`: `block: { value, vary }` is a marker with path `block`; the part block is its
  value (today `graph::read` takes the block only from a plain text, so it is lost). This covers a
  place whose values are block names; whether a candidate that changes the block is valid is the
  check of the pack tool, not metaspec. No block name is written in the code.
- `Space` in `crates/metaspec-core/src/search/space.rs`: the places of a graph in document order
  (parts, then links, then the document), each `{ subject, path, value, range }` with `subject`
  `part:<name>`, `link:<id>` or `document`; the `ignored` places; `size()` (the product of the range
  lengths, saturating); `point(n)` (point 0 is every place at its original value; points 1 upward are the other
  combinations in odometer order, the last place changing fastest and the original combination left out, so
  when the original value is the first of every range the points number as a plain odometer); and a candidate from a point: the document with each place set to
  its value, `vary` kept, and the edits from the original as a `metaspec-patch/1` of `set_field`
  edits (`story:search-result-format`). A candidate document is written as YAML into a scratch
  directory under the state directory (`$XDG_STATE_HOME/metaspec/search/`), under the file name of
  the original, and removed after use; comments of the original are not kept in a candidate.
- No command and no new format: the places, ranges and sizes are shown by the tests, against
  `fixtures/expected/space/<pack>.json`, one golden per fixture pack (a test file, not a format).
- The ESS model: `spec/domains/documents.yaml` notes that `Part.block` may be marked, and `ess
  specify validate` accepts it (`task check:spec`).
- Not in this story: scoring (`story:search-score`), the search loop (`story:search-enumerate`),
  the text of `design/metaspec.md` (`story:search-design`).

## Surfaces

- new: `crates/metaspec-core/src/search/space.rs`, `crates/metaspec-core/tests/cases/search_space.rs`,
  `fixtures/expected/space/harbor.json`, `fixtures/expected/space/orchard.json`.
- changed: `crates/metaspec-core/src/graph.rs`, `crates/metaspec-core/src/search/mod.rs`,
  `crates/metaspec-core/tests/cases/mod.rs`, `schema/metaspec-graph-1.schema.json`,
  `spec/domains/documents.yaml`, `CHANGELOG.md`. `fixtures/expected/site` stays byte-equal
  (`task check:docs`).

## Shared files

- `spec/domains/documents.yaml`: also listed by `story:cloudspec-manifest`, which this story reaches
  through `story:search-score`, so it lands after it.
- `crates/metaspec-core/src/graph.rs`: this is its only story of the epic.
- `crates/metaspec-core/src/search/mod.rs`: also changed by `story:search-enumerate` and
  `story:search-simulate-adapter`, which depend on this story through `story:search-enumerate`.
- `crates/metaspec-core/tests/cases/mod.rs`: one line, as in `story:search-score`.

## Domain relations

- Document → Marker, one-to-many, the markers outside parts and links, and Part → Marker,
  Link → Marker, one-to-many: inferable (`spec/domains/documents.yaml`, `metaspec.documents.Document`,
  `Part` and `Link`, each with the field `markers` of type `List<metaspec.documents.Marker>`; fields,
  not `relations:` entries; the struct `metaspec.documents.Marker` has `path`, `value`, `vary`).

## Constraints

- No word of a pack under `crates/`; a test names no block of a real pack.
- Tests: the marked documents of the two fixture packs and documents the test writes into a
  scratch copy; no tool starts, no model call, no network. No test over 60 s, the test step within
  120 s; a test over 5 s is named with its time. A slot job ends within 10 minutes. No Python.

## Acceptance

`cargo nextest run -p metaspec-core search_space` passes: the cases list the marked places, ranges and
product size of the marked document of each fixture pack against `fixtures/expected/space/<pack>.json`,
read both forms of `vary` and a marked `block`, and list a malformed `vary` as ignored.
