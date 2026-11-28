---
format: aep.planning-md/3
id: review-result:adversary-part-block-aliases-pass-1
kind: review-result
status: active
title: aep:adversary, pass 1, story:part-block-aliases
relations:
- reviews: story:part-block-aliases
revision: 1
---
# aep:adversary, pass 1, story:part-block-aliases

unit: story:part-block-aliases, commit 48fb83d plus one uncommitted test in <worktree>
verdict: NEEDS-CHANGE
cases: executed 182→183, red 0 (red 1 against a mutated copy)
origin: introduced 4 / pre-existing 0 / undecided 0
wrote-outside-worktree: 9 paths, all under <scratch>/part-block-aliases/adversary/
needs-coordinator: whether the test I added goes into the unit. It passes on 48fb83d, and it is the only case in the suite that fails when the alias wiring in `document.rs` is removed.

**1. Diff of the worktree**
```
 crates/metaspec/tests/main.rs | 116 ++++++++++++++++++++++++++++++++++++++++++
 1 file changed, 116 insertions(+)
```
It touches one file, a test file, and no implementation file.

**2. The case I added**
`crates/metaspec/tests/main.rs`, test `adversary_alias_a_rendered_page_draws_the_aliased_part_in_its_block`.

For each fixture pack (harbor and orchard) it does this:
- copies the pack into the test's temp directory;
- builds a manifest in the test that adds `blocks.aliases: {zz_alias_word: <first block from pack check --json>}`;
- adds a document with one part written `zz_alias_word` and one written `zz_no_block_word`;
- checks that `pack check` exits 0;
- runs `metaspec render` and reads the page data.

It asserts that `graph.parts` gives the first part the target block and the second part keeps its word. It also asserts that every scene box of the aliased part is outside the group "block the pack does not have", and that every box of the other part is in it. No pack words are written in it: the vocabulary check passes ("none in the code").

- **On 48fb83d:** green.
- **On the mutated copy:** red. In that copy `document.rs:127` is changed to `graph::read(class, &document.path, value)`, run with only this case selected:
```
FAIL [   0.143s] (1/1) metaspec::main adversary_alias_a_rendered_page_draws_the_aliased_part_in_its_block
thread 'adversary_alias_a_rendered_page_draws_the_aliased_part_in_its_block' (2299801) panicked at crates/metaspec/tests/main.rs:1965:9:
assertion `left == right` failed: harbor: the page's graph carries the block the alias names
  left: Some("zz_alias_word")
 right: Some("ledger")
```

**3. Suite runs**
- **The mutated copy, without my case.** Command: `cargo nextest run --workspace -E 'not test(adversary_alias_a_rendered_page)'`. Result: `Summary [5.557s] 182 tests run: 182 passed, 1 skipped`, exit 0. The mutant survives the whole suite.
- **The worktree with my case.** Command: `cargo nextest run --workspace`. Result: `Summary [13.583s] 183 tests run: 183 passed, 0 skipped`, exit 0.

**4. Findings**
- **The wiring is untested**
  - What was measured: `crates/metaspec-docs/src/document.rs:127`, the only call that reaches pages and docs (D47). If it goes back to `graph::read`, all 182 cargo tests still pass. The four `alias_` tests call `graph::read_with_aliases` directly. Only `scripts/acceptance.sh` would notice, and it needs the real packs, which the gate does not run.
  - What reaches it: `metaspec render` and `docs build`, both through `document::build`.
  - Fix: take my test into the unit. NEEDS-CHANGE, introduced, warning.
- **Schema description is out of date**
  - What was measured: `schema/metaspec-pack-check-1.schema.json:97`. The `subject` description lists the subject forms and leaves out `alias:<word>`, which `pack.rs:243` now emits.
  - What reaches it: `pack check --json` on a refused alias.
  - CONFIRMED, introduced, note.
- **Alias words are not in the vocabulary list**
  - What was measured: `crates/metaspec-core/src/words.rs:40`. `metaspec pack words` does not list the keys of `blocks.aliases`, so `scripts/check-vocabulary.sh` would not catch a pack's alias word (one with an underscore) in metaspec code.
  - What reaches it: every gate run of the vocabulary check against a pack with aliases (cloudspec).
  - CONFIRMED, introduced, note.
- **Views that name an alias word**
  - What was measured: `crates/metaspec-core/src/pack.rs:336` and `crates/metaspec-layout/src/lib.rs:254`. If a view's `blocks` or `hubs.blocks` names an alias key, it still gets only the generic `view_unknown_block` warning, with no hint about the alias. Before the alias it matched parts written with that word; after it, it matches none.
  - What reaches it: nothing found. No pack's views name an alias word, and cloudspec's views name no blocks.
  - INFEASIBLE, introduced, note.
- **A key that is a block's full domain name**
  - What was measured: a probe with `aliases: {harbor.quay: berth}` loads with `valid: true`. The key is the domain of the block `quay`, so a part that writes that domain is drawn in `berth`. D43 checks keys only against the short block names (after `strip_prefix`).
  - What reaches it: nothing found. I built this case myself.
  - Not in the findings block or the origin count, because I wrote no test for it and it does not hold the unit.

**5. Attacked and not broken**
- **Other readers of a document's graph:** `graph::read_with_aliases` has one non-test caller. Pages, docs document pages, block pages (`blocks.rs:808`), views, hubs, style and `page.rs` all read that graph. `tool::check` uses the graph only for part and link ids. `pack check` counts no parts.
- **Alias chains:** `{a: b, b: <block>}` is refused, because `b` is not a block.
- **A target written as the full domain:** a target given as the domain with its prefix is refused with `manifest_invalid`.
- **Odd keys:** integer, boolean and `"zz x/<y>"` keys load and match the way `name_of` turns a written word into text. An empty value is refused by the schema. A key equal to its own block is refused.
- **`extends`:** it merges nothing in the code (only `extends_issues`), so no aliases are inherited and nothing differs.
- **Pages of packs without aliases:** with an empty map the reader takes the same path, `skip_serializing_if` keeps the manifest bytes, and the golden page and site tests pass.
- **Acceptance rows on zero parts:** `alias_row` sets the value to `-` when its own count is 0. A missing page counts 0 on the page, which differs from the document's count, so the row fails. The implementor's run shows 10 aliased parts and 3 kept, matching the docs pages (3 "no block of the pack").
- **The four `alias_` tests:** each would fail if the function it tests returned a default value.

**6. Paths written outside the worktree**
- <scratch>/part-block-aliases/adversary/mutant.sh
- <scratch>/part-block-aliases/adversary/mutant.log
- <scratch>/part-block-aliases/adversary/mutant/ (copy of the tree with the mutation, 5.2M)
- <scratch>/part-block-aliases/adversary/mutant-target/ (build directory, 485M)
- <scratch>/part-block-aliases/adversary/mutant-suite.sh
- <scratch>/part-block-aliases/adversary/mutant-suite.log
- <scratch>/part-block-aliases/adversary/probe.sh
- <scratch>/part-block-aliases/adversary/probe/
- <scratch>/part-block-aliases/adversary/head-suite.log

My session lease on the worktree was taken and has been released.

**7. Findings block**

```findings
[
{"file":"crates/metaspec-docs/src/document.rs","line":127,"category":"mutant","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"reverting the one alias site to graph::read leaves all 182 cargo tests green; only the acceptance script with real packs would see it, and adversary_alias_a_rendered_page_draws_the_aliased_part_in_its_block is the case that catches it"},
{"file":"schema/metaspec-pack-check-1.schema.json","line":97,"category":"contract-drift","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"the subject description lists every subject form of an issue but not alias:<word>, which the loader now emits"},
{"file":"crates/metaspec-core/src/words.rs","line":40,"category":"judgement","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"pack words does not list blocks.aliases keys, so check-vocabulary.sh cannot catch an alias word of a pack in metaspec code"},
{"file":"crates/metaspec-core/src/pack.rs","line":336,"category":"judgement","severity":"note","verdict":"INFEASIBLE","origin":"introduced","message":"a view or hub naming an alias key gets only the generic view_unknown_block warning and selects nothing after the alias, where it matched the word before; no pack shown to do this"}
]
```
