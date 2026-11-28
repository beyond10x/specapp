---
format: aep.planning-md/3
id: decision-blocker:private-pack-baseline
kind: decision-blocker
status: cleared
title: Nobody has decided where the quality baseline of a private pack lives
summary: The ESS spec commits every baseline in metaspec; the design keeps every trace of a private pack out of it.
relations:
- blocks: epic:kpi-quality
revision: 6
transitions:
- {from: "open", to: "cleared", at: "2026-11-28T01:28:19Z", actor: "human:operator", revision: 4}
---
# Nobody has decided where the quality baseline of a private pack lives

## The relation

Baseline → Pack, for a pack whose manifest says `visibility: private`: who owns the file that
holds the baseline, and therefore where it is kept and what removing the pack does to it.

## Why it is open

- The ESS spec says: "One baseline per measure and pack, kept in a file committed in metaspec"
  (`spec/domains/quality.yaml`,
  entity `metaspec.quality.Baseline`, summary).
- `design/metaspec.md`, "Private packs": "metaspec's repository holds no content of any real pack"
  and "Drafts, runs and the cache of a private pack are in the user's state directory, never in
  the metaspec checkout." A private pack is also never named by the public one.
- A private pack's baseline committed in metaspec would put its name and its numbers in a public
  repository. Neither source says which rule wins for a private pack.

## Options

- **A (default)**: a private pack's baseline lives in the user's state directory, keyed by the
  pack name, like its drafts and runs; public and fixture packs keep theirs committed in metaspec.
- **B**: a file the owner of the private pack keeps beside its private manifest, given by path at
  run time like the manifest.
- **C**: a private pack has no ratchet; its reports are printed only.

## What it stops

The ratchet for private packs. `story:quality-ratchet` covers public and fixture packs and writes
no baseline for a private pack.

## Decision

A, taken on 2026-11-28 under the owner's standing rule (decide the reversible option, report it). The design's rule "Drafts, runs and the cache of a private pack are in the user's state directory, never in the metaspec checkout" already decides it: a private pack's baseline lives in the user's state directory, keyed by pack name; public and fixture packs keep theirs committed in metaspec. The ESS spec's Baseline summary gets the same split.
