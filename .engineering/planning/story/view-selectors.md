---
format: aep.planning-md/3
id: story:view-selectors
kind: story
status: active
title: Views select parts by a field and links by a name pattern
summary: 'View gains carries prefix patterns and where: {field: values}.'
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:cloudspec-manifest
- depends_on: story:part-block-aliases
scope:
- confidence: inferred
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: inferred
  path: crates/metaspec-core/src/words.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/manifest.rs
- confidence: cited
  path: crates/metaspec-layout/src/lib.rs
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
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: inferred
  path: spec/domains/documents.yaml
revision: 21
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T13:51:13Z", actor: "human:operator", revision: 2}
- {from: "proposed", to: "active", at: "2026-11-28T13:51:13Z", actor: "human:operator", revision: 3}
---
# Views select parts by a field and links by a name pattern

## Context

An architecture of about 100 parts and 200 links does not read as one picture. The manifest
already has views per document class (`views: [{id, title, layout, rows, blocks, carries}]`,
`crates/metaspec-core/src/manifest.rs`; orgspec uses two). A view selects parts by block and
links by exact name. cloudspec's link names are protocols and contract operations (`http`,
`store.put`, `queue.enqueue`), and a view of one concern cuts across blocks (workloads of one kind).

## Scope

- `View` gains:
  - `carries` entries that end in `*` match by prefix (`store.*`, `queue.*`);
  - `where: {<field>: [<values>]}` selects parts by a field the part writes (`kind`, `role`);
    a part without the field is not selected by `where`.
- A link is shown when both ends are shown and its name matches, as today.
- Schema `metaspec-pack-1`, the graph's view selection, and the "All parts and links" view stay.
- A fixture pack gains a view that uses both.

## Acceptance

`cargo nextest run -p metaspec-core -p metaspec -E 'test(view_)'` passes with cases showing a
prefix pattern selects exactly the links whose name starts with it, `where` selects exactly the
parts whose field holds one of the values, and the fixture view's page holds those parts and
links only; the existing pages of both real packs stay byte for byte.
