---
format: aep.planning-md/3
id: review-result:search-design-pass-4
kind: review-result
status: active
title: aep:plan-critic-design, pass 4, epic:shared-search
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

Findings:

story:search-hill-climb — Scope says `--simulate` "is" a flag of `story:search-enumerate`, but enumerate lists it under "Not in this story" and `story:search-simulate-adapter` creates it, so the body should name simulate-adapter as the owner; only the path through `story:search-design` orders it today — .engineering/planning/story/search-hill-climb.md:60, .engineering/planning/story/search-enumerate.md:99

story:search-beam — Scope has the same wrong owner for `--simulate`, so the body should name `story:search-simulate-adapter` as the owner of `--simulate` and of the batch inside `score_points` — .engineering/planning/story/search-beam.md:52, .engineering/planning/story/search-enumerate.md:99

story:search-dominance — it names `dominates`, `better` and a `front` member that `search::run` computes, but exposes no function for the non-dominated set. `story:search-beam` restates the same filter per round, so two bodies each own half of that rule. Dominance should name a shared `search::order` function that `front` and beam both call — .engineering/planning/story/search-dominance.md:68-69, .engineering/planning/story/search-beam.md:57-59

What I read: 14 search stories in full, plus `aep plan artifact relations`, `validate` and `graph`, and your revision-3 brief. I walked 99 `depends_on` and `blocks` edges (380 graph edges in all, including to `story:cloudspec-manifest`, `story:cloudspec-runs`, the view stories and the blockers). `tsort` finds no cycle. I traced about 17 acceptance reads to a producer in the set; the producing story is a declared dependency for each, 2 of them only through `story:search-design`. `validate` reports only the five view stories without a scope, which are not search stories.

Pass 3 decisions:
- **G1 holds.** Design depends on orgspec in neither direction. Orgspec writes its own row and depends on design. Design names the six later writers, each with its edge.
- **G2 holds apart from the ownership slip above.** Both strategies name `run.rs` and `score_points`.
- **G3 holds.** Gate writes `feasible`; result-best reads it and states no predicate.
- **G4 holds.** `compare` gives verdicts only. `order` owns the ranking, and the `compare(b, a)` wording in dominance is consistent.
- **S1, S2, S3, A6 to A11 and P4/P5 hold.** The one-member front that dominates the baseline is consistent across dominance, hill-climb and result-best.

What I could not establish (out of my lane, none of it set the verdict):
- **Beam stop rule.** The beam stops as soon as a round has no candidate dominating a beam member. That is hill climbing's rule, so beam cannot "leave a local top" as its Context promises (search-beam.md:41-42, 60-63). This is a scope or acceptance question.
- **Acceptance coverage.** The hill-climb and beam acceptances never exercise `--simulate` or a batch. The result-best acceptance never draws a front that contains the `baseline` member.
- **Shared-files wording.** search-gate.md:90 says it depends on "the first three" stories and lists `story:search-enumerate` as a schema writer. Enumerate does not touch the schema.
- **Design as a join node (decided in D16, not re-opened).** Because of the shared `design/metaspec.md` file, the 12-story chain is almost a queue. Gate waits on batch-scoring, simulate-adapter and result-page through `story:search-design`, though its code needs none of them. The alternative is to split the design file's paragraphs into separate files so the edges can go.

```findings
[
{"file":".engineering/planning/story/search-hill-climb.md","line":60,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"undecided","message":"Scope says --simulate is a flag of story:search-enumerate, which lists it under Not in this story; story:search-simulate-adapter owns it and the order holds only through story:search-design, so the body should name simulate-adapter as the owner"},
{"file":".engineering/planning/story/search-beam.md","line":52,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"undecided","message":"Scope says --simulate is a flag of story:search-enumerate, which lists it under Not in this story; story:search-simulate-adapter owns it and the batch inside score_points, so the body should name simulate-adapter as the owner"},
{"file":".engineering/planning/story/search-dominance.md","line":68,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"front is a result member computed in search::run and no set function is exposed, while story:search-beam restates the same non-dominated filter per round (search-beam.md:57-59); dominance should name a search::order function that front and beam both call"}
]
```
