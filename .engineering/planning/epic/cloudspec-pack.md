---
format: aep.planning-md/3
id: epic:cloudspec-pack
kind: epic
status: draft
title: metaspec draws the architectures of cloudspec
summary: A cloudspec manifest, containment by in and per, views by field and link pattern, collapsed regions, later runs.
revision: 4
---
# metaspec draws the architectures of cloudspec

## Context

The owner's decision of 2026-11-28 (relayed by the operator session): the architecture page of a
cloud stack moves to metaspec once metaspec has a cloudspec manifest. `design/metaspec.md` adds
`packs/cloudspec.yaml` "once it has blocks"; cloudspec main (562a5a6) has `blocks/` in numbered
layers (0-protocols .. 13-actors), `cloudspec check --json`, documents in the shape
`parts-links/1` (parts with `block`, links `{from, to, carries}`, `border`, `in`, `per`) and its
own `cloudspec render html`.

The consumer is a private pack, loaded with `--manifest <file>` and `extends: cloudspec` from its
own repository; an architecture of about 100 parts and 200 links. Nothing of it enters metaspec.

## Outcome

- `packs/cloudspec.yaml` at levels L0, L2 and L3 (blocks, documents, check), held by the
  acceptance like the other real packs.
- One document class may hold parts contained by `in` and by `per` at once.
- A large architecture reads in views chosen by blocks, by a field of the part and by link name
  patterns (for example data, edge and messaging).
- A region (`in:`) can be drawn collapsed, with the links that cross its border rolled up, and a
  drill-down to the region's own view.
- Parts that sit in no region can be drawn collapsed too: one box per block, optionally split by
  the value of a part field, with the count and the member names, the links rolled up and a
  drill-down as for a region (story:block-groups).
- A band of the layout `rows` can wrap at a set number of boxes, so a view of many boxes in one
  layer is not one row wide (story:band-wrap).
- The page of a document shows the warnings of cloudspec's check as findings that do not gate
  (story:cloudspec-warnings).
- A part whose block word names an entity inside a cloudspec block (for example `group`,
  `node_pool`, `endpoints`) is drawn in that block, through an alias map in the manifest
  (story:part-block-aliases; the coordinator's D41, 2026-11-28).
- A pack that `extends: cloudspec` inherits its blocks and tool from the base pack's directory
  given at run time, so the consumer copies neither (story:extends-inherits; D42).
- Later: cloudspec's runs (simulate, search) once metaspec has runs (build order, wave 7) and
  cloudspec has a verb that reads an architecture and answers in JSON.

## Found on 2026-11-28 (read-only, cloudspec 562a5a6)

- cloudspec has no `architectures/` directory yet; its valid documents are under
  `fixtures/cloudspec/` (`minimal.yaml`, `agent/`, `host-pool.yaml`,
  `hosted-contract/architecture.yaml`), broken ones under `fixtures/cloudspec/broken/`. Its
  `docs/layers.md` promises `architectures/<name>.yaml`.
- No KPI catalogue (`story:kpis-catalogue` of cloudspec is draft); the field `kpis` of
  `cloudspec check --json` is a count per rule, so the manifest maps it to `counts`.
- `fixtures/render/shop.yaml` uses `in: operator` and `per: agent` on one part; metaspec's
  manifest takes one containment field per class (`Contain::In | Per`,
  `crates/metaspec-core/src/manifest.rs`).
- Blocks used in documents that are no ESS domain (`stream`, `endpoints`, `queue`, ...) are drawn
  as blocks the pack does not have, as for agenticspec.
- `View` selects `blocks` and exact `carries` names only; cloudspec's `carries` holds protocol
  names and contract operations (`http`, `store.put`, `queue.enqueue`).
- No collapse state exists in `metaspec-scene/1` or the layouts.
