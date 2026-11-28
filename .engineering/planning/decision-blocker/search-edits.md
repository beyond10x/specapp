---
format: aep.planning-md/3
id: decision-blocker:search-edits
kind: decision-blocker
status: cleared
title: Nobody has decided which edits beyond the marked values a search may generate
summary: The epic names generic edits and pack edit verbs as candidate space without saying which; default A is marked places only.
relations:
- blocks: epic:shared-search
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-11-28T15:02:03Z", actor: "human:operator", revision: 3}
---
# Nobody has decided which edits beyond the marked values a search may generate

## The question

`epic:shared-search` lists as its candidate space "the marked values (`{value, vary}`) and the
generic edits a search may use; a pack own edit verbs where its manifest declares them". The
marked values say what to try. Nothing says which generic edit (`add_part`, `remove_part`,
`set_field` on an unmarked field, `add_link`, `remove_link`, `move`, `set_border`) a search may
generate unprompted, with which arguments, or where the arguments of an `add_part` come from.

## Why it is open

- `design/metaspec.md` "Operations" and `spec/domains/drafts.yaml` define the edits as one
  operation set for a user, an agent and a search, and say nothing of which a search picks.
- A search that removes a part or a link changes what the document means; no rule says whether
  that is wanted. An `add_part` has no source of its block or its fields.
- The edit verbs of a pack need an `apply` adapter in the manifest. `metaspec-pack/1` declares
  only `check`; no story builds `apply`.

## Options

- **A (default)**: a search generates only `set_field` edits on marked places. Any other
  change is something a pack author wrote as a marked place (`block` included, for a choice of
  a block), so the author decides what may change. Pack edit verbs come with the `apply`
  adapter, a later epic.
- **B**: also the removal of a part or a link (`remove_part`, `remove_link`), each as a
  neighbour, scored like any other candidate.
- **C**: also edits an author lists in the manifest as allowed for a class.

## What it stops

Neighbours that are not a change of a marked value: no story is drafted for them. Under A
nothing is stopped, and the stories that read marked places are complete.

## Answer

Option A, decided on 2026-11-28 (D6 = A, relayed by the operator session): a search generates only set_field edits on marked places ({value, vary}, a block choice included). A pack's own edit verbs come with an apply adapter in a later epic.
