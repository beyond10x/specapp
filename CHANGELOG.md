# Changelog

## Unreleased

- metaspec carries its own KPI catalogue, `kpis/catalogue.yaml` (format `metaspec-kpis/1`): one
  row per quality measure of the two questions, 10 of the KPI catalogue (`coverage.blocks` to
  `planned`) and 30 of the specification (`validity.refusals` to `drift`, with one
  `mutation.<class>` row for each of the 14 mutation classes of ess). Every row names the block
  `quality`, its question as measure kind (`kpi_catalogue`, `specification`), its unit and its
  direction in metaspec's words (`higher_is_better`, `lower_is_better`); no row sets `gate`, and
  every row is `planned`. `metaspec_core::quality::catalogue` embeds it and loads it: it refuses
  a row whose id is empty or blank, an id on two rows, a row without unit or direction, and a
  direction outside the two words, every refusal named. The format has its schema at last,
  `schema/metaspec-kpis-1.schema.json`; it takes metaspec's catalogue and the KpiCatalogue data
  of the generated KPI pages, and a test holds both fixture sites' pages to it.
- `metaspec quality kpis <NAME | --manifest FILE> --pack <DIR> [--json] [--deadline SECONDS]`
  measures a pack's KPI catalogue: one measurement per measure of the question `kpi_catalogue`
  of metaspec's catalogue, in its order, a value with the names behind it or a gap. It executes
  nothing of the pack but `ess specify compile`. New format `metaspec-quality/1`
  (`schema/metaspec-quality-1.schema.json`), shared by both questions: pack, question, source
  digest, `measurements[{measure, unit, direction, reading, behind}]` where a reading is
  `{kind: value, value}` or `{kind: gap, gap}`, and `gaps[{kind, feature, needs, summary}]`;
  each gap reading is also a gap of kind `measure`, with no name behind it. The command
  validates its report against the schema before it prints it. Three measures are built:
  `coverage.blocks` (blocks with a KPI over blocks), `completeness.fields` (KPIs with unit,
  direction, a direction for every mode the catalogue writes, and a threshold where `gate` sets
  a gate; `gate: true` counts as incomplete) and `planned` (KPIs whose block the pack does not
  have). A KPI row with no id or a blank one is named `#<index>`. A pack without a `kpis`
  section has a gap naming it for each, with `needs: L1`. The seven other measures of the
  question answer the gap reading "not built yet" (`needs: null`). Goldens:
  `fixtures/expected/{harbor,orchard}/quality/{coverage.blocks,completeness.fields,planned}.json`.
- The layout every later story of the quality initiative fills: `metaspec quality spec` and
  `metaspec quality ratchet`, and `metaspec render --quality <report.json>` (repeatable), are
  wired and exit 2 with "not built yet". `metaspec_core::quality::registry` maps each of the 40
  measures of metaspec's catalogue to a function in the module file of its story (`kpis/` and
  `spec/`, stubs where not built) and refuses a catalogue whose `measured` row has a stub or no
  entry, and a registry measure without a row. `scripts/acceptance.sh` sources every
  `scripts/acceptance.d/*.sh` in name order after its wave rows; `Taskfile.yml` includes
  `taskfiles/slow.yml` and `taskfiles/quality.yml` when they exist.
- `metaspec render ... --quality <REPORT>` (repeatable) writes `quality.html` beside
  `index.html` from reports `metaspec quality kpis --json` or `metaspec quality spec --json`
  wrote; render measures nothing itself. One section per report, in the order of the questions
  (`kpi_catalogue`, then `specification`) whatever the order of the flags: one row per
  measurement the report holds (`id="measure-<id>"`), with its value or gap (and the level a gap
  needs), unit, direction and the names behind it grouped by kind, then the report's gaps. A
  measure the page has not seen before is shown like every other. The page follows the rules of
  render's pages (one file, CSS inline, no script that runs, no request) and carries each report
  in `<script type="application/json" id="metaspec-quality-<question>">`. With the page written,
  `index.html` gets one line more that links it; without `--quality` the index is unchanged. A
  report that cannot be read, is not JSON, does not follow `metaspec-quality/1`, is the report of
  another pack, or repeats the question of another report given is refused with exit 1 after the
  other pages are written; no quality page is written and the index links none. Recorded reports
  `fixtures/quality-reports/{harbor,orchard}/kpis.json` (what `quality kpis --json` printed for
  the fixture packs; the test names the command that writes them again) and the goldens
  `fixtures/expected/{harbor,orchard}/pages/quality.html` rendered from them, so a later measure
  changes neither. Acceptance rows `quality:page`, `quality:page-measures` and
  `quality:page-link` for the four packs (`scripts/acceptance.d/kpi-quality-page.sh`). Every
  render removes a `quality.html` an earlier render left in `--out` before it writes its pages,
  so a refused report or a render without `--quality` leaves no page with old numbers.

## 0.1.0 (2026-11-28)

- `spec/` pins ess 0.99.11 (was 0.99.9). Its synthesis expects the patch of the Variant row
  that `AcceptVariant` names, so `metaspec.drafts.AcceptVariant/outcome/accepted` passes and
  leaves `spec/known-failing.txt`; the list is empty and stays as the mechanism of
  `scripts/check-spec-conform.sh`. `check:spec` runs 135 scenarios: 135 passed, 0 known failing
  (before: 134 passed, 1 known failing). CI installs the release the pin names. A second
  mutation audit (`spec/hardening/mutation-audit-2.md`, score 138/139) runs the authored
  scenario with `--scenarios`, now that no `--known-failing` declaration is needed: it kills the
  guard-boundary survivor of audit 1 (`domains/quality.yaml:534`); the TrustPack `tool_digest`
  mutant that audit 1 counted as unwitnessed is now scored as the one survivor.

- `scripts/acceptance.sh` reads the KPI count of a real pack with one helper: when
  `scripts/check-kpis.sh` prints the count per catalogue (agenticspec main:
  `check-kpis: N KPIs ok in M catalogue(s): <path> <n>, ...`), the count of the catalogue at
  the manifest's `kpis.path`, else the total of `check-kpis: N KPIs ok` (orgspec). The row
  `agenticspec kpis` was empty since agenticspec added its packs' and fields' catalogues.
  "Gaps per pack" names those catalogues as not read by metaspec.

- KPI values per document: the `check` adapter has a new pointer `kpis`, a mapping from a KPI id
  to a number in the tool's answer, read into `metaspec-findings/1` as `kpis` (both schemas
  extended). A document's page gets a section "KPIs" with each value, in the order of the KPI
  catalogue, with its unit and summary and a link to its block's KPIs; an id the catalogue does
  not hold is listed after them. `packs/agenticspec.yaml` reads `/kpis` of
  `agenticspec sim check --json`, the KPIs of an architecture as a whole (agenticspec main,
  `agents_total` to `capacity_ratio`), so every accepted architecture shows them. The
  recorded answers of agenticspec are re-recorded against its main. The fixture pack harbor
  answers values for `cove` (`kpis: /gauges`), held in the committed site. `scripts/parity.sh`
  counts agenticspec's section "The architecture as a whole" as the section "KPIs".

- `task ux` keeps its test budget: axe-core runs once per page and theme, at 1440x900, and in the
  page itself (`setLegacyMode`, no blank page per call); 1280x800 checks the layout (no sideways
  scroll, the navbar and every component of the page visible). On a host at load 26 to 51 the
  gates step went from 100 s (and two runs of three cut off at 120 s) to 30 to 53 s, the slowest
  test from 12.2 s to under 4.5 s.

- One docs site for every pack, on the owner's word about the orgspec site ("the header says
  orgspec all the time, but this is now about metaspec"). The title and header are `metaspec`
  on every page; the navbar has one item per pack with its title, each pack has its own sidebar
  (one docs plugin instance per pack), and its pages are under `/<pack>`. The page `/` lists the
  packs: title, summary, block, document and KPI counts and the check of its documents (valid,
  refused or not checked).
- `metaspec docs build --pack <NAME|FILE>=<dir> [--pack ...] --out <dir>` writes the site of
  every pack given; `[NAME | --manifest <file>] --pack <dir>` still writes a site of one pack.
  The output: per pack `<pack>/docs`, `<pack>/pages` and `<pack>/site.json` (`metaspec-site/1`,
  the `slug` of a page and every `route` in the components' data now carry `/<pack>`; the MDX is
  unchanged), and for the site `index.json` (new format `metaspec-index/1`,
  `schema/metaspec-index-1.schema.json`), `_index/index.mdx` (the page `/`) and one
  `palette.css`. Two packs of one name are bad usage.
- `@beyond10x/metaspec-docusaurus`: `withMetaspecSite(config, {out, siteDir})` reads
  `index.json` and adds a docs plugin instance per pack and a pages plugin instance for `/`;
  new `packNavbarItems`, `readIndex`, and `readSite(out, pack)`. The `routeBasePath` and `id`
  options are gone. `web/site` drops `METASPEC_SITE_PACK`.
- The fixture sites `fixtures/expected/<pack>/site` are replaced by one committed site of both
  fixture packs, `fixtures/expected/site`, held by `task check:docs` and the tests. `task ux`
  builds and serves it once and also checks that the navbar title is metaspec, every pack is a
  navbar item, and `/` lists every pack; a page loads once per theme and is resized for the
  second viewport. `scripts/acceptance.sh` builds the one site of both real packs and compares
  its HTML pages per pack; `scripts/parity.sh` reads the pack's part of its site.

- agenticspec reaches L3: `packs/agenticspec.yaml` declares the `check` adapter of
  `agenticspec sim check --json <architecture>` (`agenticspec-check/1`: `valid`, `counts`, and
  per violation its rule, its gate, its message and a subject that is a link when it contains
  `->`, else a part, else the document). The docs and the rendered pages of agenticspec show its
  findings once its tool is built and trusted. No code changed: the adapter already runs the
  tool once per document.
- `fixtures/answers/<pack>/<class>/`: invented documents with what a known pack's tool answered
  to them (stdout and exit code, `scripts/record-answers.sh <pack> <pack-dir>`) and the
  `metaspec-findings/1` the manifest's adapter reads from that answer, compared byte for byte by
  a test that runs no pack's tool (`METASPEC_BLESS=1` rewrites it). agenticspec has one
  accepted and one refused architecture.
- `scripts/acceptance.sh` builds agenticspec's tool and adds its rows: the rule ids of every
  architecture's page and of the recorded cases equal `agenticspec sim check --json`, and every
  finding about a part or link is marked in a scene.

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
