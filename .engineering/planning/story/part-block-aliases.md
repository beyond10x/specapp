---
format: aep.planning-md/3
id: story:part-block-aliases
kind: story
status: implemented
title: A manifest names which block a part's block word stands for
summary: blocks.aliases maps a part's block word to a block of the pack; cloudspec declares group, node_pool and endpoints.
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:cloudspec-warnings
scope:
- confidence: inferred
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/graph.rs
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: cited
  path: crates/metaspec-core/src/pack.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/graph.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/manifest.rs
- confidence: cited
  path: crates/metaspec-docs/src/document.rs
- confidence: inferred
  path: design/metaspec.md
- confidence: cited
  path: packs/cloudspec.yaml
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: scripts/acceptance.sh
- confidence: inferred
  path: spec/domains/packs.yaml
revision: 23
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T16:19:54Z", actor: "human:operator", revision: 21}
- {from: "proposed", to: "active", at: "2026-11-28T16:19:54Z", actor: "human:operator", revision: 22}
- {from: "active", to: "implemented", at: "2026-11-28T19:04:12Z", actor: "human:operator", revision: 23, decided_on: {"recorded":{"test_result":1,"review_outcome":1}}}
---
# A manifest names which block a part's block word stands for

## Context

A part's `block:` is looked up by exact string among the domain names of the pack minus
`strip_prefix` (`crates/metaspec-core/src/pack.rs:482-575`, `crates/metaspec-layout/src/lib.rs:54-134`;
the miss is at `lib.rs:128-134`; `crates/metaspec-core/src/graph.rs:202` copies `part.block`).
cloudspec's parts name `group`, `node_pool` and `endpoints`: entities inside the domains
`architecture`, `infrastructure` and `edge`, not domains. `record_store` is a layer-1 contract with
no block. On the consumer's page these parts land in the group "block the pack does not have", in
slate, apart from the layer they belong to.

Decision D41 (taken, relayed by the operator session): an explicit manifest map, optional, applied
where a part's block is resolved; an alias whose target is no block refuses the manifest.
Decision D43: an alias whose key is itself a block of the pack refuses the manifest too.
Decision D47: one site applies it, when the graph is read. Decision D51: `graph::read` keeps its
signature.

## Domain relations

- Part → Block, many-to-one, zero or one block per part — inferable (`spec/domains/documents.yaml`,
  entity `Part`, relation `block`, `references metaspec.blocks.Block`, cardinality one).
- Alias word → Block, many-to-one, a word is the alias of one block — settled by D41, not by an
  `ess/1` document: no domain declares an alias. `spec/domains/packs.yaml` gains the field where
  it models `strip_prefix`.

## Scope

- `blocks.aliases: {<part block word>: <block>}` in the manifest, optional, in
  `schema/metaspec-pack-1.schema.json`; the format stays `metaspec-pack/1` and the CHANGELOG says
  it. `Blocks` in `crates/metaspec-core/src/manifest.rs` gains the field; `spec/domains/packs.yaml`
  models it; `design/metaspec.md` describes it (a manifest format change is written there,
  AGENTS.md:30).
- One site applies it: when the graph is read. `graph::read` (`crates/metaspec-core/src/graph.rs:182`)
  keeps its signature and its callers. A second function, `graph::read_with_aliases`, takes the
  alias map, and only `crates/metaspec-docs/src/document.rs:126` calls it. No other caller
  changes. The graph's part carries the target block; the written word is not kept. Everything
  downstream reads that target and needs no alias code: the style, the view `blocks` selection
  (`crates/metaspec-layout/src/lib.rs:230-234`), `hubs.blocks` and the matrix grouping.
- An alias whose target is not a block of the pack refuses the manifest, and so does an alias whose
  key is itself a block of the pack (`manifest_invalid`, naming the key or the target); the check
  runs where the pack's blocks are known (`pack.rs`), at `pack check` and at every command that
  loads the pack.
- A word with no alias and no block stays in the group "block the pack does not have".
- `packs/cloudspec.yaml` declares `aliases: {group: architecture, node_pool: infrastructure,
  endpoints: edge}`.
- Four tests, each named with `alias_`, on a fixture manifest built in the test (an alias over the
  harbor or orchard pack): `alias_names_the_target_block`, `alias_unknown_word_keeps_its_word`,
  `alias_target_not_a_block_is_refused`, `alias_key_that_is_a_block_is_refused`. The tests leave
  `fixtures/manifests/` alone.
- `scripts/acceptance.sh`: two cloudspec rows.

## Surfaces

`crates/metaspec-core/src/manifest.rs`, `crates/metaspec-core/src/pack.rs`,
`crates/metaspec-core/src/graph.rs`, `crates/metaspec-docs/src/document.rs`,
`schema/metaspec-pack-1.schema.json`, `spec/domains/packs.yaml`, `design/metaspec.md`,
`packs/cloudspec.yaml`, `scripts/acceptance.sh`, `CHANGELOG.md`, the test files of core
(`tests/cases/manifest.rs`, `tests/cases/graph.rs`). `graph::read` keeps its signature (D51);
`document.rs:126` is the only caller of `graph::read_with_aliases`. Shared with
story:cloudspec-warnings (`packs/cloudspec.yaml`, `scripts/acceptance.sh`), with
story:collapsed-regions, story:matrix-view, story:focus-pages and story:view-selectors
(`crates/metaspec-core/src/manifest.rs`, the schema, `design/metaspec.md`), and with
story:collapsed-regions and story:focus-pages (`crates/metaspec-docs/src/documents.rs`, next to
`document.rs`). It depends on story:cloudspec-warnings, so it lands after it.

## Not in this story

- A block the pack does not have and no alias names: it stays as today.
- `record_store`, a layer-1 contract with no block: no alias is declared for it.
- Aliases between the words of two blocks, a many-to-many map or a regular-expression alias.
- An alias site in the style or in a view: none; they read the graph.
- No word of the pack enters metaspec's code or tests; code tests use the fixture packs.

## Acceptance

- `cargo nextest run -p metaspec-core alias_names_the_target_block` passes: a part whose word is
  aliased has the target block in the graph.
- `cargo nextest run -p metaspec-core alias_unknown_word_keeps_its_word` passes: a word with
  neither alias nor block keeps its word in the graph.
- `cargo nextest run -p metaspec-core alias_target_not_a_block_is_refused` passes: the manifest is
  refused with `manifest_invalid`.
- `cargo nextest run -p metaspec-core alias_key_that_is_a_block_is_refused` passes: the manifest is
  refused with `manifest_invalid`, naming the key.
- `task acceptance -- <orgspec> <agenticspec> <cloudspec>` has a cloudspec row showing that every
  part of `groups.yaml`, `node-pools.yaml` and `l4-media.yaml` whose block word is `group`,
  `node_pool` or `endpoints` has the aliased block in the graph the page carries; the count is
  taken at run time from the architecture files (6, 3 and 1 at cloudspec main 6259dce are
  examples, not fixed numbers).
- The same run has a row showing that every part whose word is neither an alias nor a block of the
  pack (for example `payment_api` and `mail_relay` in `groups.yaml`) keeps its word, so it is drawn
  in "block the pack does not have" (`crates/metaspec-layout/src/lib.rs:137` draws that group for
  a block the pack lacks); the row compares the page's part blocks with the pack's block list from
  `metaspec pack check --json`.
- `task acceptance -- <orgspec> <agenticspec> <cloudspec>` exits 0 with both rows green.
- `task check:docs` passes: the pages of the fixture packs are unchanged byte for byte.

