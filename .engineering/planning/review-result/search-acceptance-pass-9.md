---
format: aep.planning-md/3
id: review-result:search-acceptance-pass-9
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 9, epic:shared-search
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

I read all five stories that Revision 8 touched, in full: result-page, result-best, result-format, enumerate and design. I grepped the other nine of the 14 for stop-reason text and found nothing new. I did not read those nine in full, because the earlier passes covered them and nothing in them changed. X3 and X4 hold. X1 and X2 each leave a defect.

Findings:
- `story:search-result-page` — Scope says the vitest cases are "(not the acceptance)", but its second acceptance paragraph is a vitest command (`result-sections.test.tsx -t "stop reason"`). X1 made it the acceptance without removing that line, so a reader can't tell whether it is a gate. The same case is also listed in the scope's own vitest bullet ("the stop reason"). Citation: `story:search-result-page` show line 70 against lines 128-131.
- `story:search-enumerate` — the acceptance header still reads "three cases (and a unit test of the stop function, the last bullet)". X2 added a second unit test (StopReason serialisation) as the last bullet, so "the last bullet" is now the wrong test and the count is stale. Citation: `story:search-enumerate` show line 141 against lines 152-157.
- `story:search-result-best` — the summary still says "shows strategy, beam width and the stop reason". X1 moved the stop reason to `story:search-result-page` and removed it from Scope and acceptance. The summary now promises something no acceptance checks. Citation: `story:search-result-best` show line 5. The wording is the drafter's call; the fix is to delete "and the stop reason".

Held:
- **X1:** `story:search-result-page` names all four wordings and has one test case per reason. Each case renders a hand-written `stop-<reason>.json` and checks exactly one sentence, with N equal to `stop.evaluated`. `story:search-result-best` has no stop reason in its Scope or acceptance.
- **X2:** `story:search-result-format` now says the schema owns the four strings. `story:search-enumerate` has a serialisation unit test that compares the four variants and the whole schema enum, so a fifth entry on either side fails it.
- **X3:** lines 51, 72 and 110 of `story:search-result-best` carry no stop-reason duty.
- **X4:** the `story:search-design` acceptance has no line numbers. Its old fragments run in grep order: L4, verbs, Operations. Its new fragments run in grep order: verbs, L4, Operations, "does not do". I ran the nine greps on `design/metaspec.md`. They print `1 1 1 0 0 0 0 0 0` today, and the expected after-state `0 0 0 1 1 1 1 1 1` has nine entries.

Not established: whether the hand-written `stop-*.json` files are schema-valid. Nothing in either acceptance validates them, since the cargo case reads only `harbor-failed.json`. That is a small gap, not a blocker.

Out of my lane: none.

```findings
[
{"file":"story:search-result-page","line":70,"category":"acceptance","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"Scope says 'The vitest cases of this story (not the acceptance)' but the second acceptance paragraph (line 128) is a vitest command; revision 8 made a vitest case the acceptance without removing the exclusion, so it is unclear whether it gates the story, and the same 'stop reason' case is also listed in the scope bullet"},
{"file":"story:search-enumerate","line":141,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the acceptance header says 'three cases (and a unit test of the stop function, the last bullet)' but the last bullet is now the StopReason serialisation test (line 155), so the count and the pointer are stale"},
{"file":"story:search-result-best","line":5,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"summary still says 'shows strategy, beam width and the stop reason' although revision 8 moved the stop reason to story:search-result-page and no acceptance of this story checks it"}
]
```
