---
format: aep.planning-md/3
id: approval-record:accept-cloudspec-pack-stories
kind: approval-record
status: draft
title: The operator session accepted 3 stories of epic:cloudspec-pack on the owner's D3 = A; the coordinator added their dependency
relations:
- decides: story:cloudspec-manifest
- decides: story:view-selectors
- decides: story:collapsed-regions
- decides: story:contain-in-and-per
revision: 2
---
# The operator session accepted the stories of epic:cloudspec-pack that serve the owner's D3 = A

The owner decided on 2026-11-28, through the operator session: the architecture page of a cloud
stack moves to metaspec once metaspec has a cloudspec manifest (D3 = A). On that decision the
operator session wrote to this repository's coordinator, the same day:

"accept cloudspec-manifest, view-selectors and collapsed-regions now; they run after the quality
wave in flight (cloudspec-manifest stays blocked until cloudspec moves its documents to
architectures/). story:cloudspec-runs stays draft until cloudspec has simulate/search. Record it
as accepted by [the operator session] on [the owner's] D3 = A."

Accepted, walked draft -> proposed -> active by the coordinator on that answer:

- story:cloudspec-manifest (stays blocked by dependency-blocker:cloudspec-architectures-dir)
- story:view-selectors
- story:collapsed-regions
- story:contain-in-and-per: not named in the answer; accepted by the coordinator with it because
  story:cloudspec-manifest depends on it and no wave takes a story whose dependency is a draft.

Stays draft: story:cloudspec-runs (blocked by dependency-blocker:cloudspec-run-verbs).
