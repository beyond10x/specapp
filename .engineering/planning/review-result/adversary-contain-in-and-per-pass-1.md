---
format: aep.planning-md/3
id: review-result:adversary-contain-in-and-per-pass-1
kind: review-result
status: active
title: aep:adversary, pass 1, story:contain-in-and-per
relations:
- reviews: story:contain-in-and-per
revision: 1
---
# aep:adversary, pass 1, story:contain-in-and-per

unit: story:contain-in-and-per, metaspec commit eac7d73 plus 4 test files the adversary added
verdict: CONFIRMED (one red case, but no consumer reaches it, so it is marked INFEASIBLE; two spec findings with no test)
cases: executed 144 -> 149, red 1
origin: introduced 3 / pre-existing 0 / undecided 0

## Cases added

All names start `contain_`, so the story's acceptance filter `test(contain_)` selects them; each
under 0.07 s.

- Red: `metaspec-layout contain_edges::contain_both_same_holder_rows_line_does_not_repeat_the_host_it_is_drawn_in`.
  The part is `{in: node, per: node}`. `rows` draws it inside `node`, and its line still adds
  `· in node`; `nested` leaves that note out (`nested.rs:111-125`). Output:
  `drawn inside node, and its line says it again: "agent · work · in node"`.
- Green: `contain_both_nested_in_container_not_shown_draws_it_in_its_per_and_notes_its_in` (a
  branch no test covered before).
- Green: `contain_both_cycle_through_in_and_per_draws_every_part_once_and_nests_no_box_in_itself`.
- Green: `metaspec-core contain_both_fields_named_in_and_per_read_one_holder_each_and_groups_in_document_order`
  (cloudspec's shape: fields named `in` and `per`, one holder for both words, a part with one of
  the two, a missing `in` holder).
- Green: `contain_empty_object_and_null_values_are_refused` (`{}`, `{ in: ~ }`,
  `{ in: a, per: ~ }`, `{ per: [a] }`; `minProperties: 1` still holds).

Gate after the cases: spec 135 of 135, docs check passed, 149 tests run, 148 passed, 1 failed (the
red case).

## Findings

- `crates/metaspec-layout/src/rows.rs:647`: `child_label` adds ` · in X` whenever the part has an
  `in` holder, even when its box is drawn inside X. No consumer reaches it: cloudspec main's only
  part with both (`node-refresh`) has two different holders. INFEASIBLE, introduced.
- `spec/domains/documents.yaml:112`: `contain: List<Contain{kind, field}>` admits two entries of
  one kind, which the manifest cannot express; no invariant replaced the removed one. CONFIRMED,
  introduced, note.
- `spec/domains/drafts.yaml:60`: `Move` says "the class's `contain` field names the field
  written"; with both keys there are two fields. No code implements `Move` yet. CONFIRMED,
  introduced, note.

## Attacked and not broken

Empty or null `contain` refused; the JSON shape of `Contain` unchanged; group order stable, `in`
before `per`, members in document order; a missing holder skipped, `nested` falls back to `per`;
cycles through `in` and `per` draw no box inside itself; a part with one key in a class that
declares both drawn as before; single-key classes give the base's result, so the pages of orgspec
(`in`) and agenticspec (`per`) do not change; goldens outside lagoon changed only in counts and
lists.

```findings
- file: crates/metaspec-layout/src/rows.rs
  line: 647
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: rows appends "· in X" to a part drawn inside X when its in and per holders are the same part (contain_both_same_holder_rows_line_does_not_repeat_the_host_it_is_drawn_in), unlike nested; no consumer document has such a part.
- file: spec/domains/documents.yaml
  line: 112
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: List<Contain{kind, field}> admits two entries of one kind, which the manifest cannot express, and no invariant replaced the removed one.
- file: spec/domains/drafts.yaml
  line: 60
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: Move writes "the class's contain field", which no longer names one field when a class declares both in and per.
```
