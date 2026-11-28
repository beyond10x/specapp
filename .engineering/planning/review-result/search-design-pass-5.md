---
format: aep.planning-md/3
id: review-result:search-design-pass-5
kind: review-result
status: active
title: aep:plan-critic-design, pass 5, epic:shared-search
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

- story:search-hill-climb — its stop rule says "`exhausted` is not a case of this story", but on a space the climb scores completely (a one-place space of 3 values) neither `no_better_neighbour` (needs unscored points left) nor `budget` applies; the owner's precedence gives `exhausted`, so the body should name `exhausted` for "every point scored" — .engineering/planning/story/search-hill-climb.md:81-83, .engineering/planning/story/search-enumerate.md:93-95
- story:search-beam — it restates `budget` as "the budget is reached with unscored points remaining" and `exhausted` as "every point is scored", so a budget reached on the last point gives `exhausted`. The stop-reason owner says "A budget that equals the number of candidates therefore gives `budget`". Take the owner's precedence verbatim — .engineering/planning/story/search-beam.md:70, .engineering/planning/story/search-enumerate.md:95
- story:search-enumerate — it defines `no_better_neighbour` as "a strategy that finds no dominating neighbour while unscored points remain", but beam writes it for "a round adds no new member" (no unscored neighbour of the beam, or no feasible candidate in the round). The owner's definition should cover both strategies' stop rules, or beam should get its own reason — .engineering/planning/story/search-enumerate.md:93-94, .engineering/planning/story/search-beam.md:68-70
- story:search-beam — the new beam is `search::order::front`, which holds feasible candidates only, so a beam from an infeasible baseline stops after round 1. Hill climbing keeps moving through `search::order::better`, which ranks infeasible candidates. The body does not say whether the beam is meant to differ, so say it, or give the round cut a rule for rounds with no feasible candidate — .engineering/planning/story/search-beam.md:68, .engineering/planning/story/search-dominance.md (Scope, `search::order::front`: "the feasible ones only")
- story:search-hill-climb — the step rule names two functions: "the first one that dominates the current candidate (`search::order::better`...)". `better` ranks infeasible candidates and does not mean dominance, so the body should pick one function and state what happens when the current candidate is infeasible — .engineering/planning/story/search-hill-climb.md:76-78
- story:search-gate — the G7 revision wrote run.rs and the score schema as "changed before or after it by `story:search-simulate-adapter` and `story:search-batch-scoring` (the planner orders them)". The graph already puts both before the gate (gate depends_on design, design depends_on simulate-adapter and batch-scoring), so the body should say "before it", by name — .engineering/planning/story/search-gate.md:91-93, `aep plan artifact graph` (story:search-gate -> story:search-design, story:search-design -> story:search-batch-scoring and story:search-simulate-adapter)

What I read:
- All 14 story:search-* drafts in full, plus epic:shared-search and brief 4.
- Commands: `aep plan artifact show` for each, `relations`, `graph`, `validate` (valid; its 5 notes are scope-less active view stories, not mine), and `waves --kind story --status draft`.
- Graph: I walked all 95 `depends_on` edges in the store, including edges outside the set (cloudspec-manifest and the quality drafts), and `tsort` finds no cycle.
- Acceptance reads: I traced about 24 reads of state another story produces. All have a depends_on edge to their producer. Two are transitive only: beam to enumerate (`rng::shuffle`) and gate to simulate-adapter and batch-scoring (through design).
- Chain: the long chain from enumerate to result-best has a written reason on each edge (shared design/metaspec.md paragraph order, `score_points` extension, real data flow), so I do not call it a defect. It is a trade-off: design joins 8 incoming edges only to order paragraphs in one file.
- Pass 4 decisions: G5, G6, G7 (partly), T1 and T2 landed. G6 holds: beam names `order::front` as its only filter. Hand-check of the T1/T2 seeds and the beam rounds matched (stream outputs, neighbours, variant counts, `runs` 15).

What I could not establish:
- None in my lane.
- Out of my lane (parallel safety or scope), not counted in the verdict:
  - The wave collisions on CHANGELOG.md, tests/cases/mod.rs, search/mod.rs and run.rs, which put the search stories in 14 waves.
  - search-simulate-adapter says "created by `story:search-candidate-space`" for run.rs and search.rs. Neither is created by that story, and mod.rs comes from search-score.
  - search-dominance adds `strategy` and `beam_width` to the result schema, but only the strategy stories write them.

```findings
[
{"file":"<integration tree>/.engineering/planning/story/search-hill-climb.md","line":81,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"its stop rule says `exhausted` is not a case of this story, but on a space the climb scores completely neither `no_better_neighbour` (needs unscored points remaining) nor `budget` applies, and the owner's precedence in story:search-enumerate gives `exhausted`; name `exhausted` for every point scored"},
{"file":"<integration tree>/.engineering/planning/story/search-beam.md","line":70,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"it restates `budget` as reached with unscored points remaining and `exhausted` as every point scored, so a budget reached on the last point gives `exhausted`, while the stop-reason owner story:search-enumerate says a budget that equals the number of candidates gives `budget`; take the owner's precedence verbatim"},
{"file":"<integration tree>/.engineering/planning/story/search-enumerate.md","line":93,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"as owner of the stop reasons it defines `no_better_neighbour` as a strategy that finds no dominating neighbour while unscored points remain, but story:search-beam writes it for a round that adds no new member (no unscored neighbour of the beam or no feasible candidate); widen the definition to cover both strategies or give beam its own reason"},
{"file":"<integration tree>/.engineering/planning/story/search-beam.md","line":68,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the new beam is search::order::front, which holds feasible candidates only, so a beam from an infeasible baseline stops after round 1 while hill climbing keeps moving through search::order::better; the body does not say whether the beam is meant to differ or what the cut does for a round with no feasible candidate"},
{"file":"<integration tree>/.engineering/planning/story/search-hill-climb.md","line":76,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"pre-existing","message":"the step rule names two functions, the first neighbour that dominates the current candidate and search::order::better, which ranks infeasible candidates and is not dominance; pick one and say what happens when the current candidate is infeasible"},
{"file":"<integration tree>/.engineering/planning/story/search-gate.md","line":93,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"Shared files says story:search-simulate-adapter and story:search-batch-scoring change run.rs 'before or after it' and 'the planner orders them', but gate depends_on design and design depends_on both, so both land before the gate; say 'before it' by name as G7 asked"}
]
```
