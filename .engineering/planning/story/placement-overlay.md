---
format: aep.planning-md/3
id: story:placement-overlay
kind: story
status: draft
title: 'A placement view: the parts of one pack''s document inside the units of another''s'
summary: Draws a document's parts inside the units of a translated document of another pack, from a placement file; found by a consumer.
relations:
- depends_on: story:band-wrap
scope:
- confidence: inferred
  path: crates/metaspec-core/src/manifest.rs
- confidence: inferred
  path: crates/metaspec-docs/src/documents.rs
- confidence: inferred
  path: schema/metaspec-pack-1.schema.json
revision: 3
---
# A placement view: the parts of one pack's document inside the units of another's

## Context

Found 2026-11-28 by a consumer that keeps agenticspec architectures in its own repository and
translates them to cloudspec architectures with `cloudspec translate agenticspec`. The translation
writes a coverage file that names, per agenticspec part, the cloudspec unit it is packed in. The
cloudspec page shows a unit as one box: the parts packed in it are not drawn, and every part outside
cloudspec's blocks sits in "block the pack does not have". The consumer asked for "a render
overlay/option which would render the thing within final placements".

## Scope (to be refined)

- A view of a document of pack B drawn with the parts of a document of pack A placed inside B's
  units and scopes, read from a placement file that maps A's parts to B's parts.
- The placement file stays the translating pack's own format; the manifest maps it: which pointer
  gives the list of entries, which field the A part (`object`, prefix `architecture.document.parts.`)
  and which the B part (`to`, prefix `parts.`). An A part whose B part has another id is placed in
  that B part (a unit); one whose B part has its own id is drawn as itself; entries with no B part
  are skipped (the coordinator's D58, 2026-11-28, on the consumer's pick).
- The view is a third view on the page of the B document, "Placement of <A document>", and the A
  document's page links to it (D58).
- Links of A between two parts placed in the same B part are drawn inside that unit; links that
  cross units are drawn between the units. To settle when refined: whether such a link is drawn
  once per A link or rolled up with a count.
- Known shape (cloudspec-coverage/1 at cloudspec 73bb2b4): `steps[].entries[]` with `object`,
  `status`, `to`, `note`; no typed "placed in" field, so placement is read by comparing ids.

## Not in this story

- Translating documents between packs: that stays in the packs' tools.

## Acceptance

To be written when the story is refined.
