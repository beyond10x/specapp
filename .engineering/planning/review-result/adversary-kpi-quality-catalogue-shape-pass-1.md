---
format: aep.planning-md/3
id: review-result:adversary-kpi-quality-catalogue-shape-pass-1
kind: review-result
status: active
title: aep:adversary, pass 1, story:kpi-quality-catalogue-shape
relations:
- reviews: story:kpi-quality-catalogue-shape
revision: 1
---
Report of aep:adversary (opus), pass 1 on story:kpi-quality-catalogue-shape, as it returned.
`<scratch>` is the unit's scratch directory on the host.

```
unit: story:kpi-quality-catalogue-shape, working tree of wave2-kpi-quality-catalogue-shape against ebbb3b7 (uncommitted)
verdict: NEEDS-CHANGE
cases: executed 97→100, red 3
origin: introduced 2, pre-existing 0, undecided 0
wrote-outside-worktree: 1 path (<scratch>/probe/)
needs-coordinator: none
```

The acceptance itself passes, and I could not break any of the three goldens. One input a pack can really write makes `metaspec quality kpis` refuse its whole report: a KPI row with `id: ""`.

## 1. What I touched

```
crates/metaspec-core/tests/cases/quality/mod.rs   +1   (mod kpis_adversary;)
crates/metaspec/tests/main.rs                     +36  (one adversary_ case)
crates/metaspec-core/tests/cases/quality/kpis_adversary.rs   new, untracked
```
`git diff --stat` against HEAD also lists the implementor's own uncommitted changes. My part is only the lines above, and every one is a test file. I changed nothing in the implementation, schema, goldens or scripts.

## 2. Cases added (each run alone first, filter `test(adversary_)`, before the suite ran)

**a) `kpis_adversary.rs::adversary_a_kpi_row_with_an_empty_id_still_gives_a_report_that_follows_the_schema`** (red)
- Asserts: a pack whose catalogue holds one row `{"id": "", "block": <first block>}` gives a report that passes `report::validate`.
- Red output:
```
assertion `left == right` failed: the report of a pack whose loader accepted every row follows metaspec-quality/1
  left: Err([Issue { code: "quality_report_invalid", subject: "report", message: "at /measurements/4/behind/0/name: \"\" is shorter than 1 character" }])
 right: Ok(())
```

**b) `crates/metaspec/tests/main.rs::adversary_quality_kpis_prints_the_report_of_a_pack_with_a_kpi_row_whose_id_is_empty`** (red)
- Builds a copy of the first fixture pack and adds one row `{id: ""}` to its catalogue.
- Asserts: `pack check` exits 0 (this passes), then `quality kpis --json` exits 0.
- Red output:
```
assertion `left == right` failed: quality kpis on a pack that loads: metaspec: quality_report_invalid report: at /measurements/4/behind/1/name: "" is shorter than 1 character
  left: Some(1)
 right: Some(0)
```

**c) `kpis_adversary.rs::adversary_the_schema_refuses_names_behind_a_gap_reading`** (red)
- Asserts: a report with one entry in `behind` under a gap reading is refused by the schema.
- Red output:
```
panicked at crates/metaspec-core/tests/cases/quality/kpis_adversary.rs:56:5:
a gap reading with a name behind it passes the schema
```

All three new tests ran in under 0.15 s.

## 3. Suite run, after the cases existed

`cargo nextest run -p metaspec-core -p metaspec --no-fail-fast`
```
Summary [   0.977s] 100 tests run: 97 passed, 3 failed, 0 skipped
    FAIL metaspec::main adversary_quality_kpis_prints_the_report_of_a_pack_with_a_kpi_row_whose_id_is_empty
    FAIL metaspec-core::main cases::quality::kpis_adversary::adversary_a_kpi_row_with_an_empty_id_still_gives_a_report_that_follows_the_schema
    FAIL metaspec-core::main cases::quality::kpis_adversary::adversary_the_schema_refuses_names_behind_a_gap_reading
error: test run failed
```
- Exit status: 100.
- The 97 other tests pass, so the implementor's 97/97 holds.
- `cargo fmt -p metaspec-core -p metaspec --check` exits 0.
- The acceptance filter `test(layout_) | test(golden_)` selects none of my cases.

## 4. Findings

**F1: an empty KPI id makes the whole report refused.** `crates/metaspec-core/src/quality/kpis/catalogue_measures.rs:97` (`id_of`)
- **Measured** (cases a and b):
  - `id_of` falls back to `#<index>` only when `id` is not a string. It passes `""` through as a name behind `completeness.fields`.
  - The schema requires `name` to have at least 1 character, the CLI checks its own report, and it exits 1.
  - The pack loads without errors: `pack check` exits 0, and the loader warns only when `id` is not a string.
- **What reaches it:** a pack catalogue with `id: ""`. That is a file a pack author writes; the test builds nothing beyond that file. The docs reader (`crates/metaspec-docs/src/kpis.rs:89-91`) already leaves blank-id rows out, so the two readers disagree.
- **Fix to name, not applied:** in `id_of`, also use `#<index>` for an id that is empty or blank after trimming.
- Origin: introduced (the file is new in this unit).

**F2: the schema does not enforce its own sentence about `behind`.** `schema/metaspec-quality-1.schema.json:46`
- **Measured** (case c): the description of `behind` says "empty for a gap reading", and so does the doc comment in `report.rs`. The schema does not enforce it.
- **What reaches it:** nothing found. metaspec is the only producer and it always writes an empty list for a gap. So this is INFEASIBLE today; it matters only for a hand-edited report given to the later ratchet or page stories.
- **Fix to name:** a draft 2020-12 `if`/`then` on the measurement: if `reading.kind` is `gap`, then `behind` has `maxItems: 0`.
- Origin: introduced.

## 5. What I attacked and could not break
- **Goldens:** I recomputed all 6 numbers and `behind` lists by hand from `fixtures/packs/{harbor,orchard}` and the story's definitions. Harbor gives 3/5, 3/4 and 1 (crane_lift_rate); orchard gives 3/6, 2/3 and 0. Each matches.
- **Manifest field names:** harbor's `tides` and orchard's `seasons` reach the "direction per mode" rule through `kpis.fields.modes`.
- **No `kpis` section:** a manifest without one gives a gap reading for each of the three measures, and 10 gaps in all.
- **Empty and broken catalogues:**
  - an empty catalogue gives `completeness.fields` a gap and `coverage.blocks` the value 0;
  - an unreadable catalogue, one with no list `kpis`, and a missing pack directory each exit 1;
  - an unknown pack name exits 2.
- **Gaps:** each gap reading appears once in `gaps`, with kind `measure` and the measure id as its feature.
- **Digest:** `source_digest` is the same for two copies of a pack in different directories.
- **Registry and layout:**
  - all 40 ids are mapped, and a measured stub, a missing entry and an entry without a row are each refused, naming the row.
  - A duplicate in `REGISTRY` would pass `check_with`, but the layout test (length 40 plus set equality) catches it in the one registry that exists.
  - Every stub can be filled by changing only its own file: `super::measure` is reachable from `quality/spec.rs`, and the ratchet flags in `quality/mod.rs` match `story:quality-ratchet`'s CLI.
- **CLI stubs:** `render --quality`, `quality spec` and `quality ratchet` each exit 2 with "not built yet".
- **Acceptance scripts:**
  - `acceptance.sh` handles an empty or missing `acceptance.d`: the glob does not match and `[ -f ]` skips it.
  - I sourced the row file with `bash --posix` against both fixture packs and every row came out equal.
  - The optional Taskfile includes parse on go-task 3.54.0.
- **Cost and words:** no new expensive work runs per test, and the new code holds no pack word and no private path.

## 6. Paths written outside the worktree
- `<scratch>/probe/`: two pack copies `a/harbor` and `b/harbor`, `nokpis.yaml`, `state/`, `work/`, and `rows-harness.sh`, a probe script that is not committed.

```findings
- file: crates/metaspec-core/src/quality/kpis/catalogue_measures.rs
  line: 97
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: a KPI row with id "" loads without error, but id_of passes "" as a name behind completeness.fields, so the report fails its own schema and quality kpis exits 1 for the whole pack instead of falling back to #<index>
- file: schema/metaspec-quality-1.schema.json
  line: 46
  category: contract-drift
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: the schema says behind is empty for a gap reading but accepts a gap reading with names behind it; metaspec, the only producer, never writes one
```
