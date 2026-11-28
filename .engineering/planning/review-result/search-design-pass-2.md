---
format: aep.planning-md/3
id: review-result:search-design-pass-2
kind: review-result
status: active
title: aep:plan-critic-design, pass 2, epic:shared-search
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

Pass-1 items 11, 13 and 14 hold. Item 12 holds only without `--simulate`. I found three new defects.

- **Item 11.** `search-enumerate` creates the check stubs and `search-simulate-adapter` owns `search --simulate`. The edge `simulate-adapter` → `enumerate` is present and `enumerate` has no edge back.
- **Item 13.** The "extends this story's scope" clause is gone. `story:search-design` says a later story owns its own paragraph and gets a `depends_on` edge to `story:search-design`.
- **Item 14.**
  - `search::score::compare` lives in `story:search-score`.
  - The `verdicts` property is defined in `story:search-result-format`.
  - `story:search-result-page` reads the verdicts and says it computes nothing.
- **Item 12.** `story:search-result-format` and `story:search-enumerate` own `runs` from the start. `story:search-batch-scoring` says it only changes the value. With `--simulate` the count is wrong, which is finding 1.

The graph has no cycle. I walked 62 edges, including the edges to `story:cloudspec-manifest`, `story:cloudspec-runs`, the view stories and `epic:cloudspec-pack`. I traced about 14 acceptance reads to a producer in the set, and an edge records every one. The spine score → result-format → candidate-space → enumerate → simulate-adapter → batch-scoring is ordered by shared files, and each story's Shared files section says so. That is a trade-off, so it is not a finding.

The old design sentences still exist at `design/metaspec.md` lines 233, 350 and 434. `story:search-design`'s `grep -c` command reports 3 before the change and prints `0 4` after it.

**Findings**

1. `story:search-simulate-adapter` — Two owners of `runs`, and the meaning breaks under `--simulate`.
   - `story:search-simulate-adapter` says validity and findings "still come from `check`", so a `--simulate` search runs `check` and `simulate` for each candidate.
   - `story:search-result-format` defines `runs` as one run per candidate plus the baseline.
   - `story:search-enumerate` repeats that count.
   - `story:search-batch-scoring` sets it to "batch runs plus the baseline".
   - No body says whether `runs` counts the `simulate` runs. No body says which state a candidate gets when `check` ran and `simulate` timed out.
   - The simulate-adapter body should state both rules, and the other two bodies should be consistent with them.
   - Citations: `.engineering/planning/story/search-simulate-adapter.md:87`, `.engineering/planning/story/search-result-format.md:57`.
2. `story:search-result-page` — The page draws `best` and `front`, which no schema or story defines yet.
   - Its scope says "`best` and `front` are drawn when the result has them", and `ResultData` must follow a schema that has neither member.
   - `story:search-result-format` says the story after `decision-blocker:kpi-order` adds them.
   - That later story therefore has to edit code that this story wrote against an unspecified shape.
   - The clause should be dropped from the page story. The later story then owns the drawing, with a `depends_on` edge to `story:search-result-page`.
   - Citations: `.engineering/planning/story/search-result-page.md:69`, `.engineering/planning/story/search-result-format.md:63`.
3. `story:search-orgspec` — The handoff to `story:search-design` has no carrier.
   - `story:search-design` writes the orgspec row of "Gaps per pack" "from this story result".
   - `story:search-orgspec` changes only `packs/orgspec.yaml`, `CHANGELOG.md`, the acceptance row and the recorded answers.
   - None of those surfaces records the gap lines.
   - The orgspec body should name where the lines are recorded, such as a named section of the story, so the design story has an input to read. Alternatively it writes its own row, as `story:cloudspec-manifest` does for cloudspec.
   - Citations: `.engineering/planning/story/search-orgspec.md:45`, `.engineering/planning/story/search-design.md:66`.
4. `story:search-orgspec` — Two harnesses score recorded answers.
   - `story:search-score` scores recorded answers in `tests/cases/search_score.rs` through `scripts/replay-answers.sh`.
   - `story:search-orgspec` adds "one test function" to `tests/cases/answers.rs`, and its Surfaces parenthetical points at `search_score.rs`.
   - The body should say which file gets orgspec's cases. It should reuse the `search_score.rs` loop, so there is one harness for `<case>.score.json` goldens.
   - Citations: `.engineering/planning/story/search-orgspec.md:66`, `.engineering/planning/story/search-orgspec.md:72`.

**What I read:** 14 artifacts: the 9 `story:search-*` drafts, the four blockers, `epic:shared-search`, plus `dependency-blocker:cloudspec-architectures-dir` and `story:cloudspec-manifest` for context. I ran `aep plan artifact relations`, `graph`, `show` and `validate`, then read `design/metaspec.md`, `scripts/record-answers.sh`, `tests/cases/answers.rs` and the fixtures. `validate` reports only the five view stories without a scope, which is not my finding.

**What I could not establish:**
- Whether `search::score::compare` is called and tested in a story's acceptance. `story:search-result-format` lists `score.rs` under changed surfaces without saying what changes, and `story:search-enumerate` implies the call without naming it. This is an unease, not a finding.
- Out of my lane, for the acceptance critic:
  - `story:search-score` says its tests use stub tools, but its acceptance says "without starting a pack tool".
  - `story:search-batch-scoring`'s acceptance tests only the `check` batch.
  - `scripts/acceptance.d/search-enumerate.sh` is a surface but is absent from the `story:search-enumerate` acceptance.
- Out of my lane, for the parallel-safety critic: `CHANGELOG.md` is edited by all nine stories.

```findings
[
{"file":".engineering/planning/story/search-simulate-adapter.md","line":87,"category":"design","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"search --simulate runs check (validity, findings) and simulate (KPI values) per candidate, but runs is defined as one run per candidate plus the baseline (search-result-format.md:57, repeated by search-enumerate and batch-scoring) and no body says whether simulate runs count or which state a candidate gets when check ran and simulate timed out; the body must state both rules"},
{"file":".engineering/planning/story/search-result-page.md","line":69,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the page draws best and front, members no schema or story defines yet (search-result-format.md:63 adds them in the story after decision-blocker:kpi-order), so that later story must edit code written against an unspecified shape; drop the clause and let the later story own the drawing with a depends_on edge to this one"},
{"file":".engineering/planning/story/search-orgspec.md","line":45,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"story:search-design writes the orgspec row of Gaps per pack from this story result (search-design.md:66) but no surface of this story records the gap lines; name where they are recorded or write the row here"},
{"file":".engineering/planning/story/search-orgspec.md","line":72,"category":"design","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"recorded answers are scored by search_score.rs through replay-answers.sh in search-score but this story adds a scoring test function to answers.rs while its Surfaces parenthetical points at search_score.rs, so two harnesses own one surface; name one file and reuse the search_score.rs loop"}
]
```
