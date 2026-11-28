---
format: aep.planning-md/3
id: story:cloudspec-manifest
kind: story
status: implemented
title: 'packs/cloudspec.yaml: cloudspec at levels L0, L2 and L3'
summary: cloudspec's manifest with blocks, the architecture class and the check verb, held by the acceptance.
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:contain-in-and-per
scope:
- confidence: inferred
  path: AGENTS.md
- confidence: inferred
  path: CHANGELOG.md
- confidence: inferred
  path: Taskfile.yml
- confidence: cited
  path: crates/metaspec-core/src/manifest.rs
- confidence: cited
  path: crates/metaspec-core/src/pack.rs
- confidence: cited
  path: crates/metaspec-core/tests/cases/manifest.rs
- confidence: cited
  path: design/metaspec.md
- confidence: cited
  path: packs/cloudspec.yaml
- confidence: cited
  path: schema/metaspec-pack-1.schema.json
- confidence: cited
  path: scripts/acceptance.sh
- confidence: inferred
  path: spec/domains/documents.yaml
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T13:51:13Z", actor: "human:operator", revision: 2}
- {from: "proposed", to: "active", at: "2026-11-28T13:51:13Z", actor: "human:operator", revision: 3}
- {from: "active", to: "implemented", at: "2026-11-28T15:31:25Z", actor: "human:operator", revision: 7, decided_on: {"recorded":{"test_result":1,"review_outcome":1}}}
---
# packs/cloudspec.yaml: cloudspec at levels L0, L2 and L3

## Context

`design/metaspec.md` adds cloudspec's manifest once cloudspec has blocks; it has them (562a5a6).
A read of cloudspec main found the fields, and one gap in cloudspec's layout (below).

## Scope

- `packs/cloudspec.yaml`:
  - `blocks: {path: blocks, strip_prefix: cloud.}` (`blocks/system.yaml`, domains `cloud.*`).
  - one document class `architecture`, shape `parts-links/1`, `paths: [architectures/*.yaml]`,
    `contain` with `in` and `per` (story:contain-in-and-per).
  - `tool.run: [target/debug/cloudspec]`, verb `check`: `args: [check, --json]`, exit 0 or 1,
    `valid: /valid`, `counts: /kpis` (a count per rule), `findings` from `/violations` with rule,
    gate, message and subject (a link subject contains `->`, else a part, else the document).
  - a second class `broken-architecture`, same shape, `paths: [architectures/broken/*.yaml]`.
  - no `kpis:` section until cloudspec has a KPI catalogue.
- A document class gains `exclude: [<glob>]`: a file matched by `paths` and by an `exclude` glob
  is not read. cloudspec's `architectures/` holds `*.binding.yaml` and `*.contract.yaml` files
  that are no architectures (cloudspec `docs/layers.md`; at e79e07a `host-pool.binding.yaml`,
  `registry.binding.yaml`, `job_queue.contract.yaml`, `broken/contract_form.contract.yaml`), and
  the `glob` crate has no negation. Lands in `crates/metaspec-core/src/manifest.rs`,
  `crates/metaspec-core/src/pack.rs` (the documents reader) and `schema/metaspec-pack-1.schema.json`,
  after story:contain-in-and-per, which edits the same files.
- `scripts/acceptance.sh`: a third real pack, `task acceptance -- <orgspec> <agenticspec>
  <cloudspec>`, with the rows the other packs have (counts equal to `cloudspec check`, every
  architecture drawn, every broken architecture shows its rule on the part or link it names, no
  word of the pack in the code).
- `design/metaspec.md`: "Gaps per pack" gains cloudspec.

## Waits on

Nothing: dependency-blocker:cloudspec-architectures-dir is cleared (cloudspec#17, e79e07a).

## Acceptance

`cargo nextest run -p metaspec-core -E 'test(exclude_)'` passes with a case where a file matched
by `paths` and by `exclude` is not read and a file matched by `paths` only is.

`task acceptance -- <orgspec> <agenticspec> <cloudspec>` exits 0 with cloudspec rows: `metaspec
pack check cloudspec --pack <cloudspec>` level 3; blocks, layers and documents equal to what
cloudspec's own files hold; one page per architecture; every broken architecture's page shows the
rule `cloudspec check --json` names on the part or link it names.
