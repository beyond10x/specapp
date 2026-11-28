---
format: aep.planning-md/3
id: dependency-blocker:metaspec-studio
kind: dependency-blocker
status: open
title: metaspec has no studio yet; metaspec serve does not exist (waves 5 and 6 of the build order)
summary: The studio half of the search result page waits on a studio.
relations:
- blocks: epic:shared-search
revision: 1
---
metaspec has no studio yet: `metaspec serve` does not exist, there is no `crates/metaspec-server`
and no `web/studio/`. The studio is waves 5 (read-only studio) and 6 (drafts) of the build order in
`design/metaspec.md`, and the studio nouns are modelled in `spec/` but no `ess-ui/1` model is written.

`epic:shared-search` promises a page of a search result "in the site and the studio". The site half
is `story:search-result-page`. The studio half cannot be built before the studio: it reads the same
component (`ResultSections`) and the same `metaspec-result/1`, and needs the Runs screen of the studio.

This blocker clears when a studio exists that can show a document page. The story for the studio page
is drafted then, with a `depends_on` edge to `story:search-result-page` and to `story:search-design`.
