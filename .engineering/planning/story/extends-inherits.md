---
format: aep.planning-md/3
id: story:extends-inherits
kind: story
status: implemented
title: A manifest that extends a pack inherits its blocks and its tool
summary: A manifest with extends may omit blocks and tool; they come from the base pack, read in --base-pack.
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:part-block-aliases
- depends_on: story:collapsed-regions
scope:
- confidence: inferred
  path: CHANGELOG.md
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: cited
  path: crates/metaspec-core/src/pack.rs
- confidence: cited
  path: crates/metaspec-core/src/paths.rs
- confidence: inferred
  path: crates/metaspec-core/src/tool.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/manifest.rs
- confidence: inferred
  path: crates/metaspec-core/tests/cases/trust.rs
- confidence: cited
  path: crates/metaspec/src/main.rs
- confidence: inferred
  path: crates/metaspec/tests/main.rs
- confidence: inferred
  path: design/metaspec.md
- confidence: inferred
  path: fixtures/extends
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: scripts/acceptance.sh
- confidence: inferred
  path: spec/domains/packs.yaml
revision: 24
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T16:19:55Z", actor: "human:operator", revision: 20}
- {from: "proposed", to: "active", at: "2026-11-28T16:19:55Z", actor: "human:operator", revision: 21}
- {from: "active", to: "implemented", at: "2026-11-28T20:37:44Z", actor: "human:operator", revision: 24, decided_on: {"recorded":{"test_result":1,"review_outcome":1}}}
---
# A manifest that extends a pack inherits its blocks and its tool

## Context

A private pack that extends cloudspec copies cloudspec's `blocks` section and its `tool` section
into its own manifest. Today `extends` is only validated (`crates/metaspec-core/src/manifest.rs:43`,
`crates/metaspec-core/src/pack.rs:105-148`, warnings at `crates/metaspec/src/main.rs:214-216`);
nothing is inherited. `blocks` is required (`manifest.rs:44`, the schema's `required`). Every
path stays inside its pack (`crates/metaspec-core/src/paths.rs:6-41`, `path_leaves_pack`); a
`tool.run` with a `/` is joined to the pack root and a bare name is looked up on PATH
(`pack.rs:716-731`), so a copied tool path points into the private pack's directory, where the
base's tool is not.

Decision D42 (taken, relayed by the operator session): a manifest with `extends: <name>` may omit
`blocks` and `tool`; they are inherited from the base pack's manifest and their paths resolve
inside the base pack's directory.

## Domain relations

- Pack → Pack (`extends`), many-to-one, a pack extends at most one base, the base may exist
  without its extenders — inferable (`spec/domains/packs.yaml`, entity `Pack`, relation
  `extends`, `references metaspec.packs.Pack`, cardinality one; a pack with `extends` is private).
- Inherited section → base pack, a section is read from the base's manifest and its paths are
  read in the base's directory — settled by D42, not by an `ess/1` document.

## Scope

- The base pack's directory is given at run time by one new flag, `--base-pack <DIR>`, accepted
  by the single-pack commands (`PackArgs`, `crates/metaspec/src/main.rs`), beside `--pack <DIR>`.
  The base's manifest comes as it does today: the known pack the name `extends` resolves to, or
  the file of the existing `--extends <FILE>`. `docs build` with several packs is not in this
  story.
- A manifest with `extends` may omit `blocks` and `tool`. An omitted section is the base's, with
  its paths read in the `--base-pack` directory. A section the manifest writes is its own, read in
  its own pack. A manifest without `extends` that omits `blocks` is refused as today.
- `blocks` is inherited whole, `aliases` (story:part-block-aliases) included; the alias target
  check runs against the base's blocks. An extending pack that declares `blocks` declares all of
  it: no merge with the base's.
- The inherited tool runs with the base pack directory as its root and working directory
  (`crates/metaspec-core/src/tool.rs:126`).
- Every path still resolves inside its own pack: no path of the private pack leaves it, none of the
  base's leaves the base. A refusal names which pack the path was in.
- A manifest that inherits and is run without `--base-pack` is refused, saying what to give.
- A base that itself extends is refused with a message (the coordinator's decision D44), with a
  test.
- `metaspec trust` covers the base's tool digest too: a changed base tool is untrusted until trusted
  again.
- `kpis` is inherited when the extending manifest omits it (the coordinator's D56, 2026-11-28,
  from a second consumer whose documents live in its own repository): `kpis.path` resolves inside
  the base pack. A manifest that declares `kpis` declares all of it, as for `blocks`.
- `documents` and `presentation` are not inherited: the extending pack names its own documents,
  which is how documents kept outside the base pack's checkout are drawn.
- Fixture tests with an invented base pack and an extending pack under a new `fixtures/extends/`,
  the tool a stub, each named `inherit_<case>`: `inherit_blocks`, `inherit_kpis`, `inherit_tool_runs_in_base_dir`,
  `inherit_alias_target_checked_against_base_blocks`, `inherit_without_base_pack_is_refused`,
  `inherit_path_leaving_extending_pack_is_refused`, `inherit_path_leaving_base_pack_is_refused`,
  `inherit_refusal_names_the_pack`, `inherit_base_that_extends_is_refused`,
  `inherit_trust_digest_covers_base_tool`. No real pack tool.
- `scripts/acceptance.sh`: two cloudspec rows (below). The extending manifest the rows write to the
  acceptance work directory declares its own `documents` (they are not inherited).
- `spec/domains/packs.yaml`, `schema/metaspec-pack-1.schema.json` (`blocks` no longer required
  by the schema; the loader holds the rule), `CHANGELOG.md`, `design/metaspec.md`.

## Surfaces

`crates/metaspec-core/src/manifest.rs`, `crates/metaspec-core/src/pack.rs`,
`crates/metaspec-core/src/paths.rs`, `crates/metaspec-core/src/tool.rs`,
`crates/metaspec/src/main.rs`, `schema/metaspec-pack-1.schema.json`, `spec/domains/packs.yaml`,
`scripts/acceptance.sh`, `CHANGELOG.md`, `design/metaspec.md`, `fixtures/extends`, the test files
of core and of `metaspec`. Shared, by story:
- story:view-selectors: `crates/metaspec-core/src/manifest.rs`, the schema, `design/metaspec.md`.
- story:collapsed-regions: `crates/metaspec-core/src/manifest.rs`, the schema, `design/metaspec.md`.
- story:matrix-view: `crates/metaspec-core/src/manifest.rs`, the schema, `design/metaspec.md`.
- story:focus-pages: `crates/metaspec-core/src/manifest.rs`, the schema, `design/metaspec.md`,
  `scripts/acceptance.sh`.
- story:part-block-aliases: `manifest.rs`, `pack.rs`, the schema, `spec/domains/packs.yaml`,
  `design/metaspec.md`, `scripts/acceptance.sh`.
- story:cloudspec-warnings: `scripts/acceptance.sh`, `packs/cloudspec.yaml`, through the chain.
- story:search-score: `crates/metaspec-core/src/manifest.rs`, the schema, `design/metaspec.md`,
  `crates/metaspec/src/main.rs`. It lands after this story. The order is kept so the consumer's
  gaps land first; the alternative was an additive merge on those files.
It depends on story:part-block-aliases and story:collapsed-regions and lands before
story:view-selectors, story:matrix-view and story:focus-pages, which depend on it (the
coordinator's D57, 2026-11-28: two consumers need inheritance, none waits on the other three).
The shared files are the same; only the order changed.

## Not in this story

- Inheritance of `documents` and `presentation`. `kpis.directions` comes from story:search-score,
  which lands after this story; an inherited `kpis` carries it once search-score lands.
- A merge of the base's blocks with blocks of the extending pack.
- A base pack that is not given: no download, no search of the machine for it.
- `--base-pack` on `docs build` with several packs.
- No word of the pack enters metaspec's code or tests; code tests use invented fixture packs.

## Acceptance

- `cargo nextest run -p metaspec-core -p metaspec inherit_blocks` passes: the blocks of an
  extending pack are the base's.
- `cargo nextest run -p metaspec-core -p metaspec inherit_kpis` passes: an extending pack that
  omits `kpis` reads the base's catalogue, and one that declares `kpis` reads its own.
- `cargo nextest run -p metaspec-core -p metaspec inherit_tool_runs_in_base_dir` passes: the tool
  is the base's and runs with the base pack directory as its root.
- `cargo nextest run -p metaspec-core -p metaspec inherit_alias_target_checked_against_base_blocks`
  passes: the alias target check runs against the base's blocks.
- `cargo nextest run -p metaspec-core -p metaspec inherit_without_base_pack_is_refused` passes: a
  manifest that inherits and is run without `--base-pack` is refused.
- `cargo nextest run -p metaspec-core -p metaspec inherit_path_leaving_extending_pack_is_refused`
  passes.
- `cargo nextest run -p metaspec-core -p metaspec inherit_path_leaving_base_pack_is_refused`
  passes.
- `cargo nextest run -p metaspec-core -p metaspec inherit_refusal_names_the_pack` passes: the
  refusal message of a path names which pack the path was in.
- `cargo nextest run -p metaspec-core -p metaspec inherit_base_that_extends_is_refused` passes.
- `cargo nextest run -p metaspec-core -p metaspec inherit_trust_digest_covers_base_tool` passes:
  the trust digest changes with the base's tool.
- `task acceptance -- <orgspec> <agenticspec> <cloudspec>` has a cloudspec row: an extending
  manifest written to the acceptance work directory renders `groups.yaml`, and its graph (each
  part's block, the aliased parts included) equals that of `packs/cloudspec.yaml` for the same
  document; the pack name, the title and the visibility differ by design.
- The same run has a row: the same extending manifest renders `journeys.yaml`, and its findings
  (`rule`, `subject`, `gate`, the warnings included) equal those of `packs/cloudspec.yaml`.
- `task acceptance -- <orgspec> <agenticspec> <cloudspec>` exits 0 with both rows green (the `groups.yaml` graph row and the `journeys.yaml` findings row).
- `task check:docs` passes: the pages of the fixture packs are unchanged byte for byte.

