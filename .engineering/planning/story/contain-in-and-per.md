---
format: aep.planning-md/3
id: story:contain-in-and-per
kind: story
status: active
title: One document class holds parts contained by in and by per
summary: The manifest's contain takes in, per or both; graph and nested layout keep both.
relations:
- decomposes: epic:cloudspec-pack
scope:
- confidence: inferred
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/graph.rs
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/fixtures.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/graph.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/manifest.rs
- confidence: cited
  path: crates/metaspec-layout/src/nested.rs
- confidence: inferred
  path: crates/metaspec-layout/src/rows.rs
- confidence: inferred
  path: crates/metaspec-layout/tests/cases/scenes.rs
- confidence: inferred
  path: design/metaspec.md
- confidence: inferred
  path: fixtures/expected/harbor
- confidence: inferred
  path: fixtures/expected/harbor.json
- confidence: inferred
  path: fixtures/expected/site/harbor
- confidence: inferred
  path: fixtures/manifests/harbor.yaml
- confidence: inferred
  path: fixtures/packs/harbor/harbors
- confidence: inferred
  path: schema/metaspec-graph-1.schema.json
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: spec/domains/documents.yaml
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T13:51:13Z", actor: "human:operator", revision: 2}
- {from: "proposed", to: "active", at: "2026-11-28T13:51:13Z", actor: "human:operator", revision: 3}
---
# One document class holds parts contained by `in` and by `per`

## Context

cloudspec's documents place one part `in` an operator and `per` an agent at once
(`fixtures/render/shop.yaml` of cloudspec 562a5a6). metaspec's manifest takes one containment
field per document class (`Contain::In(field) | Contain::Per(field)`,
`crates/metaspec-core/src/manifest.rs`), so a cloudspec manifest can declare only one of them.
metaspec-only; nothing waits on cloudspec.

## Scope

- Manifest: `contain` takes `in`, `per` or both (`{in: <field>, per: <field>}`); the schema
  `metaspec-pack-1` and the manifest loader follow; the old single forms stay valid.
- Graph: `metaspec-graph/1` keeps `inside` and `per` of a part both; `groups` from both.
- Layout `nested` draws a part inside its `in` container and marks its `per` like today.
- A fixture: one of the fixture packs gains a document whose part has both.

## Acceptance

`cargo nextest run -p metaspec-core -p metaspec-layout -E 'test(contain_)'` passes with cases
showing a manifest with both fields loads, the graph of a part with both keeps both, and the
fixture document's page draws the part inside its `in` container; the existing pages of both real
packs stay byte for byte (the docs check of `task check`).
