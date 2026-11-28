---
format: aep.planning-md/3
id: story:collapsed-regions
kind: story
status: active
title: A region is drawn collapsed, with drill-down
summary: A view collapses regions to one box with rolled-up border links and a drill-down view.
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:cloudspec-manifest
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T13:51:13Z", actor: "human:operator", revision: 2}
- {from: "proposed", to: "active", at: "2026-11-28T13:51:13Z", actor: "human:operator", revision: 3}
---
# A region is drawn collapsed, with drill-down

## Context

Layout `nested` draws every part inside its container. For a large architecture a region (a part
other parts are `in`) should be drawable as one box. Nothing in `metaspec-scene/1` or the layouts
holds a collapse state today. cloudspec's own renderer rolls links up by from, to and carries
with a count (`src/render/map.rs` of cloudspec).

## Scope

- A view may name regions to collapse (`collapse: [<part id>]`, or `collapse: all` for every
  container at its top level).
- `metaspec-scene/1`: a collapsed region is one box with the number of parts inside; links that
  cross its border are drawn from or to the box, rolled up by far end and name, with a count.
- Drill-down: each collapsed region links to a view of its own children (generated, id
  `<view>/<region>`), on the static page and in the docs site.
- Findings on a part inside a collapsed region are counted on the box and drawn in the region's
  own view.

## Acceptance

`cargo nextest run -p metaspec-core -p metaspec-layout -p metaspec -E 'test(region_)'` passes with
cases showing a collapsed region is one box carrying its part count, two links from inside to one
far part with one name become one edge with count 2, the region's page holds its children, and a
finding inside the region is counted on the box; the existing pages of both real packs stay byte
for byte.
