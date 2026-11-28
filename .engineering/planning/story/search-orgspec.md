---
format: aep.planning-md/3
id: story:search-orgspec
kind: story
status: active
title: orgspec through score and search
summary: orgspec directions and KPI values in its manifest where its tool answers them, recorded answers, an acceptance row that scores every example and searches a marked copy.
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-score
- depends_on: story:search-enumerate
- depends_on: story:search-design
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: inferred
  path: crates/metaspec-core/tests/cases/search_score.rs
- confidence: cited
  path: design/metaspec.md
- confidence: inferred
  path: fixtures/answers/orgspec
- confidence: cited
  path: packs/orgspec.yaml
- confidence: inferred
  path: scripts/acceptance.d/search-orgspec.sh
revision: 18
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:11Z", actor: "human:operator", revision: 17}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:12Z", actor: "human:operator", revision: 18}
---
# orgspec through score and search

## Context

After agenticspec, the epic names orgspec and cloudspec as consumers "once their tools score a
document". `packs/orgspec.yaml` has a `check` adapter (`orgspec check --json`: `valid`, `/refusals`,
`/violations`, `counts: /kpis`) and no `kpis` pointer, no `kpis.directions` and no `simulate`. The
manifest reads the KPI catalogue with `fields: { measure: measures, modes: postures }`. The
plan store already holds `story:cloudspec-runs` for cloudspec, which waits on
`dependency-blocker:cloudspec-run-verbs`; it is not part of this story.

## Scope

- Read first, read-only, on a checkout of orgspec main (not committed anywhere): the words its
  catalogue writes for `direction`; whether `orgspec check --json` answers KPI values per KPI id (and
  where in the answer); whether any design in `organizations/` marks a place `{value, vary}`.
  Each answer that is a lack is one gap line. This story writes the orgspec row of "Gaps per pack" in
  `design/metaspec.md` itself, one line per gap found (a row with no gap says so), after `story:search-design`,
  which it depends on and which writes no orgspec row. Nothing of the checkout is copied into this
  repository.
- `packs/orgspec.yaml` gains `kpis.directions` with the words found (`story:search-score`), and
  `check.kpis` when the answer holds KPI values by id. No other change to the manifest.
- The acceptance row `scripts/acceptance.d/search-orgspec.sh` runs, on the checkout, `metaspec score
  orgspec --pack <dir> <design> --json` for every example design, and `metaspec search orgspec --pack
  <dir> <copy> --seed 1` on a copy of one example under the target directory of the run in which
  the row marks one numeric field with `vary` (the row writes the copy; nothing is added to
  the checkout). The search validates its result against the schema before it writes it.
- If the check answer of orgspec carries no KPI value by id, this story cannot meet its acceptance: it
  stops and files a dependency-blocker for the tool of orgspec (the side of the pack), as the plan store
  does for cloudspec, and a score of orgspec stays validity and gated findings with no KPIs;
  that is the pack gap, shown as it is and not worked around in code.
- Not in this story: a `simulate` adapter for orgspec (it has no simulator), the cloudspec manifest,
  any change in the orgspec repository, the text of `design/metaspec.md` beyond the orgspec row of "Gaps per pack".

## Surfaces

- changed: `packs/orgspec.yaml`, `design/metaspec.md` (the orgspec row of "Gaps per pack"), `CHANGELOG.md`.
- new: `scripts/acceptance.d/search-orgspec.sh`; recorded answers of `orgspec check --json` on invented
  documents under `fixtures/answers/orgspec/organization/` (`scripts/record-answers.sh orgspec <dir>`) with
  the `<case>.score.json` goldens; the cases go into `crates/metaspec-core/tests/cases/search_score.rs`
  (created by `story:search-score`) through the replay loop that story builds (`scripts/replay-answers.sh`),
  not into `answers.rs`.

## Shared files

- `design/metaspec.md`: one row after `story:search-design`, which this story depends on; the other later
  writers are `story:search-gate`, `story:search-dominance`, `story:search-hill-climb`, `story:search-beam`
  and `story:search-result-best`, each in its own paragraph.
- `packs/orgspec.yaml`: co-written by draft `story:kpi-quality-values` and
  draft `story:spec-quality-conformance` (drafts of other epics; the coordinator orders the waves when they are accepted; no edge to drafts).
- `crates/metaspec-core/tests/cases/search_score.rs`: created by `story:search-score`, which this story depends
  on; this story adds test functions to it.

## Domain relations

- Pack → Verb, one-to-many: inferable (`spec/domains/packs.yaml`, `metaspec.packs.Verb`, field `tool_verbs`
  of type `List<metaspec.packs.Verb>`). No other relation is assumed.

## Constraints

- No word of a pack under `crates/`, nothing of orgspec content here beyond the manifest under
  `packs/` and recorded answers of invented inputs (`scripts/check-public.sh`, `scripts/check-vocabulary.sh`).
- Tests: recorded answers only; the real tool runs only in the acceptance row, outside `task check`.
  No test over 60 s, the test step within 120 s; a test over 5 s is named with its time. A slot job ends
  within 10 minutes. No Python.

## Acceptance

`metaspec score orgspec --pack <orgspec-dir> <example design> --json`, the command of
`scripts/acceptance.d/search-orgspec.sh`, prints for the KPI id that script names a `direction` of `higher` or
`lower` and a value, where before this story it printed no KPI entry or the direction `none`, because
`packs/orgspec.yaml` now maps the direction words of orgspec and reads the KPI values of its check answer.
