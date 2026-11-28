---
format: aep.planning-md/3
id: review-result:search-acceptance-pass-3
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 3, epic:shared-search
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

Pass 2 items A1-A5 hold. The five new stories carry four acceptance defects, and `search-dominance` has a smaller fifth. I did not run any cargo or vitest command. I ran only the `search-design` grep.

**Pass 2, checked**
- A1/A2: `search-result-page` now names the vitest command, and the site golden is in Scope.
- A3: `search-score` now says "without a real pack tool".
- A4: the `search-design` grep prints `1 1 1 0 0 0 0` on the current `design/metaspec.md`. The old fragments are on lines 233, 350 and 434.
- A5: no `scripts/acceptance.d/search-*.sh` row remains except `search-score` and `search-orgspec`.
- The Design and Parallel-safety items are also in place: D1 (`runs`), D2, D3, D4, P1, P2 and P3 (`--simulate` flags in the Args structs of `score.rs` and `search.rs`).

**Findings**

```
story:search-hill-climb — `--budget 2` must give stop reason `budget`, but each fixture pack's marked document has one place with 3 values, so the space is 2 candidates plus the baseline. `--budget` counts candidates (not the baseline), so budget 2 scores the whole space and `budget`, `exhausted` and `no_better_neighbour` all hold at once. No precedence is stated, so the acceptance cannot tell them apart. The same holds for "stop.reason is no_better_neighbour" at seed 3. — .engineering/planning/story/search-hill-climb.md:110-114, :66; fixtures/packs/harbor/harbors/inlet.yaml:6; fixtures/packs/orchard/orchards/hillside.yaml:9
story:search-beam — beam width 2 on a 3-point space scores both neighbours of the baseline in round 1, so every point is scored. `exhausted` and `no_better_neighbour` both hold, as do `budget` and `exhausted` under `--budget 2`. The acceptance demands `no_better_neighbour` and `budget` with no precedence rule. `search-batch-scoring` already marks a second place to make a 12-point space; the same fix, or a stated precedence, would settle these two. — .engineering/planning/story/search-beam.md:98-101, :56
story:search-result-best — the acceptance is conditional, "the mark on `best` when the result has it". The recorded results it renders are `story:search-enumerate`'s, with stop reason `exhausted` and strategy `enumerate`. It can pass without `best` ever being drawn. Scope also promises `strategy`, `beam_width` and `no_better_neighbour` in words, and the acceptance observes none of them. The climb and beam recordings are not in its `depends_on`. Name a recorded or hand-written result that has a one-member front, `strategy` `beam` with `beam_width`, and `no_better_neighbour`. — .engineering/planning/story/search-result-best.md:93-96 (Scope lines 317-321 in the show output)
story:search-result-page — `docs build --result` and its refusals (another pack or document, invalid result, digest mismatch, exit 1) are the transition this story makes. The acceptance is only the vitest component cases, so it passes with the flag never built. The cargo case in `crates/metaspec/tests/search_page` is not named in it. The A1/A2 decision narrowed the acceptance correctly, but the flag is now unobserved. Scope also lists a "failed variant" case, and the recorded results (2 candidates) have none. — .engineering/planning/story/search-result-page.md:125-127, :88, :95
story:search-design — the acceptance greps only the 3 removals and 4 sentences. The heading "Decided: shared search" is where search-gate, search-dominance, search-hill-climb and search-beam each write a paragraph (and "Docs" for search-result-best). A design edit can pass the acceptance without creating it. Add one `grep -c` for the heading, so the output is `0 0 0 1 1 1 1 1`. — .engineering/planning/story/search-design.md:70, :104-110
story:search-dominance — "a variant of unequal KPIs are checked" does not name the incomparable pair. Two variants with unequal KPIs can still have one dominate the other, so the acceptance passes without the case Constraints name ("an incomparable pair"). — .engineering/planning/story/search-dominance.md:125
```


**Read:** all 14 `story:search-*` drafts, in full (14 of 14). I did not read `epic:shared-search` or the ADR (not my lane). I checked the fixtures, `design/metaspec.md`, `web/ui/vitest.config.ts` and `.config/nextest.toml`.

**Test budget:** the stories bound tests at 60 s and 120 s, use stubs only, and make no model call or network request. The one 10 s sleep stub is killed at 1 s. Nothing here breaks the test-speed rule.

**Could not establish:**
- Whether the other fixtures' stubs give a worse, equal and better candidate for the gate cases. The harbor and orchard catalogues each have one gated KPI (`vessel_unmoored_count`, `basket_overflow_count`), so the cases are possible.
- Whether `orgspec`'s real tool answers KPI values. The story's own stop rule covers that.

**Out of my lane:**
- `search-dominance` Scope (lines 38-41 of its file) says `compare(a, b)` gives "the verdicts of b against a" and then "a dominates b when no verdict is `worse`, at least one is `better`". That inverts the argument order, which is a design or wording problem.
- `search-hill-climb` says `crates/metaspec/src/search.rs` is created by `search-candidate-space`; `search-enumerate` creates it.

```findings
[
{"file":"story:search-hill-climb","line":null,"category":"acceptance","severity":"blocker","verdict":null,"origin":"undecided","message":"--budget 2 -> budget and no_better_neighbour are not distinguishable from exhausted on the 3-point fixture spaces; no stop-reason precedence stated (citation: .engineering/planning/story/search-hill-climb.md:110)"},
{"file":"story:search-beam","line":null,"category":"acceptance","severity":"blocker","verdict":null,"origin":"undecided","message":"round 1 scores the whole 3-point space, so no_better_neighbour and budget (--budget 2) coincide with exhausted; no precedence or larger space named (citation: .engineering/planning/story/search-beam.md:98)"},
{"file":"story:search-result-best","line":null,"category":"acceptance","severity":"blocker","verdict":null,"origin":"undecided","message":"mark on best 'when the result has it' is vacuous on the enumerate recordings; strategy, beam_width and no_better_neighbour drawing unobserved (citation: .engineering/planning/story/search-result-best.md:93)"},
{"file":"story:search-result-page","line":null,"category":"acceptance","severity":"warning","verdict":null,"origin":"undecided","message":"acceptance is only the vitest cases; docs build --result and its refusals can be unbuilt and it still passes; the 'failed variant' case has no source (citation: .engineering/planning/story/search-result-page.md:125)"},
{"file":"story:search-design","line":null,"category":"acceptance","severity":"warning","verdict":null,"origin":"undecided","message":"heading 'Decided: shared search' that four later stories write under is not observed by the grep (citation: .engineering/planning/story/search-design.md:104)"},
{"file":"story:search-dominance","line":null,"category":"acceptance","severity":"note","verdict":null,"origin":"undecided","message":"'a variant of unequal KPIs' does not name the incomparable pair (citation: .engineering/planning/story/search-dominance.md:125)"}
]
```
