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
scope:
- confidence: inferred
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/manifest.rs
- confidence: cited
  path: crates/metaspec-docs/src/documents.rs
- confidence: cited
  path: crates/metaspec-layout/src/lib.rs
- confidence: cited
  path: crates/metaspec-layout/src/nested.rs
- confidence: inferred
  path: crates/metaspec-layout/src/rows.rs
- confidence: cited
  path: crates/metaspec-layout/src/scene.rs
- confidence: inferred
  path: crates/metaspec-layout/src/svg.rs
- confidence: inferred
  path: crates/metaspec-layout/tests/cases/scenes.rs
- confidence: cited
  path: crates/metaspec/src/page.rs
- confidence: inferred
  path: crates/metaspec/tests/main.rs
- confidence: inferred
  path: design/metaspec.md
- confidence: inferred
  path: fixtures/expected/harbor/pages/plan
- confidence: inferred
  path: fixtures/expected/orchard/pages/layout
- confidence: inferred
  path: fixtures/expected/site/harbor/docs/plan
- confidence: inferred
  path: fixtures/expected/site/harbor/pages/plan
- confidence: inferred
  path: fixtures/expected/site/orchard/docs/layout
- confidence: inferred
  path: fixtures/expected/site/orchard/pages/layout
- confidence: inferred
  path: fixtures/manifests/harbor.yaml
- confidence: inferred
  path: fixtures/manifests/orchard.yaml
- confidence: inferred
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: schema/metaspec-scene-1.schema.json
- confidence: inferred
  path: spec/domains/documents.yaml
revision: 27
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
