---
format: aep.planning-md/3
id: review-result:quality-parallel-round-1
kind: review-result
status: active
title: aep:plan-critic-parallel-safety, round 1, initiative:measure-quality
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
- reviews: story:spec-quality-layering
- reviews: story:spec-quality-mutation
- reviews: story:spec-quality-page
- reviews: story:spec-quality-refusal
- reviews: story:spec-quality-self
- reviews: story:spec-quality-static
revision: 1
---
needs-revision

story:quality-catalogue — all 14 stories carry `scope: []`, so the store's wave derivation returns "0 wave(s), 0 collision(s), 14 unassessed", and a plan reading as parallel is unassessed; record each story's Surfaces with `aep plan artifact scope` (cited paths) — `aep plan artifact waves` output
story:kpi-quality-page — it changes `crates/specapp/src/main.rs` (flag) while `story:spec-quality-static` and `story:quality-ratchet` change the same file, and no `depends_on` orders the page story against either; neither body names the others; add an ordering edge recording `main.rs`, or split the subcommand wiring out of `main.rs` — .engineering/planning/story/kpi-quality-page.md:36, spec-quality-static.md:47, quality-ratchet.md:46
story:kpi-quality-grounding — it shares `quality/kpis/mod.rs`, `fixtures/packs/harbor/kpis/catalogue.yaml` and `fixtures/expected/*/quality-kpis.json` with `story:kpi-quality-values` (and `fixtures/packs/harbor/` with `story:kpi-quality-rules`); grounding and values are unordered and no body names the other; add an edge naming those files, or split the fixture files — .engineering/planning/story/kpi-quality-grounding.md:42-44, kpi-quality-values.md:50-52, kpi-quality-rules.md:44-45
story:kpi-quality-grounding — `scripts/acceptance.sh` is a changed surface of 12 stories (shape, grounding, page, values, rules, ratchet, static, conformance, layering, mutation, spec-page, refusal), which includes many unordered pairs (e.g. grounding, page, values, static, layering, conformance), and not one body admits the sharing; add edges recording the file, or split it into one row file per story that the script sources — .engineering/planning/story/kpi-quality-grounding.md:44, kpi-quality-page.md:36, spec-quality-layering.md:37
story:kpi-quality-values — it claims whole directories `fixtures/expected/` and `fixtures/packs/harbor/`, which hold every other story's goldens and recorded reports (the `quality.html` pages, `quality-kpis.json`, `quality-spec.json`, the harbor catalogue), so it collides with `story:kpi-quality-page`, `story:kpi-quality-grounding` and `story:quality-ratchet`; narrow it to the files it changes — .engineering/planning/story/kpi-quality-values.md:50-51
story:spec-quality-layering — it shares `crates/specapp-core/src/quality/spec/mod.rs` with conformance, mutation and refusal, and `fixtures/expected/*/quality-spec.json` with `story:spec-quality-refusal`; layering is unordered against all three and no body names the others; add an edge recording the files, or split the registry and fixture files — .engineering/planning/story/spec-quality-layering.md:36-37, spec-quality-refusal.md:38-39, spec-quality-conformance.md:43, spec-quality-mutation.md:43-44
story:spec-quality-conformance — it changes `packs/orgspec.yaml` (`blocks.known_failing`) and so does `story:kpi-quality-values` (the `expect: invalid` class); the body admits `manifest.rs` and the schema as shared but not this file, and no edge orders them; name the file and add an edge, or split the manifest edits — .engineering/planning/story/spec-quality-conformance.md:42, kpi-quality-values.md:50
story:spec-quality-self — it changes `kpis/catalogue.yaml` by dropping the planned block `quality`, a change to the `block` column across rows, while `story:quality-catalogue` promises "Later stories change only the `status` of their own rows"; so it collides with every story that flips a status and no body says so; state it and add edges, or split the `block` change into the catalogue story — .engineering/planning/story/spec-quality-self.md:45, quality-catalogue.md:56
story:spec-quality-self — it changes `Taskfile.yml` (task `quality:self`) as `story:spec-quality-mutation` does (task `slow:mutation`), the two are unordered, and neither names the other; add an edge recording `Taskfile.yml` or split the tasks into included files — .engineering/planning/story/spec-quality-self.md:44, spec-quality-mutation.md:43
story:spec-quality-self — its acceptance prints a value or a listed gap "for every row of `kpis/catalogue.yaml`" and holds them to a baseline its first run writes, so it reads what grounding, values, rules, conformance, layering, mutation and refusal create, and it has `depends_on` only to static, shape and ratchet; add edges to those seven stories, or merge — .engineering/planning/story/spec-quality-self.md:78-80, kpi-quality-grounding.md:78
story:quality-ratchet — a measure with no baseline writes its value into `quality/baselines/{harbor,orchard,orgspec,agenticspec}.json`, and `scripts/acceptance.sh` runs the ratchet over the reports, so each later measure story (grounding, values, rules, conformance, layering, mutation, refusal) changes those files; none of them lists them and no edge orders them behind the ratchet; name the files in those bodies and add edges, or split the baselines per measure — .engineering/planning/story/quality-ratchet.md:29, quality-ratchet.md:34, quality-ratchet.md:43
story:kpi-quality-page — its golden `fixtures/expected/<pack>/pages/quality.html` is rendered from a KPI report that grounding, values and rules each extend via `fixtures/expected/*/quality-kpis.json`, and none of them lists the page golden (inferred: the report source of the golden is not named); say which report the golden reads and add edges or pin it to a recorded report — .engineering/planning/story/kpi-quality-page.md:33-34, kpi-quality-grounding.md:44
story:spec-quality-page — its golden pages are rendered from both reports while layering and refusal change `fixtures/expected/*/quality-spec.json`, and it has no edge to either (inferred: the report source of the golden is not named); say which report the golden reads and add edges or pin it to a recorded report — .engineering/planning/story/spec-quality-page.md:33-34, spec-quality-layering.md:37, spec-quality-refusal.md:39

What I read: 14 stories, 2 epics, the initiative and 2 decision-blockers, through `aep plan artifact list`, `graph`, `relations`, `show` on every story, `waves` and `scope --help`, plus `scripts/acceptance.sh` and the tests directory in the tree.

Surface counts: 14 cited (every story has a Surfaces section), 0 inferred, 0 unplaced; 0 of 14 have a recorded `scope`. Acceptance reads traced to a producer in the set: 26, of which 19 are recorded by a `depends_on` edge (the 7 missing are all in `story:spec-quality-self`).

Acknowledged, not findings: `manifest.rs` and `schema/specapp-pack-2.schema.json` are named as shared in both `story:kpi-quality-values` and `story:spec-quality-conformance`, and the `status` column is named in `story:quality-catalogue` as single-row edits. Both are admitted collisions for a person to decide; no edge records them.

What I could not establish: which report file the two `quality.html` goldens render from, so those two findings rest on an inferred surface. I also did not check whether `tests/cases/mod.rs` and `quality/kpis/mod.rs` registrations collide beyond adjacent-line merges.

Out of my lane: `story:spec-quality-conformance` moves its measure to the slow lane of `story:spec-quality-mutation` if it runs over 10 minutes, and mutation depends on conformance. `task acceptance` takes two arguments, but `scripts/acceptance.sh` takes three. `epic:spec-quality` has a `depends_on` edge to `story:quality-ratchet`, which belongs to the other epic. The design and acceptance critics should judge these.

```findings
- file: "aep plan artifact waves"
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "all 14 stories carry scope [] so waves derives 0 waves and reports 14 unassessed; record each story's cited Surfaces with aep plan artifact scope"
- file: .engineering/planning/story/kpi-quality-page.md
  line: 36
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "changes crates/specapp/src/main.rs like spec-quality-static and quality-ratchet, no depends_on orders them and no body names the others"
- file: .engineering/planning/story/kpi-quality-grounding.md
  line: 42
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "shares quality/kpis/mod.rs, fixtures/packs/harbor/kpis/catalogue.yaml and fixtures/expected/*/quality-kpis.json with kpi-quality-values (and fixtures/packs/harbor/ with kpi-quality-rules); grounding and values are unordered and unnamed"
- file: .engineering/planning/story/kpi-quality-grounding.md
  line: 44
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "scripts/acceptance.sh is changed by 12 stories with many unordered pairs and no body admits the sharing"
- file: .engineering/planning/story/kpi-quality-values.md
  line: 50
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "claims the whole directories fixtures/expected/ and fixtures/packs/harbor/, which collide with the goldens and recorded reports of page, grounding and ratchet stories; narrow to the files changed"
- file: .engineering/planning/story/spec-quality-layering.md
  line: 36
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "shares quality/spec/mod.rs with conformance, mutation and refusal and fixtures/expected/*/quality-spec.json with refusal; unordered and unnamed"
- file: .engineering/planning/story/spec-quality-conformance.md
  line: 42
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "packs/orgspec.yaml is also changed by kpi-quality-values and the body admits only manifest.rs and the schema as shared; no edge orders the two"
- file: .engineering/planning/story/spec-quality-self.md
  line: 45
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "changes the block column of kpis/catalogue.yaml although quality-catalogue states later stories change only status, so it collides with every story that flips a status and says nothing of it"
- file: .engineering/planning/story/spec-quality-self.md
  line: 44
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "changes Taskfile.yml like spec-quality-mutation; unordered and unnamed"
- file: .engineering/planning/story/spec-quality-self.md
  line: 78
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "acceptance prints a value or gap for every catalogue row and reads the baseline, so it reads what grounding, values, rules, conformance, layering, mutation and refusal create; no depends_on edge runs to those seven stories"
- file: .engineering/planning/story/quality-ratchet.md
  line: 29
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "a measure with no baseline writes quality/baselines/*.json, so each later measure story changes those files; none lists them and no edge orders them behind the ratchet"
- file: .engineering/planning/story/kpi-quality-page.md
  line: 33
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "inferred: the quality.html golden renders a KPI report that grounding, values and rules extend via quality-kpis.json, and none lists the page golden; the report source is not named"
- file: .engineering/planning/story/spec-quality-page.md
  line: 33
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "inferred: the quality.html golden renders a specification report that layering and refusal extend via quality-spec.json, with no edge to either; the report source is not named"
```
