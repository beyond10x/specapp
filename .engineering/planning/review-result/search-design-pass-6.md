---
format: aep.planning-md/3
id: review-result:search-design-pass-6
kind: review-result
status: active
title: aep:plan-critic-design, pass 6, epic:shared-search
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

R1, R2, R3, R8 and R9 all hold. I found two new findings, both about stop reasons, which are the pass-5 R1 surface.

What I checked against pass 5:
- **R1 (one owner of stop reasons):**
  - The precedence block that hill-climb and beam quote is word for word the one in enumerate (`search-enumerate.md:90`).
  - `exhausted` is allowed for hill climbing.
  - Beam no longer restates budget or exhausted.
- **R2 (hill climbing moves by `better`):** hill-climb names `search::order::better(neighbour, current)` as the one move rule. It says that for feasible candidates `better` is dominance, and that an infeasible current candidate moves to any candidate ranked above it. This matches dominance's `better` (feasible beats infeasible; among infeasible, ran, then fewer gated findings, then fewer failed gates).
- **R3 (beam's next beam):**
  - Beam takes `order::front` of the round when it has a feasible candidate, cut to `--beam-width` in the seeded order.
  - With no feasible candidate it takes the first `--beam-width` by `better`, ties in the seeded order.
  - This matches dominance, where `front` is feasible-only.
- **R8 and R9:** gate's Shared files and simulate-adapter's Shared files say what the brief asked. Batch-scoring does not touch the score schema, so gate's schema bullet correctly names only result-format and simulate-adapter.
- **Derivations:** I recomputed SplitMix64 for seeds 1 and 2 and got the stream outputs quoted in the stories. All the shuffles, the hill-climb case counts and beam case 1 (15 variants, `runs` 16, `exhausted`) hold.
- **Shape:** no cycle across all 95 `depends_on` edges, checked with `tsort`. The set is a long chain, from score and enumerate through design, gate, dominance, hill-climb and beam to result-best. Every ordering edge has its shared file written beside it (`run.rs`, `manifest.rs`, `design/metaspec.md`), so it is a recorded trade-off, not a finding.

Findings:
1. **`story:search-enumerate`.** Only the text of the stop reasons has one owner. Hill-climb ("When the budget is reached the search stops here, before it moves") and beam ("When the budget is reached the search stops here") each decide the budget stop inside their own step. Enumerate names `score_points` as the one scoring path but names no equivalent for the reason. Its Surfaces line puts "the stop reasons" in `run.rs` without a function. The fix is for enumerate to name one function in `run.rs` that gives `stop.reason` from the deadline, the scored count, the budget, whether the step moved and the unscored count. The strategy stories would call it.
2. **`story:search-hill-climb` and `story:search-beam`.** Their `design/metaspec.md` paragraphs each "state ... the stop reasons" (`search-hill-climb.md:96`, `search-beam.md:87`). `story:search-design` also records "the stop reasons" in "Decided: shared search" (`search-design.md:74`). That is three writers of one text, so the R1 owner is undone in the design file. The fix is for design to write the four reasons and the precedence once, from enumerate's text. The hill-climb and beam paragraphs would then say how their strategy ends by naming the reason and quoting none.

Out of my lane, which did not set the verdict:
- Gate's acceptance depends on the stubs enumerate creates making the gated KPI worse for every candidate (`search-gate.md:191-192`). Enumerate says only that the stubs answer "in the shape of each fixture pack" (acceptance).
- Beam's no-feasible-candidate branch (R3's second half) has no acceptance case (acceptance).
- Hill-climb's title and summary still say "dominates", while the body says `better` (wording).
- Gate and the other site writers still say "the order of the waves is the planner call" for `fixtures/expected/site`. That concerns the unlinked view stories (parallel safety).

What I read: 14 `story:search-*` bodies via `aep plan artifact show`, plus `aep plan artifact relations`, `graph` and `validate` (`valid`). I walked about 95 `depends_on` edges across the whole store, including `story:cloudspec-manifest` and `story:cloudspec-runs`, and found no cycle. I traced about 25 acceptance reads to a producer in the set, and every one has a `depends_on` edge.

What I could not establish: none.

```findings
[
{"file":".engineering/planning/story/search-enumerate.md","line":112,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the stop reasons have one owner as text but no one function: Surfaces puts 'the stop reasons' in run.rs without naming one, and search-hill-climb ('the search stops here, before it moves') and search-beam ('the search stops here') each decide the budget stop in their own step; the body does not name a single run.rs function that gives stop.reason from the deadline, the scored count, the budget, whether the step moved and the unscored count, for the strategies to call"},
{"file":".engineering/planning/story/search-hill-climb.md","line":96,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"its design/metaspec.md paragraph 'states ... the stop reasons' while story:search-design records 'the stop reasons' in Decided: shared search (search-design.md:74) and search-beam does the same, so three stories write the one text that story:search-enumerate owns; the paragraph should say how a climb ends by naming the reason and quote none, design writing the four reasons and the precedence once"},
{"file":".engineering/planning/story/search-beam.md","line":87,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"its design/metaspec.md paragraph states 'its stop reasons' while story:search-design (search-design.md:74) and story:search-hill-climb (search-hill-climb.md:96) also write the stop reasons there; the paragraph should say how a beam search ends by naming the reason and quote none"}
]
```
