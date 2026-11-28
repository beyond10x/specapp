---
format: aep.planning-md/3
id: review-result:adversary-spec-quality-static-pass-2
kind: review-result
status: active
title: aep:adversary, pass 2, story:spec-quality-static
relations:
- reviews: story:spec-quality-static
revision: 1
---
Report of aep:adversary (opus), pass 2 of 2 on story:spec-quality-static, as it returned; the coordinator wrote its findings table in part 4 as a list.
`<scratch>` is the unit's scratch directory on the host; two probe file names in part 6 are written with a `v` before the version.

unit: story:spec-quality-static, pass 2, uncommitted working tree on 96f198f in worktree wave3-spec-quality-static
verdict: INFEASIBLE (4 red cases, all notes; no blocker or warning)
cases: executed 120→134, red 4 (120 is the implementor's gate count. The run in part 3 has 130 passing, my 4 deselected by arithmetic only, not re-run.)
origin: introduced 5 / pre-existing 0 / undecided 0
wrote-outside-worktree: 1 path, `<scratch>/adv2/` (contents in part 6)
needs-coordinator: none

**1. Diff stat**

`spec_adversary.rs` is untracked (pass 1 created it), so `git diff --stat` cannot show it. The only file I touched:
```
?? crates/metaspec-core/tests/cases/quality/spec_adversary.rs   143 → 242 lines (+4 cases, +1 helper `report_script`)
```
It is a test file. I changed no implementation file, fixture or golden. I wrote a fifth case (a block file that is a symbolic link should be part of the cache key) and deleted it: it failed for the wrong reason, because ess 0.99.2 refuses symlinked input files ("list a real contained file"). The defect is not reachable.

**2. Cases added** (all in `crates/metaspec-core/tests/cases/quality/spec_adversary.rs`, all red now)

- **:163 `adversary_a_process_ess_leaves_behind_holding_stdout_does_not_outlast_the_deadline`**
  - Stand-in runs `sleep 15 &`, prints a valid report, exits 0. Deadline 1 s. Asserts the run takes under 8 s.
  - Red, alone: `a 1 s deadline bounds the run; it took 15.047692157s`, FAIL [15.053s].
- **:183 `adversary_a_run_killed_at_its_deadline_ends_though_a_process_it_started_left_its_group`**
  - Stand-in runs `setsid sleep 15 &` then `sleep 30`. Deadline 1 s. Asserts a "ran over 1 s" gap and under 8 s.
  - Red, alone: `killed at the 1 s deadline, the run ends; it took 15.055118688s`, FAIL [15.061s].
- **:207 `adversary_the_stub_refuses_a_path_that_is_not_its_blocks_directory`**
  - Runs `fixtures/ess/ess specify validate --path no-such-directory` in the blocks directory. Asserts a non-zero exit.
  - Red, alone: `assertion left != right failed: the stub answered for a --path that does not exist: { "valid": false, "system": "harbor", ... left: Some(0) right: Some(0)`.
- **:237 `adversary_a_deadline_of_metaspec_ess_deadline_is_never_zero`**
  - Asserts `deadline_of("1e-300")` and `deadline_of("0.0000000001")` are above zero.
  - Red, alone: `"1e-300" gives 0ns`.

Two cases take 15 s each while red. When fixed they take about 1 s.

**3. Suite run** (after the cases existed)

`cargo nextest run --workspace --locked` (cargo shim, so through slot), log `<scratch>/adv2/suite.log`:
```
    Starting 134 tests across 3 binaries
        FAIL [   0.012s] ( 96/134) ...spec_adversary::adversary_a_deadline_of_metaspec_ess_deadline_is_never_zero
        FAIL [   0.021s] (101/134) ...spec_adversary::adversary_the_stub_refuses_a_path_that_is_not_its_blocks_directory
        FAIL [  15.108s] (133/134) ...spec_adversary::adversary_a_process_ess_leaves_behind_holding_stdout_does_not_outlast_the_deadline
        FAIL [  15.143s] (134/134) ...spec_adversary::adversary_a_run_killed_at_its_deadline_ends_though_a_process_it_started_left_its_group
     Summary [  16.560s] 134 tests run: 130 passed, 4 failed, 0 skipped
exit 100
```
Pass 1's two cases now pass, so F1 and F2 are fixed.

**4. Findings** (all cover the working tree on 96f198f)

- A1 `crates/metaspec-core/src/quality/spec/static_measures/ess_answer.rs:369`: After ess exits normally, the stdout reader is joined with no time bound. A process ess left in its group keeps the run alive past the deadline the module doc promises. INFEASIBLE / introduced. Reaches it: nothing today. The ess dispatcher uses `exec` (ess `toolchain.rs:428`), and `specify validate` starts no process.
- A2 `ess_answer.rs:360-364`: The deadline kill sends SIGKILL to ess's process group only, then joins the readers with no bound. A process in another group that holds the pipe keeps metaspec waiting. INFEASIBLE / introduced. Reaches it: a later story that sends `ess verify conform run --target process` through `answer`. ess starts that target in its own process group with stderr inherited (ess `target_process.rs:161-170`). ess kills the target on SIGTERM, but SIGKILL cannot be handled. Fix: SIGTERM to the group, a grace period, then SIGKILL, and bound the joins.
- A3 `fixtures/ess/ess:32`: The stub ignores `--path`. If `ess_answer.rs:237` regressed to `--path blocks` (still run in the blocks directory), the stub and pass 1's pin stand-in would still answer, and the suite would stay green. CONFIRMED / introduced. Reaches it: every quality spec test that uses the stub.
- A4 `ess_answer.rs:48-53`: `deadline_of` accepts any positive value, but values below 1 ns become zero, so every run is killed at once. Through the CLI with `METASPEC_ESS_DEADLINE=1e-300`: "ran over 0 s and was killed" in 0.05 s. INFEASIBLE / introduced. Reaches it: only a user who sets an absurd value.
- A5 `crates/metaspec-core/src/pack.rs:57`: Now that ess runs in the blocks directory, ess 0.99.11 delegates to the pinned 0.99.2. If 0.99.2 is not cached, ess downloads it from GitHub. My probe saw a 114 MB download into a fresh XDG cache. The test step loads packs this way (common.rs `compiled` calls `ess_compile`), and AGENTS.md says the test step makes no network request. Before this change, 0.99.11 refused instead. INFEASIBLE / introduced. Reaches it: no setup I know of. CI's gate.yml puts 0.99.2 on PATH, and this host's PATH ess is 0.99.2 with 0.99.2 cached.

**5. Attacked and could not break**

- **Runner:**
  - A same-group child holding stdout past the kill: ended at the 2 s deadline.
  - A child ignoring SIGTERM: killed at 2 s.
  - 8 MB on stdout and 8 MB on stderr: no deadlock, answered in 0.44 s.
  - Non-UTF-8 stderr: printed lossily in the gap.
- **`METASPEC_ESS_DEADLINE` values:** 0, negative, NaN, inf and empty fall back to 600 s. "1e18" works. " 2 " works. "600s" falls back to 600 s without a word.
- **Remembered failures:** a timeout, a non-zero exit or output with no JSON is never written to the disk cache. Only the in-memory scope remembers it, for one CLI run.
- **Blocks paths through the CLI** (ess 0.99.2): nested `spec/my blocks` in a pack directory with a space, and `.`. All seven measures are values.
- **`ess_compile`, old way vs new:** byte-identical JSON under 0.99.2 for harbor, orchard, orgspec and agenticspec. Under 0.99.11 the old way exits 1 for all four and the new way exits 0 with the same bytes as 0.99.2. `ess --version` in the blocks directory prints `ess 0.99.11\ness 0.99.2`, so the cache key names the release that answers.
- **ess 0.99.11 first on PATH:** `metaspec pack check` exits 0 for both fixture packs. `metaspec quality spec` exits 0 for both, and all seven measures are values (0 0 0 0 0 0 3), matching ess's own validate. Nothing still fails.
- **The stub answering for the wrong pack:** every test copy keeps the pack's name, so I found no test that passes on another pack's answer. A nested blocks path would make the stub refuse, not answer wrongly. The weakness is `--path` (A3).
- **Symbolic links:** ess refuses symlinked input files, so a symlink skipped in the cache key cannot serve a stale answer.

**6. Paths written outside the worktree**

All under `<scratch>/adv2/`:
- Probe scripts: `compare-compile.sh`, `probe-v0.99.11.sh`, `probe-paths.sh`, `probe-deadline.sh`, `probe-runner.sh`
- Logs: `cases.log`, `suite.log`, `build.log`, `compare-compile.log`, `probe-v0.99.11.log`
- Output directories: `compile/`, `paths/`, `deadline/`, `runner/`
- `probe11/`, which holds a 114 MB ess 0.99.2 binary under `probe11/xdg/ess/toolchains/0.99.2/` that the ess dispatcher downloaded

Two `sleep` processes left by the red cases end by themselves within 15 s. In the worktree, the build wrote only under `target/`.

```findings
- file: crates/metaspec-core/src/quality/spec/static_measures/ess_answer.rs
  line: 369
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: after ess exits the stdout reader is joined without a bound, so a process ess left in its group holding stdout makes the run outlast the deadline (red case at spec_adversary.rs:163, 15 s against a 1 s deadline).
- file: crates/metaspec-core/src/quality/spec/static_measures/ess_answer.rs
  line: 360
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: the deadline kill is SIGKILL to ess's group only, followed by unbounded joins, so a process in another group, such as the target of ess verify conform run --target process (own group, inherited stderr), keeps metaspec waiting and gets no SIGTERM cleanup from ess (red case at spec_adversary.rs:183).
- file: fixtures/ess/ess
  line: 32
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: the stub answers whatever --path names, so a regression of ess_answer.rs:237 to --path blocks leaves the suite green (red case at spec_adversary.rs:207).
- file: crates/metaspec-core/src/quality/spec/static_measures/ess_answer.rs
  line: 48
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: deadline_of turns a positive value below 1 ns into a zero deadline, against its doc, and every run is then killed at once (red case at spec_adversary.rs:237).
- file: crates/metaspec-core/src/pack.rs
  line: 57
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: run in the blocks directory, ess 0.99.11 delegates to the pinned 0.99.2 and downloads it from GitHub when it is not cached (114 MB seen), so the test step's ess_compile would make a network request with 0.99.11 on PATH, a setup neither CI nor this host has.
```
