---
format: aep.planning-md/3
id: review-result:quality-acceptance-round-3
kind: review-result
status: active
title: aep:plan-critic-acceptance, round 3, initiative:measure-quality
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
- reviews: initiative:measure-quality
revision: 1
---
needs-revision

- story:kpi-quality-catalogue-shape — the title and summary promise that `metaspec quality kpis` "reports coverage.blocks, completeness.fields and planned for every pack", but the acceptance now checks only the `layout_*` cases (stubs, `--quality` stub, registry refusal). The three measures' goldens and the row file are marked "(not the acceptance)" in scope, so the story closes with its headline outcome unchecked — .engineering/planning/story/kpi-quality-catalogue-shape.md:262-266 (scope at :179-186, title at :106)
- story:quality-ratchet — the acceptance names only the `private_pack_baseline` location case, so it never states the transition in the title, "the first run sets a baseline, a worse value fails". It does not say a worse value exits 1 with the file unchanged, and drops the check that the committed baselines of the four packs hold (`task acceptance` exits 0 with `git status --porcelain quality/baselines/` empty) — .engineering/planning/story/quality-ratchet.md:222-226 (worse-value case at :154-156, committed baselines at :142-149)
- story:spec-quality-mutation — the acceptance, "a second `scripts/slow-mutation.sh` run over blocks whose digest already has a result adds no line to the stub's call log and exits 0", passes for a script that never calls `ess` and never writes a result. It names neither the first run, where one `ess` job per class is logged and a result is written for the digest, nor the score read from it — .engineering/planning/story/spec-quality-mutation.md:268-270 (first-run case at :20-21)
- initiative:measure-quality — the body has no `## Acceptance` section; `## Outcome` is a two-question paragraph with no command, output or exit status to check — .engineering/planning/initiative/measure-quality.md:12-29

What I read: 20 of 20 given ids; initiative, epics and blockers in full; every story's Acceptance; full bodies of quality-catalogue, quality-ratchet, kpi-quality-catalogue-shape, spec-quality-self, spec-quality-drift and the spec-quality-mutation scope; acceptance-r2.md.

What I could not establish:
- Whether the acceptance rows and `task` targets exist on the target branch; commands judged as written.
- Round-2 findings judged landed; the ratchet and catalogue-shape fixes moved the gap to the other half of each story (findings 1 and 2).
- Unease without a one-line fix: "committed" baselines vs `task quality:self`/`task acceptance` exiting 0 on a first run that writes them.
- Out of lane: `spec-quality-conformance` runs a real `ess verify conform run` on orgspec (892 scenarios) inside `task acceptance`, which may break the 120 s budget; whether ratchet and catalogue-shape should be split is scope.

```findings
- file: .engineering/planning/story/kpi-quality-catalogue-shape.md
  line: 262
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the title and summary promise that quality kpis reports coverage.blocks, completeness.fields and planned for every pack, but the acceptance checks only the layout_ cases (stubs, --quality stub, registry refusal); the three measures' goldens and row file are marked '(not the acceptance)' in scope, so the headline outcome has no check"
- file: .engineering/planning/story/quality-ratchet.md
  line: 222
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the acceptance names only the private_pack_baseline location case, so it never states the title's transition (the first run sets a baseline, a worse value exits 1 with the file unchanged) and drops the committed-baselines check of the four packs (task acceptance exits 0 with git status --porcelain quality/baselines/ empty)"
- file: .engineering/planning/story/spec-quality-mutation.md
  line: 268
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance, 'a second scripts/slow-mutation.sh run over blocks whose digest already has a result adds no line to the stub's call log and exits 0', passes for a script that never calls ess and writes no result; it names neither the first run (one ess job per class logged, a result written for the digest) nor the score read from it"
- file: .engineering/planning/initiative/measure-quality.md
  line: 12
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: undecided
  message: "the initiative has no Acceptance section; its Outcome is a two-question paragraph with no command, output or exit status that someone could check"
```
