---
format: aep.planning-md/3
id: review-result:quality-design-round-2
kind: review-result
status: active
title: aep:plan-critic-design, round 2, initiative:measure-quality
relations:
- reviews: epic:kpi-quality
- reviews: epic:spec-quality
- reviews: story:kpi-quality-catalogue-shape
- reviews: story:kpi-quality-grounding
- reviews: story:kpi-quality-page
- reviews: story:kpi-quality-rules
- reviews: story:kpi-quality-values
- reviews: story:quality-catalogue
- reviews: story:quality-ratchet
- reviews: story:spec-quality-conformance
- reviews: story:spec-quality-drift
- reviews: story:spec-quality-layering
- reviews: story:spec-quality-mutation
- reviews: story:spec-quality-page
- reviews: story:spec-quality-refusal
- reviews: story:spec-quality-self
- reviews: story:spec-quality-static
revision: 1
---
approve

I read the whole set as it stands and found nothing that would change what was drafted.

**What I read:** 22 artifacts, namely the initiative, 2 epics, 2 cleared decision-blockers and 15 stories. I ran `aep plan artifact show` on each, plus `relations`, `graph` and `validate`. `validate` printed `valid`. I did not open the review-result records.

**Edges walked:** all declared edges in `aep plan artifact graph`, including those leaving the set. There are 37 `depends_on`, 17 `decomposes`, 2 `blocks` and the `reviews` edges from the review-result records.
- `depends_on` means needs first, and the `depends_on` graph is acyclic. The longest path is `quality-catalogue` → `kpi-quality-catalogue-shape` → `spec-quality-static` → `quality-ratchet` → `kpi-quality-values` → `kpi-quality-rules` → `spec-quality-refusal` → `spec-quality-self`.
- The set is not a queue. After `quality-ratchet`, grounding, values, layering and the KPI page can run side by side. After values, rules and conformance can. After rules and conformance, refusal, drift and mutation can.
- Every ordering edge that exists for a shared file has its reason written in the body's "Shared files" section.

**Acceptance reads traced:** I traced about 25 reads of state a sibling produces. Every one is covered by a `depends_on` edge, directly or through a recorded chain. Examples:
- Rules reads the broken class that values creates, so rules depends on values.
- Refusal reads the harbor broken plans from values and rules.
- Spec-page reads the quality page from kpi-page.
- Conformance, drift and mutation read the manifest fields and the slow-lane file their predecessors write.
- Self reads a measure from every measure story and depends on all of them.

**What I could not establish**
- One item is out of my lane (parallel-safety/acceptance). `story:spec-quality-mutation` has a test case saying "the stub `ess` of the tests refuses `mutate` outside `slow-mutation.test.sh`". `story:spec-quality-static` owns the stub and says later stories "never change" it. Mutation also commits `fixtures/ess/harbor/mutate.json`, which a stub that prints `fixtures/ess/<pack>/<verb>.json` would serve. The source is `.engineering/planning/story/spec-quality-mutation.md`, scope bullet "`task check` runs no `ess verify conform mutate`".
- I could not check the claims about the `invert-pack-knowledge` branch (manifest `specapp-pack/2`, check adapter, `scripts/acceptance.sh`) against the checked-out tree, which is `main`.

```findings
[]
```
