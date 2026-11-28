---
format: aep.planning-md/3
id: approval-record:accept-groups-stories
kind: approval-record
status: draft
title: The coordinator accepted 2 stories of epic:cloudspec-pack that close a consumer's gaps G6 to G8
relations:
- decides: story:block-groups
- decides: story:band-wrap
revision: 1
---
# The coordinator accepted 2 stories of epic:cloudspec-pack that close a consumer's gaps G6 to G8

On 2026-11-28 the consumer that draws its private architectures through a pack that
`extends: cloudspec` measured story:collapsed-regions on metaspec main 9dbf813 and reported three
more gaps: parts of one block that sit in no region are drawn one per box (G6, G7), and drill-down
views run one row per band (G8). They became two stories, and their outcomes were added to the
epic's Outcome:

- story:block-groups: parts in no region, selected by block and optionally split by one field,
  collapse into one box per block and value, with a count and the member names, through the same
  roll-up and drill-down as collapsed regions.
- story:band-wrap: `wrap: <n>` on a `rows` view cuts a band into rows of at most n boxes, and the
  generated views take it from their view.

Story acceptance is delegated to this repository's coordinator by the operator session
(2026-11-28, "decide yourself from now on and report it as decided").

The plan-critic panel approved the set: review-result:groups-scope-pass-2,
review-result:groups-parallel-pass-3, review-result:groups-acceptance-pass-5 and
review-result:groups-design-pass-5. The earlier passes are recorded as
review-result:groups-<lane>-pass-<n>.

Decisions the coordinator took on the critics' findings: D61 (order: story:extends-inherits, then
story:block-groups, then story:band-wrap, then each of story:view-selectors, story:matrix-view,
story:focus-pages, story:search-score, story:badge-width and story:placement-overlay, so the
consumer's gaps land first), D62 (a group's drill-down view carries its member part ids), D63
(`hubs.parts` and `hubs.blocks` exclude members before the roll-up; an `in_degree` hub is judged
after it), D64 (a group box is a synthetic part of the folded graph, so the row and nested layouts
and the hub check read it unchanged), D65 (an empty value of the split field counts as no value).
A group whose box id is `edges` stays plain and is named with `view_region_edges`, as a region of
that name is.
