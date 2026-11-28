---
format: aep.planning-md/3
id: review-result:search-scope-pass-4
kind: review-result
status: active
title: aep:plan-critic-scope, pass 4, epic:shared-search
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

I read the epic and ADR, then the three cleared blockers, then all 14 drafts. I extracted 15 promises from `epic:shared-search` and traced all 15 to an item. Two of the 15 are named omissions, not gaps. Nothing is uncovered. `aep plan artifact validate` prints `valid`, and `aep plan artifact graph` shows no other artifact claiming part of the epic.

**Pass 3 scope decisions**
- **S1:** story:search-gate (`.engineering/planning/story/search-gate.md:56`) keeps the missing-value rule and labels it the coordinator's completion of answer A.
- **S2:** story:search-dominance computes the front over the baseline and every feasible variant. The baseline stays a member as `baseline`, and `best` is set only for a lone variant that dominates it.
- **S3:** story:search-design "What stays open" defers a pack's own edit verbs, citing `decision-blocker:search-edits` and an `apply` adapter.

**Promises that are named omissions, not gaps**
- "a pack's own edit verbs where its manifest declares them": deferred by the answer of `decision-blocker:search-edits` and recorded in story:search-design.
- "A page … in the site and the studio": the site half is in story:search-result-page and story:search-result-best. The studio half is stated as waiting on `dependency-blocker:metaspec-studio`.

**Not reach beyond the epic:** these all trace to the epic, the ADR or a cleared answer.
- `metaspec score`
- `kpis.directions`
- `--time-limit`
- `best`, `front`, `strategy` and `beam_width`
- the "Gaps per pack" section, which already exists at `design/metaspec.md:534`

**One finding**
- story:search-dominance, line 64, defines an order among infeasible candidates: one that did not run ranks below one that ran, then fewer gated findings, then fewer failed gates.
- No source asks for this. The epic only says a failing variant is "never better than a valid one" (`epic/shared-search.md:30`), and the `kpi-order` answer covers only valid candidates.
- The story also says "no hidden tie-break" in the same section, and this order is what hill climbing moves on when the current candidate is infeasible.
- Fix: label the sentence the coordinator's completion under the accepted epic, as story:search-gate does at line 56, or remove it.

**Unease, not counted as a finding:** the epic says "agenticspec first", but its only real-checkout row (`scripts/acceptance.d/search-score.sh`) scores it. Only orgspec gets a row that runs `metaspec search`. If agenticspec's search should be shown on its checkout, story:search-enumerate would take that row.

**Out of my lane, did not set the verdict**
- story:search-hill-climb and story:search-beam say `--simulate` is "those of story:search-enumerate", but it is added by story:search-simulate-adapter. Neither depends on that story.
- story:search-hill-climb's acceptance numbers points "2 to 7", while story:search-enumerate numbers the baseline as point 0.
- story:search-result-best's Constraints say the tests read `fixtures/expected/search/<pack>.json`, but its acceptance reads `beam-best.json`.

```findings
[
{"file":"<integration tree>/.engineering/planning/story/search-dominance.md","line":64,"category":"scope","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:search-dominance's `better` ranks infeasible candidates (did not run below ran, then fewer gated findings, then fewer failed gates), which no sentence of epic:shared-search (only 'never better than a valid one', epic/shared-search.md:30), the ADR or decision-blocker:kpi-order (valid candidates only) asks for, and the story does not label it a coordinator completion as story:search-gate does at search-gate.md:56"}
]
```
