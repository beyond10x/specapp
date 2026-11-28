---
format: aep.planning-md/3
id: review-result:adversary-extends-inherits-pass-1
kind: review-result
status: active
title: aep:adversary, pass 1, story:extends-inherits
relations:
- reviews: story:extends-inherits
revision: 1
---
# aep:adversary, pass 1, story:extends-inherits

unit: story:extends-inherits, commit 80d9b59 in <worktree>, plus my two test files (uncommitted)
verdict: NEEDS-CHANGE
cases: executed 216→219, red 3
origin: introduced 4 / pre-existing 1 / undecided 0
wrote-outside-worktree: 4 paths
needs-coordinator: whether trust of an inherited tool must be tied to the base directory (finding 1). That is a design decision; the design says "Trust belongs to the directory".

**1. Diff stat**
```
 crates/metaspec-core/tests/cases/mod.rs | 1 +
 1 file changed, 1 insertion(+)
?? crates/metaspec-core/tests/cases/inherit_adversary.rs
```
Both are test files. No implementation file was touched.

**2. Cases added** in `<worktree>/crates/metaspec-core/tests/cases/inherit_adversary.rs`. All three are red now. First run alone: `cargo nextest run -p metaspec-core --locked -E 'test(/adversary|own_tool_linked/)' --no-fail-fast`. The filter also picked up 10 adversary cases that were already in the tree; all of them pass.
- `inherit_trust_is_bound_to_the_base_directory`: trust the extending pack with base dir A, then load it with base dir B, a copy of A with a different program at the same `tool.run` path. The check must be `NotTrusted`.
  ```
  panicked at crates/metaspec-core/tests/cases/inherit_adversary.rs:105:5:
  assertion `left == right` failed: the tool of another base directory ran without being trusted; its findings: ["other-code-ran"]
    left: Ran
   right: NotTrusted
  ```
- `inherit_tool_linked_out_of_the_base_pack_is_not_run`: the base's `tool/stub` is replaced by a symlink to a program outside the base. The loader warns `tool_not_started`. The check must not run the program.
  ```
  panicked at crates/metaspec-core/tests/cases/inherit_adversary.rs:148:5:
  assertion `left != right` failed: a program outside the base pack ran; its findings: ["outside-the-base-ran"]
    left: Ran
   right: Ran
  ```
- `own_tool_linked_out_of_its_pack_is_not_run`: the same symlink in a pack's own tool, without `extends`. It shows the defect existed before this change.
  ```
  panicked at crates/metaspec-core/tests/cases/inherit_adversary.rs:182:5:
  assertion `left != right` failed: a program outside the pack ran; its findings: ["outside-the-pack-ran"]
    left: Ran
   right: Ran
  ```

**3. Suite run** (after the cases existed)
- `cargo nextest run --workspace --locked --no-fail-fast`: `Summary [5.229s] 219 tests run: 216 passed, 3 failed, 0 skipped`, exit 100. The 3 failures are exactly the cases above.
- Same command with `-E 'not test(/inherit_adversary/)'`: `216 tests run: 216 passed, 3 skipped`, exit 0.
- `cargo clippy -p metaspec-core --all-targets --locked -- -D warnings` is clean, and `cargo fmt --all --check` passes. Every new case runs in under 0.2 s.

**4. Findings**
1. **Trust of an inherited tool is not tied to the base directory.** `crates/metaspec-core/src/tool.rs:324` checks trust against `pack.root` (the extending pack) and the `tool` section only. Line 337 then runs the program from `pack.tool_root()`, which is whatever `--base-pack` says on that run. `trust` in `crates/metaspec/src/main.rs:460-481` records the same key.
   - Measured: the first case above.
   - Who reaches it: `--base-pack` is given on every `render`, `docs build` and `quality` run, for example a second or fresh checkout of the base. That checkout's code then runs without anyone being asked. For a pack's own tool, a new directory has to be trusted again.
   - Fix to consider: put the resolved base directory into the trust key or the digest, and print it in `trust`'s output.
   - NEEDS-CHANGE, warning, introduced.
2. **A tool path that a symlink carries out of its pack is still run.** `tool::run` (`crates/metaspec-core/src/tool.rs:120-121`) joins `root` and `program` with no containment check. `find_program` only warns.
   - Measured: the second and third cases.
   - It contradicts the story's claim that none of the base's paths leave the base. The new `inherit_path_leaving_base_pack_is_refused` covers only blocks and KPIs, not the tool.
   - The base commit 7108754 has identical `run` code, and the own-pack case is red too.
   - Fix to consider: `check` refuses (state `Failed`) when the resolved program lies outside the tool root.
   - CONFIRMED, warning, pre-existing.
3. **Packs that loaded before are now refused when the base has `kpis` or `tool`.** See `crates/metaspec-core/src/pack.rs:252-268`. Example: a manifest with its own `blocks` and no `tool` that extends a known pack with a tool now gets `base_pack_missing` without `--base-pack`. It loaded before. The schema has no way to say "no tool" or "no KPIs" (`null` is not allowed).
   - Reading only, no test. This follows D42/D56, but nobody chose that consequence explicitly.
   - CONFIRMED, note, introduced.
4. **`extends_chain` also refuses a manifest that inherits nothing.** `crates/metaspec-core/src/pack.rs:243` runs before the `inherited.any()` check at line 257. A manifest that writes all three sections and whose base extends another pack now loads as refused; before, it loaded with no warning.
   - Reading only. The design text allows this reading of D44.
   - CONFIRMED, note, introduced.
5. **`metaspec trust` asks for `--base-pack` even when the tool is the manifest's own.** `crates/metaspec/src/main.rs:460`: a manifest with its own `tool` and inherited `blocks` exits 1 with `base_pack_missing`, although trust only needs the tool. `--base-pack` given to a manifest without `extends` is ignored without a warning (`pack.rs:200`).
   - Reading only.
   - CONFIRMED, note, introduced.

**5. Attacked, could not break**
- Readers of `blocks`/`kpis`/`tool`: docs blocks, block, catalogue and planned pages, `block_file`, `honesty`, the ess answer and its cache key, words, quality catalogue measures, `level`/`verb_gaps`, `document.rs` aliases, `collapse_warnings`, view and family alias warnings. All of them read the filled-in manifest or `blocks_root`/`kpis_root`.
- `..` or an absolute path in the base's `tool.run` or `kpis.path`: refused, and the message names the base pack.
- Symlinked blocks or KPIs out of the base: refused.
- `--base-pack` that is absolute, relative, the extending pack itself, or a file: canonicalised, read inside it, or refused.
- `--extends` naming another pack: `extends_other_pack` refuses when `blocks` is left out.
- Partial inheritance (own blocks with inherited tool or KPIs, own tool or KPIs with inherited blocks and aliases): roots and labels are correct.
- Document handed to an inherited tool: the whole path as one argv element, so spaces are safe; the working directory is the base.
- Pages without `extends`: `source_in` is identical to `source` when the root is the pack's own.
- Schema vs loader: `blocks` is optional in the schema, and the loader refuses it missing without `extends`.
- `--base-pack` on a multi-pack `docs build`: refused.
- A base trusted on its own is not reused for the extending pack. That is keyed by the extending directory, as the design says.

**6. Paths written outside the worktree**
- <scratch>/extends-inherits/ (the directory)
- <scratch>/extends-inherits/alone.log
- <scratch>/extends-inherits/suite.log
- <scratch>/extends-inherits/suite-before.log

The test scratch dirs are under the tree's own `target/` (`CARGO_TARGET_TMPDIR/metaspec-core/adversary-*`).

**7. Findings block**

```findings
[
{"file":"crates/metaspec-core/src/tool.rs","line":324,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"trust of an inherited tool is keyed by the extending pack's directory and the tool section only, so a run with another --base-pack runs that directory's program as trusted"},
{"file":"crates/metaspec-core/src/tool.rs","line":121,"category":"boundary","severity":"warning","verdict":"CONFIRMED","origin":"pre-existing","message":"tool::run starts a tool.run program that a symlink carries out of its pack (base or own) although the loader reports it as no file inside the pack"},
{"file":"crates/metaspec-core/src/pack.rs","line":252,"category":"contract-drift","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"a manifest with own blocks and no kpis or tool whose base has them is now refused without --base-pack, and the schema offers no way to decline inheriting them"},
{"file":"crates/metaspec-core/src/pack.rs","line":243,"category":"judgement","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"extends_chain refuses a manifest that inherits no section, which loaded without a warning before this change"},
{"file":"crates/metaspec/src/main.rs","line":460,"category":"judgement","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"metaspec trust refuses with base_pack_missing a manifest whose tool is its own when it inherits only blocks or kpis"}
]
```
