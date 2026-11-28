---
format: aep.planning-md/3
id: review-result:search-scope-pass-1
kind: review-result
status: active
title: aep:plan-critic-scope, pass 1, epic:shared-search
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

- epic:shared-search — "A page of a search result in the site and the studio." is promised, but story:search-result-page drops the studio half ("the page of the studio is not part of this story") and no blocker parks it. story:search-design says it stays open "with its blocker" and names none. `aep plan artifact list --kind dependency-blocker` shows only the two cloudspec blockers, and `--kind decision-blocker` has gate-threshold, kpi-order and search-edits. File a blocker on the studio, which is not built (waves 5 and 6). story:search-result-page would take it. — .engineering/planning/epic/shared-search.md:37
- story:search-candidate-space — the new verb `metaspec space` and the new format `metaspec-space/1` (`schema/metaspec-space-1.schema.json`) trace to no sentence in the epic. The epic promises a candidate space, not a way to list it. Drop them and show the places through the test, or add them to the epic. — .engineering/planning/story/search-candidate-space.md:53
- story:search-simulate-adapter — the standalone verb `metaspec simulate` and a `verb: simulate` result trace to no sentence in the epic. The epic promises only KPI values "from `check` or `simulate`" for scoring a candidate. Narrow this to the adapter and `score --simulate`. — .engineering/planning/story/search-simulate-adapter.md:43
- story:search-result-format — the command `metaspec result check <FILE>` traces to no sentence in the epic. The epic asks for the two formats and a writer, not a validator command. — .engineering/planning/story/search-result-format.md:47
- story:search-design — `scripts/check-design-search.sh` and a `check:design` gate trace to no sentence in the epic, which promises the text of `design/metaspec.md` only. The script's rule that every `schema/metaspec-*.schema.json` needs a "Neutral formats" entry also reaches the schemas of the other epics. — .engineering/planning/story/search-design.md:68

**What I read.** 13 artifacts: epic:shared-search, architecture-decision-record:shared-search, the 9 `story:search-*` drafts and the 3 decision-blockers. I ran `aep plan artifact list --kind story --status draft`, `show` on each, `graph`, `kinds`, and `list` for both blocker kinds. I took 11 promises from the epic and traced 10 to a story or a blocker.

| Promise | Claimed by |
|---|---|
| Result format | `story:search-result-format` |
| Candidate space: marked values | `story:search-candidate-space` |
| Candidate space: generic edits and pack edit verbs | parked by `decision-blocker:search-edits` |
| Score: validity and findings | `story:search-score` |
| Score: KPI values from `simulate` | `story:search-simulate-adapter` |
| Score: order and gate | parked by `decision-blocker:kpi-order` and `decision-blocker:gate-threshold` |
| Strategies: enumeration | `story:search-enumerate` |
| Strategies: hill climbing and beam search | parked by `decision-blocker:kpi-order` |
| Budget and deadline | `story:search-enumerate` |
| Batch adapter | `story:search-batch-scoring` |
| Design text | `story:search-design` |
| Page in the site and the studio | the site half is claimed by `story:search-result-page`; the studio half is untraced (first finding) |

**What I could not establish.**
- Consumers: no acceptance runs `metaspec search` on agenticspec. `story:search-score` scores it, but agenticspec has no `simulate` verb, which `story:search-simulate-adapter` names as a pack gap. I treated the epic's Consumers section as context, not an outcome.
- `story:cloudspec-runs` is outside the set and its text is stale. It says metaspec's runs are "not planned yet as stories" and expects a "best variant". That is an item for the decomposer or `plan-critic-design`, and it does not set my verdict.
- `decision-blocker:search-edits` has default A, under which "nothing is stopped". The epic's generic-edits and pack-edit-verb promises are then parked by a default rather than a hold. I counted that as named, not a gap.
- `metaspec score` is the epic's "score" exposed on its own, so I did not flag it.

```findings
- file: .engineering/planning/epic/shared-search.md
  line: 37
  category: scope
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "\"A page of a search result in the site and the studio\" is promised; story:search-result-page excludes the studio page and no blocker parks it (story:search-design says it stays open 'with its blocker' and names none; the store has only the two cloudspec dependency-blockers); file a blocker on the unbuilt studio, which story:search-result-page would take"
- file: .engineering/planning/story/search-candidate-space.md
  line: 53
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the new verb `metaspec space` and format `metaspec-space/1` trace to no sentence of epic:shared-search, which promises a candidate space and not a command or format that lists it"
- file: .engineering/planning/story/search-simulate-adapter.md
  line: 43
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the standalone verb `metaspec simulate` and a `verb: simulate` result trace to no sentence of epic:shared-search, which asks only for KPI values 'from check or simulate' to score a candidate"
- file: .engineering/planning/story/search-result-format.md
  line: 47
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the command `metaspec result check <FILE>` traces to no sentence of epic:shared-search, which asks for the two formats and a writer, not a validator verb"
- file: .engineering/planning/story/search-design.md
  line: 68
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "scripts/check-design-search.sh and a check:design gate trace to no sentence of epic:shared-search, which promises the text of design/metaspec.md only; its every-schema rule also reaches the schemas of other epics"
```
