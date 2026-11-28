---
format: aep.planning-md/3
id: approval-record:accept-cloudspec-view-stories
kind: approval-record
status: draft
title: The operator session accepted the view stories hub-badges, matrix-view and focus-pages, hub-badges first
relations:
- decides: story:hub-badges
- decides: story:matrix-view
- decides: story:focus-pages
revision: 2
---
# The operator session accepted the view stories V1, V3 and V4 of epic:cloudspec-pack

The consumer of the cloudspec pack asked on 2026-11-28 for hub elision (V1), a matrix view (V3)
and focus on one part (V4); the owner had called the stack page "still like hell", and the
owner's D3 = A moves it to metaspec. The coordinator drafted one story each and asked the operator
session, which answered the same day:

"accept hub-badges, matrix-view and focus-pages; commit with their approval record in wave 3's
PR. Put hub-badges first among the cloudspec-pack stories after the quality wave (it answers
[the owner's] "still like hell" most directly: 209 -> 52 edges)."

Accepted, walked draft -> proposed -> active by the coordinator on that answer:

- story:hub-badges (first among the cloudspec-pack stories after the quality wave)
- story:matrix-view
- story:focus-pages

Corrected the same day by the operator session: story:contain-in-and-per and
story:cloudspec-manifest run first, then hub-badges and the other view stories, because no view
reaches the stack page before the manifest exists; the five view stories depend on
story:cloudspec-manifest.
