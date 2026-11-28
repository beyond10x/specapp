---
format: aep.planning-md/3
id: story:focus-pages
kind: story
status: active
title: 'Focus on one part: its neighbourhood only'
summary: One page per part with its one-hop neighbourhood, linked from the document page.
relations:
- decomposes: epic:cloudspec-pack
- depends_on: story:cloudspec-manifest
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T13:54:28Z", actor: "human:operator", revision: 2}
- {from: "proposed", to: "active", at: "2026-11-28T13:54:29Z", actor: "human:operator", revision: 3}
---
# Focus on one part: its neighbourhood only

## Context

Asked as V4 by the consumer (2026-11-28): select a part and see only its one-hop neighbourhood.
metaspec's static pages run no script, so the selection is a link to a page.

## Scope

- Per document, one focus page per part (`<document>/focus/<part id>`): the part, every part one
  link away (in either direction), and the links between them; drawn with the document's first
  view's layout. Each part on a document page links to its focus page.
- A manifest may turn focus pages off per class (`focus: false`) or limit them to parts with at
  least N links (`focus: {min_links: N}`), so a large document does not multiply pages without
  need.
- The docs site shows the focus page of the part clicked; the studio (wave 5) may later do the
  same live.

## Acceptance

`cargo nextest run -p metaspec -E 'test(focus_)'` passes with cases showing that rendering a
fixture document writes one focus page per part, a focus page holds exactly the part, its one-hop
neighbours and the links among them, each part on the document page links to its focus page, and
`focus: false` writes none; the existing pages of both real packs stay byte for byte except for
the added links, whose goldens are updated in the same change.
