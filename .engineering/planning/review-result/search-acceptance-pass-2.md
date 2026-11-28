---
format: aep.planning-md/3
id: review-result:search-acceptance-pass-2
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 2, epic:shared-search
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

Items 6 to 10 of the brief hold in substance. I found three defects that still stop an honest close, and one low one.

1. story:search-result-page — the acceptance command cannot exit 0 "against the committed site" as written. `fixtures/expected/site` holds both packs, `_index` and a two-pack `index.json`. `task check:docs` builds it with one `--pack <manifest>=<dir>` per pack, after `metaspec trust` in `target/docs-state`. The acceptance builds harbor alone with `--manifest`/`--pack`, an unnamed `--out <dir>`, and no trust step. `--check` would list orchard, `_index` and `index.json` as differing or not owned, and the harbor check results would be missing. — `.engineering/planning/story/search-result-page.md:115`
2. story:search-result-page — the same sentence joins two outcomes. The exit status of `--check` only proves the generator equals the files the story itself commits. The clause "whose page … shows the variants … edits, scores, verdicts and stop reason" is not something that command observes, and the vitest cases that would observe it are not named in the acceptance. — `.engineering/planning/story/search-result-page.md:115`
3. story:search-score — "without starting a pack tool" contradicts the story's Constraints, "harbor and orchard through stub tools". The test does start the fixture tool (`fixtures/packs/harbor/tool/harbor-tool.sh`), so read literally the acceptance cannot pass. Say "without a real pack tool". — `.engineering/planning/story/search-score.md:154` against `search-score.md:139`
4. story:search-design — the command is a real before/after check. I ran it on the tree and it prints `3 0` today, and the story wants `0 4`. Two weaknesses remain.
   - The patterns are line-based `grep` on a hard-wrapped file. A correct edit that wraps inside `the agent and the search of metaspec` or `L4 runs.** The tool \`simulate\` gives KPI values` prints 3, not 4.
   - Summing four patterns in one `grep -c` does not show "each present once". Two lines of one sentence and none of another also give 4.
   Use one `grep -c` per sentence, anchored on a short fragment that cannot wrap, and print `0 0 0 1 1 1 1`. — `.engineering/planning/story/search-design.md:106`
5. Low: story:search-enumerate, story:search-batch-scoring, story:search-simulate-adapter and story:search-result-page each list an `acceptance.d/search-*.sh` row in Surfaces. No Scope bullet says what it runs or asserts, and none of the four acceptances names it. Either describe each row as search-score does (`search-score.md:71`) or drop it. — `search-enumerate.md:102`, `search-batch-scoring.md:101`, `search-simulate-adapter.md:109`, `search-result-page.md:91`


Read 13 of 13 ids: the 9 `story:search-*` drafts, `decision-blocker:kpi-order`, `decision-blocker:gate-threshold`, `decision-blocker:search-edits` and `dependency-blocker:metaspec-studio`, each in full. I also checked the brief, the `design/metaspec.md` greps, the fixture packs and tools, and `main.rs` and `Taskfile.yml` for `docs build`.

- **Brief items 6 to 10:**
  - 6 holds apart from finding 4.
  - 7 holds: three cases, and the slow stub is killed at 1 s.
  - 8 holds: 12 points make 11 candidates, limit 4 gives 3 batches, so `runs` is 4 against 12.
  - 9 holds: the harbor and orchard catalogues use the words `higher` and `lower`, so the mapping through `kpis.directions` is observable.
  - 10 holds, with findings 1 to 3.
- **Not established:** I did not run the real orgspec or agenticspec checkouts. The search-orgspec acceptance depends on orgspec's answer carrying KPI values by id. The story declares its own exit for that case.
- **Out of my lane:** nothing to pass on.

```findings
[
{"file":"story:search-result-page","line":null,"category":"acceptance","severity":"blocker","verdict":null,"origin":"undecided","message":"The acceptance command builds harbor alone with --manifest/--pack and no trust step, but the committed fixtures/expected/site holds both packs plus _index and a two-pack index.json (task check:docs uses --pack FILE=DIR per pack after trust in target/docs-state). --check cannot exit 0 against it as written. (citation: .engineering/planning/story/search-result-page.md:115)"},
{"file":"story:search-result-page","line":null,"category":"acceptance","severity":"warning","verdict":null,"origin":"undecided","message":"One sentence joins \"exits 0 against the committed site\" with \"whose page shows the variants, edits, scores, verdicts and stop reason\". The command observes only byte equality with files the story commits; the content claim is not checked by it, and the vitest cases are not in the acceptance. (citation: .engineering/planning/story/search-result-page.md:115)"},
{"file":"story:search-score","line":null,"category":"acceptance","severity":"warning","verdict":null,"origin":"undecided","message":"\"without starting a pack tool\" contradicts Constraints \"harbor and orchard through stub tools\"; the fixture tool is started, so the acceptance as worded cannot pass. Say \"without a real pack tool\". (citation: .engineering/planning/story/search-score.md:154)"},
{"file":"story:search-design","line":null,"category":"acceptance","severity":"warning","verdict":null,"origin":"undecided","message":"The grep patterns are line-based on a hard-wrapped file and can miss a correct sentence that wraps. The summed grep -c prints 4 for two lines of one sentence and none of another, so \"each present once\" is not shown. Use one grep -c per sentence on a short unwrappable fragment and print 0 0 0 1 1 1 1. (citation: .engineering/planning/story/search-design.md:106)"},
{"file":"story:search-enumerate","line":null,"category":"acceptance","severity":"note","verdict":null,"origin":"undecided","message":"scripts/acceptance.d/search-enumerate.sh (and search-batch.sh, search-simulate.sh, search-page.sh in the other three stories) is a listed surface with no stated command or assertion, and no acceptance names it. (citation: .engineering/planning/story/search-enumerate.md:102)"}
]
```
