---
format: aep.planning-md/3
id: review-result:gaps-scope-pass-1
kind: review-result
status: active
title: aep:plan-critic-scope, pass 1, cloudspec gap stories
relations:
- reviews: epic:cloudspec-pack
- reviews: story:cloudspec-warnings
- reviews: story:part-block-aliases
- reviews: story:extends-inherits
revision: 1
---
needs-revision

All three stories claim every outcome the brief asks for. Three of them claim something the brief did not ask for.

Findings:
- story:part-block-aliases — the body fixes a precedence rule, "A word that is a block of the pack is never rewritten by an alias of the same word". Neither the brief nor D41 states that rule. The brief says only that an alias target must be a block and that a word with no alias and no block stays in the "does not have" group. The body should drop the sentence, or name it as a drafter's default for the operator to confirm. — `.engineering/planning/story/part-block-aliases.md:69`
- story:extends-inherits — the "Not in this story" section adds a behaviour: "a base that itself extends is refused with a message". The brief's D42 and its list of exclusions do not mention this refusal, and the tests do not cover it. The body should list "base that itself extends" as not handled, with no new refusal, or the refusal should be marked as the drafter's addition. — `.engineering/planning/story/extends-inherits.md:106`
- story:cloudspec-warnings — the body claims a new Rust test, `test(warning_list_)` in `tools.rs`, plus an open clause "A change that turns out to be needed is made here". The brief says "no Rust change expected" and gives a single acceptance, an `acceptance.sh` row. The story should either drop the test, or the body should say the test is the drafter's addition and why. — `.engineering/planning/story/cloudspec-warnings.md:48`

Coverage against the brief, gap by gap:
- **Gap 5 (warnings):** claimed.
  - It covers the `/warnings` list with `gate: false` and the row showing 3 findings, none gated, equal to the `check --json` warnings count, with the document still valid.
  - It names the `resilience` block under "Not in this story".
- **Gap 4 (aliases):** claimed.
  - It covers `blocks.aliases`, optional, in the schema, with the format staying `metaspec-pack/1` and a CHANGELOG line.
  - It applies in graph and style, refuses an alias whose target is not a block, and keeps unaliased words in "does not have".
  - The cloudspec map is declared as `{group: architecture, node_pool: infrastructure, endpoints: edge}`.
  - The tests and the acceptance row match, including the three public files `groups.yaml`, `node-pools.yaml` and `l4-media.yaml`.
- **Gap 1 (extends):** claimed.
  - It covers omitting `blocks` and `tool` under `extends`, and `--base-pack` as the one named flag.
  - Paths stay inside their own pack, and a manifest without `extends` that omits `blocks` is still refused.
  - Refusal without the base directory, and trust covering the base's tool digest, are claimed.
  - `kpis`, `documents` and `presentation` are stated as not inherited.
  - The fixture tests and the acceptance row match.
- **Dependencies:** the brief's order is in the graph. warnings depends on hub-badges, aliases on warnings, and extends on aliases and collapsed-regions. Shared files are named in each body.
- **Epic:** `epic:cloudspec-pack` does not promise any of the three gaps. Its Outcome section lists the manifest, in/per, views, collapse and later runs. The brief is the only source for these stories, so I did not treat that as a gap or as reach beyond the parent.

What I read: 5 artifacts. I ran `aep plan artifact show` on the epic and the three stories, and `aep plan artifact graph`. I also read the brief and spot-checked `packs/cloudspec.yaml`, `manifest.rs`, `AGENTS.md`, `spec/domains/packs.yaml` and `design/metaspec.md`. Promises extracted from the brief: 22. Traced to a story: 22.

Not established, and out of my lane:
- **Acceptance (not scope):** the acceptance row of story:extends-inherits asserts "the same blocks and page as the manifest `packs/cloudspec.yaml` gives". `documents` is not inherited, so the extending manifest must write its own `documents`. The body does not say so.
- **Design (not scope):** story:part-block-aliases changes the manifest format, but it does not list `design/metaspec.md`. `AGENTS.md:30` says a format change goes in `design/metaspec.md` and `schema/`. story:extends-inherits does list it.

```findings
[
{"file":"<integration tree>/.engineering/planning/story/part-block-aliases.md","line":69,"category":"scope","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the body fixes a precedence rule, 'A word that is a block of the pack is never rewritten by an alias of the same word', that neither the brief nor D41 states; drop it or mark it as the drafter's default for the operator to confirm"},
{"file":"<integration tree>/.engineering/planning/story/extends-inherits.md","line":106,"category":"scope","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the body adds a behaviour the brief and D42 do not name, 'a base that itself extends is refused with a message'; list it only as not handled, or mark the refusal as the drafter's addition"},
{"file":"<integration tree>/.engineering/planning/story/cloudspec-warnings.md","line":48,"category":"scope","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the body claims a new core test (test(warning_list_) in tools.rs) and an open 'a change that turns out to be needed is made here' clause, while the brief says 'no Rust change expected' and asks for one acceptance row; drop the test or mark it as the drafter's addition"}
]
```
