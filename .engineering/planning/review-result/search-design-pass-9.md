---
format: aep.planning-md/3
id: review-result:search-design-pass-9
kind: review-result
status: active
title: aep:plan-critic-design, pass 9, epic:shared-search
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

X1 and X2 hold apart from one leftover. A fresh `depends_on` walk finds no cycle, and no two stories own one surface.

Findings:
- `story:search-result-best` — its summary still says it "shows strategy, beam width and the stop reason". That contradicts X1, which gives `stop.reason` in words to `story:search-result-page` alone. Edit the summary to "ResultSections marks the front and the best variant, shows strategy and beam width; reads the result and computes no order." — `.engineering/planning/story/search-result-best.md:7`. The body now agrees with X1 at lines 51, 61 and 71. It is a second statement of ownership in the same file, and the summary is what `aep plan artifact list` and the waves view print.

Checks that hold:
- **X1:** `story:search-result-page` is the only story that draws `stop.reason`.
  - Scope lines 79-83 fix one sentence per reason and say the schema owns the four values.
  - Surfaces line 107 creates the four `stop-<reason>.json` fixtures.
  - Acceptance lines 146-149 render each reason from a fixture.
  - `story:search-result-best` Scope (line 51), its "Not in this story" (line 61) and its fixture note (line 71) now defer to `story:search-result-page`. Its Acceptance (lines 107-111) no longer mentions the stop reason.
  - The other stories name a reason only as a value, never as a drawing.
- **X2:** `story:search-result-format` line 61 says "the schema owns the four strings … `StopReason` … serialises to them". `story:search-enumerate` lines 171-173 add a unit test that serialises each variant, compares it with the schema enum, and also compares the whole set, so a fifth entry on either side fails. `story:search-dominance` line 83 and the `story:search-result-format` line 65 both say no story adds a stop reason.
- **Four copies of the strings:** the schema, the enumerate enum, the page wording and the `story:search-design` line in `design/metaspec.md` all list the four reasons. The enumerate test binds the enum to the schema, and the design line is documentation that derives from the enumerate code. I did not count the design line as a second owner.
- **Edges:** the consumers of each surface carry a `depends_on`.
  - `story:search-enumerate` depends on `story:search-result-format`.
  - `story:search-result-page` depends on `story:search-result-format` and `story:search-enumerate`.
  - `story:search-result-best` depends on `story:search-page`, `story:search-dominance` and `story:search-beam`.
  - There is no chain that serialises the set.
- **Shared file:** `story:search-result-page` and `story:search-result-best` both edit `ResultSections.tsx`. The edge records it and each story draws a different thing, so I did not flag it.

What I read: 14 `story:search-*` bodies in full or by targeted grep for stop, strategy, `ResultSections` and `beam_width`, plus the revision 8 brief. I ran `aep plan artifact relations`, `graph` and `validate` (74 artifacts, valid). I walked 85 edges, including the ones to `story:cloudspec-manifest`, the `decision-blocker:*`, `architecture-decision-record:shared-search` and `epic:shared-search`. I traced 9 acceptance reads to a producer in the set, and an edge records all 9.

What I could not establish:
- Whether X4's order of the three old fragments matches the file. I did not open `design/metaspec.md`, and it is an acceptance-lane question.
- Acceptance lane: `story:search-result-format` pins the four strings only through the enumerate test, not in its own acceptance. This did not set my verdict.

```findings
[
{"file":".engineering/planning/story/search-result-best.md","line":7,"category":"design","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the summary still says ResultSections 'shows strategy, beam width and the stop reason', a second claim on drawing stop.reason after X1 gave it to story:search-result-page alone; the summary names no stop reason"}
]
```
