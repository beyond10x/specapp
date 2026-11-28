---
format: aep.planning-md/3
id: review-result:search-acceptance-pass-1
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 1, epic:shared-search
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

story:search-design — the acceptance says the script exits 0 "naming that ... L4 and 'What metaspec does not do' state the decision", but the script's three checks never read L4, so that clause cannot be observed — aep plan artifact show story:search-design, "## Acceptance" (<private path>), against "## Scope" (same file:66-68)
story:search-enumerate — the acceptance asks for "one variant for each point of the space" and also for stop reasons `budget` and `deadline`, and a stopped search has fewer variants than points, so the sentence contradicts itself — aep plan artifact show story:search-enumerate, "## Acceptance" (same file:496-499)
story:search-enumerate — the constraints bound only "a 1 second deadline" (one tool run), and name no `--time-limit` for the case that must end with `stop.reason: deadline`, whose default is 300 s against the 60 s test cap — aep plan artifact show story:search-enumerate, "## Constraints" (same file:488-491) and "## Scope" `--time-limit` (same file:437)
story:search-batch-scoring — the acceptance claims `runs` is "candidates divided by 4, rounded up, plus the baseline", but the only document it names, `harbors/inlet.yaml`, has one marked place with 3 values, so a batch of 4 holds both candidates and the check proves no grouping, no `limit` and no rounding. The acceptance should name a document or a limit that gives more than one batch — aep plan artifact show story:search-batch-scoring, "## Acceptance" (same file:395-398), against `fixtures/packs/harbor/harbors/inlet.yaml:7` (`vary: [2m, 3m, 4m]`) and `fixtures/packs/orchard/orchards/hillside.yaml:9` (`vary: [1, 2, 3]`)
story:search-orgspec — the acceptance reads the same before and after the work, because `metaspec score orgspec` and `search` already run and exit 0 on the unchanged manifest with direction `none`, and no clause checks that `kpis.directions` or `check.kpis` was added. The existing `answers` tests also pass before the story — aep plan artifact show story:search-orgspec, "## Acceptance" ("`scripts/acceptance.d/search-orgspec.sh <orgspec-dir>` exits 0 after ...")
story:search-score — the acceptance joins two independent outcomes with "and": a real-checkout row outside `task check` that compares against `agenticspec sim check --json`, and a stub-based `cargo nextest` run. One can pass while the other fails — aep plan artifact show story:search-score, "## Acceptance" (same file:292-298)
story:search-result-page — the acceptance joins `docs build ... --check` against the committed site and `task check:web` vitest cases. These are two independent outcomes — aep plan artifact show story:search-result-page, "## Acceptance"

What I read: 13 artifacts, all opened in full. These were the 9 `story:search-*` drafts, `epic:shared-search`, `architecture-decision-record:shared-search` and the 3 decision-blockers. I used `aep plan artifact list --kind story --status draft`, `aep plan artifact show <id>` for each id, `aep plan artifact lifecycle story`, `aep plan artifact lifecycle decision-blocker` and `aep plan artifact kinds`. I checked the fixtures, `.config/nextest.toml` (60 s per test, 120 s global), `Taskfile.yml`, `docs build --check` and `packs/agenticspec.yaml` in the worktree.

What I could not establish:
- Whether `agenticspec sim check --json` runs as described. The manifest declares it, but no agenticspec checkout is readable here.
- Whether any orgspec design has a numeric field to mark. There is no orgspec checkout.
- The three decision-blockers carry no clearing condition of their own. The open → cleared move has no observable test, only options and a default. I treated that as a blocker's nature, not a finding.
- Out of my lane, and they do not set the verdict:
  - The epic's hill climbing and beam search are covered by no story. They are deferred behind `decision-blocker:kpi-order`. That is scope.
  - `decision-blocker:search-edits` says "Under A nothing is stopped" yet is listed as blocking the epic. That is design.
  - `--deadline` (one tool run) and `stop.reason: deadline` (the time limit) share a name. That is design.
  - The shared files `main.rs`, `search/mod.rs` and `fixtures/expected/site` are for parallel-safety.

```findings
- file: "aep plan artifact show story:search-design"
  line: 97
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance says the script exits 0 naming that L4 and 'What metaspec does not do' state the decision, but the script's three checks (old sentences, schema entries, a mention of the search) never read L4, so that clause cannot be observed"
- file: "aep plan artifact show story:search-enumerate"
  line: 496
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance asks for one variant for each point of the space and also for stop reasons budget and deadline, and a stopped search has fewer variants than points, so the sentence contradicts itself"
- file: "aep plan artifact show story:search-enumerate"
  line: 488
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the constraints bound only a 1 second per-run deadline and name no --time-limit for the case that must end with stop.reason deadline, whose default of 300 seconds exceeds the 60 s test cap"
- file: "aep plan artifact show story:search-batch-scoring"
  line: 395
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance claims runs is candidates divided by 4 rounded up plus the baseline, but harbors/inlet.yaml has one marked place of 3 values (2 candidates), so one batch holds them all and the check proves no grouping, limit or rounding; it must name a document or limit that makes more than one batch"
- file: "aep plan artifact show story:search-orgspec"
  line: 0
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance reads the same before and after the work: score and search of orgspec already exit 0 on the unchanged manifest with direction none, and no clause observes the added kpis.directions or check.kpis"
- file: "aep plan artifact show story:search-score"
  line: 292
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance joins two independent outcomes with 'and': the real-checkout acceptance row compared with agenticspec sim check --json, and the stub-based cargo nextest run"
- file: "aep plan artifact show story:search-result-page"
  line: 0
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance joins two independent outcomes with 'and': docs build --check against the committed site, and task check:web passing the ResultSections vitest cases"
```
