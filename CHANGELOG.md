# Changelog

## Unreleased

- Wave 2, "Graph".
- The reader of `parts-links/1` documents (`specapp-core::graph`): a document to
  `specapp-graph/1` (`schema/specapp-graph-1.schema.json`) with parts, links (id
  `<from>-><to>:<carries>`), groups from containment, the border and marked places.
- `crates/specapp-layout`: a graph and a view to `specapp-scene/1`
  (`schema/specapp-scene-1.schema.json`), the same bytes for the same input. Layouts `rows`
  (a row per layer or per family; barycentre sweeps and isotonic regression, lifted from
  agenticspec's renderer and made generic) and `nested` (a part inside its container). A
  scene as SVG, the palette's values per theme with a contrast test, 17 icons.
- The manifest's view takes `rows: layer | family`.
- `specapp trust <dir> [--manifest <file>]`: records the digest of the manifest's `tool`
  section in `$XDG_STATE_HOME/specapp/trust.json` (`schema/specapp-trust-1.schema.json`).
- The tool runner (`specapp-core::tool`): runs a verb of a trusted pack's tool in the pack
  directory, kills it with what it started at the deadline, validates `specapp-findings/1`.
- `specapp render --pack <dir> [--manifest <file>] --out <dir> [--deadline <s>]`: one
  self-contained HTML page per `parts-links/1` document (each view of its class drawn, the
  findings as numbered marks and as a list, the parts, links and marked places as tables,
  both themes) and an index page. The page's data is `specapp-document/1`
  (`schema/specapp-document-1.schema.json`).
- Fixtures: `harbor/harbors/reef.yaml` (a link to a name no part has, a link finding, a
  finding about the document); a link finding in `orchard`; golden scenes and pages under
  `fixtures/expected/<pack>/` (`SPECAPP_BLESS=1` rewrites them).
- `scripts/acceptance.sh` draws every document of all four packs and compares the rules of
  every orgspec broken design and example with `orgspec check --json`.

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
