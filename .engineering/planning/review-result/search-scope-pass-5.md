---
format: aep.planning-md/3
id: review-result:search-scope-pass-5
kind: review-result
status: active
title: aep:plan-critic-scope, pass 5, epic:shared-search
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

**What you read.** I read the epic, the ADR, the 3 cleared blockers, `dependency-blocker:metaspec-studio`, `story:cloudspec-runs` and the 14 `story:search-*` drafts, whole, with `aep plan artifact show`. I also read `aep plan artifact graph` and the pass-4 brief. I extracted 12 promises from the epic and ADR, and traced all 12 to an item or to a named exclusion or blocker:
1. The command on any trusted pack with a `check` adapter: `search-enumerate`.
2. `metaspec-result/1` with its edits and findings: `search-result-format`.
3. The stop reasons (budget, deadline, no better neighbour): `search-enumerate` and `search-dominance`.
4. The candidate space: `search-candidate-space`. Pack edit verbs are deferred by the answer of `decision-blocker:search-edits`, and `search-design` names them as open.
5. The score from `check` or `simulate`: `search-score` and `search-simulate-adapter`.
6. The direction order and the gate, with a failing variant never better than a valid one: `search-dominance` and `search-gate`.
7. The three strategies, seeded: `search-enumerate`, `search-hill-climb` and `search-beam`.
8. Budget and deadline, with a killed run shown as such: `search-enumerate`. The batch story covers a killed batch.
9. The batch adapter: `search-batch-scoring`.
10. The page in the site: `search-result-page` and `search-result-best`. The studio half is held by `dependency-blocker:metaspec-studio`, which the page story names.
11. The `design/metaspec.md` text: `search-design`.
12. Consumers: `search-score` for agenticspec, `search-orgspec` for orgspec, and `story:cloudspec-runs` for cloudspec, which is outside the set but already claims it.

Of the epic's three exclusions, the two I could check hold. There is no rule and no simulation in metaspec, and no search over ESS blocks. A pack's own `search` verb is only mentioned as possible, and nothing builds it.

**Pass-4 decisions on my lane.** The S4 label is in place: `story:search-dominance` calls the order among infeasible candidates "the coordinator completion under the accepted epic", as `story:search-gate` does for its missing-value rule. The other scope-relevant decisions are also present:
- A12, A13 and A14 (the added cases).
- T2 (the beam stops on a round with no new member).
- G6 (`search::order::front`, with the beam stating no filter of its own).
- P6 and P7 (the draft co-writers named).

Nothing in the set reaches beyond the epic, ADR or cleared blockers, and no outcome is claimed twice. The strategy stories write `strategy`, `beam_width` and the stop reason without changing a schema, and `search-dominance` owns the schema change.

**What I could not establish, all outside my lane.**
- Acceptance: I did not hand-check the grid derivations in `search-hill-climb` and `search-beam`.
- Design: `story:search-hill-climb` Scope says "`exhausted` is not a case of this story". `story:search-enumerate` and `story:search-beam` give `exhausted` whenever every point is scored. A small space scored in one step reaches that case, so the three texts disagree.
- Parallel safety: for G7, `story:search-gate` Shared files name `search-simulate-adapter` and `search-batch-scoring` as writing `run.rs` "before or after it (the planner orders them)". The brief asked for the stories before it. `story:search-gate` has no edge to either story.

```findings
[]
```
