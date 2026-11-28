---
format: aep.planning-md/3
id: story:badge-width
kind: story
status: draft
title: Badges do not widen the scene
summary: A caller's hub badges stay within the width its box has without them; found after hub-badges.
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:collapsed-regions
- depends_on: story:band-wrap
scope:
- confidence: inferred
  path: crates/metaspec-layout/src/lib.rs
- confidence: inferred
  path: crates/metaspec-layout/src/nested.rs
- confidence: inferred
  path: crates/metaspec-layout/src/rows.rs
revision: 2
---
# Badges do not widen the scene

## Context

Found 2026-11-28 by a consumer after story:hub-badges, on four private architectures of 95 to 121
parts in layout `rows` with `hubs: { in_degree: 6 }`: the view with hubs is about 1,800 px wider
than its `<id>/edges` view (for example 16,077 px against 14,264 px), because the badges are
drawn inside their caller's box and widen it. Hubs cut the edges (242 to 147 on the largest) but
make the page wider, not narrower.

## Scope

- A caller's badges do not widen its box beyond the width the box has without them, in `rows` and
  in `nested`: badges wrap onto further lines, or a caller with more badges than fit shows a count
  badge (`+N`) whose title lists the rest. Which of the two is decided when this story is
  refined.

## Not in this story

- Collapsing regions (story:collapsed-regions), which is the consumer's main need for width.

## Acceptance

To be written when the story is refined; it names a fixture view whose scene with hubs is no
wider than its `<id>/edges` scene.
