---
format: aep.planning-md/3
id: story:ux-lane-budget
kind: story
status: implemented
title: The UX lane's gates step ends under 60 s
summary: axe once per page and theme, in the page; the second viewport checks the layout
scope:
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: scripts/ux.sh
- confidence: cited
  path: web/site/e2e/gates.spec.ts
- confidence: cited
  path: web/site/playwright.config.ts
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T08:34:20Z", actor: "human:operator", revision: 3, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-11-28T08:34:20Z", actor: "human:operator", revision: 4, decided_on: {"recorded":{"test_result":1}}}
- {from: "active", to: "implemented", at: "2026-11-28T08:34:20Z", actor: "human:operator", revision: 5, decided_on: {"recorded":{"test_result":1}}}
---
# The UX lane's gates step ends under 60 s

## Context

A test-budget defect (repository rule "Tests stay fast": the step ends within 120 s, no test
over 60 s; a step over budget is fixed before the next feature, and a raised timeout, more shards
or workers, or a longer wait is not a fix).

Measured on a507c6b: `task ux` (`scripts/ux.sh`, `web/site/e2e/gates.spec.ts`,
`web/site/playwright.config.ts`) builds the fixture site once and runs 32 Playwright tests (`/`
and 31 pack pages), each page in both themes at 1440x900 and 1280x800 with axe-core at each of
the four. A green run's gates step took 100 s; two runs of three went over the 120 s step budget
on a loaded host (load 40 to 55 on 20 cores) and were cut off. 17 tests ran over 5 s, the slowest
`orchard: blocks/2-hands/picker` 12.2 s.

The cost was axe: four calls per page, and each call in the default mode of
`@axe-core/playwright` also opens a blank page, injects axe-core (650 kB) into it and runs
`axe.finishRun` there. One call took about 1 s on the loaded host, in the page itself
(`setLegacyMode`) about 0.5 s, and about 0.45 s with only violations reported in full.

The fix keeps every gate and removes the repeated work:

- axe-core runs once per page and theme, at 1440x900. Both viewports are wider than the theme's
  desktop breakpoint (996px) and render the same elements with the same styles; a probe that ran
  axe at both viewports on every page and theme found no violation of any impact at either.
- axe runs in the page (`setLegacyMode`; the site has no frame) with `resultTypes:
  ['violations']`, so passes and incomplete results are not serialized.
- 1280x800 gates the layout: the navbar title is visible and is metaspec, one navbar item per
  pack, every component the page names is visible, no sideways scroll.
- The site is still built once per run (`scripts/ux.sh`), served once (`webServer`), and each
  test loads its page once per theme in one browser context.

Checks per page, before: per theme and viewport (4): navbar title and items, components visible,
no sideways scroll, axe (no serious or critical violation). After: per theme at 1440x900 (2):
navbar title visible and its text, navbar items, components visible, no sideways scroll, axe; per
theme at 1280x800 (2): the same without axe.

Measured after, on the same host: gates step 30 s (load 30.5 before, 26.2 after) and 48 s (load
26.2 before, 36.8 after); an earlier run without `resultTypes` 53 s at load 34 to 47. No test
over 5 s; slowest 4.2 s (`orchard: blocks/index`). A planted paragraph in dark grey (`#3a3a3a`)
on `orchard: blocks/2-hands/picker` failed the lane with `color-contrast (serious): .planted` at
`dark 1440x900` (light passed), exit non-zero.

## Scope

- `web/site/e2e/gates.spec.ts`: axe once per theme at the first viewport, legacy mode, violations
  only; navbar title visible at both viewports.
- `scripts/ux.sh`, `web/site/playwright.config.ts`, `Taskfile.yml` (`ux` description),
  `AGENTS.md`, `CHANGELOG.md`: the description of what the lane checks.

## Acceptance

`slot task ux` prints `ux: gates passed in N s` with N under 60 on this host, and with a
paragraph styled `color: #3a3a3a` planted in
`fixtures/expected/site/orchard/docs/blocks/2-hands/picker.mdx` the same command exits non-zero
with `color-contrast (serious)` at `orchard blocks/2-hands/picker dark 1440x900`.
