---
format: aep.planning-md/3
id: story:cloudspec-runs
kind: story
status: draft
title: 'cloudspec''s runs: simulate, and metaspec''s search over its architectures'
summary: cloudspec's simulate and search through metaspec-result/1, after metaspec's runs and cloudspec's verbs.
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:cloudspec-manifest
- informed_by: architecture-decision-record:shared-search
- depends_on: story:search-simulate-adapter
- depends_on: story:search-enumerate
revision: 3
---
# cloudspec's runs: simulate, and metaspec's search over cloudspec architectures

## Context

architecture-decision-record:shared-search (the owner's decision B, 2026-11-28): metaspec runs the
search for every pack and a pack's tool only scores a candidate. cloudspec therefore needs a
`simulate` verb that reads one architecture and answers in JSON; it needs no `search` verb.
cloudspec keeps its own search (`cloudspec-sim`, modes exhaustive and fit) as it is.

The adapters come from epic:shared-search: story:search-simulate-adapter (the `simulate` verb in
`metaspec-pack/1`) and story:search-enumerate (`metaspec search`).

## Scope

- `packs/cloudspec.yaml` declares `simulate` with the mapping of cloudspec's answer to KPI values
  per document and per part.
- Marked places are the `{value, vary}` fields cloudspec's architectures write, including a
  choice of block.
- Acceptance rows for cloudspec's runs.

## Waits on

- metaspec: story:search-simulate-adapter and story:search-enumerate.
- cloudspec: a `simulate` verb that reads an architecture and answers in JSON with KPI values per
  document and per part. Recorded as dependency-blocker:cloudspec-run-verbs.

## Acceptance

`task acceptance -- ... <cloudspec>` exits 0 with a row showing that `metaspec search` of one
cloudspec architecture with a marked place writes a `metaspec-result/1`, valid against its schema,
whose variants' KPI values equal cloudspec's own `simulate` answer for each variant.
