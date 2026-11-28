---
format: aep.planning-md/3
id: review-result:gaps-acceptance-pass-2
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 2, cloudspec gap stories
relations:
- reviews: epic:cloudspec-pack
- reviews: story:cloudspec-warnings
- reviews: story:part-block-aliases
- reviews: story:extends-inherits
revision: 1
---
needs-revision

- story:cloudspec-warnings — the first acceptance bullet joins two checks with "and": the page shows 3 findings, none gated, equal to `warnings`, "and the document stays valid". D50 forbids that join, so the validity check is a second bullet — `.templates/../cloudspec-warnings.md:74-76`
- story:cloudspec-warnings — the new row's "3" depends on a cloudspec checkout whose `check --json` writes `warnings`, but no bullet names the commit (part-block-aliases pins 6259dce). The bullet `task acceptance -- <orgspec> <agenticspec> <cloudspec>` also leaves the checkouts unnamed — `.engineering/planning/story/cloudspec-warnings.md:78`
- story:part-block-aliases — the cloudspec-row bullet holds two independent outcomes across three files: the aliased parts carry the target block, and `payment_api` and `mail_relay` stay unaliased. It also does not say where "stay in 'block the pack does not have'" is read. That label is drawn by `crates/metaspec-layout/src/lib.rs:137`, and the page data holds only the part's `block` word — `.engineering/planning/story/part-block-aliases.md:116-121`
- story:part-block-aliases — the row's hard numbers (6, 3, 1) are pinned to a commit, but the row reads a moving checkout. I checked them against cloudspec 6259dce and they are correct there. The row fails as soon as cloudspec adds a `group` part. It should count from the files at run time or name the pin as an input — `.engineering/planning/story/part-block-aliases.md:117-120`
- story:part-block-aliases — the case "a word with neither alias nor block keeps its word in the graph" is in Scope (lines 68-69) and in no acceptance bullet. The refusal bullet says "in the same run" but does not require test names containing `alias_`. A refusal test not named that way would not be in the run — `.engineering/planning/story/part-block-aliases.md:114-115`
- story:part-block-aliases — the bullet "`scripts/acceptance.sh` has a cloudspec row" is checked by reading the script, not by running it. No bullet says `task acceptance` exits 0 with the row green, as cloudspec-warnings does — `.engineering/planning/story/part-block-aliases.md:116`
- story:extends-inherits — the acceptance row "renders one public cloudspec architecture" names no document, so two checkers can pick different ones. Naming `groups.yaml` would make the block comparison meaningful (it has aliased parts). Naming `journeys.yaml` would do the same for the findings comparison (it has the 3 warnings). A document with neither makes "equal" trivially true — `.engineering/planning/story/extends-inherits.md:140-143`
- story:extends-inherits — the same missing check as in part-block-aliases: the row is never required to pass under `task acceptance`. It also does not say that the extending manifest declares its own `documents`, which are not inherited — `.engineering/planning/story/extends-inherits.md:140`
- story:extends-inherits — the bullets do not fix test names. "the case where X" cannot be located unless the tests carry `inherit_` and a case name. The scope's promise that "a refusal names which pack the path was in" (line 69) is in no bullet — `.engineering/planning/story/extends-inherits.md:130-139`

D43–D50 hold in my lane:
- **D43:** the key-is-a-block refusal has a test bullet.
- **D44:** the base-that-extends refusal has a test bullet.
- **D45:** the warnings story carries no new Rust test, and `acceptance.sh` is the check.
- **D46:** the edges are present as the brief lists them. I read them in the store: view-selectors, collapsed-regions, matrix-view and focus-pages each `depends_on part-block-aliases`, and search-score `depends_on extends-inherits`. cloudspec-runs `depends_on cloudspec-warnings` and extends-inherits has the four view edges.
- **D47:** `graph::read` at `graph.rs:182` is called from `document.rs:126` with `block` set at `graph.rs:202`, so one site is feasible.
- **D49:** `tool.rs:126` joins a `/` path to the root and sets `current_dir(root)`, so the base-directory claim is feasible.
- **D50:** the acceptance sections are bullet lists. The defects above are the exceptions.
- **`.violations` rows:** `theirs_cloudspec` is at `acceptance.sh:260-262` and `named` at `335-341`, as the story says.

What I read: 3 of 3 artifacts, with `aep plan artifact show` (plus `kinds`, `lifecycle story` and the relations of the other 7 stories), `scripts/acceptance.sh`, `Taskfile.yml` and `.config/nextest.toml` in the store, the graph, tool and layout sources, and cloudspec at `origin/main` 6259dce (block-word counts and block domains).

What I could not establish:
- Whether `journeys.yaml` yields exactly 3 warnings, because I may not build or run cloudspec.
- Whether `nextest` fails on a filter that matches zero tests. I recall that it does by default, but did not run it.

Outside my lane, not weighed in the verdict:
- **Dependencies:** part-block-aliases and extends-inherits are drafts, but the active stories view-selectors, collapsed-regions, matrix-view and focus-pages now depend on part-block-aliases. That is a parallel-safety and design question.
- **Overlapping surfaces:** part-block-aliases and extends-inherits both edit `manifest.rs`, `pack.rs` and the schema (parallel-safety).

```findings
[
{"file":".engineering/planning/story/cloudspec-warnings.md","line":74,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the first bullet joins two checks with 'and' (3 findings none gated equal to the length of warnings, 'and the document stays valid'); D50 forbids the join"},
{"file":".engineering/planning/story/cloudspec-warnings.md","line":78,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the row's 3 depends on a cloudspec checkout whose check --json writes warnings, and the bullet leaves the three checkouts and the cloudspec commit unnamed"},
{"file":".engineering/planning/story/part-block-aliases.md","line":116,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the cloudspec-row bullet holds two independent outcomes (aliased parts carry the target block; payment_api and mail_relay stay unaliased) and does not say where the 'block the pack does not have' group is read, since the page data holds only the part's block word"},
{"file":".engineering/planning/story/part-block-aliases.md","line":117,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the hard counts 6, 3 and 1 are pinned to cloudspec main 6259dce but the row reads a moving checkout, so it fails when cloudspec adds a group part; count from the files at run time or name the pin as an input"},
{"file":".engineering/planning/story/part-block-aliases.md","line":114,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the case 'a word with neither alias nor block keeps its word in the graph' is in Scope and in no acceptance bullet, and 'in the same run' does not require the refusal tests to carry alias_ in their names"},
{"file":".engineering/planning/story/part-block-aliases.md","line":116,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the row is checked by reading the script; no bullet requires task acceptance to exit 0 with the row green"},
{"file":".engineering/planning/story/extends-inherits.md","line":140,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the acceptance row renders 'one public cloudspec architecture' without naming the document, so the comparison can be trivially equal; name one with aliased parts and the warnings (groups.yaml or journeys.yaml), state that the extending manifest declares its own documents, and require task acceptance to exit 0 with the row green"},
{"file":".engineering/planning/story/extends-inherits.md","line":130,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the bullets say 'the case where X' without fixing test names carrying inherit_, and the scoped refusal message naming which pack the path was in (line 69) is in no bullet"},
{"file":".engineering/planning/story/part-block-aliases.md","line":36,"category":"acceptance","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"D43 and D47 hold in my lane: the key-is-a-block refusal has a test bullet, and graph::read (graph.rs:182) called from document.rs:126 with block set at graph.rs:202 is one feasible site"},
{"file":".engineering/planning/story/extends-inherits.md","line":71,"category":"acceptance","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"D44 holds in my lane: the base-that-extends refusal has its own test bullet (line 138), and D49's base-directory root matches tool.rs:126"}
]
```
