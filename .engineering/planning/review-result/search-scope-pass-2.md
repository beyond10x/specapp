---
format: aep.planning-md/3
id: review-result:search-scope-pass-2
kind: review-result
status: active
title: aep:plan-critic-scope, pass 2, epic:shared-search
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
approve

Pass-1 items 1-5 hold, and nothing is drafted beyond the epic. I also checked items 6-20 of the brief where they touch scope. They hold, and none adds anything the epic did not ask for.

Items 1-5:
- **Item 1:** `dependency-blocker:metaspec-studio` exists and `blocks` the epic. Its body claims the studio half of the page. `story:search-result-page` names it twice, in the summary and in "Not in this story". `story:search-design` names it under "What stays open".
- **Item 2:** `story:search-candidate-space` says "No command and no new format". The places show only through tests and goldens, with no `metaspec space` and no `metaspec-space/1`.
- **Item 3:** `story:search-simulate-adapter` says "No standalone `metaspec simulate` command and no result of a simulate run". Simulated values enter only through `score --simulate` and `search --simulate`.
- **Item 4:** `story:search-result-format` adds no command ("No command is added: the tests validate the recorded files against the schemas"), so there is no `metaspec result check`.
- **Item 5:** `story:search-design` has no script and no `check:design`. Its acceptance is one `grep -c` line. I ran it on the worktree's `design/metaspec.md` and it prints `3 0`, so the old sentences are present and the new ones are not yet. After the work it should print `0 4`.

Coverage of the epic. I extracted 8 promises (the Outcome and the 7 "What it covers" bullets) and checked the 3 exclusions. All 8 trace to an item.

| Promise | Claimed by |
|---|---|
| `metaspec search` command and `metaspec-result/1` | `story:search-enumerate`, `story:search-result-format` |
| Candidate space: marked values | `story:search-candidate-space` |
| Candidate space: generic edits and pack edit verbs | parked by `decision-blocker:search-edits` |
| Score: validity, findings, KPI values from `check` or `simulate`, direction | `story:search-score`, `story:search-simulate-adapter` |
| Score: gate test | parked by `decision-blocker:gate-threshold` |
| Score: the order of valid candidates | parked by `decision-blocker:kpi-order` |
| Strategies: bounded enumeration, seeded | `story:search-enumerate` |
| Strategies: hill climbing, beam search, "no better neighbour" | parked by `decision-blocker:kpi-order` |
| Budget and deadline, killed tool run shown as such | `story:search-enumerate` |
| Batch adapter | `story:search-batch-scoring` |
| Page in the site and the studio | site: `story:search-result-page`; studio: parked by `dependency-blocker:metaspec-studio` |
| `design/metaspec.md` | `story:search-design` |

The drafters name every parked promise in the story bodies (`story:search-enumerate`, `story:search-candidate-space`, `story:search-design`), so none is a silent gap. The three exclusions hold. No story changes ESS blocks, builds a rule, or builds a pack's own `search` verb.

I found no outcome claimed twice. `runs` is owned by `story:search-result-format`, written by `story:search-enumerate`, and only re-valued by `story:search-batch-scoring`. The verdicts are computed in `story:search-score` and carried by `story:search-result-format`.

The reach checks I made:
- **`--result` and the URL-state rows** trace to the epic's page promise and to `design/metaspec.md:501` ("Every scene, table and filter state has a URL").
- **The `kpis.directions` manifest property** serves "ordered by the catalogue's `direction`".
- **The `--time-limit` flag** serves "a wall-clock limit".

What I could not establish:
- **`story:search-design` scope (judgement call):** it edits more of `design/metaspec.md` than the three sections the epic names (the verbs section, L4, "What metaspec does not do"). The extra edits are Operations, Neutral formats, Gaps per pack, Build order and a new "Decided: shared search" section. I read them as the documentation of what the other stories build. I did not call them a finding because the epic's parent sentence is "says the decision".
- **Out of my lane:** whether `story:search-candidate-space` and `story:search-orgspec` use `crates/metaspec-core/tests/cases/search_score.rs` correctly is for the design and parallel-safety critics.
- **Out of my lane:** whether the acceptance of each story is checkable is for the acceptance critic.

I ran `aep plan artifact show` on all 9 stories, the 3 decision-blockers, the studio blocker, the epic and the ADR. I also ran `aep plan artifact graph` and `list`, and the grep from `story:search-design`.

```findings
[]
```
