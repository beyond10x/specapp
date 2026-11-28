---
format: aep.planning-md/3
id: review-result:gaps-acceptance-pass-3
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 3, cloudspec gap stories
relations:
- reviews: epic:cloudspec-pack
- reviews: story:cloudspec-warnings
- reviews: story:part-block-aliases
- reviews: story:extends-inherits
revision: 1
---
needs-revision

story:extends-inherits — the closing bullet says "exits 0 with the row green", but the story's two bullets above it each add a row, so the bullet names one row where two exist and a reader can't tell which must be green — `.engineering/planning/story/extends-inherits.md:161` ("`task acceptance -- <orgspec> <agenticspec> <cloudspec>` exits 0 with the row green.")

story:cloudspec-warnings — one bullet covers two rows, `examples:rules-matched` and `examples:named-drawn`, so one can pass while the other fails, which breaks the one-command-one-outcome rule (D50). It also says "stay equal", which reads the same before and after the work. The transition is carried only by the first bullet — `.engineering/planning/story/cloudspec-warnings.md:82` ("The rows `examples:rules-matched` and `examples:named-drawn` stay equal with warnings counted.")

Pass-2 findings in my lane:
- **K1, K2, K5, K6, K7, K9:** fixed.
- **K8:** the documents are named and the extending manifest declares its own `documents`, but the exit-0 bullet has the singular-row wording above.

The edits introduced nothing else. The three stories' bullets and the epic's three new Outcome bullets are single observable statements.

What I read: all 3 ids (`story:cloudspec-warnings`, `story:part-block-aliases`, `story:extends-inherits`) in full, plus `epic:cloudspec-pack`, `story:search-score` (relations only), `gaps-revise-2.md`, and the tree at `scripts/acceptance.sh`, `document.rs:126`, `layout/lib.rs:137` and `graph.rs:182`.

What I could not establish: I could not read the private-pack files or cloudspec's architecture files. `payment_api`, `mail_relay`, and the counts 6, 3 and 1 are taken from the story text.

Out of my lane: the `extends-inherits` and `search-score` ordering (D46) belongs to the design and parallel-safety critics.

```findings
[
{"file":".engineering/planning/story/extends-inherits.md","line":161,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"\"`task acceptance -- <orgspec> <agenticspec> <cloudspec>` exits 0 with the row green.\" names one row, but the story adds two cloudspec rows (groups.yaml graph, journeys.yaml findings); say \"both rows green\"."},
{"file":".engineering/planning/story/cloudspec-warnings.md","line":82,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"undecided","message":"\"The rows `examples:rules-matched` and `examples:named-drawn` stay equal with warnings counted.\" joins two rows in one bullet (D50) and \"stay equal\" reads the same before the work; split into one bullet per row, each stating that the row counts `.violations + .warnings` and is green on journeys.yaml."}
]
```
