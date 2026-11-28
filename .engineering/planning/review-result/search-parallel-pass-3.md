---
format: aep.planning-md/3
id: review-result:search-parallel-pass-3
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, pass 3, epic:shared-search
relations:
- reviews: epic:shared-search
- reviews: story:search-batch-scoring
- reviews: story:search-beam
- reviews: story:search-candidate-space
- reviews: story:search-design
- reviews: story:search-dominance
- reviews: story:search-enumerate
- reviews: story:search-gate
- reviews: story:search-hill-climb
- reviews: story:search-orgspec
- reviews: story:search-result-best
- reviews: story:search-result-format
- reviews: story:search-result-page
- reviews: story:search-score
- reviews: story:search-simulate-adapter
revision: 1
---
needs-revision

Pass 2's parallel-safety decisions P1, P2 and P3 all hold. Three new findings come from the five new stories.

- P1: `story:search-score` names `fixtures/manifests/harbor.yaml` and `orchard.yaml` with draft `story:kpi-quality-values` and `story:spec-quality-drift`, with no edge to drafts (`.engineering/planning/story/search-score.md`, Shared files).
- P2: `story:search-result-page` puts its case in `crates/metaspec/tests/search_page/mod.rs` with one `mod` line in `tests/main.rs`. It names `main.rs` and `Taskfile.yml` with `story:spec-quality-page` and `story:spec-quality-self` (Surfaces and Shared files). `crates/metaspec/tests/main.rs` has no `mod` line today, so the first one is added there.
- P3: `story:search-simulate-adapter` states that the `--simulate` flags live in the Args structs of `score.rs` and `search.rs` and that `main.rs` is not changed.

## Findings

- story:search-gate — it re-records `fixtures/expected/search/<pack>.json`, which the committed site is built from, and says "the story that merges later regenerates `fixtures/expected/site`". By its own edges (gate to design to result-page) it is that later story, yet `fixtures/expected/site` is in neither its Surfaces nor its scope, so the golden change is an unnamed shared file (cited). Remedies: list the site as a surface and name result-page, result-best and the view stories as co-owners; or split the re-record from the golden regeneration.
- story:search-dominance — the same defect. It re-records the same results, defers regeneration to "the story that merges later", and merges after result-page through design, gate and result-format. `fixtures/expected/site` is not in its Surfaces or scope (cited). The remedies are the same.
- story:search-result-best — it adds a paragraph to `design/metaspec.md` under "Docs", while `story:search-hill-climb` and `story:search-beam` each add one under "Decided: shared search". No edge orders result-best against either, and no body names the others on this file (cited, all three bodies). That is a collision on an existing file, which the planner's wave order currently separates only because it reads the shared scope entry. Remedies: add an ordering edge that records `design/metaspec.md` as its reason; or name hill-climb and beam in Shared files with the two sections as the separation.

## What you read

- All 14 `story:search-*` bodies, via `aep plan artifact show` for each.
- `aep plan artifact waves --kind story` and `aep plan artifact graph` (collisions and edges).
- `aep plan artifact list --kind story` for statuses.
- `aep plan artifact validate`, which prints "valid".
- The pass-2 brief.
- The tree files the bodies cite: `crates/metaspec/src/main.rs`, `crates/metaspec/tests/main.rs`, `crates/metaspec-docs/src/documents.rs`, `Taskfile.yml`, `fixtures/manifests/harbor.yaml`, `fixtures/packs/harbor/tool/harbor-tool.sh`, and `grep -rn vary` over `fixtures/packs`.

## What you could not establish

- Surface counts: 14 items cited, 0 inferred, 0 unplaced. Every body names paths; the store marks many of them "inferred" only because the files do not exist yet.
- Acceptance reads traced to a producer in the set: about 42, and a `depends_on` edge records all 42. The tally is from my reading of each Context and Acceptance section.
- Other-epic collisions, for the coordinator:
  - The pack manifest and schema files, and `adapter.rs`, collide with drafts `story:kpi-quality-values`, `story:spec-quality-conformance` and `story:spec-quality-drift`. These files are named in Shared files, but the drafts are not.
  - `packs/orgspec.yaml` collides with `story:kpi-quality-values` and `story:spec-quality-conformance`. `story:search-orgspec` says "no other story of this epic", which is literally true.
  - `Taskfile.yml` collides with `story:ux-lane-budget`, which is implemented.
- Not a finding: `story:search-gate`'s sentence "the simulate story is ordered by the planner" is stale. The gate to design to simulate-adapter edges already order them.
- Not a finding: `CHANGELOG.md` is in all 14 scopes. Result-page, orgspec and simulate-adapter are unordered against each other on it, and so are result-best, hill-climb and beam. I treated it as an append-only "Unreleased" list.
- Out of my lane, for the acceptance and design critics:
  - `crates/metaspec-docs/src/documents.rs:435-441` draws the "Run" section only when the manifest declares a `simulate` or `search` verb. The fixture manifests declare neither, and no search story edits them to add one. Result-page does not say how the fixture site gets a Run section. If the answer is to declare `simulate` in the fixtures, result-page needs an edge to simulate-adapter.
  - `story:search-result-best`'s Scope draws `strategy`, `beam_width` and `no_better_neighbour`, but its tests read only the recorded `fixtures/expected/search/<pack>.json`. The strategy results live in the `climb/` and `beam/` subdirectories, and it has no edge to hill-climb or beam.
  - `story:search-result-page` wants "a failed variant" in the recorded enumerate results, and the stubs may never produce one.

```findings
[
{"file":".engineering/planning/story/search-gate.md","line":85,"category":"parallel-safety","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:search-gate re-records fixtures/expected/search/<pack>.json, which the committed site is built from, and defers regeneration to 'the story that merges later', which by its own edges (search-design, search-result-page) is this story, yet fixtures/expected/site is in neither its Surfaces nor its scope"},
{"file":".engineering/planning/story/search-dominance.md","line":101,"category":"parallel-safety","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:search-dominance re-records fixtures/expected/search/<pack>.json read by the site golden and defers regeneration to 'the story that merges later', which is this story after story:search-result-page, yet fixtures/expected/site is in neither its Surfaces nor its scope"},
{"file":".engineering/planning/story/search-result-best.md","line":75,"category":"parallel-safety","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:search-result-best adds a paragraph to design/metaspec.md (under 'Docs') that story:search-hill-climb and story:search-beam also write to (under 'Decided: shared search'); no edge orders result-best against either and no body names them, so it needs an ordering edge recording the file or a Shared files entry naming both"}
]
```
