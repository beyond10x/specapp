---
format: aep.planning-md/3
id: review-result:adversary-kpi-quality-page-pass-1
kind: review-result
status: active
title: aep:adversary, pass 1, story:kpi-quality-page
relations:
- reviews: story:kpi-quality-page
revision: 1
---
Report of aep:adversary (opus), pass 1 on story:kpi-quality-page, as it returned. `<scratch>` is
the unit's scratch directory on the host.

```
unit: story:kpi-quality-page, working tree on top of base 96f198f (uncommitted)
verdict: NEEDS-CHANGE
cases: executed 28→31, red 2
origin: introduced 4, pre-existing 0, undecided 0
wrote-outside-worktree: `<scratch>/adv/` (harbor-copy/, harbor.json, orchard.json, suite.log, mutant/)
needs-coordinator: none
```

**1. Diff stat**

```
 crates/metaspec/src/page.rs         |  16 +-
 crates/metaspec/src/quality_page.rs | 262 +++++++++++++++++++-
 crates/metaspec/src/render.rs       |  16 +-
 crates/metaspec/tests/main.rs       | 459 ++++++++++++++++++++++++++++++++++--
 4 files changed, 719 insertions(+), 34 deletions(-)
```

The whole change is uncommitted, so this stat includes the implementor's diff. My only change is three tests appended at the end of `crates/metaspec/tests/main.rs` (+129 lines). I did not touch any other file.

**2. Cases added** (in `crates/metaspec/tests/main.rs`; each was run alone before the suite)

- `adversary_a_refused_report_leaves_no_quality_page_of_an_earlier_render` (RED). It renders with the pack's own recorded report, then renders into the same `--out` with the other pack's report. It expects exit 1 and no `quality.html`. Red output:
  ```
  panicked at crates/metaspec/tests/main.rs:1401:5:
  a refused report leaves no quality page in the directory, yet the earlier one is still there
  ```
- `adversary_render_without_a_report_leaves_no_quality_page_of_an_earlier_render` (RED). It renders with a report, then renders without one into the same `--out`. It expects no `quality.html`. Red output:
  ```
  panicked at crates/metaspec/tests/main.rs:1430:5:
  no report given, yet the quality page of the earlier render is still there
  ```
- `adversary_the_quality_page_escapes_what_a_report_carries` (GREEN). The report carries `</script><script>alert(1)</script><!--` in four places: a name behind a value, a gap reading, a gap summary and `source_digest`. It also carries `a&b"c'd` as a name. The test checks:
  - every one of them is escaped in the HTML;
  - `</script>` appears exactly once;
  - the embedded JSON reads back equal to the report.

  The code is correct here; this test guards a mutant the suite misses (finding 4). In a scratch copy I removed `.replace('<', "\\u003c")` (quality_page.rs:166) and ran the whole suite. All 28 original tests passed and this case failed: `panicked at crates/metaspec/tests/main.rs:1054:5: assertion left == right failed: harbor: no script that runs, left: 5`.

**3. Suite run, after the cases existed**

`cargo nextest run -p metaspec --no-fail-fast` exited with status 100:
```
Summary [0.884s] 31 tests run: 29 passed, 2 failed, 0 skipped
FAIL adversary_a_refused_report_leaves_no_quality_page_of_an_earlier_render
FAIL adversary_render_without_a_report_leaves_no_quality_page_of_an_earlier_render
```
Every test took under 1 s; the new ones each took under 0.1 s. `cargo fmt -p metaspec --check`, `cargo clippy -p metaspec --all-targets -- -D warnings` and `scripts/check-vocabulary.sh` all exit 0.

**4. Findings**

1. **A refused report leaves the old page behind** (`crates/metaspec/src/quality_page.rs:55`; warning, NEEDS-CHANGE, introduced)
   - *What was measured:* when a report is refused, `write` returns before touching `out`. A `quality.html` from an earlier render stays, with the old numbers, while the index is rewritten without a link to it. The implementor claims "no page"; that holds only for a fresh directory. The red case is at tests/main.rs:1401.
   - *What reaches it:* rendering repeatedly into the same `--out`, which is the normal workflow.
   - *Fix:* remove `out/quality.html` before the reports are read.
2. **Rendering without a report also leaves the old page** (`crates/metaspec/src/main.rs:400`; note, CONFIRMED, introduced)
   - *What was measured:* `if quality.is_empty() { return SUCCESS }` never removes a `quality.html` written by an earlier render. The red case is at tests/main.rs:1430.
   - *What reaches it:* the same workflow as finding 1.
   - *Context:* render has never removed stale document pages either. The same one-line removal fixes both findings.
3. **Blessing a new fixture pack now panics** (`crates/metaspec/tests/main.rs:395`; note, CONFIRMED, introduced)
   - *What was measured:* bless now calls `files(&expected)`, which unwraps `read_dir` and panics when `fixtures/expected/<pack>/pages` does not exist yet. Before, `remove_dir_all` ignored that case. In the same way, :1118 writes the quality golden without `create_dir_all`.
   - *What reaches it:* adding a third fixture pack and running `METASPEC_BLESS=1`. Nothing in the tree does that today.
   - *How shown:* by reading the code; no red case.
4. **Removing the `<` escape in the embedded JSON passes the old suite** (`crates/metaspec/src/quality_page.rs:166`; note, CONFIRMED, introduced)
   - *What was measured:* the code is correct, but none of the unit's tests can catch this mutant.
   - *Status:* now covered by the green escape case in part 2.

**5. Attacked and not broken**

- **Every measure shown:** each measurement shows its value or gap, unit, direction and the names behind it, grouped by kind. The golden pages and the implementor's tests show this.
- **Golden determinism:**
  - numbers come from serde_json's own formatting (for example `0.6666666666666666`), in a fixed field order;
  - there is no timestamp and no host path;
  - the `regenerate` command rebuilds both recorded reports byte for byte (`cmp`);
  - `source_digest` does not change when the pack is copied elsewhere.
- **The weakened existing test:** it still compares every other page byte for byte. On a fresh directory it also asserts there is no `quality.html` and no link. It hides no regression of the other pages.
- **Another pack's report:** matched by the `pack` name only (quality_page.rs:103); the digest is not compared. I did not raise this: comparing the digest would break the recorded-report design.
- **A specification report:** accepted and shown after the KPI section, whatever the flag order.
- **A missing or unknown measure:** accepted and shown as given, as the story's Scope says. `kpi-quality-catalogue-shape.sh:40` checks the report against the catalogue, one row per measure.
- **`index.html` written twice:** both writes use the same `page::index` on the same data and differ only in the link line, which a test checks. The link is written only after `quality.html` exists.
- **`scripts/acceptance.d/kpi-quality-page.sh`:** passes `sh -n`. It checks exit 0 and the page file, that the page's measures are exactly the report's, and the index link. I did not run it, because it needs the real pack checkouts.

**6. Paths written outside the worktree**

- `<scratch>/adv/harbor-copy/` (a copy of the fixture pack, used for the digest probe)
- `<scratch>/adv/harbor.json`, `<scratch>/adv/orchard.json` (the regenerated reports)
- `<scratch>/adv/suite.log`
- `<scratch>/adv/mutant/` (a copy of the tree's files with the line-166 mutation; I deleted its `target/`)

The worktree lease `adversary-kpi-quality-page-p1` is released.

**7. Findings block**

```findings
- file: crates/metaspec/src/quality_page.rs
  line: 55
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: a refused report returns before out/quality.html is removed, so an earlier render's page with its old numbers stays beside an index that no longer links it (red case tests/main.rs:1401)
- file: crates/metaspec/src/main.rs
  line: 400
  category: boundary
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: render without --quality returns early and leaves a quality.html of an earlier render in the out directory (red case tests/main.rs:1430)
- file: crates/metaspec/tests/main.rs
  line: 395
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: bless now walks fixtures/expected/<pack>/pages with an unwrapped read_dir and panics for a newly added fixture pack, where remove_dir_all before ignored the missing directory
- file: crates/metaspec/src/quality_page.rs
  line: 166
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: dropping the '<' escape of the embedded report passed all 28 original tests; only the added escape case catches it
```
