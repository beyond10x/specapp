Technique 1 (mutation audit) on metaspec `spec/`, second run: 173 mutants over 14 classes, 138 killed, 1 survived, 4 equivalent, 2 unwitnessed, 28 stillborn, 0 inconclusive; score 138/139 (killed over killed and survived). 195 sites were found and not mutated or are unavailable. The authored scenario `spec/scenarios/ratchet-equal-lower-is-better.yaml` now runs inside the audit and kills the guard-boundary survivor of audit 1 (`domains/quality.yaml:534`). The one survivor is the TrustPack `tool_digest` mutant that audit 1 counted as unwitnessed: with an authored scenario in the run, ess no longer counts an unchanged suite as unwitnessed, so it is scored as a survivor. 5 findings, none a blocker.

Command, from inside `spec/`, where `spec/ess-inputs.yaml` pins ess 0.99.11 (the dispatcher on `PATH` delegates to it), one run per class:

```sh
ess verify conform mutate --path . --target interpreted --class <class> \
  --scenarios scenarios/ratchet-equal-lower-is-better.yaml --report-out <class>-report.json
```

No `--known-failing` declaration: since ess 0.99.11, `spec/known-failing.txt` is empty (the AcceptVariant synthesis defect behind its one entry is fixed), and the baseline passes every scenario. ess 0.99.9 refused `--scenarios` beside `--known-failing`, so audit 1 could not run the authored scenario. Specification digest `d2a59ce84230a2b35653c5ae480aae797d168736dc7baa10ebe98efcac5e556a` (the specification is unchanged since audit 1). Baseline: 135 scenarios (134 synthesized, 1 authored), 1 synthesis refusal, all 135 passed.

Per class:

| class | mutants | killed | survived | equivalent | unwitnessed | stillborn | not mutated or unavailable |
|---|---|---|---|---|---|---|---|
| error-swap | 76 | 75 | 0 | 0 | 1 | 0 | 0 |
| emit-drop | 29 | 1 | 0 | 0 | 0 | 28 | 0 |
| guard-negate | 25 | 24 | 0 | 0 | 1 | 0 | 0 |
| sets-retarget | 23 | 23 | 0 | 0 | 0 | 0 | 43 |
| guard-boundary | 8 | 8 | 0 | 0 | 0 | 0 | 0 |
| guard-connective | 6 | 6 | 0 | 0 | 0 | 0 | 0 |
| precedence-swap | 4 | 0 | 0 | 4 | 0 | 0 | 0 |
| sets-drop | 2 | 1 | 1 | 0 | 0 | 0 | 85 |
| emit-swap | 0 | 0 | 0 | 0 | 0 | 0 | 29 |
| grant-drop | 0 | 0 | 0 | 0 | 0 | 0 | 30 |
| transition-to | 0 | 0 | 0 | 0 | 0 | 0 | 8 |
| from-drop, order-flip, grant-add | 0 | 0 | 0 | 0 | 0 | 0 | 0 (no site, ESS-MUTATE-003) |
| total | 173 | 138 | 1 | 4 | 2 | 28 | 195 |

Change from audit 1: guard-boundary 7 killed and 1 survived became 8 killed; sets-drop 1 killed and 1 unwitnessed became 1 killed and 1 survived. Every other class has the same counts. Not mutated or unavailable is unchanged from audit 1: emit-swap `no_compatible_event_alternative` (29), sets-drop creating branches and `Optional` fields (85), sets-retarget no sibling input or admissible literal (43), grant-drop `command_not_served` (30), transition-to `no_admissible_state` (8). The 28 stillborn emit-drop mutants drop the only event of an outcome (ESS-COMMAND-007).

The survivor of audit 1 is killed. `guard-boundary/metaspec.quality.Ratchet/worse-higher/when_subject/0` (`domains/quality.yaml:534`, `value < input.value` to `value <= input.value`) is killed by `metaspec.quality/authored/an-equal-value-of-a-lower-is-better-measure-is-taken`, the authored scenario, and by no synthesized scenario. The same scenario also kills `guard-connective/metaspec.quality.Ratchet/worse-higher/when_subject/0`, beside the synthesized `metaspec.quality.Ratchet/outcome/worse-higher`.

The survivor. `sets-drop/metaspec.packs.TrustPack/retrusted/tool_digest` (`domains/packs.yaml:486`): without the write every scored scenario still passes, so a retrust that keeps the old digest passes. Audit 1 counted it unwitnessed, because the mutant's synthesized suite is the baseline's. ess counts that case as unwitnessed only when no authored scenario ran (`mutate --help`: "unless authored scenarios ran"), so this run scores it as a survivor. It is the same gap: no synthesized scenario retrusts a directory and reads its `tool_digest`, because the refusals that guard the retrust are external (`domains/packs.yaml:451-454`, ESS-SYNTH-004). An authored scenario that trusts a directory, retrusts it with another digest and reads the Trust row's `tool_digest` would kill it.

The unwitnessed mutants. `guard-negate/metaspec.runs.StartRun/other-document/when_related` (`domains/runs.yaml:226`) and `error-swap/metaspec.runs.StartRun/other-document` (`domains/runs.yaml:227`): ess 0.99.11 still refuses to synthesize `metaspec.runs.StartRun/outcome/other-document` (ESS-SYNTH-003, D27 in `design/metaspec.md`), so no scenario reaches the outcome.

The equivalent mutants (ESS-MUTATE-005) are the same four precedence swaps as in audit 1: `RecordCheck` `ran-without-valid` and `valid-without-ran`, and `EndRun` `finished`/`killed` and `killed`/`refused`, where no input satisfies both guards; and `Ratchet` `worse-lower` and `worse-higher`, which both refuse with `Regressed` and the same payload and change nothing else.

```findings
- file: spec/domains/quality.yaml
  line: 534
  category: missing-scenario
  severity: note
  verdict: CONFIRMED
  origin: pre-existing
  message: The guard-boundary survivor of audit 1 (Ratchet worse-higher `value < input.value` to `value <= input.value`) is killed by the authored scenario spec/scenarios/ratchet-equal-lower-is-better.yaml, which mutate now runs with --scenarios; the finding of audit 1 is closed.
- file: spec/domains/packs.yaml
  line: 486
  category: missing-scenario
  severity: note
  verdict: CONFIRMED
  origin: pre-existing
  message: The sets-drop mutant of TrustPack retrusted's `tool_digest` survives (audit 1 counted it unwitnessed; with an authored scenario in the run ess scores an unchanged suite as a survivor), so a retrust that keeps the old digest passes; its arranging refusals are external for ESS-SYNTH-004, and an authored scenario that retrusts and reads the digest would kill it.
- file: spec/domains/runs.yaml
  line: 226
  category: ess-limit
  severity: note
  verdict: CONFIRMED
  origin: pre-existing
  message: The guard-negate mutant of StartRun other-document is unwitnessed because ess 0.99.11 synthesis still writes no scenario for the outcome (ESS-SYNTH-003, D27).
- file: spec/domains/runs.yaml
  line: 227
  category: ess-limit
  severity: note
  verdict: CONFIRMED
  origin: pre-existing
  message: The error-swap mutant of StartRun other-document is unwitnessed for the same refusal, ESS-SYNTH-003, so RevisionOfOtherDocument is held by no scenario.
- file: spec/domains/quality.yaml
  line: 532
  category: equivalent
  severity: note
  verdict: CONFIRMED
  origin: pre-existing
  message: The four precedence swaps (Ratchet worse-lower/worse-higher, EndRun finished/killed and killed/refused, RecordCheck ran-without-valid/valid-without-ran) stay equivalent, as in audit 1.
```
