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
- depends_on: story:search-enumerate
- depends_on: story:search-score
- depends_on: story:search-batch-scoring
- depends_on: story:cloudspec-warnings
revision: 6
---
# cloudspec's runs: metaspec's search over cloudspec architectures, scored by check

## Context

architecture-decision-record:shared-search (the owner's decision B, 2026-11-28): metaspec runs the
search for every pack and a pack's tool only scores a candidate. cloudspec keeps its own search
(`cloudspec-sim`, modes exhaustive and fit) as it is.

cloudspec plans a `cloudspec simulate` verb to the terms of story:search-simulate-adapter
(cloudspec's story:cloudspec-simulate). Which engine it uses is an open architecture question on
cloudspec's side. This story is planned on `check` alone: the KPI values of a candidate are the
`/kpis` counts that `cloudspec check --json` answers. Declaring `simulate` is a later story, once
cloudspec's verb exists.

The adapters come from epic:shared-search: story:search-score (`kpis.directions`),
story:search-batch-scoring (`batch`) and story:search-enumerate (`metaspec search`).

## Scope

- Marked places are the `{value, vary}` fields cloudspec's architectures write, including a
  choice of block.
- `packs/cloudspec.yaml` declares `kpis.directions` (cloudspec's direction words mapped to higher or
  lower, story:search-score) and a `batch` entry on `check`:
  `batch: {args: [check, --json, --batch], limit: 256, at: /results}`, with `exit: [0, 1]`
  (story:search-batch-scoring). The manifest lives in metaspec; cloudspec holds no manifest of
  metaspec.
- Acceptance rows for cloudspec's runs.

## Not in this story

- `simulate` in `packs/cloudspec.yaml`: a later story once cloudspec's `simulate` verb exists
  (dependency-blocker:cloudspec-run-verbs).

## Waits on

- metaspec: story:search-score, story:search-batch-scoring and story:search-enumerate.
- cloudspec: `cloudspec check --json --batch <p1>..<pn>` answering `results` in path order: on
  cloudspec main since 8d919bf (cloudspec PR 26), limit 256, exit [0, 1].

## Acceptance

`task acceptance -- ... <cloudspec>` exits 0 with a row showing that `metaspec search` of one public
cloudspec architecture with a marked place writes a `metaspec-result/1`, valid against its schema,
whose variants' KPI values equal the `/kpis` of `cloudspec check --json` for each variant, and a
row showing that the same search with the batch entry gives the same result as without it.
