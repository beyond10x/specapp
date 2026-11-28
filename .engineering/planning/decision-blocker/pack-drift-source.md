---
format: aep.planning-md/3
id: decision-blocker:pack-drift-source
kind: decision-blocker
status: cleared
title: Nobody has decided how metaspec learns which files a pack generates and how to regenerate them
summary: No manifest verb names a generation check, so the drift measure has no source.
relations:
- blocks: epic:spec-quality
revision: 6
transitions:
- {from: "open", to: "cleared", at: "2026-11-28T01:28:19Z", actor: "human:operator", revision: 4}
---
# Nobody has decided how metaspec learns which files a pack generates and how to regenerate them

## The question

`epic:spec-quality` names the measure **drift**: generated files of the pack that differ from a
fresh generation, where the pack commits generated output with a check. metaspec knows a pack
only through its manifest and runs the pack's own tool only through an adapter of a declared verb.

## Why it is open

- The verbs of the manifest are `graph`, `check`, `simulate`, `search`, `apply`
  (`spec/domains/packs.yaml`,
  type `metaspec.packs.Verb`; `design/metaspec.md` on branch `invert-pack-knowledge`, "The tool
  adapter"). None names a generation or its check.
- Packs differ: agenticspec has `agenticspec-docs --check`; orgspec has `scripts/blocks-digest.sh`.
  Their outputs are not JSON, and the adapter reads JSON.
- Any shape metaspec picks is a new part of the manifest format and of the ESS model.

## Options

- **A (default)**: a new adapter verb (for example `regenerate`) that runs the pack's own check of
  its generated files, read like `check`: the files that differ, or a gap for a pack without one.
- **B**: the manifest lists the generated files and the generating command; metaspec regenerates
  into a scratch directory and compares bytes.
- **C**: drop drift from `epic:spec-quality`.

## What it stops

The drift measure: no story and no row in metaspec's catalogue until this is decided.

## Decision

A, taken on 2026-11-28 under the owner's standing rule (decide the reversible option, report it): a new adapter verb in the manifest names a pack's own generation check, mapped like `check`; a pack without it gets the listed gap "drift". The rule that metaspec reads a pack only through its manifest adapter settles the form.
