---
format: aep.planning-md/3
id: story:matrix-view
kind: story
status: active
title: 'A matrix view: callers by targets'
summary: 'layout: matrix draws callers as rows, targets as columns, link names in the cells.'
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:cloudspec-manifest
- depends_on: story:part-block-aliases
- depends_on: story:collapsed-regions
- depends_on: story:extends-inherits
scope:
- confidence: inferred
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/manifest.rs
- confidence: cited
  path: crates/metaspec-layout/src/lib.rs
- confidence: inferred
  path: crates/metaspec-layout/src/matrix.rs
- confidence: cited
  path: crates/metaspec-layout/src/scene.rs
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
revision: 23
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T13:54:28Z", actor: "human:operator", revision: 2}
- {from: "proposed", to: "active", at: "2026-11-28T13:54:28Z", actor: "human:operator", revision: 3}
---
# A matrix view: callers by targets

## Context

For document classes where a node-link drawing fails (many links into few targets), the consumer
asked (V3, 2026-11-28) for a matrix: callers as rows, targets as columns, the link name in the
cell.

## Scope

- A view may have `layout: matrix` with `targets: {in_degree: <N>}` (default 3) or named parts:
  columns are the targets, rows the parts that link to at least one of them, a cell the link
  names from row to column (several names in one cell, sorted).
- `metaspec-scene/1` gains the matrix form; the static page draws it as a table with sticky
  headers, rows and columns grouped by block; a finding on a link is marked in its cell.
- Parts and links outside the matrix are listed below it, so the view hides nothing.

## Acceptance

`cargo nextest run -p metaspec-core -p metaspec -E 'test(matrix_)'` passes with cases showing that
a fixture view with `layout: matrix` and `targets: {in_degree: 2}` has one column per part with
at least 2 incoming links, one row per caller of them, the link names in the cells, a finding on
a link marked in its cell, and every other link listed below; the existing pages of both real
packs stay byte for byte.
