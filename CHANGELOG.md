# Changelog

## Unreleased

- Wave 1, "Contract".
- The manifest format `specapp-pack/1` and its JSON Schema (`schema/specapp-pack-1.schema.json`).
  A manifest is validated against the schema when it is read; a path that leaves the pack is
  refused, symbolic links included.
- `specapp pack check <dir> [--manifest <file>] [--json]` (`crates/specapp`, on
  `crates/specapp-core`): loads a pack, compiles its blocks with `ess specify compile`, and
  prints the counts of blocks, layers, KPIs, planned blocks, document classes and documents per
  class, the state of the pack's tool (found or not; nothing is run), and warnings. Its JSON is
  `specapp-pack-check/1` (`schema/specapp-pack-check-1.schema.json`).
- `specapp pack words <dir> [--manifest <file>]`: the block, layer, link, KPI and rule names of
  a pack.
- Schemas of the tool's answers `specapp-tool/1` and `specapp-findings/1`.
- Two invented fixture packs, `fixtures/packs/harbor` (a part sits `in` another, families and
  colours) and `fixtures/packs/orchard` (a part exists `per` another, no presentation, private),
  each with an ESS system in numbered layers, a KPI catalogue with its own field names, two
  document classes and a tool in POSIX sh that answers `describe` and `check` from files.
- `scripts/check-vocabulary.sh`: fails when a word of a pack is in specapp's code.
- `scripts/check-public.sh` with its test, the first step of `task check`: fails on a home path,
  an e-mail address, an IP address, or a term of the deny list `SPECAPP_DENYLIST` names (a file
  outside the repository) in a file or a file name.
- `scripts/acceptance.sh`: the acceptance of a wave on both fixture packs and both real packs.
- The gate: `Taskfile.yml` (`task check`), `.config/nextest.toml`, `rust-toolchain.toml`
  (1.99.1), `.github/workflows/gate.yml`, and `scripts/planning-only.sh` with its test.
