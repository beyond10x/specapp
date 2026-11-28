---
format: aep.planning-md/3
id: story:cloudspec-runs
kind: story
status: draft
title: 'cloudspec''s runs: simulate and search'
summary: cloudspec's simulate and search through metaspec-result/1, after metaspec's runs and cloudspec's verbs.
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:cloudspec-manifest
revision: 1
---
# cloudspec's runs: simulate and search

## Context

Wave 7 of the build order in `design/metaspec.md` ("Runs"): the verbs `simulate` and `search` map
a tool's answer to `metaspec-result/1`. Neither exists in metaspec yet (`metaspec-pack/1`
declares only `check`; no `schema/metaspec-result-1.schema.json`). cloudspec's simulator
(`cloudspec-sim`) simulates an ESS model, takes no architecture path and has no JSON answer per
part.

## Scope

- `packs/cloudspec.yaml` declares `simulate` and `search` with the mapping of cloudspec's answer
  to `metaspec-result/1`, and marked places from the `{value, vary}` fields cloudspec already
  writes.
- Acceptance rows for cloudspec's runs.

## Waits on

- metaspec: runs (build order, wave 7), not planned yet as stories.
- cloudspec: a `simulate` and a `search` verb that read an architecture and answer in JSON with
  KPI values per document and per part. Recorded as dependency-blocker:cloudspec-run-verbs.

## Acceptance

`task acceptance -- ... <cloudspec>` exits 0 with rows showing `metaspec simulate` of one
cloudspec architecture gives a `metaspec-result/1` valid against its schema whose KPI values
equal cloudspec's own answer, and `metaspec search` gives a best variant whose marked places are
fields of the architecture with `vary`.
