---
format: aep.planning-md/3
id: architecture-decision-record:shared-search
kind: architecture-decision-record
status: accepted
title: metaspec runs the search; a pack's tool scores a candidate
relations:
- decides: epic:shared-search
revision: 2
transitions:
- {from: "proposed", to: "accepted", at: "2026-11-28T14:36:40Z", actor: "human:operator", revision: 2}
---
# metaspec runs the search; a pack's tool scores a candidate

## Context

`design/metaspec.md` (section on the verbs to come) made `search` a verb of each pack's tool:
metaspec passes a budget and a deadline, the tool searches and answers in `metaspec-result/1`.
Each pack would then write its own search loop. What a search needs is already generic in
metaspec's formats:

- the moves: the generic edits of every `parts-links/1` document (`add_part`, `remove_part`,
  `set_field`, `add_link`, `remove_link`, `move`, `set_border`) and the edit verbs a pack
  declares in its manifest;
- the places a search may change: a value written `{value, vary}`;
- the goal: each KPI of a pack's catalogue has a `direction` and a `gate`.

Only the score of a candidate needs the pack's rules: validity and findings from `check`, KPI
values from `check` or `simulate`.

## Decision

The owner decided on 2026-11-28 (option B of three): metaspec runs the search once for every
pack. It generates candidates with the moves above, asks the pack's tool to score each one
through the `check` and `simulate` adapters, and keeps the best by the catalogue's directions and
gates, within the budget and deadline it already passes to a run. The strategies are metaspec's:
hill climbing, beam search, and bounded enumeration of the marked values.

A pack may still declare its own `search` verb; metaspec then shows both results in
`metaspec-result/1`.

## Consequences

- metaspec still knows no rule and simulates nothing: every score comes from the pack's tool.
- A search starts the pack's tool once per candidate unless the tool scores many variants in one
  run. A batch adapter for `check` and `simulate` is part of the epic; a pack without it is
  scored one candidate per run. (Inferred cost, not measured.)
- A search runs a pack's tool, so it runs only for a trusted pack, as every run does.
- story:cloudspec-runs needs `simulate` from cloudspec, no `search`.

## Alternatives

- A: keep `search` a verb of each pack's tool. Every pack writes its own loop.
- C: a Rust library each pack's tool links. Only Rust tools could use it.
