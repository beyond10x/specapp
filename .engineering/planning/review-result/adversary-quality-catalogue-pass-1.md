---
format: aep.planning-md/3
id: review-result:adversary-quality-catalogue-pass-1
kind: review-result
status: active
title: aep:adversary, pass 1, story:quality-catalogue
relations:
- reviews: story:quality-catalogue
revision: 1
---
Report of aep:adversary (opus), pass 1 on story:quality-catalogue, as it returned. The coordinator
replaced the host's absolute paths: `<worktree>` is the unit's worktree, `<scratch>` the unit's
scratch directory.

```
unit: story:quality-catalogue, working tree of <worktree> against 0474241
verdict: CONFIRMED (findings at note level only, no blocker; under the adversary rules this role never approves)
cases: executed 58→60, red 1
origin: introduced 3 / pre-existing 0 / undecided 0
wrote-outside-worktree: 4 paths (part 6)
needs-coordinator: the brief's header offers "approve", but my charter (hard rule 5) forbids approving, so I give the strongest finding verdict instead
```

**1. Diff stat**

`git diff --stat` shows only the implementor's two tracked files. It cannot show my edits, because everything under `tests/cases/quality/` is untracked. The paths I touched:
- `crates/metaspec-core/tests/cases/quality/mod.rs`: added one line, `mod catalogue_adversary;`. This is a test file the implementor wrote.
- `crates/metaspec-core/tests/cases/quality/catalogue_adversary.rs`: new, two cases.

Both are test files. I touched no implementation, catalogue or schema file.

**2. Cases added** (`catalogue_adversary.rs`). Both were run alone before the suite.

- `a_row_whose_id_is_only_blanks_is_refused` asserts that a row with `id: "   "`, a unit and a valid direction is refused. It is **red now**. Output of the first run, alone:
  ```
  FAIL [0.017s] cases::quality::catalogue_adversary::a_row_whose_id_is_only_blanks_is_refused
  panicked at crates/metaspec-core/tests/cases/quality/catalogue_adversary.rs:15:23:
  a row with a blank id loaded as [""]
  exit 100
  ```
- `a_row_keeps_every_column_it_writes` asserts literal values for the columns a loaded row keeps: block, measure, summary, how, direction, modes (map form), `gate: Some(true)`, reads and status. It is **green on the tree** (run alone: 1 passed). I ran it against a mutated copy in scratch with two changes at `catalogue.rs:154-155`:
  - `modes` replaced by `BTreeMap::new()`
  - `gate` replaced by `None`

  Result: all 10 of the implementor's cases passed, and this case failed:
  ```
  panicked at .../catalogue_adversary.rs:46:5: assertion `left == right` failed
    left: {}
   right: {"loose": "lower_is_better", "strict": "higher_is_better"}
  Summary 12 tests run: 10 passed, 2 failed
  ```

**3. Suite run** (after both cases existed)
- `cargo nextest run -p metaspec-core`: `Starting 60 tests across 1 binary` … `60 tests run: 59 passed, 1 failed`, exit 100. The one failure is the blank-id case. The whole step took 0.98 s.
- `cargo clippy -p metaspec-core --all-targets -- -D warnings`: exit 0.
- `cargo fmt -p metaspec-core --check`: exit 0.

**4. Findings** (all cover the working tree above)

1. **Blank id loads as the measure `""`.** `crates/metaspec-core/src/quality/catalogue.rs:111` (with `field` at :217-221).
   - What was measured: the schema rejects an empty id (`minLength: 1`, `schema/metaspec-kpis-1.schema.json:21`). But `field()` trims, so an id of spaces passes the schema check and loads as `""`. The red case above shows it.
   - What reaches it: nothing found. The only caller of `parse` is `embedded()`, which reads the committed `kpis/catalogue.yaml`. The id case would catch a blank id there, because it checks the exact set of 40 ids.
   - The story does not demand this refusal. It matters because the module's own doc says the loader refuses "what the runner cannot read".
   - Fix: refuse an empty id after trimming, e.g. with a `measure_without_id` issue.
   - INFEASIBLE, introduced, note.
2. **The suite missed a mutant on `modes` and `gate`.** `catalogue.rs:154-155`.
   - What was measured: with both columns dropped, all 10 unit cases stayed green. Every committed row has `modes: {}` and no `gate`, and no case loads a row that has either.
   - What reaches it: no reader uses these columns yet. The later runner and `story:kpi-quality-catalogue-shape` will read them.
   - My added case now covers it; keep it.
   - CONFIRMED, introduced, note.
3. **The shared schema lets a catalogue row carry page-only fields.** `schema/metaspec-kpis-1.schema.json:46-49`, and the top-level `measure`/`modes` at :11-12.
   - What was measured: because one schema covers both the catalogue and the KPI-page data, `kpis/catalogue.yaml` may carry `layer`, `planned`, `route` and `extra`. The loader ignores them without a word. The story says whether a block is planned "is not written in the catalogue".
   - What reaches it: nothing found. No producer writes these fields into the catalogue.
   - This is judgement only; I wrote no case, because the story does not require a refusal.
   - INFEASIBLE, introduced, note.

**5. What I attacked and could not break**
- **Ids and directions:** all 40 ids are present once each and match both epics, including the 14 mutation classes. The id case reads the committed file through `repo()` and compares exact sets. The 12 lower-is-better directions match the story, and the embedded case checks every row's direction against a literal list.
- **Refusals:** duplicate id, missing or blank unit, missing direction, a direction outside the two words, a wrong or missing `format`, an unknown field, a non-string value and a non-list `kpis` are all refused (by the loader or the schema). Every refusal is reported.
- **No loosening of KPI-page validation:** the base had no `metaspec-kpis-1` schema, so there was nothing to loosen. The page writer (`metaspec-docs/src/catalogue.rs`, `kpis.rs`) only emits shapes the schema allows, and the case checks both fixture pages with a `pages >= 2` guard.
- **Embedding:** `include_str!` is tracked through rustc's dep-info, so a changed catalogue rebuilds the crate. The schemas already use the same pattern. The workspace has `publish = false`, so packaging does not apply. Leaving `build.rs` unchanged differs from the story's Surfaces line but breaks nothing.
- **Repository checks:** `scripts/check-vocabulary.sh` reports "37 word(s) of 2 pack(s), none in the code". `scripts/check-public.sh` reports 0 findings (no deny list set). My own KPI ids under `crates/` will meet the vocabulary check later; `story:spec-quality-self` already plans for that.

**6. Paths written outside the worktree**
- `<scratch>/mutant/` (304 MB: the source copy with the mutant, plus its build output)
- `<scratch>/mutant-run.log`
- `<scratch>/adv-red-1.log`
- `<scratch>/adv-suite.log`

`scripts/check-vocabulary.sh` also recreated its own work directory, `target/check-vocabulary`, inside the worktree.

**7. Findings block**

```findings
- file: crates/metaspec-core/src/quality/catalogue.rs
  line: 111
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: an id of blanks passes the schema's minLength and is trimmed to "", so parse loads a measure with an empty id; only a hand-edited committed catalogue reaches it and the id case would catch that
- file: crates/metaspec-core/src/quality/catalogue.rs
  line: 154
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: dropping the modes and gate columns in the loader leaves all ten unit cases green; the added case a_row_keeps_every_column_it_writes kills that mutant
- file: schema/metaspec-kpis-1.schema.json
  line: 46
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: the shared schema lets kpis/catalogue.yaml carry page-only fields (planned, layer, route, extra) that the loader silently ignores, though the story says planned-ness is not written in the catalogue
```
