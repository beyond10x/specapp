---
format: aep.planning-md/3
id: review-result:search-scope-pass-3
kind: review-result
status: active
title: aep:plan-critic-scope, pass 3, epic:shared-search
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

- story:search-gate — a candidate that fails the gate when it does not answer a gated KPI the baseline answered goes beyond answer A, which tests only "its value is worse, in the KPI's direction, than the original document's"; drop the clause, or record it as a rule this story adds and put it to the operator — .engineering/planning/story/search-gate.md:53 against .engineering/planning/decision-blocker/gate-threshold.md:48
- story:search-dominance — "The baseline is not a member of the front" is a membership rule that answer A does not give ("candidates neither of which dominates the other are all kept as the front"). It lets `front` and `best` hold variants that do not improve on the original, against the epic's "finds better variants". Drop the rule, or record it as an added decision for the operator — .engineering/planning/story/search-dominance.md:64 against .engineering/planning/decision-blocker/kpi-order.md:53 and .engineering/planning/epic/shared-search.md:18

**What I read.** I read 16 artifacts: the epic, the architecture decision record, the 3 cleared blockers, `dependency-blocker:metaspec-studio` and all 14 `story:search-*` drafts in full. I ran `aep plan artifact show` on each, `aep plan artifact graph`, and `grep -n` on the store files for line citations.

**Promises.** I extracted 11 epic promises and traced 10 to a story. The studio half of the result page is the eleventh and is parked by the open blocker.
1. `search` command and `metaspec-result/1` output: enumerate, result-format
2. Candidate space (marked values): candidate-space
3. Pack's own edit verbs: not claimed (see below)
4. Score from `check` and `simulate`: score, simulate-adapter
5. Order by direction and gate: dominance, gate
6. Strategies seeded: enumerate, hill-climb, beam
7. Budget and deadline: enumerate
8. Batch adapter: batch-scoring
9. Result page in the site: result-page, result-best
10. `design/metaspec.md` decision: search-design
11. Result page in the studio: parked by `dependency-blocker:metaspec-studio`

The exclusions and the consumers also trace. agenticspec is covered by search-score, orgspec by search-orgspec, and cloudspec by `story:cloudspec-manifest` and `story:cloudspec-runs`.

**What I could not establish.**
- Promise 3: `epic:shared-search` still promises "a pack's own edit verbs where its manifest declares them". Only the cleared `decision-blocker:search-edits` answer (line 50) defers them, to "a later epic". No open blocker and no story names the deferral, and `story:search-design` lists only the studio page as open. I did not count this as a finding because the owner's answer records it. Revising the epic text, or adding one line to `story:search-design`'s "what stays open", would make the narrowing visible.
- Out of my lane: whether beam search's rule for choosing the next beam can move away from the baseline (design), and whether the acceptance of each of the five new stories is checkable (acceptance).

```findings
[
{"file":".engineering/planning/story/search-gate.md","line":53,"category":"scope","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"gate_failed also lists gated KPIs the baseline answered and the candidate does not, a rule decision-blocker:gate-threshold option A does not state (it tests only a value worse than the original's); remove the clause or record it as an added decision for the operator"},
{"file":".engineering/planning/story/search-dominance.md","line":64,"category":"scope","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the baseline is excluded from the front, a membership rule decision-blocker:kpi-order option A does not give, so front and best can hold variants that do not improve on the original against the epic's 'finds better variants'; remove the rule or record it as an added decision for the operator"}
]
```
