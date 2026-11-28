---
format: aep.planning-md/3
id: review-result:adversary-cloudspec-warnings-pass-1
kind: review-result
status: active
title: aep:adversary, pass 1, story:cloudspec-warnings
relations:
- reviews: story:cloudspec-warnings
revision: 1
---
# aep:adversary, pass 1, story:cloudspec-warnings

unit: story:cloudspec-warnings, commit b555838 on branch cloudspec-warnings (diffed against 5d87235), worktree <worktree>
verdict: CONFIRMED (two notes; no blocker or warning, and no red case left in the repository)
cases: executed 5→5 script tests (the unit's own test has 2 cases), red 0
origin: introduced 2 / pre-existing 0 / undecided 0
wrote-outside-worktree: 2 paths (the cloudspec checkout's `target/`, and the adversary scratch directory)
needs-coordinator: none

1. **`git --no-pager diff --stat`**: empty. I changed no file in the worktree. The only write inside it was the cargo shim refreshing the ignored binary `target/debug/metaspec`.

2. **Cases added**: none in the repository. Both findings are notes, so I did not leave a red script test that would block `check:scripts`.
   - **Filter probe** (in scratch): `adversary/probe-filter.sh`. It uses a stub `ess` that prints `warning[implied_relation]: the field route is the dispatcher of a job, ...`. The copied script printed `check-spec-conform: sample v1 — 1 file(s), valid` and went on to synthesis. So the real warning was hidden.
   - **Journeys rows**: these are checked by reading the rows against a real render of the cloudspec pages (in scratch). The journeys page has 3 findings, all with gate false and all drawn, and is valid. That matches `cloudspec check --json` at 6259dce.

3. **Suite** (`slot --timeout 3m task check:scripts`, exit 0): `check:scripts: 5 script test(s) passed`; `check-spec-conform.test: 2 cases passed`. Run alone first, the unit's test also passed.

4. **Findings**
   - **`scripts/acceptance.sh:377`**: note, CONFIRMED, introduced. `journeys:valid` checks that the page's `valid` equals cloudspec's `valid`. It does not check that the value is `true`. `journeys:warnings` also passes 0 = 0 when there are no warnings. If cloudspec gated the rule or rejected the document, all three journeys rows would still be green. The acceptance says the document "stays valid".
     - What reaches it: only a later cloudspec checkout. At 6259dce the rows show 3 = 3, 0 = 0, true = true.
     - Fix: compare `journeys:valid` with `true`, or require at least one warning.
   - **`scripts/check-spec-conform.sh:100`**: note, INFEASIBLE, introduced. `grep -v 'is the dispatcher'` drops any line holding that phrase, so a real warning holding it would be hidden (shown by the probe).
     - What reaches it: nothing found. ess builds its compiler warnings as `warning[code]: ...` from identifiers. The only line with "is the dispatcher" is the dispatcher's note (`ess` source, `toolchain.rs:385`).
     - Fix: anchor the pattern to `^note: ess [^ ]* is the dispatcher; `, or set `ESS_TOOLCHAIN_QUIET`.

5. **Attacked and could not break**
   - **`--strict`**: cloudspec moves the warnings into `violations` and empties `warnings` (`arch/mod.rs:499-504`), so nothing is counted twice. The pack does not pass `--strict`. If it did, `journeys:warnings` would go red, because the acceptance's own call runs without it.
   - **Warnings on links, or on subjects that are not parts**: every warning at 6259dce is on a part of the document. A link or unknown subject would be mapped the same way as a violation. The `named` rows agree on journeys, l4-media, node-pools and broken/single_point_of_failure, including the two warnings on the same part.
   - **Old cloudspec without a `warnings` key**: `warnings` and `journeys.yaml` came in the same commit (1c7f331). An older checkout has no page for that file, so the row shows `-` against `0` and fails.
   - **Cloudspec refusing the document**: `parts-links` fails, because the answer has no `parts`/`links`.
   - **Dispatcher's "installing ess" note**: the path in that note does not count as a warning, because it never reaches `validate.txt`. `digest()` runs `ess --version 2>/dev/null` first, which installs the pinned release.
   - **Multi-line ess warnings**: the first line always holds `warning[`.
   - **The `warnings of <name>` loop**: it reads only `$work/*.json`. The journeys files are `.answer` and `.page`, so there is no clash.
   - **`set --` in the journeys block**: no later code uses the overwritten arguments.
   - **The unit's test**: it can fail. Against the base (no filter), its first case goes red.

6. **Paths written outside the worktree**
   - `<scratch>/cloudspec-warnings/cloudspec/target/`: the cloudspec binary was missing, so I built it.
   - `<scratch>/cloudspec-warnings/adversary/`: `cloudspec-build.log`, `state/`, `pages/`, `render.log`, `journeys.page.json`, `probe-filter.sh`, `probe-filter/`.

```findings
[
{"file":"scripts/acceptance.sh","line":377,"category":"acceptance","severity":"note","verdict":"CONFIRMED","origin":"introduced","message":"journeys:valid compares the page's valid with cloudspec's instead of asserting true, and journeys:warnings passes 0 = 0, so the rows stay green if cloudspec gated or refused the document the acceptance says stays valid"},
{"file":"scripts/check-spec-conform.sh","line":100,"category":"boundary","severity":"note","verdict":"INFEASIBLE","origin":"introduced","message":"the unanchored filter grep -v 'is the dispatcher' would hide a real ess warning holding that phrase (shown with a stub); ess builds warning texts from identifiers, so nothing found produces one"}
]
```
