---
format: aep.planning-md/3
id: story:hub-badges
kind: story
status: implemented
title: Hubs drawn as badges on their callers
summary: 'A view elides parts with many incoming links: drawn once, each link to them a badge on its caller.'
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:cloudspec-manifest
scope:
- confidence: inferred
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: inferred
  path: crates/metaspec-core/src/pack.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/manifest.rs
- confidence: inferred
  path: crates/metaspec-docs/src/documents.rs
- confidence: cited
  path: crates/metaspec-layout/src/lib.rs
- confidence: cited
  path: crates/metaspec-layout/src/nested.rs
- confidence: cited
  path: crates/metaspec-layout/src/rows.rs
- confidence: cited
  path: crates/metaspec-layout/src/scene.rs
- confidence: cited
  path: crates/metaspec-layout/src/svg.rs
- confidence: inferred
  path: crates/metaspec-layout/tests/cases/hubs.rs
- confidence: inferred
  path: crates/metaspec-layout/tests/cases/mod.rs
- confidence: inferred
  path: crates/metaspec-layout/tests/cases/scenes.rs
- confidence: inferred
  path: crates/metaspec/src/page.rs
- confidence: inferred
  path: crates/metaspec/tests/main.rs
- confidence: inferred
  path: design/metaspec.md
- confidence: inferred
  path: fixtures/manifests
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: schema/metaspec-scene-1.schema.json
revision: 23
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T13:54:28Z", actor: "human:operator", revision: 2}
- {from: "proposed", to: "active", at: "2026-11-28T13:54:28Z", actor: "human:operator", revision: 3}
- {from: "active", to: "implemented", at: "2026-11-28T18:19:56Z", actor: "human:operator", revision: 23, decided_on: {"recorded":{"test_result":1,"review_outcome":1}}}
---
# Hubs drawn as badges on their callers

## Context

Measured by the consumer on a private cloud architecture (109 parts, 209 links): 103 links go
into 10 targets. Drawn as edges they make the page unreadable; a probe that drew each such target
once and each link to it as a badge on the caller left 52 edges and read well. Asked as V1 by the
consumer, 2026-11-28.

## Scope

- A view may elide hubs: `hubs: {in_degree: <N>}` (parts with at least N incoming links shown in
  the view) and/or `hubs: {parts: [<id>], blocks: [<block>]}` (named by the manifest).
- A hub is drawn once, apart (a band of hubs); each link to a hub is a badge on its caller with
  the hub's name and the link name, not an edge. Links between hubs stay edges.
- `metaspec-scene/1` carries badges; the static page draws them; a finding on an elided link is
  drawn on the caller's badge.
- The static page has no script: the toggle is two views, the view with hubs elided and the same
  view without (`<view>` and `<view>/edges`); the docs site shows them as one view with a switch.

## Acceptance

`cargo nextest run -p metaspec-core -p metaspec-layout -p metaspec -E 'test(hub_)'` passes with
cases showing that in a fixture view with `hubs: {in_degree: 3}` a part with 3 incoming links is
drawn once with no incoming edge, each of its callers carries one badge per link to it, the
`/edges` view draws the same links as edges, and a finding on an elided link is drawn on the
badge; the existing pages of both real packs stay byte for byte.
