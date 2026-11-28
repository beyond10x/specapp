# Changelog

## Unreleased

- Renamed from specapp to metaspec on 2026-11-28, on the owner's order: the repository
  (`github.com/beyond10x/metaspec`), the crates `metaspec-core`, `metaspec-layout`,
  `metaspec-docs` and `metaspec` (binary `metaspec`), the formats and their schemas
  (`metaspec-*/1`, `schema/metaspec-*-1.schema.json`), the node packages
  `@beyond10x/metaspec-ui` and `@beyond10x/metaspec-docusaurus`, the environment variables
  (`METASPEC_DENYLIST`, `METASPEC_BLESS`, `METASPEC_SITE_OUT`, `METASPEC_SITE_PACK`) and the
  state directory (`$XDG_STATE_HOME/metaspec`). Nothing was published, so every format version
  restarts at /1: the manifest format of the reversal below is `metaspec-pack/1`. Behaviour is
  unchanged.
- Wave 4, "Model": the ESS system `metaspec` in `spec/` (the meta model of a pack, the studio's
  nouns and pack quality; ess pinned to 0.99.9 in `spec/ess-inputs.yaml`), held by `task check`'s
  new `check:spec` (`scripts/check-spec-conform.sh`: validate, then the conformance suite on the
  interpreted target, synthesized once per digest under `target/spec-conform/`); CI installs the
  pinned release.
- The 22 open questions of `spec/` are settled (D1 to D22 in `design/metaspec.md`, "Decided for
  the meta model"): the check adapter of metaspec-pack/1, gaps per feature, dependency rows,
  typed KPI columns, the check of a draft's revision (`CheckRevision`), a finding's rule, the
  `pack_verb` edit, revision findings, the pack carried by the draft, evidence rows behind a
  measured value and a required KPI row per measure. `spec/` holds no `UNMAPPED:` marker.
- Hardening of `spec/`: a design review (`spec/hardening/design-review-1.md`, 25 findings, fixed
  as D23 to D34), a mutation audit (`spec/hardening/mutation-audit-1.md`, score 137/138) and an
  authored scenario that pins its survivor (`spec/scenarios/ratchet-equal-lower-is-better.yaml`,
  the provided rows arranged with `arrange:`).

- Wave 3, "Docs".
- `crates/metaspec-docs` and `metaspec docs build [NAME | --manifest <file>] --pack <dir> --out
  <dir> [--check] [--deadline <s>]`: the docs site of a pack as MDX pages with their data
  (`metaspec-page/1`, `schema/metaspec-page-1.schema.json`), `site.json` (`metaspec-site/1`,
  `schema/metaspec-site-1.schema.json`: counts, pages, sidebar, presentation table) and
  `palette.css`. Pages: the pack, the layer map, one per block, planned blocks, the KPI
  catalogue (`metaspec-kpis/1` rows), per document class an index and one page per document
  (views, findings, parts, links, border, marked places) or per record document its tables, and
  the rules page. The same bytes for the same pack; `--check` lists changed, missing and
  not-owned files and exits 1; a write refuses a directory it did not write.
- `metaspec-pack/1` takes `source_file_url` (`https://...{path}`), the link of each page's
  source; `packs/orgspec.yaml` and `packs/agenticspec.yaml` set it.
- `web/ui`, `@beyond10x/metaspec-ui` (private): `LayerMap`, `BlockHeader`, `Lifecycle`,
  `KpiCatalogue` (filters in the URL query), `GraphView` (a `metaspec-scene/1` as inline SVG),
  `FindingsList`, `RecordTable`, `ResultSections` (says that no result is committed), with
  their CSS. Each takes data and a presentation table and knows no pack.
- `web/docusaurus`, `@beyond10x/metaspec-docusaurus` (private): `withMetaspecSite(config, {out,
  siteDir, routeBasePath})` on `@beyond10x/docs-system` (pinned at the commit agenticspec
  uses).
- `web/site`: the site shell over the committed pages of a fixture pack, or over any docs build
  (`METASPEC_SITE_OUT`).
- The fixture packs' sites are committed under `fixtures/expected/<pack>/site`.
- `task check` adds `check:docs` (`docs build --check` of both fixture packs, with their
  manifests from `fixtures/manifests/`) and `check:web` (typecheck of `web/ui` and
  `web/docusaurus`, vitest of `web/ui` on both fixture packs' pages); the vocabulary check
  covers `web/src` and `web/*/src`. CI installs `web/` with `npm ci`.
- `task ux` (`scripts/ux.sh`): the Playwright and axe-core gates on both fixture sites, every
  page at 1440x900 and 1280x800 in both themes. `task parity` (`scripts/parity.sh
  <agenticspec-dir> [<manifest>]`, by default the known manifest `packs/agenticspec.yaml`): the
  generated agenticspec site against agenticspec's own pages.
- `scripts/acceptance.sh` builds the docs of all four packs (page counts equal pack counts), the
  orgspec site, and prints the agenticspec parity.
- `metaspec render` takes its document pages from `metaspec-docs`; its output is unchanged.

- Decided: reversed the dependency (2026-11-28, ordered by the owner). metaspec knows the packs;
  a pack knows nothing of metaspec.
- The manifests of the public packs are metaspec's: `packs/orgspec.yaml` and
  `packs/agenticspec.yaml`, embedded when metaspec is built. `metaspec pack check <name> --pack
  <dir>` reads a known pack by its name; `--manifest <file> --pack <dir>` reads any other (a
  private pack's manifest stays outside this repository). `metaspec pack known` lists the names.
  The same arguments for `pack words`, `trust` and `render`. metaspec no longer reads
  a manifest from a pack directory.
- The manifest format is `metaspec-pack/1` (`schema/metaspec-pack-1.schema.json`): `tool.verbs`
  maps a verb to its adapter, which runs the pack's own command and maps its own JSON answer
  (`at`, `exit`, `error`, `valid`, `counts`, and lists of findings with templates for rule and
  message, a gate test and a subject rule: `link_when_contains`, or `kind` with `kinds`, and
  `unknown: document`). A manifest in another format or version is refused with both named.
- Removed the tool protocol: no `describe`, no `metaspec-tool/1`; `metaspec-findings/1` is now
  produced by metaspec's adapter (`metaspec-core::adapter`). Trust stays the digest of the `tool`
  section, adapters included.
- `extends` resolves to a known pack or to a manifest given with `--extends <file>`; `pack check`
  warns `extends_not_found` or `extends_other_pack`.
- Fixtures: the manifests moved to `fixtures/manifests/<name>.yaml`; the pack directories hold
  nothing of metaspec; the two fixture tools answer in shapes of their own (a list with exit 1
  for an invalid plan; an object with a verdict word and a kind per finding). The committed
  pages are unchanged.
- `scripts/check-pack-dirs.sh` with its test: fails when a pack directory holds a file named like
  a manifest of metaspec (fixture packs: anything of metaspec). Part of `check:fixtures` and of
  the acceptance.
- `scripts/acceptance.sh <orgspec-dir> <agenticspec-dir>`: manifests from `packs/`; every wave 1
  and wave 2 row is equal as before. `scripts/check-vocabulary.sh` takes `--pack DIR` with
  `--manifest FILE` or `--name NAME` and searches `crates/` and `web/*/src`.
- Rules: no feature for one pack only (owner, 2026-11-28); the gaps of each known pack are
  listed in `design/metaspec.md`; cloudspec is the third known pack, its manifest comes with its
  blocks.

- Wave 2, "Graph".
- The reader of `parts-links/1` documents (`metaspec-core::graph`): a document to
  `metaspec-graph/1` (`schema/metaspec-graph-1.schema.json`) with parts, links (id
  `<from>-><to>:<carries>`), groups from containment, the border and marked places.
- `crates/metaspec-layout`: a graph and a view to `metaspec-scene/1`
  (`schema/metaspec-scene-1.schema.json`), the same bytes for the same input. Layouts `rows`
  (a row per layer or per family; barycentre sweeps and isotonic regression, lifted from
  agenticspec's renderer and made generic) and `nested` (a part inside its container). A
  scene as SVG, the palette's values per theme with a contrast test, 17 icons.
- The manifest's view takes `rows: layer | family`.
- `metaspec trust <dir> [--manifest <file>]`: records the digest of the manifest's `tool`
  section in `$XDG_STATE_HOME/metaspec/trust.json` (`schema/metaspec-trust-1.schema.json`).
- The tool runner (`metaspec-core::tool`): runs a verb of a trusted pack's tool in the pack
  directory, kills it with what it started at the deadline, validates `metaspec-findings/1`.
- `metaspec render --pack <dir> [--manifest <file>] --out <dir> [--deadline <s>]`: one
  self-contained HTML page per `parts-links/1` document (each view of its class drawn, the
  findings as numbered marks and as a list, the parts, links and marked places as tables,
  both themes) and an index page. The page's data is `metaspec-document/1`
  (`schema/metaspec-document-1.schema.json`).
- Fixtures: `harbor/harbors/reef.yaml` (a link to a name no part has, a link finding, a
  finding about the document); a link finding in `orchard`; golden scenes and pages under
  `fixtures/expected/<pack>/` (`METASPEC_BLESS=1` rewrites them).
- `scripts/acceptance.sh` draws every document of all four packs and compares the rules of
  every orgspec broken design and example with `orgspec check --json`.

- Wave 1, "Contract".
- The first manifest format and its JSON Schema (replaced by the format of the reversal above).
  A manifest is validated against the schema when it is read; a path that leaves the pack is
  refused, symbolic links included.
- `metaspec pack check <dir> [--manifest <file>] [--json]` (`crates/metaspec`, on
  `crates/metaspec-core`): loads a pack, compiles its blocks with `ess specify compile`, and
  prints the counts of blocks, layers, KPIs, planned blocks, document classes and documents per
  class, the state of the pack's tool (found or not; nothing is run), and warnings. Its JSON is
  `metaspec-pack-check/1` (`schema/metaspec-pack-check-1.schema.json`).
- `metaspec pack words <dir> [--manifest <file>]`: the block, layer, link, KPI and rule names of
  a pack.
- Schemas of the tool's answers `metaspec-tool/1` and `metaspec-findings/1`.
- Two invented fixture packs, `fixtures/packs/harbor` (a part sits `in` another, families and
  colours) and `fixtures/packs/orchard` (a part exists `per` another, no presentation, private),
  each with an ESS system in numbered layers, a KPI catalogue with its own field names, two
  document classes and a tool in POSIX sh that answers `describe` and `check` from files.
- `scripts/check-vocabulary.sh`: fails when a word of a pack is in metaspec's code.
- `scripts/check-public.sh` with its test, the first step of `task check`: fails on a home path,
  an e-mail address, an IP address, or a term of the deny list `METASPEC_DENYLIST` names (a file
  outside the repository) in a file or a file name.
- `scripts/acceptance.sh`: the acceptance of a wave on both fixture packs and both real packs.
- The gate: `Taskfile.yml` (`task check`), `.config/nextest.toml`, `rust-toolchain.toml`
  (1.99.1), `.github/workflows/gate.yml`, and `scripts/planning-only.sh` with its test.
