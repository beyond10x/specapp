---
format: aep.planning-md/3
id: story:graph-viewport
kind: story
status: draft
title: 'A wide graph on the docs site: fit to width, zoom, pan, and follow one part'
summary: GraphView fits, zooms, pans and dims all but a selected part on the docs site; found by a consumer.
relations:
- depends_on: story:focus-pages
revision: 1
---
# A wide graph on the docs site: fit to width, zoom, pan, and follow one part

## Context

Found 2026-11-28 by a consumer: a graph wider than its frame only scrolls sideways. The pack
renderers' own pages open at 24 to 31 % zoom and were called unreadable by the consumer's owner.
`GraphView` and `GraphSwitch` of `@beyond10x/metaspec-ui` have no zoom, pan or selection; the
static page has no script by design. story:focus-pages draws a page per part with its
neighbourhood (static), which is the no-script form of following one part.

## Scope (to be refined)

- On the docs site: a graph opens fitted to the frame's width, can be zoomed and panned, and a
  selected part keeps its links and its neighbours while the rest dims.
- The static page stays without script.

## Not in this story

- The studio (dependency-blocker:metaspec-studio).

## Acceptance

To be written when the story is refined; a vitest case per behaviour and a Playwright case in the
`task ux` lane.
