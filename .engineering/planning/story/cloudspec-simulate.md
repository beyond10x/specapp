---
format: aep.planning-md/3
id: story:cloudspec-simulate
kind: story
status: draft
title: packs/cloudspec.yaml declares simulate, with a batch of 144
summary: cloudspec's simulate verb (cloudspec main 90e1939) declared in the manifest with seed, profile, exit mapping, batch limit 144 and the KPI directions; one acceptance row on a public architecture.
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:cloudspec-runs
- depends_on: story:search-simulate-adapter
- depends_on: story:search-batch-scoring
- depends_on: story:search-score
- informed_by: dependency-blocker:cloudspec-run-verbs
scope:
- confidence: inferred
  path: CHANGELOG.md
- confidence: cited
  path: packs/cloudspec.yaml
- confidence: inferred
  path: scripts/acceptance.d/cloudspec-simulate.sh
- confidence: cited
  path: scripts/acceptance.sh
revision: 4
---
# packs/cloudspec.yaml declares simulate, with a batch of 144

## Context

dependency-blocker:cloudspec-run-verbs said cloudspec had no verb that reads an architecture and
answers in JSON. It has one now: cloudspec main 90e1939d21a5ef5fd0251212288e9a8178cb79b5
(cloudspec PR #31).

- `cloudspec simulate --load <profile> --seed <n> <architecture>` prints one JSON line
  `{"kpis","per_part","per_journey"}`. Exit 0 is an answer, exit 1 is `{"error"}` (a refused
  document), exit 2 is bad usage. It is deterministic per seed.
- `cloudspec simulate ... --batch <p1> ... <pn>` prints `{"results":[...]}` in argument order, each
  entry byte-equal to the single run, a refused document as its own `{"error"}` entry. The batch
  limit is 144.
- Six KPI ids, all `simulated` in cloudspec's `kpis/catalogue.yaml`: `cloud.throughput_rps` is
  higher is better; `cloud.latency_p50_ms`, `cloud.latency_p95_ms`, `cloud.latency_p99_ms`,
  `cloud.failure_rate` and `cloud.saturation` are lower is better.

metaspec's side: `tool.verbs.simulate` (story:search-simulate-adapter) and `batch` inside an
adapter (story:search-batch-scoring) are planned as manifest features; `packs/cloudspec.yaml` has
only `check` today (`packs/cloudspec.yaml:38-58`) and no `kpis` section. story:cloudspec-runs
declares the `check` batch in that file and names this story as the later one for `simulate`
("Not in this story"); the `kpis` section, `kpis.directions` included, is this story's.

## Which story owns it

A new story, not a line in story:search-simulate-adapter. That story is active, it belongs to
epic:shared-search and to no pack file (its scope names no `packs/` path; `packs/agenticspec.yaml`
is only in its text), so putting `packs/cloudspec.yaml` in it would give the file a second owner
beside story:cloudspec-runs. This story belongs to epic:cloudspec-pack, owns the `simulate` entry
and the `kpis` section of `packs/cloudspec.yaml`, and runs after story:cloudspec-runs, which owns
the file's `check` batch.

## Scope

- `packs/cloudspec.yaml` declares `tool.verbs.simulate`:
  `args: [simulate, --load, <profile>, --seed, "1"]`, where `<profile>` is a load profile that
  cloudspec ships in its checkout, a path inside the pack (the file is named when the story is
  refined, from cloudspec main 90e1939; this draft does not read it). The seed is fixed in the
  arguments, so a document has one score.
- `exit: [0, 1]`: 0 is an answer, 1 is a refused document (`error: /error`); exit 2 (usage) is not
  listed and is a failed run, as is any other exit. `kpis: { at: /kpis, value: ... }` and
  `per_part: { at: /per_part, value: ... }` as the adapter takes them (the pointer to the number
  inside an entry is read from the answer when the story is refined). `per_journey` is not
  mapped: the adapter has no field for it.
- `batch` inside it, for story:search-batch-scoring:
  `batch: { args: [simulate, --load, <profile>, --seed, "1", --batch], limit: 144, at: /results }`.
  A refused document is an entry with `error`; the other entries are answered. The limit is
  cloudspec's, 144, not the 256 of `check` (story:cloudspec-runs).
- `kpis` section: `path: kpis/catalogue.yaml` and the catalogue's own `fields`, and
  `kpis.directions` (story:search-score) as the words of cloudspec's catalogue rows for
  higher and lower is better, so that `cloud.throughput_rps` reads `higher` and the five others
  read `lower`. This story owns the whole `kpis` section of `packs/cloudspec.yaml`
  (`path`, `fields` and `directions`); story:cloudspec-runs declares none of it.
- The manifest is then L4 (story:search-simulate-adapter, `Manifest::level`). The level row of
  `scripts/acceptance.sh` for cloudspec changes from L3 to L4 (line 197 and the header comment
  at line 29).
- A KPI id the simulation answers that the catalogue does not list is reported, not refused (as
  for agenticspec, story:search-simulate-adapter).
- No code in `crates/`: the change is a manifest, an acceptance row and a line in `CHANGELOG.md`
  only if a manifest change is listed there.

## Surfaces

- changed: `packs/cloudspec.yaml`, `scripts/acceptance.sh` (the cloudspec level row),
  `CHANGELOG.md` (if it lists pack manifest changes).
- new: `scripts/acceptance.d/cloudspec-simulate.sh`, sourced by `scripts/acceptance.sh` after its
  wave rows like the other `acceptance.d` files (`scripts/acceptance.sh:68-70`).
- Shared with other planned stories: `packs/cloudspec.yaml` with story:cloudspec-runs (`depends_on`:
  it first, this story after); `scripts/acceptance.sh` with story:extends-inherits and
  story:focus-pages (both cite it; this story only changes one row); `kpis` words with
  story:search-score (this story depends on it).

## Not in this story

- `metaspec search` of cloudspec, and the `check` batch (story:cloudspec-runs).
- `per_journey`, and any use of the simulation other than `score --simulate` and
  `search --simulate`.
- Any change in cloudspec, or to the adapter code of metaspec.

## Acceptance

`task acceptance -- ... <cloudspec>` exits 0, with the rows of `scripts/acceptance.d/cloudspec-simulate.sh`
run on a checkout of cloudspec main 90e1939 or later and on the first valid architecture of
`cloud_architectures architectures/*.yaml` (the list `scripts/acceptance.sh:204-211` already makes):

1. `cloudspec simulate:level`: `metaspec pack check` of the manifest reports level L4.
2. `cloudspec simulate:score`: `metaspec score cloudspec --pack <cloudspec-dir> <architecture>
   --simulate --json` is valid against `schema/metaspec-score-1.schema.json`, and each of the six
   KPI ids has a numeric value with `source` `simulate` equal to the same id in
   `cloudspec simulate --load <profile> --seed 1 <architecture>`.
3. `cloudspec simulate:directions`: the direction of `cloud.throughput_rps` in that score is
   `higher`, and that of each of the five others is `lower`.
4. `cloudspec simulate:batch`: with two valid architectures and one broken one (the first of `architectures/broken/`, which cloudspec refuses),
   `cloudspec simulate ... --batch` of the three has 3 entries in argument order, entries 1 and 2
   byte-equal to the single runs and entry 3 an `error`; and the manifest's `batch.limit` is 144.
5. `cloudspec simulate:search-same`: `metaspec search ... --simulate` of the architecture writes
   a `metaspec-result/1`, valid against its schema, equal with and without the `batch` entry apart
   from `runs` (story:search-batch-scoring).

The rows run outside `task check` (they need the checkout), as the other real-checkout rows do.
