---
format: aep.planning-md/3
id: story:search-design
kind: story
status: active
title: 'design/metaspec.md says the decision: metaspec runs the search, a pack tool scores a candidate'
summary: 'The one story that edits the design: verbs, levels, operations, formats, gaps per pack, what metaspec does not do; checked by a grep of old and new sentences.'
relations:
- decomposes: epic:shared-search
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-score
- depends_on: story:search-result-format
- depends_on: story:search-simulate-adapter
- depends_on: story:search-candidate-space
- depends_on: story:search-enumerate
- depends_on: story:search-batch-scoring
- depends_on: story:search-result-page
- depends_on: story:cloudspec-manifest
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: design/metaspec.md
revision: 15
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T15:48:11Z", actor: "human:operator", revision: 14}
- {from: "proposed", to: "active", at: "2026-11-28T15:48:11Z", actor: "human:operator", revision: 15}
---
# design/metaspec.md says the decision: metaspec runs the search, a pack tool scores a candidate

## Context

architecture-decision-record:shared-search moves the search from the verbs of a pack tool to
metaspec. `design/metaspec.md` still says the opposite: "The verbs to come" makes `search` a verb
of the tool that metaspec passes a budget and a deadline to; L4 is "the tool simulate and search";
"Operations" names "the tool search"; "What metaspec does not do" does not mention a search. The
stories that build formats, adapters and commands write no text in the design file: this story owns the
change of `design/metaspec.md` for the decision. Six later stories of the epic each write one paragraph or
row of it after this story, each with a `depends_on` edge to it: `story:search-orgspec` (the orgspec row of
"Gaps per pack"), `story:search-gate`, `story:search-dominance`, `story:search-hill-climb`,
`story:search-beam` and `story:search-result-best` (a paragraph each under "Decided: shared search" or
"Docs").

## Scope

- "The tool adapter", the paragraph "The verbs to come": the sentence that makes `simulate` and
  `search` verbs that metaspec passes a budget and a deadline to is removed. In its place: "`search` is
  not a verb of the manifest: metaspec runs the search, generates candidates from the marked places and
  asks the tool to score each through `check` and `simulate`." `simulate` is described as an adapter
  (`story:search-simulate-adapter`: its fields, what the tool must accept and answer) and `batch` as an
  optional part of the `check` and `simulate` adapters (`story:search-batch-scoring`: its fields and the
  contract of the tool); `graph` and `apply` stay to come; a pack may still declare a verb `search` of its
  own, which is not built, and metaspec would then show both results.
- "Levels of adoption", the line L4 is replaced by: "- **L4 runs.** The tool `simulate` gives KPI values of
  a document from a run; the search of metaspec needs only L3." "What the packs share" no longer lists
  search among what the tool can do beyond the check. L5 stays as it is.
- "Operations": the sentence "One operation set for the user, the agent and the tool search" becomes "the
  agent and the search of metaspec"; a search emits only `set_field` edits on marked places (decided by
  `decision-blocker:search-edits`, answer A, and written here as the state of the code).
- "What metaspec does not do" gains the bullet: "- It scores nothing and simulates nothing: its search
  generates candidates and the tool of the pack scores each."
- "Neutral formats": entries for `metaspec-score/1` (`story:search-score`, with `verdicts` of
  `story:search-result-format` and `source` and `per_part` of `story:search-simulate-adapter`), and the full
  entries of `metaspec-result/1` and `metaspec-patch/1` (`story:search-result-format`), each with its schema
  file; the revisions of `metaspec-pack/1` (`kpis.directions`, `tool.verbs.simulate`, `batch`) and of
  `metaspec-graph/1` (`vary` as a list or `{ min, max, step }`), with the file names the pack repositories cite.
- "Docs": a result is given to the build with `--result` (`story:search-result-page`).
- "Gaps per pack": agenticspec (its tool has no verb that simulates one architecture document and
  answers on stdout, `sim run` takes an experiment and writes files; no batch verb; its own search
  stays); orgspec (its row is written by `story:search-orgspec` after this story, from what it reads in the orgspec
  checkout; this story writes none); cloudspec: its row is written by
  `story:cloudspec-manifest`, and this story adds to it only that its own search stays and its runs are
  `story:cloudspec-runs`. A gap is never worked around in code.
- "Build order", wave 7 "Runs", and "Decided in wave 3" where it says `ResultSections` has no result:
  updated to what the stories built. A new section "Decided: shared search" records what the
  stories decided: the two forms of `vary`, a marked `block`, candidates written to a scratch
  directory, seeded drawing when the space exceeds the budget, the four stop reasons `deadline`, `budget`, `no_better_neighbour` and `exhausted` with their precedence, written once, as one line that starts "Stop reasons by precedence:" and lists the four in
  that order, from `search::run::stop_reason` of `story:search-enumerate` (the paragraphs of the strategy stories name the
  reason by which their strategy ends and restate none), the batch contract,
  the words of a direction per pack.
- What stays open is written as open, with its blocker: the studio page (`dependency-blocker:metaspec-studio`),
  and a pack own edit verbs, deferred to a later epic by the answer of `decision-blocker:search-edits` (an
  `apply` adapter).
  The order of valid candidates, the gate test of a KPI value, hill climbing and beam search are decided
  (`decision-blocker:kpi-order`, `decision-blocker:gate-threshold`) and drafted as `story:search-gate`,
  `story:search-dominance`, `story:search-hill-climb`, `story:search-beam` and `story:search-result-best`;
  each owns its own paragraph of this file and depends on this story, so this story writes none of them and
  words `--strategy` as `enumerate` only, the others to come.
- Not in this story: any code, schema, manifest or script.

## Surfaces

- changed: `design/metaspec.md`, `CHANGELOG.md`.

## Shared files

- `design/metaspec.md`: also listed by `story:cloudspec-manifest`, which this story depends on, and may be
  written by other epics of the plan store (the quality stories). Six later stories of this epic write it
  after this story, each depending on it: `story:search-orgspec`, `story:search-gate`,
  `story:search-dominance`, `story:search-hill-climb`, `story:search-beam` and `story:search-result-best`.
  The stories that build code write nowhere in it.

## Domain relations

- None assumed: the design text states the decision of architecture-decision-record:shared-search and
  cites the formats the other stories build.

## Constraints

- Public repository: no content of a real pack, no private name, no home path (`scripts/check-public.sh`).
- No build and no network are needed to check it. No Python.

## Acceptance

In `design/metaspec.md` the three old sentences (L4, the verbs section, Operations, in the order the greps run them) are gone and the four
new ones (the verbs section, L4, Operations, "What metaspec does not do"), the heading "Decided: shared
search" and the one line of the stop reasons are each present once. One `grep -c` per fragment, each fragment short enough to stay on one line of
the file, so the line prints `0 0 0 1 1 1 1 1 1`; today, before the story, it prints `1 1 1 0 0 0 0 0 0` (the
first three greps, in that order, find the old sentences of L4, the verbs section and Operations):

```sh
echo $(grep -c '`simulate` and `search`: results' design/metaspec.md) $(grep -c 'and `search` (to `metaspec-result/1`' design/metaspec.md) $(grep -c 'the agent and the tool.s search' design/metaspec.md) $(grep -c '`search` is not a verb of the manifest' design/metaspec.md) $(grep -c 'The tool `simulate` gives KPI values' design/metaspec.md) $(grep -c 'the agent and the search of metaspec' design/metaspec.md) $(grep -c 'It scores nothing and simulates nothing' design/metaspec.md) $(grep -c '^## Decided: shared search$' design/metaspec.md) $(grep -c 'Stop reasons by precedence: `deadline`, `budget`, `no_better_neighbour`, `exhausted`' design/metaspec.md)
```
