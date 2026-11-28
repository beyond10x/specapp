---
format: aep.planning-md/3
id: story:search-result-page
kind: story
status: active
title: The page of a search result in the docs site
summary: 'docs build --result and ResultSections draw metaspec-result/1: baseline, variants with edits, scores, verdicts, stop reason; the studio half waits on dependency-blocker:metaspec-studio.'
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-result-format
- depends_on: story:search-enumerate
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: crates/metaspec-docs/src/documents.rs
- confidence: cited
  path: crates/metaspec-docs/src/model.rs
- confidence: cited
  path: crates/metaspec/src/docs.rs
- confidence: cited
  path: crates/metaspec/src/main.rs
- confidence: cited
  path: crates/metaspec/tests/main.rs
- confidence: inferred
  path: crates/metaspec/tests/search_page
- confidence: inferred
  path: fixtures/expected/search/page
- confidence: cited
  path: fixtures/expected/site
- confidence: cited
  path: web/ui/src/ResultSections.tsx
- confidence: cited
  path: web/ui/src/styles.css
- confidence: cited
  path: web/ui/src/types.ts
- confidence: inferred
  path: web/ui/test/result-sections.test.tsx
revision: 26
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:11Z", actor: "human:operator", revision: 25}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:11Z", actor: "human:operator", revision: 26}
---
# The page of a search result in the docs site

## Context

A document page of the docs site has a section "Run" with the component `ResultSections` when the
manifest declares `simulate` or `search` (`crates/metaspec-docs/src/documents.rs`); today it always
gets no data and says "No run result is committed beside this document", and the component
(`web/ui/src/ResultSections.tsx`, `ResultData` in `web/ui/src/types.ts`) reads a `fitness` and a
best variant, which `metaspec-result/1` does not have. The docs build runs no simulation or search.
A pack directory holds nothing of metaspec, so a result cannot be committed in it: the result is
given to the build, as `metaspec render` is given a quality report.
The epic promises the page "in the site and the studio". This story is the site half. The studio half
waits on `dependency-blocker:metaspec-studio` (no `metaspec serve`, no `crates/metaspec-server`; waves 5
and 6 of `design/metaspec.md`): it reads the same component when the studio exists.

## Scope

- `metaspec docs build ... --result <FILE>` (repeatable): a `metaspec-result/1`, validated against
  its schema. It must name a pack of the site and a document of that pack; a result for another
  pack or document, an invalid result or a digest that differs from the document as read is a
  refused build, exit 1, naming the file. A document with a result shows it; the others say none.
- The section "Run" of a document page is drawn when the manifest declares `simulate` or `search`, as today
  (`crates/metaspec-docs/src/documents.rs`, lines 435 to 441), and also whenever a `--result` is given for the
  document: a search needs only `check`, so a pack without `simulate` or `search` can show a result.
- `ResultSections` draws a `metaspec-result/1`: the baseline (score, KPI values with the unit and
  direction of the catalogue row), seed, budget, the number of tool runs and why the search stopped
  (`stop.reason` in words), and one row per variant: id, its edits as lines (`part:<name> path: from -> to`),
  its state, validity, gated findings and KPI values. The change from the baseline is the `verdicts`
  of the variant score (`story:search-result-format`), drawn as better or worse; the page reads them and
  computes no comparison. A variant with state `timed_out`, `refused` or `failed` shows its state and no
  values. Rows are sortable and every filter
  state is in the URL, as the KPI catalogue does.
- `ResultData` in `web/ui/src/types.ts` follows `schema/metaspec-result-1.schema.json`; the page data
  (`metaspec-page/1`, `schema/metaspec-page-1.schema.json`) carries it as the data of the component
  `ResultSections`; no new format and no schema change besides these.
- The stop reason is drawn in words by `ResultSections` for all four values of `stop.reason`, this story being the only
  one that draws it; the wording, fixed here, is one sentence per reason, read from `stop` and computing nothing:
  `deadline`: "Stopped: the time limit passed."; `budget`: "Stopped: the budget was used up after N candidates." (N is
  `stop.evaluated`); `no_better_neighbour`: "Stopped: no better neighbour was left to move to."; `exhausted`: "Stopped:
  every candidate was scored.". A `stop.reason` outside the four is a schema error, not a fifth wording.
- The fixture results are the recorded results of `story:search-enumerate`,
  `fixtures/expected/search/<pack>.json` (their digests match the documents). The committed site
  `fixtures/expected/site` is regenerated through the existing `task check:docs` path (both packs, with the
  trust step of that task, the results given by `--result`), as one commit with the generator change.
- The vitest cases of this story (its "stop reason" cases are also the second acceptance paragraph), in `web/ui/test/result-sections.test.tsx`, run by
  `task check:web`, render `ResultSections` on the
  recorded results `fixtures/expected/search/<pack>.json` of both fixture packs: the rows of a result with a
  better, a worse and a failed variant (variants, edits, scores, verdicts), the stop reason, the empty
  state, and that no comparison is computed in the component.
- Not in this story: drawing `best` and `front` (`story:search-result-best` owns that
  drawing and depends on this story), the studio page (`dependency-blocker:metaspec-studio`), any search, the text of
  `design/metaspec.md`.

## Surfaces

- changed: `crates/metaspec/src/main.rs` (`--result`), `crates/metaspec/src/docs.rs`,
  `crates/metaspec-docs/src/documents.rs` (the condition of the section "Run"), `crates/metaspec-docs/src/model.rs`,
  `web/ui/src/ResultSections.tsx`, `web/ui/src/types.ts`, `web/ui/src/styles.css`,
  `fixtures/expected/site` (the committed site of both fixture packs,
  rewritten as one commit with the generator change), `Taskfile.yml` (`check:docs` passes the
  results), `crates/metaspec/tests/main.rs` (one `mod` line), `CHANGELOG.md`.
- new: `web/ui/test/result-sections.test.tsx`, the hand-written recorded result
  `fixtures/expected/search/page/harbor-failed.json` (valid against `schema/metaspec-result-1.schema.json`, its
  digest the one of the harbor document, with a better, a worse and a failed variant); four hand-written results `fixtures/expected/search/page/stop-deadline.json`,
  `stop-budget.json`, `stop-no-better-neighbour.json` and `stop-exhausted.json` (each valid against the same schema, one
  `stop.reason` each, one variant each); `crates/metaspec/tests/search_page/mod.rs`, the case for the
  refusals of `--result`, a module of the one test binary (`crates/metaspec/tests/main.rs` gains one
  `mod search_page;` line).

## Shared files

- `fixtures/expected/site`, `crates/metaspec-docs/src/documents.rs`, `web/ui/src/types.ts`,
  `web/ui/src/styles.css`: also changed by `story:hub-badges`, `story:matrix-view`, `story:focus-pages`,
  `story:view-selectors` and `story:collapsed-regions`; the story that merges later regenerates the
  committed site, and the order of the waves is the planner call.
- `crates/metaspec/src/main.rs`: the argument `--result` of `DocsCommand::Build`, where the Args of `docs build` live.
- `crates/metaspec/tests/main.rs` (one `mod` line) and `Taskfile.yml` (`check:docs` passes the results): also
  changed by draft `story:spec-quality-page` and draft `story:spec-quality-self`, drafts of other epics; the
  coordinator orders the waves when they are accepted. No edge to drafts.

## Domain relations

- Run → Document, many-to-one: inferable (`spec/domains/runs.yaml`, `metaspec.runs.Run`, relation `document`).
- Run → Variant, one-to-many, owned: inferable (`spec/domains/runs.yaml`, `metaspec.runs.Run`,
  relation `variants`, kind `owns`).

## Constraints

- No word of a pack in the component or the generator: units, directions and names come from
  the catalogue and the result. Contrast and keyboard rules of "UX rules and gates" in `design/metaspec.md` apply.
- Tests: the committed fixture results and the vitest cases on the committed pages of both fixture
  packs; no tool runs, no network, no server per test. No test over 60 s, the test step within 120 s;
  a test over 5 s is named with its time. A slot job ends within 10 minutes. No Python.

## Acceptance

`cargo nextest run -p metaspec -E 'test(search_page)'` passes: the case runs `metaspec docs build ... --result
fixtures/expected/search/page/harbor-failed.json` and the page JSON of that document holds the variants, their
edits, scores and verdicts, the failed variant with its state and no values, and the stop reason; and the
refusals exit 1 naming the file (a result for another pack, a result for another document, an invalid result,
a digest that differs from the document).

`cd web/ui && slot npx vitest run --maxWorkers=4 test/result-sections.test.tsx -t "stop reason"` passes: one case per reason renders
`ResultSections` on `fixtures/expected/search/page/stop-<reason>.json` (four fixtures) and observes its sentence of the
list above, exactly one of the four sentences each time, and `N` in the budget sentence equal to `stop.evaluated` of the
fixture.
