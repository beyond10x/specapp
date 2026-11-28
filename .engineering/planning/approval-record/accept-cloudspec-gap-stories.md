---
format: aep.planning-md/3
id: approval-record:accept-cloudspec-gap-stories
kind: approval-record
status: draft
title: The coordinator accepted 3 stories of epic:cloudspec-pack that close a consumer's gaps
relations:
- decides: story:cloudspec-warnings
- decides: story:part-block-aliases
- decides: story:extends-inherits
revision: 1
---
# The coordinator accepted three stories of epic:cloudspec-pack that close a consumer's gaps

On 2026-11-28 a consumer drew its private architectures with metaspec 29ca1f7 through a pack that
`extends: cloudspec` and reported five gaps. Two need no new story: the tool is found on PATH by a
bare name today, and the width of a large architecture is the work of story:hub-badges and
story:collapsed-regions. The other three became stories, and their outcomes were added to the
epic's Outcome:

- story:cloudspec-warnings: the page shows cloudspec's check warnings as findings that do not gate.
- story:part-block-aliases: a manifest alias map draws a part whose block word names an entity of
  a cloudspec block in that block (D41).
- story:extends-inherits: a pack that extends another inherits its blocks and tool from the base
  pack's directory given at run time (D42).

The operator session delegated story acceptance to this repository's coordinator on 2026-11-28
("decide yourself from now on and report it as decided").

The plan-critic panel approved the set: review-result:gaps-scope-pass-3,
review-result:gaps-design-pass-3, review-result:gaps-parallel-pass-3 and
review-result:gaps-acceptance-pass-4. The earlier passes are recorded as
review-result:gaps-<lane>-pass-<n>.

Decisions the coordinator took on the critics' findings: D43 (an alias key that is a block of the
pack refuses the manifest), D44 (a base pack that itself extends is refused), D45 (no new Rust
test for warnings; the acceptance rows show it), D46 (order: cloudspec-warnings, then
part-block-aliases, then the four view stories, then extends-inherits, then the search chain, so
the consumer's gaps land first), D47 (the alias is applied once, when the graph of a document is
read), D50 (one check per acceptance bullet), D51 (`graph::read` keeps its signature;
`graph::read_with_aliases` is called from the document reader only), D52 (cloudspec-warnings does
not depend on story:hub-badges). An aliased block word applies to a candidate of the search too,
because a candidate is read through the same document reader.

Accepted, walked draft -> proposed -> active by the coordinator: story:cloudspec-warnings,
story:part-block-aliases, story:extends-inherits.
