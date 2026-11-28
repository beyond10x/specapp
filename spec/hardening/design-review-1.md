Technique 8 (design review) on metaspec `spec/`: 25 findings, none a blocker. 13 are warnings: 9 are rules ESS could already declare (row guards, same-row invariants) and 4 are nouns or relations the design and quality epics need. The rest are notes.

Planted defect: on a copy of the spec, `SaveDraft/outcome/not-writable` (the refusal the design states at metaspec.md:339-340) was deleted; a design-to-spec checklist of 19 design statements reported metaspec.md:340 MISSING on the copy; the original is green on that line.

```findings
- file: spec/domains/documents.yaml
  line: 420
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: Document.pack is defined as the pack of the document's class, but with extends a private pack adds documents to a public pack's class (metaspec.md:361-363), so a private draft would carry the public pack and escape the private-pack consent rule.
- file: spec/domains/agents.yaml
  line: 104
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: The agent's ApplyPatch, DryRunPatch, CheckRevision and StartRun, and RecordToolCall, never check the agent channel, so an MCP agent or a session left open after TurnOffChannel works on a private pack's draft against metaspec.md:365-366.
- file: spec/domains/agents.yaml
  line: 161
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: TurnOnChannel creates a new channel row on every call with no guard for an existing channel of the pack, against D18 "a channel per pack" (metaspec.md:666).
- file: spec/domains/agents.yaml
  line: 233
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: StartSession has no outcome refusing a Saved draft and SaveDraft does not end open sessions, though the agent works on "the open draft" (metaspec.md:319-320).
- file: spec/domains/packs.yaml
  line: 250
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: Pack has no directory field, so Trust relates to no pack, TrustPack takes any caller-supplied digest, and "a changed tool section asks again" (metaspec.md:251-252, 546-549) cannot be declared.
- file: spec/domains/packs.yaml
  line: 238
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: No invariant ties visibility private to manifest given, though metaspec holds only public packs' manifests and a private pack's is given by path (metaspec.md:582-584, 357).
- file: spec/domains/packs.yaml
  line: 235
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: No invariant says a pack with extends is private, though extends is how a private pack names the public pack it builds on (metaspec.md:361-362).
- file: spec/domains/runs.yaml
  line: 539
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: StartRun has no no-revision outcome and does not check that revision_id belongs to a draft of document_id, which D12 (metaspec.md:646-651) relies on.
- file: spec/domains/checks.yaml
  line: 202
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: RecordCheck records any revision_id without checking that it exists or belongs to a draft of document_id (D5, metaspec.md:629-631).
- file: spec/domains/checks.yaml
  line: 297
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: RecordFinding accepts a second finding with the same n in one check result, though a finding's number is its position in the tool's answer (metaspec.md:542).
- file: spec/domains/checks.yaml
  line: 297
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: RecordFinding accepts findings on a check result whose answer is not ran, though findings are only what the tool answered (metaspec.md:101-102).
- file: spec/domains/checks.yaml
  line: 85
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: The invariant declares only that ran implies valid, not that valid is absent when the tool did not answer, as the field summary at checks.yaml:81 states.
- file: spec/domains/quality.yaml
  line: 395
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: SetBaseline creates a second baseline for the same pack and measure with no guard, against D22 "one per measure and pack" (metaspec.md:676).
- file: spec/domains/quality.yaml
  line: 406
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: SetBaseline takes direction from input instead of the measure's direction, so a baseline can ratchet the wrong way (D19, metaspec.md:669; kpi-quality.md "a worse value fails").
- file: spec/domains/checks.yaml
  line: 79
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: CheckResult holds no counts per rule (metaspec-findings/1, metaspec.md:214) and no row holds a KPI value per document, so the KPI-quality measures alive, discrimination, gate fidelity and redundancy have no data in the model.
- file: spec/domains/blocks.yaml
  line: 87
  category: spec-gap
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: The blocks domain has no entities, lifecycles, events or commands of a block (metaspec-model/1, metaspec.md:205-207, 279-281), so grounding, coverage.lifecycles and the block page cannot be computed from the model.
- file: spec/domains/quality.yaml
  line: 139
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: Measurement has no breakdown key and Evidence no value, so "findings per rule" and "mutation score per mutant class" (spec-quality.md) cannot be held without pack vocabulary in metaspec's measures.
- file: spec/domains/quality.yaml
  line: 58
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: A gap reading is a bare String with no relation to the pack's Gap rows, though the measure gap is meant to be part of the per-pack gap list (spec-quality.md adoption level; metaspec.md:585-586).
- file: spec/domains/quality.yaml
  line: 325
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: RecordMeasurement accepts a measure whose question differs from the report's question (kpi-quality.md and spec-quality.md report separately).
- file: spec/domains/documents.yaml
  line: 427
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: Document has no example or broken-example role, which gate fidelity (kpi-quality.md) and refusal power (spec-quality.md) need to tell a broken example that wrongly passes from a valid one.
- file: spec/domains/packs.yaml
  line: 327
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: D2 "one gap per pack and feature" (metaspec.md:621) is not declared, as Gap's identity is a Uuid with no uniqueness over pack, kind and feature.
- file: spec/domains/documents.yaml
  line: 491
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: A Part may hold both inside and per, though contain has exactly one key (metaspec.md:468).
- file: spec/domains/checks.yaml
  line: 71
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: CheckResult carries no digest of the checked document and no order, so "the documents with their check status" and "the check runs when the file changes" (metaspec.md:310, 313) have no basis, and Draft.base_digest is generated because Document has no digest field (metaspec.md:338).
- file: spec/domains/drafts.yaml
  line: 323
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: Revision.number is generated, so the spec's own "numbered from 1 in the order they were taken" (drafts.yaml:175) is not declared and a target numbering every revision 1 passes.
- file: spec/domains/packs.yaml
  line: 418
  category: spec-gap
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: Spec-only wording issues: UntrustPack has no design line (the design names only `metaspec trust`, metaspec.md:251); StartRun's NotTrusted error also answers "the tool lacks the verb"; the Agent may StartRun with search though the agent's tools give it only simulate (metaspec.md:334); and OpenDraft on a records/1 document is unclear against "generic edits, valid for every parts-links/1 document" (metaspec.md:327).
```
