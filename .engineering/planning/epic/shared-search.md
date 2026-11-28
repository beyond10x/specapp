---
format: aep.planning-md/3
id: epic:shared-search
kind: epic
status: active
title: 'Shared search: metaspec optimizes a document of any pack'
relations:
- informed_by: architecture-decision-record:shared-search
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T14:36:40Z", actor: "human:operator", revision: 2}
- {from: "proposed", to: "active", at: "2026-11-28T14:36:40Z", actor: "human:operator", revision: 3}
---
# Shared search: metaspec optimizes a document of any pack

## Outcome

`metaspec search <document> --pack <dir>` finds better variants of a composed document of any
trusted pack whose manifest has a `check` adapter (L3), with the strategies of
architecture-decision-record:shared-search, and writes `metaspec-result/1`: the variants, the
edits from the original to each (`metaspec-patch/1`), their scores and findings, and why the
search stopped (deadline, budget, no better neighbour, exhausted).

## What it covers

- The candidate space: the marked values (`{value, vary}`) and the generic edits a search may
  use; a pack's own edit verbs where its manifest declares them.
- The score: validity and findings from `check`, KPI values from `check` or `simulate`, ordered
  by the catalogue's `direction` per KPI and its `gate`; a variant that fails `check` or a gate is
  never better than a valid one.
- The strategies: hill climbing, beam search, bounded enumeration; seeded, so one seed gives one
  result.
- Budget and deadline: a number of scored candidates and a wall-clock limit; a tool run over the
  deadline is killed and shown as such.
- A batch adapter: a tool that scores many variants in one run declares it in its manifest;
  otherwise metaspec runs the tool once per candidate.
- A page of a search result in the site and the studio.
- `design/metaspec.md` says the decision: the verbs section, the L4 level and "What metaspec does
  not do".

## What it does not cover

- No rule and no simulation in metaspec.
- No search over ESS blocks; a search changes composed documents only.
- A pack's own `search` verb stays possible and is not built here.

## Consumers

agenticspec first (its architectures mark values with `vary`), then orgspec and cloudspec once
their tools score a document.
