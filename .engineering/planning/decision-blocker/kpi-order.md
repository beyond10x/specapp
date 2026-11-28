---
format: aep.planning-md/3
id: decision-blocker:kpi-order
kind: decision-blocker
status: cleared
title: Nobody has decided how the KPI values of two valid candidates combine into one order
summary: Without an order of valid candidates a search cannot pick a best; default A is dominance.
relations:
- blocks: epic:shared-search
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-11-28T15:02:02Z", actor: "human:operator", revision: 3}
---
# Nobody has decided how the KPI values of two valid candidates combine into one order

## The question

`epic:shared-search` says a score is "ordered by the catalogue `direction` per KPI and its
`gate`". A KPI row has one direction (better when higher, or when lower). A search that keeps
"the best" needs to say which of two valid candidates is better when one has the better value
of KPI x and the other the better value of KPI y. Nothing in the epic, in
architecture-decision-record:shared-search or in the catalogue formats says how the per-KPI
directions combine.

## Why it is open

- A pack own search (agenticspec experiments) combines KPIs with weights the experiment
  carries; those weights are that pack input, not metaspec, and the catalogue holds none
  (`schema/metaspec-kpis-1.schema.json` has `direction` and `gate`, no weight and no priority).
- Any order metaspec picks is a choice of the goal of the user: a weight, a priority list, or
  none. The row order of the catalogue carries no priority.

## Options

- **A (default)**: dominance. Candidate B is better than candidate A when every KPI with a
  direction is at least as good in B and at least one is better. Two candidates, neither of
  which dominates the other, are both kept; the result lists the non-dominated candidates as
  its front. No weight and no input is invented.
- **B**: the caller names the KPIs in priority order (`--objective kpi,kpi`); candidates
  compare lexicographically. A search without it is refused.
- **C**: the caller gives a weight per KPI (`--weight kpi=w`); the order is the weighted sum of
  the values, each normalized by the value of the original document.

## What it stops

The choice of a best candidate: hill climbing, beam search, the `best` (or `front`) member of
`metaspec-result/1`, and the stop reason "no better neighbour". Scoring, the candidate space,
the result format without a best, bounded enumeration that lists every scored candidate, the
batch adapter and the page are not stopped.

## Answer

Option A, dominance, decided on 2026-11-28 (D4 = A, relayed by the operator session): a candidate is better when every KPI with a direction is at least as good and one is better; candidates neither of which dominates the other are all kept as the front. B (priority order) and C (weights) may come later as options.
