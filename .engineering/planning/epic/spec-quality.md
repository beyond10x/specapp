---
format: aep.planning-md/3
id: epic:spec-quality
kind: epic
status: draft
title: 'Spec quality: measures of a specification pack'
summary: Validity, layering, ESS completeness, conformance, mutation score, open markers, refusal power and adoption level of every pack, as KPIs of metaspec.
relations:
- decomposes: initiative:measure-quality
revision: 5
---
# Spec quality: measures of a specification pack

## Outcome

`metaspec quality spec --pack <name> --pack-dir <dir>` reports, for every pack and for metaspec's
own `spec/`, the measures below as numbers with what stands behind each, and a page per pack shows
them (the page is an addition, `initiative:measure-quality`, "Additions decided in planning").
Same measures for every pack; what a pack cannot supply is a listed gap. Each value is held by
the ratchet against a baseline.

## Not in this initiative

- The table over all packs on a landing page. metaspec has no page over several packs yet; the
  table is deferred to the single metaspec site of the rename step, which is not built yet. No
  story here builds it, and nothing here closes on it.

## Measures (fixed; each is one row of metaspec's own catalogue, id in backticks)

- `validity.refusals`, `validity.warnings`: `ess specify validate` refusals and warnings
  (`implied_relation` and the like).
- `layering`: dependency edges that point to a higher layer or form a cycle inside a layer.
- `ess_completeness.unscenarioed`, `ess_completeness.unanswered`, `ess_completeness.outside`: the
  `completeness` counts of `ess specify validate --format json`.
- `conformance.synthesized`, `conformance.passed`, `conformance.known_failing`,
  `conformance.skipped`: scenarios on the interpreted reference target.
- `mutation.score`, and `mutation.<class>` for each of the 14 classes `ess verify conform mutate
  --class` names (from-drop, transition-to, guard-boundary, sets-retarget, guard-negate,
  guard-connective, error-swap, emit-drop, order-flip, sets-drop, precedence-swap, emit-swap,
  grant-drop, grant-add): share of mutants the suite kills, with survivors listed. Read from the
  `slow` lane's result for the current digest; without one, a gap naming `task slow:mutation`.
- `honesty`: open `UNMAPPED:` markers in the specification sources.
- `refusal_power`: share of the tool's rules that have a broken example document which fails
  exactly that rule.
- `document_health`: share of the pack's example documents that are valid; findings per rule.
- `adoption_level`: L0 to L5 as defined in `design/metaspec.md`, "Levels of adoption", with the
  metaspec features the pack cannot use yet as gaps of kind `verb` (the per-pack gap list the
  top-level rule requires).
- `drift`: generated files of the pack that differ from a fresh generation, read through a new
  adapter verb of the manifest that names the pack's own generation check
  (`decision-blocker:pack-drift-source`, option A); a pack without the verb has the listed gap
  "drift". Not part of the owner's ask: the planning session added drift on 2026-11-28 under the
  owner's standing rule "decide the reversible option", recorded in
  `decision-blocker:pack-drift-source` and in `initiative:measure-quality`, "Additions decided in
  planning".

## Acceptance

`task acceptance -- <orgspec-dir> <agenticspec-dir>` exits 0 with rows showing, for harbor,
orchard, orgspec and agenticspec, `metaspec quality spec --json` printing a value or a gap naming
the missing input for each measure above (no other measure and none missing), every value held
by the ratchet against its committed `quality/baselines/<pack>/<measure>.json`, and a
`quality.html` naming all of them.

## Children

`story:spec-quality-static`, `story:spec-quality-layering`, `story:spec-quality-conformance`,
`story:spec-quality-mutation`, `story:spec-quality-refusal`, `story:spec-quality-drift`,
`story:spec-quality-page`, `story:spec-quality-self`. The measures of metaspec's own `spec/` are
checked by the acceptance of `story:spec-quality-self` (`task quality:self`), not by this epic's
acceptance statement.

## Facts this rests on

- orgspec: 892 conformance scenarios, 867 passed, 25 known failing, named in
  `blocks/known-failing.txt` (orgspec CHANGELOG, first public commit).
- `ess verify conform mutate` and the completeness report are described in the `ess:hardening`
  and `ess:specifying` skills of the installed ess plugin; the 14 classes are the values of
  `--class` in `ess verify conform mutate --help` (2026-11-28).
- metaspec's own `spec/` is being written now (the ESS meta model, owner's pick 2026-11-28).

## Open

- Cost: mutation runs synthesize many suites. They run in a `slow` lane once per digest of the
  pack's sources, never in `task check`; the 120 s step budget holds.
