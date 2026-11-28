Technique 1 (mutation audit) on metaspec `spec/`: 173 mutants over 14 classes, 137 killed, 1 survived, 4 equivalent, 3 unwitnessed, 28 stillborn; score 137/138 (killed over killed and survived). 195 sites were found and not mutated or are unavailable. 7 findings, none a blocker: the survivor (a missing scenario, now pinned by the authored scenario `spec/scenarios/ratchet-equal-lower-is-better.yaml`, though `mutate` cannot show the kill yet), the ess limit that keeps `mutate` from running it, the 3 unwitnessed mutants, which no scenario can reach today, and 2 notes on equivalent mutants. The audit ran before the authored scenario was added: its baseline is the 134 synthesized scenarios.

Command, from inside `spec/`, where `spec/ess-inputs.yaml` pins ess 0.99.9 (the dispatcher on `PATH` delegates to it), one run per class:

```sh
ess verify conform mutate --path . --target interpreted --class <class> \
  --known-failing known-failures.json --report-out <class>-report.json
```

`known-failures.json` is the `ess-known-failures/1` declaration `scripts/check-spec-conform.sh` writes to `target/spec-conform/`, naming the one known failure of `spec/known-failing.txt` (`metaspec.drafts.AcceptVariant/outcome/accepted`), with its `suite_digest` bound to the suite `mutate` synthesizes as its baseline (the uncompacted synthesis; the gate's suite is `--compact`). Specification digest `d2a59ce84230a2b35653c5ae480aae797d168736dc7baa10ebe98efcac5e556a`. Baseline: 134 scenarios, 1 synthesis refusal; the known failure fails in the baseline as declared and no mutant is scored on it.

Per class:

| class | mutants | killed | survived | equivalent | unwitnessed | stillborn | not mutated or unavailable |
|---|---|---|---|---|---|---|---|
| error-swap | 76 | 75 | 0 | 0 | 1 | 0 | 0 |
| emit-drop | 29 | 1 | 0 | 0 | 0 | 28 | 0 |
| guard-negate | 25 | 24 | 0 | 0 | 1 | 0 | 0 |
| sets-retarget | 23 | 23 | 0 | 0 | 0 | 0 | 43 |
| guard-boundary | 8 | 7 | 1 | 0 | 0 | 0 | 0 |
| guard-connective | 6 | 6 | 0 | 0 | 0 | 0 | 0 |
| precedence-swap | 4 | 0 | 0 | 4 | 0 | 0 | 0 |
| sets-drop | 2 | 1 | 0 | 0 | 1 | 0 | 85 |
| emit-swap | 0 | 0 | 0 | 0 | 0 | 0 | 29 |
| grant-drop | 0 | 0 | 0 | 0 | 0 | 0 | 30 |
| transition-to | 0 | 0 | 0 | 0 | 0 | 0 | 8 |
| from-drop, order-flip, grant-add | 0 | 0 | 0 | 0 | 0 | 0 | 0 (no site, ESS-MUTATE-003) |
| total | 173 | 137 | 1 | 4 | 3 | 28 | 195 |

Not mutated or unavailable: emit-swap finds no other event with exactly the fields of any outcome's only event (`no_compatible_event_alternative`, 29); sets-drop leaves out the 85 writes of a creating branch or of an `Optional` field; sets-retarget finds no sibling input or admissible literal for 43 writes; grant-drop finds 30 grants of commands no component reached over the network serves (`command_not_served`: the grant is the caller's to enforce); transition-to finds no other state that 8 transitions could reach (`no_admissible_state`: AgentChannel turn_on and turn_off, AgentSession end, Draft save, Run fail, finish, kill and refuse). The 28 stillborn emit-drop mutants drop the only event of an outcome, which then neither emits nor names an error and does not compile (ESS-COMMAND-007).

Planted defects, on a copy of `spec/` each, running the gate's suite relabelled with the copy's spec digest on the copy's interpreter:

- control, no edit: 134 scenarios, 133 passed, 1 failed (the known failure).
- `domains/quality.yaml:525`, Ratchet `worse-lower`, `value > input.value` to `value >= input.value`: killed; `metaspec.quality.Ratchet/outcome/taken` fails beside the known failure (132 passed, 2 failed).
- `domains/quality.yaml:534`, Ratchet `worse-higher`, `value < input.value` to `value <= input.value`: not caught (133 passed, 1 failed, the known failure). This is the guard-boundary survivor `guard-boundary/metaspec.quality.Ratchet/worse-higher/when_subject/0`.

The survivor. No scenario ratchets a `lower_is_better` baseline with a value equal to it, so a target that refuses an equal value as `Regressed` passes; the synthesized `taken` scenario ratchets a `higher_is_better` baseline with an equal value, which is why the mirror edit on line 525 is caught.

The authored scenario `spec/scenarios/ratchet-equal-lower-is-better.yaml` (listed under `scenarios:` in `spec/ess-inputs.yaml`) pins it: it arranges the provided Pack and Measure rows with `arrange:` and `setup:`, sets a `lower_is_better` baseline of 2.0 and ratchets 2.0, outcome `taken`. A first draft with type-only fixtures in place of `arrange:` ended `unsupported` on the interpreted target (ESS-CF-TARGET: it answers fixtures only with rows of an entity the deployment provides). `check:spec` now runs 135 scenarios: 134 passed, 1 known failure. Planted against that suite:

- control, no edit: 135 scenarios, 134 passed, 1 failed (the known failure).
- `domains/quality.yaml:534` `<` to `<=`: caught; `metaspec.quality/authored/an-equal-value-of-a-lower-is-better-measure-is-taken` fails beside the known failure (133 passed, 2 failed).
- `domains/quality.yaml:525` `>` to `>=`: still caught by `metaspec.quality.Ratchet/outcome/taken` (133 passed, 2 failed).

`mutate` cannot show the kill yet. It runs authored scenarios only with `--scenarios`, which ess 0.99.9 refuses beside `--known-failing` (`the argument '--scenarios <SCENARIOS>' cannot be used with '--known-failing <KNOWN_FAILING>'`), and without `--known-failing` the baseline fails on the known failure (ESS-MUTATE-001). That stays so until the AcceptVariant fix ships in ess 0.99.11 and the known failure leaves `spec/known-failing.txt`. Until then the mutation score stays 137/138, and the planted run above is the evidence for the kill.

The unwitnessed mutants. `sets-drop/metaspec.packs.TrustPack/retrusted/tool_digest` (`domains/packs.yaml:486`): without the write the synthesized suite does not change, so a retrust that keeps the old digest passes; the refusals that would arrange it are external (`domains/packs.yaml:451-454`, ESS-SYNTH-004). `guard-negate/metaspec.runs.StartRun/other-document/when_related` (`domains/runs.yaml:226`) and `error-swap/metaspec.runs.StartRun/other-document` (`domains/runs.yaml:227`): the baseline refuses to synthesize `metaspec.runs.StartRun/outcome/other-document` (ESS-SYNTH-003, D27 in `design/metaspec.md`), so no scenario reaches the outcome.

The equivalent mutants (ESS-MUTATE-005) are four precedence swaps: `RecordCheck` `ran-without-valid` and `valid-without-ran`, and `EndRun` `finished`/`killed` and `killed`/`refused`, where no input satisfies both guards; and `Ratchet` `worse-lower` and `worse-higher`, which both refuse with `Regressed` and the same payload and change nothing else.

```findings
- file: spec/domains/quality.yaml
  line: 534
  category: missing-scenario
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: No synthesized scenario ratchets a lower_is_better baseline with an equal value, so the guard-boundary mutant `value < input.value` to `value <= input.value` on Ratchet worse-higher survives; the authored scenario spec/scenarios/ratchet-equal-lower-is-better.yaml now catches it in check:spec.
- file: spec/domains/quality.yaml
  line: 534
  category: ess-limit
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: ess 0.99.9 refuses `mutate --scenarios` beside `--known-failing`, so mutate cannot score the authored scenario's kill of the survivor until the AcceptVariant fix ships in ess 0.99.11.
- file: spec/domains/packs.yaml
  line: 486
  category: missing-scenario
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: The sets-drop mutant of TrustPack retrusted's `tool_digest` is unwitnessed (the suite does not change), so a retrust that keeps the old digest passes; its arranging refusals are external for ESS-SYNTH-004.
- file: spec/domains/runs.yaml
  line: 226
  category: ess-limit
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: The guard-negate mutant of StartRun other-document is unwitnessed because ess 0.99.9 synthesis writes no scenario for the outcome (ESS-SYNTH-003, D27).
- file: spec/domains/runs.yaml
  line: 227
  category: ess-limit
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: The error-swap mutant of StartRun other-document is unwitnessed for the same refusal, ESS-SYNTH-003, so RevisionOfOtherDocument is held by no scenario.
- file: spec/domains/quality.yaml
  line: 532
  category: equivalent
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: Swapping Ratchet worse-lower and worse-higher changes nothing observable: both refuse with Regressed and the same payload.
- file: spec/domains/runs.yaml
  line: 289
  category: equivalent
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: The precedence swaps of EndRun finished/killed and killed/refused, and of RecordCheck ran-without-valid/valid-without-ran (checks.yaml:279), are equivalent: no input satisfies both guards.
```
