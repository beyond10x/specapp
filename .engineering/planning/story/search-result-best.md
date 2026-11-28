---
format: aep.planning-md/3
id: story:search-result-best
kind: story
status: active
title: The result page draws the front and the best variant
summary: ResultSections marks the front and the best variant, shows strategy and beam width; reads the result and computes no order.
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-design
- depends_on: story:search-result-page
- depends_on: story:search-dominance
- depends_on: story:search-gate
- informed_by: decision-blocker:kpi-order
- depends_on: story:search-beam
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: design/metaspec.md
- confidence: inferred
  path: fixtures/expected/search/best
- confidence: cited
  path: fixtures/expected/site
- confidence: cited
  path: web/ui/src/ResultSections.tsx
- confidence: cited
  path: web/ui/src/styles.css
- confidence: cited
  path: web/ui/src/types.ts
- confidence: inferred
  path: web/ui/test/result-front.test.tsx
revision: 12
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:12Z", actor: "human:operator", revision: 11}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:12Z", actor: "human:operator", revision: 12}
---
# The result page draws the front and the best variant

## Context

`story:search-result-page` draws the variants of a `metaspec-result/1`, their edits, scores, verdicts and the
stop reason (all four reasons, in words), and leaves out `best` and `front`, which the result had no order for.
`story:search-dominance` adds both to the result (dominance, decided by `decision-blocker:kpi-order`,
answer A), and the strategy stories write `strategy` and `beam_width`. This story draws them in the
component `ResultSections`. The studio half waits on `dependency-blocker:metaspec-studio`, as in
`story:search-result-page`.

## Scope

- `ResultSections` marks the variants of `front` in the table (a "front" mark with an accessible name, not
  colour alone), marks `best` when the result has it, lists the front as its own block above the table,
  and shows `strategy` and `beam_width`; the stop reason is drawn by `story:search-result-page` for all four reasons and not here. A variant
  whose score says `feasible` false (`story:search-gate` writes it) is shown as such and is never marked. The
  member `baseline` of `front` is drawn as the baseline row. The component reads `front`, `best` and
  `feasible`, states no predicate and computes no order.
- `ResultData` in `web/ui/src/types.ts` gains the members as `schema/metaspec-result-1.schema.json` and
  `schema/metaspec-score-1.schema.json` (`feasible`, `gate_failed`) define them after `story:search-dominance` and `story:search-gate`;
  no new format and no schema change.
- The committed site `fixtures/expected/site` is regenerated through the existing `task check:docs` path
  (both packs, the trust step, the results given by `--result`), as one commit with the component
  change; this is a Scope item and not the acceptance.
- Not in this story: the stop reason in words (`story:search-result-page` owns it, for all four reasons); an order or a front computed in the page, the studio page
  (`dependency-blocker:metaspec-studio`), the text of `design/metaspec.md` beyond one paragraph.
- Its paragraph of `design/metaspec.md`: under "Docs" of `story:search-design`, one paragraph that states the
  drawing of the front and the best variant.

## Surfaces

- changed: `web/ui/src/ResultSections.tsx`, `web/ui/src/types.ts`, `web/ui/src/styles.css`,
  `fixtures/expected/site`, `design/metaspec.md` (one paragraph), `CHANGELOG.md`.
- new: `web/ui/test/result-front.test.tsx`, the hand-written recorded result `fixtures/expected/search/best/beam-best.json`
  (valid against `schema/metaspec-result-1.schema.json`: `strategy` `beam`, `beam_width` 2, a `stop` that the schema requires and this story does not draw, a front of exactly one variant that dominates the baseline, so `best` is set, and a second
  variant with `feasible` false), and `fixtures/expected/search/best/baseline-front.json` (hand-written, valid
  against the same schema: two KPIs, a `front` of the member `baseline` and one variant that is incomparable with it, each
  better on a different KPI, and no `best`).

## Shared files

- `web/ui/src/ResultSections.tsx`, `web/ui/src/types.ts`, `web/ui/src/styles.css`,
  `fixtures/expected/site`: created or changed by `story:search-result-page`, which this story depends on;
  also changed by `story:hub-badges`, `story:matrix-view`, `story:focus-pages`, `story:view-selectors` and
  `story:collapsed-regions`; the story that merges later regenerates the committed site, and the order of
  the waves is the planner call.
- `design/metaspec.md` and `fixtures/expected/site`: this story depends on `story:search-beam` (which depends on
  `story:search-hill-climb`) because the strategy stories each add a paragraph to the design file after
  `story:search-design`, and `story:search-dominance` and `story:search-gate` regenerate the site; the later of
  them regenerates it.
- `fixtures/expected/search/best/beam-best.json` and `fixtures/expected/search/best/baseline-front.json`: this story
  creates them; no other story writes them.

## Domain relations

- Run → Variant, one-to-many, owned: inferable (`spec/domains/runs.yaml`, `metaspec.runs.Run`, relation
  `variants`, kind `owns`). The relations `front` and `best` are written by `story:search-dominance`; this
  story only reads them.

## Constraints

- No word of a pack in the component. Contrast and keyboard rules of "UX rules and gates" in
  `design/metaspec.md` apply.
- Tests: the two hand-written results `fixtures/expected/search/best/beam-best.json` and
  `fixtures/expected/search/best/baseline-front.json`; no tool run, no
  network, no server per test. No test over 60 s, the test step within 120 s; a test over 5 s is named with
  its time. A slot job ends within 10 minutes. No Python.

## Acceptance

`cd web/ui && slot npx vitest run --maxWorkers=4 test/result-front.test.tsx` passes: the cases render
`ResultSections` on `fixtures/expected/search/best/beam-best.json` and observe the front block with exactly its one
variant, the mark on `best`, `strategy` beam with `beam_width` 2, and
that the variant with `feasible` false is not marked; and on `fixtures/expected/search/best/baseline-front.json` they
observe the baseline row and the incomparable variant in the front block and no best mark.
