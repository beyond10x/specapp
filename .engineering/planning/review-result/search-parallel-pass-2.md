---
format: aep.planning-md/3
id: review-result:search-parallel-pass-2
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, pass 2, epic:shared-search
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

All six pass-1 parallel-safety items (15-20) hold. Three findings remain, all warnings. Two are shared files the stories list but don't name as shared, one is an unlisted file.

- **Items 15-20 hold:**
  - 15: search-score has `depends_on story:cloudspec-manifest`. Score, simulate-adapter and batch-scoring each name cloudspec-manifest and the five view stories in "Shared files".
  - 16: simulate-adapter depends on enumerate, and its "Shared files" names `search/mod.rs`, `search/run.rs` and `crates/metaspec/src/search.rs`.
  - 17: candidate-space names `spec/domains/documents.yaml`.
  - 18: result-page names `story:view-selectors` and `story:collapsed-regions`.
  - 19: search-design has `depends_on story:cloudspec-manifest`.
  - 20: every one of the 9 has a recorded scope that matches its Surfaces section.
- **Item 1 holds:** `dependency-blocker:metaspec-studio` exists and blocks the epic. Result-page, enumerate and design name it.
- **Edge shape is sound:**
  - The search set runs as one chain: score, result-format, candidate-space, enumerate, simulate-adapter, batch-scoring, design.
  - orgspec and result-page branch off after enumerate.
  - Every file shared inside the set is ordered by an edge.

Findings:

story:search-score — it edits `fixtures/manifests/harbor.yaml` and `fixtures/manifests/orchard.yaml`, which draft `story:kpi-quality-values` (harbor, new class `broken`) and `story:spec-quality-drift` (orchard, declares `regenerate`) also edit, and its "Shared files" lists neither (cited, all three bodies). Remedy: name both files and both stories there, then either add an ordering edge that records the file as its reason, or split the fixture changes so the stories no longer share them. — `.engineering/planning/story/search-score.md:118`, `.engineering/planning/story/kpi-quality-values.md:120`, `.engineering/planning/story/spec-quality-drift.md:73`

story:search-result-page — it adds a case to `crates/metaspec/tests/main.rs`, which draft `story:spec-quality-page` also changes, and its "Shared files" does not list that file (cited, both bodies). `Taskfile.yml` is in the same position against `story:spec-quality-self`, but that story changes it only if `quality:self` joins `task check`. Remedy: name both files and the stories there, then either add an ordering edge that records the file as its reason, or put the case in a test file of its own. — `.engineering/planning/story/search-result-page.md:93`, `.engineering/planning/story/spec-quality-page.md:61`, `.engineering/planning/story/spec-quality-self.md:90`

story:search-simulate-adapter — `--simulate` is added to `metaspec score` and `metaspec search`, whose clap arguments `story:search-score` places in `crates/metaspec/src/main.rs` (`Command::Score`) and `story:search-enumerate` places there too. Simulate-adapter's Surfaces list `score.rs` and `search.rs` but not `main.rs`, and result-page also edits `main.rs` in the same wave after enumerate (inferred: the existing `Render` variant declares its flags inline in `main.rs`, so a new flag probably goes there). Remedy: add `main.rs` to its Surfaces and "Shared files" with result-page, then either add an ordering edge that records the file as its reason, or state that the flags live in `score.rs` and `search.rs`. — `.engineering/planning/story/search-simulate-adapter.md:104`, `.engineering/planning/story/search-result-page.md:93`

Notes for the coordinator's wave order (not findings):
- **Pack and manifest files:** `crates/metaspec-core/src/manifest.rs`, `adapter.rs` and `schema/metaspec-pack-1.schema.json` are shared by score, simulate-adapter and batch-scoring with `story:spec-quality-conformance`, `story:spec-quality-drift` and `story:kpi-quality-values`. `packs/orgspec.yaml` is shared by `story:search-orgspec` with `story:spec-quality-conformance` and `story:kpi-quality-values`.
- **`design/metaspec.md`:** search-design collides with `story:contain-in-and-per`, and with the quality stories if they write it.
- **Collisions with `story:contain-in-and-per`:** this story collides with candidate-space (`graph.rs`, `metaspec-graph-1.schema.json`, `documents.yaml`) and with score, simulate-adapter and batch-scoring (`manifest.rs`, `metaspec-pack-1.schema.json`).
- **Concurrent branch after enumerate:** orgspec, simulate-adapter and result-page share only `CHANGELOG.md` and `crates/metaspec-core/tests/cases/mod.rs`. I did not file those: they are append-only lines and every story in the store lists them.

**What I read:** all 9 `story:search-*` bodies, plus the bodies of `story:view-selectors`, `kpi-quality-values`, `spec-quality-drift`, `spec-quality-page` and `spec-quality-self`. I ran `aep plan artifact show` on each, plus `waves --kind story`, `graph`, `list --kind story`, and read-only greps of the repository.

**Counts:**
- Surfaces established: 9 items cited (bodies name paths), 0 inferred-only, 0 unplaced.
- Acceptance reads traced to a producer in the set: 14, and a `depends_on` edge records all 14.

**What I could not establish:**
- Where the clap arguments for `--simulate` land, since neither the score nor the enumerate body says; that is the third finding.
- Whether the `ResultSections` "Run" condition in `documents.rs` fires for the harbor manifest, which declares neither `simulate` nor `search`. That is the acceptance critic's lane and does not set my verdict.

```findings
- file: .engineering/planning/story/search-score.md
  line: 118
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "it edits fixtures/manifests/harbor.yaml and fixtures/manifests/orchard.yaml, which draft story:kpi-quality-values and story:spec-quality-drift also edit (cited, all three bodies), and its Shared files names neither file nor those stories; add an ordering edge recording the file as the reason, or split the fixture changes"
- file: .engineering/planning/story/search-result-page.md
  line: 93
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "it adds a case to crates/metaspec/tests/main.rs, which draft story:spec-quality-page also changes (cited, both bodies), and Taskfile.yml, which story:spec-quality-self changes only if quality:self joins task check; its Shared files names neither; add an ordering edge recording the file as the reason, or move the case to its own test file"
- file: .engineering/planning/story/search-simulate-adapter.md
  line: 104
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "inferred surface: --simulate on score and search is a clap flag that story:search-score and story:search-enumerate place in crates/metaspec/src/main.rs, but this story's Surfaces omit main.rs, where story:search-result-page also edits in the same wave after enumerate; add main.rs to Surfaces and Shared files with an ordering edge or a split, or state the flags live in score.rs and search.rs"
```
