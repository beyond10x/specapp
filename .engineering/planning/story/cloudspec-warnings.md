---
format: aep.planning-md/3
id: story:cloudspec-warnings
kind: story
status: active
title: The page of a document shows the warnings of the pack's check
summary: packs/cloudspec.yaml declares a /warnings finding list with gate false; the page of journeys.yaml shows its 3 findings.
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:cloudspec-manifest
scope:
- confidence: inferred
  path: CHANGELOG.md
- confidence: cited
  path: packs/cloudspec.yaml
- confidence: cited
  path: scripts/acceptance.sh
revision: 12
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T16:19:54Z", actor: "human:operator", revision: 11}
- {from: "proposed", to: "active", at: "2026-11-28T16:19:54Z", actor: "human:operator", revision: 12}
---
# The page of a document shows the warnings of the pack's check

## Context

The consumer, a private pack that extends cloudspec, drew a document whose check reports 3
warnings and read "0 findings" on its page. The check adapter reads only the finding lists the
manifest declares (`crates/metaspec-core/src/adapter.rs:80-96`, `FindingList` in
`crates/metaspec-core/src/manifest.rs:299-306`), and `packs/cloudspec.yaml` declares one,
`/violations`. `cloudspec check --json` also writes `warnings: []`, entries of the same shape as a
violation with `gate: false`: `{rule, gate, message, subject, journey, part, reason}`. The public
example is cloudspec's `architectures/journeys.yaml`: 3 warnings at cloudspec main 6259dce, rule
`single_point_of_failure`.

## Domain relations

- Document → Finding, one-to-many, a finding is on a part, a link or the document — inferable
  (inferred from `crates/metaspec-core/src/adapter.rs:80-96`, a finding list maps each entry to
  one finding; no `ess/1` document declares a relation of a finding to the list that produced it).
  A second list adds findings to the same document and creates no new relation.

## Scope

- `packs/cloudspec.yaml`: the verb `check` gains a second finding list, `at: /warnings`, with the
  rule, message and subject mapping of `/violations` and `gate: false`. No Rust change: the schema
  and the adapter already take `gate: false` and several lists. Decision D45: no new Rust test; the
  acceptance rows below are the check.
- `scripts/acceptance.sh`: the two existing rows that compare a page with `.violations` only now
  count `.violations + .warnings`:
  - `theirs_cloudspec` (`scripts/acceptance.sh:260-262`), the rule ids of a document from
    `cloudspec check --json`, behind `examples:rules-matched`;
  - `named` (`scripts/acceptance.sh:335-341`), the findings on a part or link of the document,
    behind `examples:named-drawn`.
- `scripts/acceptance.sh`: a new cloudspec row for the page of `architectures/journeys.yaml`. The
  row reads its numbers from `cloudspec check --json` at run time, never a fixed count.
- The checkout of cloudspec given to `task acceptance -- <orgspec> <agenticspec> <cloudspec>` is
  at a commit whose `check --json` writes `warnings`: 6259dce or later.
- `CHANGELOG.md`: a line.

## Surfaces

`packs/cloudspec.yaml`, `scripts/acceptance.sh`, `CHANGELOG.md`. `scripts/acceptance.sh` is shared
with story:focus-pages; the order against it is an edge the coordinator sets (D46).
story:hub-badges touches neither `scripts/acceptance.sh` nor `packs/cloudspec.yaml` and is merged
(D52), so this story has no edge to it. story:cloudspec-manifest (implemented) last edited
`packs/cloudspec.yaml`. `CHANGELOG.md` is append-only lines and no reason to order.

## Not in this story

- The `resilience` block of `cloudspec check --json`: it needs an adapter field for the tool's
  output tables, a separate story.
- Any change to how a warning is drawn: a finding with `gate: false` is drawn as the pages draw
  one today.
- A Rust test of the finding list: the adapter code is unchanged.
- No word of the pack enters metaspec's code or tests.

## Acceptance

- `task acceptance -- <orgspec> <agenticspec> <cloudspec>`, cloudspec at 6259dce or later, has a
  cloudspec row showing that the page of `architectures/journeys.yaml` shows as many findings, none
  of them gated, as the length of `warnings` in `cloudspec check --json` on the same file at run
  time (3 at cloudspec main 6259dce).
- The same run has a row showing that document stays valid on the page.
- The row `examples:rules-matched` counts `.violations + .warnings` and is green on `journeys.yaml`
  (before this story it counts `.violations` only).
- The row `examples:named-drawn` counts `.violations + .warnings` and is green on `journeys.yaml`
  (before this story it counts `.violations` only).
- `task acceptance -- <orgspec> <agenticspec> <cloudspec>` exits 0.

