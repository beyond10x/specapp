---
format: aep.planning-md/3
id: review-result:adversary-cloudspec-manifest-pass-1
kind: review-result
status: active
title: aep:adversary, pass 1, story:cloudspec-manifest
relations:
- reviews: story:cloudspec-manifest
revision: 1
---
# aep:adversary, pass 1, story:cloudspec-manifest

unit: story:cloudspec-manifest, metaspec commit 1de6d84 plus two test files the adversary added
verdict: INFEASIBLE (two red cases; neither failing state is in a committed file today)
cases: executed 154 -> 160, red 2
origin: introduced 1 / pre-existing 1 / undecided 0

## Cases added (crates/metaspec-core/tests/cases/exclude_edges.rs)

- Red: `exclude_with_a_leading_dot_slash_leaves_the_file_unread_or_is_refused`. The schema and
  `relative_inside` accept `./d/*.side.yaml`, but the exclude is matched against `shown()` paths
  without `./`, so the manifest is accepted and the exclude does nothing.
- Red: `a_link_without_carries_named_by_the_tool_is_found_on_its_link`. The real
  `cloudspec check --json` writes a carries-less link as `a->b:`; metaspec's graph names it
  `a->b:?`, so the finding lands on the document, not the link.
- Green: `exclude_with_double_star_reaches_every_directory`,
  `paths_with_a_leading_dot_slash_still_honour_the_exclude`, `exclude_that_is_no_glob_is_refused`,
  `exclude_leaves_a_link_out_of_the_pack_unread`.

Gate after the cases: 160 tests run, 158 passed, 2 failed (the red cases).

## Findings

- F1 `crates/metaspec-core/src/pack.rs:628`: an exclude written with `./` is accepted and excludes
  nothing; nothing warns about an exclude that matches no file. No manifest writes `./` today.
  INFEASIBLE, introduced, warning.
- F2 `crates/metaspec-core/src/adapter.rs:186`: a subject `from->to:` (empty carries) does not
  match the graph's `from->to:?`; the same mapping is in the orgspec and agenticspec manifests and
  unchanged from base. No committed cloudspec architecture has a carries-less link that breaks a
  rule. INFEASIBLE, pre-existing, warning.
- F3 `crates/metaspec-core/src/pack.rs:637`: when exclude removes every matched file,
  `class_empty` still says the paths match no file. CONFIRMED, introduced, note.
- F4 `scripts/check-vocabulary.sh:103`: the ESS-model exemption (every dotless key of the
  fixture packs' compiled models, 62 words) also hides cloudspec's block and layer word
  `workloads`, which has no ESS-model use in the code today; `actors` and `identity` are
  legitimate ESS-model reads. CONFIRMED, introduced, note.

## Attacked and not broken

`**` reaches nested directories; an invalid glob is refused (`path_invalid`); an exclude leaving
the pack is refused (`path_leaves_pack`); a symlink out of the pack named by exclude is left
unread; `./` on `paths` keeps the exclude; the two cloudspec classes do not overlap; case and
dot-files are handled alike by paths and exclude; real `cloudspec check --json`: exit 0/1, the
`error` entries, per-rule `kpis` equal to the violation counts for every file, findings that name
no part go to the document, `api->nowhere:http` is a link; no acceptance row compares a value with
itself; schema, spec, code and CHANGELOG agree apart from F1.

```findings
- file: crates/metaspec-core/src/pack.rs
  line: 628
  category: boundary
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "an exclude glob written with a leading ./ passes the schema and relative_inside but is matched against shown() paths without ./, so the manifest is accepted and the file it names is read (cases::exclude_edges::exclude_with_a_leading_dot_slash_leaves_the_file_unread_or_is_refused)"
- file: crates/metaspec-core/src/adapter.rs
  line: 186
  category: contract-drift
  severity: warning
  verdict: INFEASIBLE
  origin: pre-existing
  message: "a tool that writes a carries-less link as from->to: (real cloudspec output) does not match the graph's from->to:? id, so the finding moves to the document instead of the link (cases::exclude_edges::a_link_without_carries_named_by_the_tool_is_found_on_its_link)"
- file: crates/metaspec-core/src/pack.rs
  line: 637
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "when exclude removes every matched file, class_empty still says the paths match no file"
- file: scripts/check-vocabulary.sh
  line: 103
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "the ESS-model exemption also hides the real block and layer word workloads, which has no ESS-model use in the code today"
```
