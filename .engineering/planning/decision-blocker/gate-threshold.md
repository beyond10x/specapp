---
format: aep.planning-md/3
id: decision-blocker:gate-threshold
kind: decision-blocker
status: cleared
title: Nobody has decided what value of a gated KPI makes a candidate fail its gate
summary: A catalogue gate is a Boolean with no threshold; default A is no worse than the original document.
relations:
- blocks: epic:shared-search
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-11-28T15:02:02Z", actor: "human:operator", revision: 3}
---
# Nobody has decided what value of a gated KPI makes a candidate fail its gate

## The question

`epic:shared-search`: "a variant that fails `check` or a gate is never better than a valid one".
Failing `check` is known: the check result is not valid, or a finding is gated. Failing the
gate of a KPI is not: a catalogue row has `gate` as a Boolean ("whether the KPI gates"), and no
row has a threshold.

## Why it is open

- `schema/metaspec-kpis-1.schema.json` types `gate` as a Boolean. The measure
  `completeness.fields` of metaspec already counts `gate: true` as incomplete because the
  threshold is missing (`CHANGELOG.md`).
- A pack may say in prose what a gated KPI must be (a count that must be 0), which holds for a
  count and not for a share or a ratio.
- In one pack the gated KPIs of the catalogue are also rules of its check, so the check
  already refuses them; in another pack that is not known.

## Options

- **A (default)**: a candidate fails the gate of a KPI when its value is worse, in the
  direction of the KPI, than the value of the original document. No threshold is needed; the
  original is the floor.
- **B**: a gated KPI passes only at 0.
- **C**: the manifest names a threshold per gated KPI (a new field of `metaspec-pack/1`).

## What it stops

The gate test of a KPI value in a score: a story that makes a candidate infeasible because of a
gated KPI. Failing `check` (not valid, a gated finding) is not stopped.

## Answer

Option A, decided on 2026-11-28 (D5 = A, relayed by the operator session): a candidate fails the gate of a gated KPI when its value is worse, in the KPI's direction, than the original document's; the original is the floor. No new format.
