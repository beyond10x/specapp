---
format: aep.planning-md/3
id: review-result:adversary-spec-quality-static-pass-1
kind: review-result
status: active
title: aep:adversary, pass 1, story:spec-quality-static
relations:
- reviews: story:spec-quality-static
revision: 1
---
Report of aep:adversary (opus), pass 1 on story:spec-quality-static, as it returned. `<scratch>` is
the unit's scratch directory on the host.

unit: story:spec-quality-static (working tree of `wave3-spec-quality-static` against 96f198f)
verdict: NEEDS-CHANGE
cases: executed 126→128, red 2
origin: introduced 4, pre-existing 0, undecided 0
wrote-outside-worktree: 1 directory, `<scratch>/adv/` (part 6)
needs-coordinator: choose the fix for finding 1: run `ess` in the blocks directory with `--path .`, or set `ESS_TOOLCHAIN` to the pack's pin. Either way, `scripts/acceptance.d/spec-quality-static.sh:74` needs the same change.

**1. Diff stat**

```
 .../src/quality/spec/static_measures.rs            | 327 +++++++++-
 crates/metaspec-core/tests/cases/quality/mod.rs    |   1 +
 crates/metaspec-core/tests/cases/quality/spec.rs   | 677 ++++++++++++++++++++-
 crates/metaspec/src/quality/spec.rs                |  86 ++-
 crates/metaspec/tests/main.rs                      | 137 ++++-
 scripts/check-vocabulary.sh                        |  26 +-
 6 files changed, 1230 insertions(+), 24 deletions(-)
?? crates/metaspec-core/tests/cases/quality/spec_adversary.rs   (mine, untracked)
```

Only two lines are mine: `quality/mod.rs` (+1, `mod spec_adversary;`) and the new file `spec_adversary.rs`. Every other line was already in the tree I was given. I changed no implementation file.

**2. Cases added** (`crates/metaspec-core/tests/cases/quality/spec_adversary.rs`). Both are red now. This is the run of these two cases alone, before any suite run:

- `adversary_an_ess_that_exits_1_without_json_names_its_stderr_in_the_gap` (line 57). A stand-in `ess` writes a reason to stderr and exits 1 with nothing on stdout. The case asserts that the gap names that reason.
  ```
  panicked at crates/metaspec-core/tests/cases/quality/spec_adversary.rs:68:33:
  the gap names what ess said on stderr: ess specify validate gave no answer: ess specify validate printed no JSON: EOF while parsing a value at line 1 column 0
  ```
- `adversary_a_pack_whose_blocks_pin_another_ess_release_is_still_measured` (line 90). A stand-in follows the pin rule of `ess` 0.99.11 (`toolchain::unfollowed_pin`): it refuses when the `ess-inputs.yaml` of `--path` is below the working directory and pins another release. It answers when `ESS_TOOLCHAIN` is set, or when a pin is at or above its working directory. The case asserts that `validity.refusals` and `validity.warnings` are values on harbor.
  ```
  panicked at crates/metaspec-core/tests/cases/quality/spec_adversary.rs:139:37:
  validity.refusals is a gap on a pack that loads: ess specify validate gave no answer: ess specify validate printed no JSON: EOF while parsing a value at line 1 column 0
  ```
  I checked that the stand-in goes green under both fixes: run in `blocks/` with `--path .` exits 0, and `ESS_TOOLCHAIN=0.99.2` exits 0.

**3. Suite run** (after the cases existed)

```
$ cargo nextest run --workspace --locked --no-fail-fast
    Starting 128 tests across 3 binaries
     Summary [   2.755s] 128 tests run: 126 passed, 2 failed, 0 skipped
        FAIL cases::quality::spec_adversary::adversary_an_ess_that_exits_1_without_json_names_its_stderr_in_the_gap
        FAIL cases::quality::spec_adversary::adversary_a_pack_whose_blocks_pin_another_ess_release_is_still_measured
exit 100
$ cargo nextest run --workspace --locked --no-fail-fast -E "not test(/spec_adversary::/)"
     Summary [   1.990s] 126 tests run: 126 passed, 2 skipped
exit 0
```

The "before" count (126) comes from the second run, with my two cases excluded. No test runs longer than 1 s.

**4. Findings**

1. **`ess_answer.rs:188`** (also `:146` and `acceptance.d/spec-quality-static.sh:74`). Blocker, NEEDS-CHANGE, introduced.
   - **The problem:** `ess` runs in the pack root with `--path blocks`. With `ess` 0.99.11, the release `spec/ess-inputs.yaml` pins, every pack whose `blocks/ess-inputs.yaml` pins a different release is refused: exit 1, empty stdout. Both fixture packs and both real packs pin 0.99.2.
   - **What was measured:** I ran the cached 0.99.11 binary on PATH against `metaspec quality spec` on a scratch copy of harbor. The pack loads and the command exits 0, but all five ess measures are gaps ("printed no JSON: EOF…"). With 0.99.2 on PATH all five are values. `ess` itself says what to do: "`cd blocks` and run ess there, or run it as `ESS_TOOLCHAIN=0.99.2 ess …`". Run in `blocks/`, 0.99.11 hands the run to 0.99.2, and `--version` then prints 0.99.2, so the cache key stays correct.
   - **What reaches it:** any host where the `ess` on PATH is not the pack's pin, including the release this repository pins. The acceptance passed here only because PATH `ess` is 0.99.2. The acceptance row runs `ess` the same way, so under 0.99.11 it gets an empty `validate.json` and fails too.
2. **`ess_answer.rs:189-191`**. Warning, NEEDS-CHANGE, introduced.
   - **The problem:** exit 1 is in the accepted set, so when `ess` exits 1 with no JSON the error is the JSON parser's and `run.stderr` is dropped. The story says a gap names the missing input.
   - **What was measured:** red case 1.
   - **What reaches it:** the 0.99.11 pin refusal above, an input `ess` cannot read, an unknown `--component`.
3. **`ess_answer.rs:33`**, with the timeout branch at `:200-205`. Note, CONFIRMED, introduced.
   - **The problem:** `DEADLINE` is a fixed 600 s and cannot be overridden. The "ran over … and was killed" gap path has no test, and none can be written within the 60 s per-test limit.
   - **What reaches it:** a hanging `ess`. The CLI's `--deadline` does not bound it, because it only applies to the pack's tool.
4. **`scripts/check-vocabulary.sh:86-95`**. Note, INFEASIBLE, introduced.
   - **The problem:** every property name and enum or const value of every `schema/metaspec-*.schema.json` is exempt everywhere in the code. A future schema property that matches a pack word silently drops that word from the check.
   - **What was measured:** 9 words of the real and fixture packs are exempt today: `block` unit, kpi, tool, type, question; `layer` direction, value; `link` message, question. A table of block names made only of these words in `crates/` would pass.
   - **What reaches it:** nothing found. No such table exists, and the exempt words come only from metaspec's own schemas; a pack cannot add to them.

**5. Attacked and not broken**

- **Cache key:** a changed block, a changed `ess --version` or a changed `blocks.path` gives a new key. The key holds no host paths, so packs with identical blocks share an answer, which is correct because the answer depends only on the blocks.
- **Cache file:** a corrupt file falls through to a fresh run. The write goes to a per-process partial file and is renamed into place, so two processes cannot tear it. The directory is under XDG_CACHE_HOME or HOME, never the checkout.
- **Symlinks in blocks:** 0.99.11 refuses them itself, so they cannot leave a stale answer in the cache.
- **Test step and real `ess`:** no test calls the real `ess specify validate`. The CLI test sets `METASPEC_ESS` and `XDG_CACHE_HOME`.
- **Counting against the 0.99.11 source:** in a refusal, `problems` and `diagnostics` describe the same errors (`load.rs`), so counting `problems` first does not double count. The fixture fields match the `ValidationSummary`, `Completeness` and `RefusalReport` structs of 0.99.11.
- **honesty:** equals `grep -ra`: hidden files counted, two markers on one line count once, files outside `blocks.path` not counted, symlinks not followed.
- **adoption_level and verb gaps:** both use `Manifest::level` like `pack check`. Unknown verbs are refused at load (`deny_unknown_fields`).
- **Vocabulary script exits:** still 1 for a real leak and 2 for a broken schema.
- **Hygiene:** the stub is POSIX sh, no Python, no host paths in the goldens or `fixtures/ess`.

**6. Written outside the worktree** (all under `<scratch>/adv/`)
- `harbor/`: a copy of the fixture pack.
- `harbor-real.json`, `harbor-real.err`, `harbor-11.json`, `harbor-11.err`: real `ess` answers.
- `ess-0.99.11-main.rs`: the 0.99.11 source, read with `git show`.
- `bin11/ess`: a symlink to the cached 0.99.11 binary.
- `xdg/`: the metaspec cache of my probe runs.
- `realwords`, `realwords.err`, `own`: the vocabulary overlap.
- `pin-stub`, `run-cases.log`, `run-suite.log`.

The builds ran in slot's mirror trees, not in the worktree's `target/`.

```findings
- file: crates/metaspec-core/src/quality/spec/static_measures/ess_answer.rs
  line: 188
  category: acceptance
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: introduced
  message: ess runs in the pack root with --path blocks, so ess 0.99.11 (the repository's pin) refuses every pack whose blocks pin another release and all five ess measures of orgspec, agenticspec, harbor and orchard become gaps while the pack loads; run ess in the blocks directory with --path . (or set ESS_TOOLCHAIN), and change acceptance.d/spec-quality-static.sh:74 the same way.
- file: crates/metaspec-core/src/quality/spec/static_measures/ess_answer.rs
  line: 189
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: an accepted exit code (1) with empty stdout yields the JSON parser's "EOF while parsing" and drops ess's stderr, so the gap does not name why ess gave no answer.
- file: crates/metaspec-core/src/quality/spec/static_measures/ess_answer.rs
  line: 33
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: DEADLINE is a fixed 600 s with no override, so the timeout-to-gap branch has no test and cannot have one within the 60 s per-test bound.
- file: scripts/check-vocabulary.sh
  line: 86
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: every property name and enum/const value of every metaspec schema is exempt from the search everywhere in the code (9 pack words today, e.g. block unit, layer direction), so a table of such block names would pass and any future schema property widens the exemption silently.
```
