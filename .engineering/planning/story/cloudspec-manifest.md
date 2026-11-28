---
format: aep.planning-md/3
id: story:cloudspec-manifest
kind: story
status: active
title: 'packs/cloudspec.yaml: cloudspec at levels L0, L2 and L3'
summary: cloudspec's manifest with blocks, the architecture class and the check verb, held by the acceptance.
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:contain-in-and-per
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T13:51:13Z", actor: "human:operator", revision: 2}
- {from: "proposed", to: "active", at: "2026-11-28T13:51:13Z", actor: "human:operator", revision: 3}
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
  - no `kpis:` section until cloudspec has a KPI catalogue.
- `scripts/acceptance.sh`: a third real pack, `task acceptance -- <orgspec> <agenticspec>
  <cloudspec>`, with the rows the other packs have (counts equal to `cloudspec check`, every
  architecture drawn, every broken architecture shows its rule on the part or link it names, no
  word of the pack in the code).
- `design/metaspec.md`: "Gaps per pack" gains cloudspec.

## Waits on

- cloudspec: architectures in a pack directory, `architectures/*.yaml`, and broken ones in a
  directory of their own (today they are only under `fixtures/cloudspec/`). Recorded as
  dependency-blocker:cloudspec-architectures-dir.

## Acceptance

`task acceptance -- <orgspec> <agenticspec> <cloudspec>` exits 0 with cloudspec rows: `metaspec
pack check cloudspec --pack <cloudspec>` level 3; blocks, layers and documents equal to what
cloudspec's own files hold; one page per architecture; every broken architecture's page shows the
rule `cloudspec check --json` names on the part or link it names.
