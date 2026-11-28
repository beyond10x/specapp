---
format: aep.planning-md/3
id: story:nested-readable
kind: story
status: draft
title: Layout nested stays readable at about 50 parts and 100 links
summary: No link label over box text and edges routed around boxes in nested; found by a consumer at 46 parts.
revision: 1
---
# Layout nested stays readable at about 50 parts and 100 links

## Context

Found 2026-11-28 by a consumer drawing an agenticspec architecture of 46 parts and 101 links in
layout `nested` (no hubs): edges run through boxes, and link labels lie over part names (at least
four names partly covered in one screen).

## Scope (to be refined)

- In `nested`, no link label is drawn over the text of a box, and edges are routed around boxes
  they do not start or end at.
- A fixture of about that size (invented parts, a fixture pack) holds the measure: the number of
  label-over-text overlaps and of edges crossing a box, both 0.

## Not in this story

- Layout `rows`, which routes in bands.

## Acceptance

To be written when the story is refined; it counts overlaps and crossings in the fixture scene.
