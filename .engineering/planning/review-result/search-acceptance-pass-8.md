---
format: aep.planning-md/3
id: review-result:search-acceptance-pass-8
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 8, epic:shared-search
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

- story:search-result-best — its acceptance still has "the stop reason `no_better_neighbour` in words" as work this story does. Since pass 7 W4, the schema has all four reasons from the start. story:search-result-page already draws every `stop.reason` in words, so that clause holds before this story and cannot tell its work apart. Drop the reason from Scope and acceptance, or name what this story adds beyond result-page's drawing — `.engineering/planning/story/search-result-best.md:110` (also :51 and :72)
- story:search-design — the acceptance dates the old fragments "at the time of writing on lines 242, 359 and 443 of the file", but in the integration worktree they are on lines 244 (L4), 361 (verbs section) and 445 (Operations). The merge of story:cloudspec-manifest moved them by 2. Drop the line numbers or correct them, and put the three old fragments in the order the prose names them (verbs section, L4, Operations) — `.engineering/planning/story/search-design.md:117`

What I read: all 14 drafts (`aep plan artifact show` of each, whole bodies), pass 7's brief, and the integration worktree's `design/metaspec.md`.

Pass 7 items:
- **W2:** holds. The design acceptance has 9 greps. Run on the worktree, the old greps print `1 1 1` and the new ones `0 0 0 0 0 0`, so the printed "before" line `1 1 1 0 0 0 0 0 0` is right.
- **W3:** holds. hill-climb and beam point to `stop_reason` and "Decided: shared search", and each acceptance has the same grep.
- **W4 in the two target stories:** holds in result-format and dominance. The finding above is a stale spot elsewhere.
- **W5:** holds. I checked the `best` claims of hill-climb cases 1, 2 and 4, beam cases 1 to 3, dominance, and result-best's two fixtures against the rule.
- **W6:** holds. hill-climb says "none of the four cases" and says why: 11, 11, 12 and 9 points are left unscored.

Numbers checked with a probe in `<scratch>/p8` (SplitMix64, shuffle, simulated stop and range arithmetic):
- **Streams:** the seed 1 and seed 2 outputs match the text. Seed 7 is not quoted.
- **Shuffle picks:**
  - hill-climb cases 1, 2 and 4 are right, including `r4 mod 4 = 0`, `r5 mod 3 = 1`, `r6 mod 2 = 1` and the result `[9, 6, 4, 1]`.
  - beam case 1 gives `[p8, p2, p5]` and beam case 3 gives `[p4, p1]` and `[p8, p5]`.
  - enumerate case 2 gives `[p2, p1]`.
  - seed 1 keeps p2 and p5, as beam case 2 says.
- **Counts:** I re-derived scored and unscored counts, `runs`, variant counts and every `stop_reason` row. The beam variant count is 2+3+4+2+2+2 = 15. The batch case has 12 points, 3 batches and 4 runs.

What I could not establish:
- The grep in hill-climb and beam prints 1 before and after, so it shows only that no verbatim copy of the line appears. It does not show that the story wrote its own paragraph, and it cannot see a reordered list of the four reasons. The same holds for the paragraphs of gate, dominance, orgspec and result-best, which have no observation at all. It is pre-existing and follows W3 as ordered, so I did not count it.
- The epic body says "search stopped (budget, deadline, no better neighbour)", which lists three reasons. The epic is not in the drafts and the point is out of my lane.

```findings
[
{"file":".engineering/planning/story/search-result-best.md","line":110,"category":"acceptance","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the acceptance clause 'the stop reason `no_better_neighbour` in words' is already true after story:search-result-page, which draws every stop.reason in words and whose schema lists all four reasons from the start since W4, so it cannot tell this story's work from the world before it"},
{"file":".engineering/planning/story/search-design.md","line":117,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"undecided","message":"the acceptance dates the old fragments 'on lines 242, 359 and 443 of the file' but in the integration worktree they are on lines 244 (L4), 361 (verbs section) and 445 (Operations), and the prose lists them in a different order than the greps"}
]
```
