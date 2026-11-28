# metaspec: one docs site and one studio for every specification pack (draft 1, 2026-11-27)

Status: waves 1 ("Contract"), 2 ("Graph") and 3 ("Docs") are built: the manifest, the schemas,
two fixture packs, `metaspec pack check`, the `parts-links/1` reader, the layout, `metaspec trust`,
`metaspec render` (a static page per document with the findings of the pack's tool) and
`metaspec docs build` with the components and the Docusaurus preset of `web/`. The build order is
at the end; "Decided in wave 1" to "Decided in wave 3" list what the builds decided that this
design had left open. On 2026-11-28 the owner reversed the dependency between metaspec and the
packs: metaspec knows the packs, a pack knows nothing of metaspec ("Decided: reversed the
dependency"). Where an older section below says otherwise, that section wins.

## What it is

Several Beyond10x repositories have the same build: an ESS system of building blocks in numbered
layers, a KPI catalogue, documents that compose the blocks into a graph of parts and links, and a
tool that checks such a document and, in some of them, simulates it. agenticspec describes
agentic development this way. orgspec describes an organization this way.

metaspec is the user interface of such a repository, written once:

- **Docs**: a static site. Every block by layer, every lifecycle, the KPI catalogue, every
  document drawn as a graph with its check result.
- **Studio**: a local application. The same pages live, plus work on a document: edit it, check
  it at every change, run it, compare variants, keep drafts, and do all of that together with an
  agent.

A repository that metaspec can load is a **pack**. metaspec describes a pack in one manifest of
its own; the pack knows nothing of metaspec. metaspec's code contains no word of any pack: no
block name, no layer name, no colour of a block, no rule id. The words are in the manifests.

metaspec is the top-level application over all spec packs. The packs it knows today are
agenticspec and orgspec; cloudspec is the third (a planning store so far, no blocks), and
`packs/cloudspec.yaml` is added once it has blocks.

## What exists today

Read on 2026-11-27 in agenticspec's working tree (uncommitted work included) and in orgspec's.

agenticspec has:

- A generator, `crates/agenticspec-docs` (2,836 lines of Rust). It compiles `blocks/` with
  `ess specify compile --format json`, reads `kpis/catalogue.yaml`, `architectures/*.yaml` and
  one experiment per architecture, re-runs each experiment, and writes Markdown pages and three
  JSON files. `--check` compares bytes with a fresh generation.
- A renderer, `crates/agenticspec-render` (4,314 lines of Rust). `page(architecture, result)`
  returns one self-contained HTML page: the graph with a deterministic layout (rows, barycentre
  sweeps, isotonic regression), and the sections of a simulation result.
- A Docusaurus site on `@beyond10x/docs-system` with five React components: `BlockMap`,
  `BlockHeader`, `Lifecycle`, `KpiCatalogue`, `ArchitectureFrame`.
- A Vue application over recorded agent sessions. It shares no code with the docs site and is
  not a UI of the building blocks.
- No studio. `epic:agenticstudio` is a draft: an agent channel, the graph as an editor, live
  checks, experiments from the editor, KPIs per part, drafts with revisions, and one operation
  set (select, then transform) for the user, the agent and the optimizer.

orgspec has blocks, types, example designs, a checker with `--json`, a KPI catalogue, and no user
interface.

What ties agenticspec's code to agenticspec:

- The table block → family → colour → icon exists three times (generator, renderer, site), and
  one block is in none of them.
- The renderer places a part in one of seven rows by a table of block names in the source, not
  by the layer of its block.
- Copy in the generator: the name of the field, a count of blocks that is stale, the prefix it
  strips from domain names, the name of a CLI command.
- The KPI vocabulary: its kinds, its statuses, its word for a mode.
- The source URL, the title, the navigation.

What is already data: the block list, the layers, the dependency edges, lifecycles, KPI rows,
document pages, the parts, links and markers of a document.

Evidence that the common part is small: on 2026-11-27 a scratch viewer for orgspec, 85 lines of
jq and sh around Graphviz, drew two views of any design (structure; who answers for what) with
the checker's violations in red. It reads only `parts`, `links`, `border` and the checker's JSON.

## What the packs share

Same in both repositories, apart from names:

- `blocks/system.yaml`, `blocks/ess-inputs.yaml`, `blocks/<N>-<layer>/<block>.yaml`: one ESS
  system, one domain per block, the layer in the directory name, dependencies downward only.
- `kpis/catalogue.yaml`: rows with `id`, `block`, a measure kind, `unit`, `summary`, `how`,
  `direction`, a direction per mode, `gate`, `reads` (events of the blocks), `status`.
- Composed documents: `parts` (a map from a name to `{block, ...fields}`), `links` (a list of
  `{from, to, carries, ...fields}`), `border` (names of parts), and a marked place written
  `{value, vary}` where a search may change a value.
- A tool that reads one document and answers: valid or not, and a list of findings that name a
  rule and a part or link.

Different, and therefore declared by the pack:

- Names: of the system, the blocks, the document classes, the link names, the rules.
- Containment. In orgspec a part sits in another part (`in`). In agenticspec a part exists once
  per instance of another part (`per`).
- What a good picture shows. orgspec needs two views of one design. agenticspec needs one.
- Extra document classes: types in orgspec; profiles and experiments in agenticspec.
- What the tool can do beyond the check: simulate, search, lower to ESS commands.

metaspec's own KPI catalogue is `kpis/catalogue.yaml`, format `metaspec-kpis/1`, embedded in the
binary. One row per quality measure: `id`, `block` (`quality`, the domain `metaspec.quality` of
metaspec's specification), `measure` (the question: `kpi_catalogue` or `specification`), `unit`,
`summary`, `how`, `direction` (`higher_is_better` or `lower_is_better`, ESS type
`metaspec.quality.Direction`), `modes` (empty), `reads` (empty), `status`. No row sets `gate`: no
threshold is invented; the ratchet holds the first value. The quality runner reads unit and
direction from it; `metaspec_core::quality::catalogue` refuses a row whose id is empty or blank,
an id on two rows, a row without unit or direction, and a direction outside the two words.

## The principle: metaspec knows the packs; a pack knows nothing of metaspec

1. metaspec reads a pack through a manifest of its own and a small set of neutral formats. The
   manifest and its adapters are metaspec's: the manifests of public packs are in metaspec's
   `packs/`, a private pack's manifest stays outside this repository and is given by its path
   at run time. A pack holds no file of metaspec, names no format of metaspec and has no command
   for metaspec; it never adapts to metaspec. metaspec adapts to the pack: the manifest states
   where the pack's things are, what they are called and how its own tool answers.
2. Reading costs nothing to adopt and executes nothing. Behaviour (check, run, apply) comes from
   the pack's own tool, run as the pack ships it and read by an adapter in the manifest (see
   "The tool adapter"), generic in metaspec's code.
3. One model, one component library, two shells. The docs site is the static export of the
   read-only part of the studio. A page looks the same in both.
4. What metaspec shows is what the pack's tool says. metaspec implements no rule of any pack. A
   finding, a KPI value and a refusal are the tool's.
5. A private pack stays on the machine. See "Private packs".
6. One application for every pack. No feature of metaspec is built for one pack only. A feature
   that one pack's tool supports is offered for every pack through the manifest's adapter, and
   what a pack lacks for it is listed as that pack's gap ("Gaps per pack").

## A pack

metaspec holds the manifest of every public pack it knows, `packs/<name>.yaml`, format
`metaspec-pack/1`; `metaspec pack check <name> --pack <dir>` reads it by the pack's name. A pack
metaspec does not know, such as a private pack, is read with a manifest given by its path:
`metaspec pack check --manifest <file> --pack <dir>`. The pack directory is always given
separately; nothing in it is read as a manifest.

```yaml
format: metaspec-pack/1
pack: orgspec
title: orgspec
summary: Building blocks of an organization.
visibility: public                 # public | private
source_url: https://example.org/orgspec

blocks:
  path: blocks                     # an ESS system; a layer is a numbered directory
  strip_prefix: org.

kpis:
  path: kpis/catalogue.yaml
  fields: { measure: measures, modes: postures }

documents:
  - class: organization
    title: Organizations
    paths: [organizations/*.yaml]
    shape: parts-links/1
    name: organization              # the key that holds the document's name
    contain: { in: in }             # a part sits inside the part its field `in` names
    views:
      - id: structure
        title: Structure
        layout: nested
        blocks: [organization, unit, position]
        carries: [reports_to, answers_to, escalates, delegates, hands_off]
      - id: subjects
        title: Who answers for what
        layout: rows
        carries: [accountable, decides, approves, proposes, responsible, serves, demand, settles]
  - class: type
    title: Types
    paths: [types/*.yaml]
    shape: records/1
    name: type
    tables:
      - { title: Kinds, at: kinds, key: name, columns: [block, is, summary] }
      - { title: Rules, at: rules, columns: [id, rule, summary, gate] }
      - { title: KPIs,  at: kpis,  columns: [id, block, measures, unit, summary] }

tool:
  run: [target/debug/orgspec]       # the pack's own tool, as the pack ships it
  verbs:
    check:                          # the adapter; see "The tool adapter"
      args: [check, --json]
      exit: [0, 1]
      at: /0
      error: /error
      valid: { at: /valid }
      findings:
        - at: /violations
          rule: "{/rule}"
          gate: { at: /gate }
          message: "{/message}"
          subject: { id: "{/subject}", link_when_contains: "->", unknown: document }

presentation:
  families:
    structure: { blocks: [organization, unit, role, position], colour: blue,  icon: boxes }
    authority: { blocks: [decision, delegation, policy],  colour: violet, icon: gavel }
```

The same manifest for agenticspec differs in names and in four places: `strip_prefix: agentic.`,
`kpis.fields: { measure: kind, modes: modes }`, one document class `architecture` with
`contain: { per: per }` and one view, and a `tool` whose `check` runs
`target/debug/agenticspec sim check --json <architecture>` and reads `valid`, `counts` and the
list `violations` (rule, gate at `/gate`, message, subject as for orgspec).

Rules of the manifest:

- It is metaspec's file and lies outside the pack: under `packs/` for a public pack metaspec
  knows, anywhere outside this repository for a private one. A pack directory holds no file
  named like it; `scripts/check-pack-dirs.sh` fails on one.
- Every path is inside the pack. A path that leaves it is refused.
- `presentation` is optional. Without it a block's row is the layer of its domain and its colour
  is the colour of its layer, from a palette of metaspec. This is the one place a pack states a
  colour or an icon; a pack that needs none writes none.
- No number about the pack is written in the manifest or in any copy: counts are counted.
- `extends` names another pack: a pack metaspec knows (`packs/<name>.yaml`), or one whose manifest
  is given at run time (`--extends <file>`); see "Private packs".

### Levels of adoption, cheapest first

A pack is useful from the first level and may stop at any.

- **L0 blocks.** `blocks.path` only. Layer map, one page per block (depends on, used by,
  entities with lifecycle, commands, events, views, types). Works for any ESS system in numbered
  layers. Nothing is executed but `ess specify compile`.
- **L1 KPIs.** The catalogue: a searchable table, and the KPIs on each block's page.
- **L2 documents.** Document classes with a shape. Every document is a page; a `parts-links/1`
  document is drawn. Nothing of the pack is executed.
- **L3 check.** The manifest's adapter for `check`: the pack's tool runs on a document, and its
  findings are drawn on the graph and listed, per document and as a table over all documents.
- **L4 runs.** The tool's `simulate` and `search`: results, KPIs per part, variants.
- **L5 edit.** The tool's `apply`, or metaspec's generic edits: drafts, revisions, comparison,
  the agent channel.

orgspec today reaches L3: its manifest in metaspec reads `orgspec check --json` through an
adapter. agenticspec reaches L3 too: its manifest reads `agenticspec sim check --json` (see
"Gaps per pack").

## Neutral formats

metaspec's own formats. A pack's formats stay the pack's.

- `metaspec-pack/1`: the manifest, with the adapters of the pack's tool. One producer: metaspec's
  maintainers (`packs/`), or the owner of a private pack (a file outside this repository).
- `metaspec-pack-check/1`: what `metaspec pack check --json` prints. `valid`, `errors` and
  `warnings` (`{code, subject, message}`); for a pack that loads also its name, visibility,
  `extends`, the level it declares, `counts` (blocks, layers, KPIs, planned blocks, document
  classes, documents), the layers with their blocks, the documents per class, and the state of
  the tool (`none`, `found`, `not_found`). One producer: metaspec.
- `metaspec-model/1`: the blocks, from the compiled ESS model. Layers, blocks, dependency edges
  with counts, and per block its entities, lifecycles, commands, events, views, types and source
  lines. One producer: metaspec, from `ess specify compile --format json`.
- `metaspec-kpis/1`: the catalogue with neutral field names (`measure`, `modes`), the pack's
  words for their values kept as data, and the pack's own columns kept as extra columns. Two
  producers, both metaspec: its own catalogue `kpis/catalogue.yaml` (rows as a pack writes them:
  `modes` a map from a mode to its direction, `gate` absent where no threshold is set), and the
  KpiCatalogue data of a generated KPI page (top-level `measure` and `modes` name the pack's own
  columns, `modes` of a row a list, the pack's other columns in `extra`, plus `layer`, `planned`
  and `route`). Schema `schema/metaspec-kpis-1.schema.json` takes both.
- `metaspec-graph/1`: one composed document. `parts[{id, block, kind, fields, inside, per,
  on_border, markers}]`, `links[{id, from, to, carries, fields, added_by}]`, `groups` from
  containment. `added_by` names a default of the pack that added a link the document did not
  write. Produced by metaspec's reader for `parts-links/1`, or by an adapter of the tool's
  `graph`.
- `metaspec-findings/1`: a check result. `valid`, and `findings[{rule, gate, subject, message}]`
  where `subject` is `part:<id>`, `link:<id>` or `document`. Counts per rule, and `kpis`, the
  KPI values of the document by KPI id, when the adapter declares them. Produced by
  metaspec only: the adapter of the manifest maps the tool's own answer to it.
- `metaspec-scene/1`: a laid-out view of a graph: boxes, groups, edge paths, labels, in
  coordinates. Produced by metaspec's layout, deterministic for the same input. The docs site
  and the studio draw the same scene.
- `metaspec-result/1`: a run. Baseline and best, KPI values per document and per part, the
  marked places, variants, a trace. agenticspec's `agenticspec-sim-result/1` is the model for
  it; an adapter of the manifest maps the tool's own result to it.
- `metaspec-patch/1`: an ordered list of edits. See "Operations".
- `metaspec-page/1`: one generated page: front matter, Markdown, and the data of its components.
- `metaspec-document/1`: the data of one page `metaspec render` writes (wave 2): the graph, the
  check, the findings with the views they are drawn in, the scenes. One producer: metaspec.
- `metaspec-trust/1`: the trust file in the user's state directory. One producer: metaspec.
- `metaspec-quality/1`: the report of one quality question over one pack, what
  `metaspec quality kpis --json` (question `kpi_catalogue`) and `metaspec quality spec --json`
  (question `specification`) print. `pack`, `question`, `source_digest` (`sha256:` over the
  manifest, the compiled blocks, the KPI rows and the documents the measures read; no path of
  the checkout), `measurements[{measure, unit, direction, reading, behind}]`, one per measure of
  the question in metaspec's catalogue, in its order, unit and direction copied from that
  catalogue; a reading is `{kind: value, value}` or `{kind: gap, gap}`; `behind` is
  `[{kind, name}]` with the names as the pack writes them (kinds of
  `metaspec.quality.BehindKind`), empty for a gap (the schema refuses a name behind a gap
  reading). `gaps[{kind, feature, needs, summary}]`: each gap reading is also a gap of kind
  `measure` with the measure id as its feature; `needs` is the level of adoption whose input the
  pack lacks, or null for metaspec's own gap, a measure not built yet. One producer: metaspec;
  the command validates the report against the schema before it prints it.

Every format has a JSON Schema in `schema/` and a version in its name. A manifest in a version
metaspec does not know is refused with both versions named. A pack's tool answers in the pack's
own format; metaspec never asks it for one of its own.

## The tool adapter

The pack's tool is a process the pack ships for its own use; it knows nothing of metaspec. For
each verb metaspec offers, the manifest's `tool.verbs` holds an adapter: how metaspec runs the
tool and how it maps the tool's own answer to a neutral format. The mapping is declarative and
generic in metaspec's code; no adapter names a word of a pack in the code. A verb the manifest
does not declare is not offered for that pack; nothing is asked of the tool (there is no
`describe`).

`check` (L3) is the verb built today. metaspec runs `<tool.run> <args> <document>` in the pack
directory with stdin closed and reads stdout as JSON:

- `exit`: the exit codes that come with an answer (default `[0]`). Another exit code is a failed
  run, unless `error` finds a text.
- `at`: a JSON pointer to the document's result in the answer (default: the whole answer).
  Every other pointer is relative to it.
- `error`: a pointer to a text that, when the result has it, refuses the document with that
  text, whatever the exit code.
- `valid`: a test, `{ at: <pointer> }` for a boolean or `{ at: <pointer>, equals: <value> }`;
  without it, a document is valid when no finding is gated.
- `counts`: a pointer to a mapping from a rule to a count.
- `kpis`: a pointer to the KPI values of the document, a mapping from a KPI id to a number (an
  answer without it: no values; a value that is no number: a failed check). The document's page
  has a section "KPIs" with one row per value, in the order of the pack's KPI catalogue and
  with its unit and summary, then the ids the catalogue does not hold. These are values the
  tool reads from the document as written, before any run; a simulation's values wait for
  `simulate`.
- `findings`: lists of findings, each `{at, rule, gate, message, subject}`, in the order the
  findings are shown. `rule` and `message` are templates: `{<pointer>}` stands for the text at
  that pointer in one entry. `gate` is `true`, `false` or a test of the entry. `subject` is
  `document`, or a rule: `id` (a template) and either `link_when_contains: <text>` (an id that
  contains the text is a link, any other a part) or `kind: <template>` with `kinds` (the
  template's value names `part`, `link` or `document`). `unknown: document` makes an id that the
  drawn graph does not have a finding about the document, with the id put before the message
  unless it is the document's name.

The result is `metaspec-findings/1`, validated against its schema; an answer the adapter cannot
read is a failed check, shown as such. orgspec's adapter reads `orgspec check --json`,
agenticspec's reads `agenticspec sim check --json`; the two
fixture packs answer in two other shapes (a list with exit 1 for an invalid plan, and an object
with a verdict word and a kind field per finding), so the mapping is tested on both.

What a known pack's tool answers is also recorded: `fixtures/answers/<pack>/<class>/` holds
invented documents, the tool's stdout and exit code for each (`scripts/record-answers.sh <pack>
<pack-dir>`, run by hand on a checkout of the pack), and the `metaspec-findings/1` the adapter
of `packs/<pack>.yaml` reads from them. The tests compare that result byte for byte and run no
pack's tool; the acceptance runs the tool itself on the same documents.

The verbs to come take the same form when they are built: `graph` (an adapter to
`metaspec-graph/1`, where the generic reader is not enough, such as a pack whose defaults add
links), `simulate` and `search` (to `metaspec-result/1`; metaspec passes the budget and a
deadline, a run over the deadline is killed and shown as such), and `apply` (a
`metaspec-patch/1` translated to the tool's own edit, or metaspec's generic edits).

Running a pack's tool is running code from a repository. So:

- L0 to L2 execute nothing of the pack.
- metaspec runs a tool only for a pack the user trusted: `metaspec trust <name> --pack <dir>` (or
  `--manifest <file>`) records the digest of the manifest's `tool` section, adapters included.
  A changed `tool` section asks again. `metaspec trust --remove <name> --pack <dir>` withdraws
  the trust of that directory.
- The tool gets the pack directory as its working directory and no network by default.

## Presentation

One table per pack, in the manifest, read by every part of metaspec. It replaces the three copies
in agenticspec.

- A family groups blocks under one colour and one icon. Colours are names of a palette of
  metaspec that passes the contrast rules in both themes; a pack never writes a colour value.
- A block in no family is drawn with the colour of its layer, and metaspec says so in
  `metaspec pack check`. No block is ever "other".
- A view of a document class says which blocks and which link names it shows and how it is laid
  out: `rows` (a part's row is its block's layer or its family's row) or `nested` (a part is
  drawn inside the part that contains it).
- Words: a pack may rename a block or a link name for display (`vocabulary`). The identifier
  stays visible on hover and in every link.

## Docs

One site for every pack, titled metaspec ("Decided: one site for every pack"):
`metaspec docs build --pack <NAME|FILE>=<dir> [--pack ...] --out <dir>` (or
`[NAME | --manifest <file>] --pack <dir>` for a site of one pack) writes pages and data, each
pack's under `/<pack>`; `--check` compares bytes with a fresh generation and lists files it does
not own. No file of the output is edited by hand.

Pages, all generated:

- The layer map: every block on its layer, dependency edges on hover.
- One page per block: what it depends on, what uses it, each entity with its lifecycle as a
  diagram, commands with their refusals, events, views, types, its KPIs, the documents that use
  it.
- Planned blocks: blocks the KPI catalogue or the pack's design names before `blocks/` has them.
- The KPI catalogue: search, filter by block, measure, status and gate.
- Per document class an index, and one page per document: its views as graphs, its check result,
  its parts and links as tables, its marked places, and, at L4, the result of its run.
- Per `records/1` class one page per document with the tables the manifest names.
- A rules page: every rule id the tool reports across all documents, with the documents that
  break it. For a pack with broken examples (one per rule) this is the proof that each rule can
  refuse.

Hand-written pages stay in the pack (`docs/`), as today.

Delivery: a Docusaurus preset, `@beyond10x/metaspec-docusaurus`, on `@beyond10x/docs-system`. A
pack's `website/` holds its identity and its hand-written pages; the preset adds the generated
pages, the sidebar and the components. The docs build does not run a simulation: a page shows a
result that is committed beside the document, or none.

The components are a package, `@beyond10x/metaspec-ui`: `LayerMap`, `BlockHeader`, `Lifecycle`,
`KpiCatalogue`, `GraphView` (draws a `metaspec-scene/1` as SVG in the page, no frame),
`FindingsList`, `RecordTable`, `ResultSections`. They take data and a presentation table and
know no pack.

## Studio

`metaspec serve --pack <dir> [--pack <dir>]` serves on the loopback address only.

### Screens

- **Pack**: the layer map, counts, the state of the pack (valid, warnings, what
  `metaspec pack check` says), the documents with their check status.
- **Reference**: the docs pages, live. A change of a file of the pack shows without a reload.
- **Document**: the graph in its views; the findings beside it and on it; a part or link opens
  its fields, its block's page and its KPIs. The check runs when the file changes.
- **Draft**: the document as an editor. Every edit is checked before it is taken; a refused
  edit shows the rule and changes nothing. Revisions, undo, comparison of two revisions side by
  side (graph and findings and KPIs).
- **Runs**: start a run of the tool on a document or a draft; the result; KPIs per part,
  baseline against best; the variants; accept a variant as a new revision of the draft.
- **Agent**: a conversation with an agent that works on the open draft through the same
  operations as the user.

### Operations

One operation set for the user, the agent and the tool's search, as agenticspec's draft epic
states it: select, then transform.

- Generic edits, valid for every `parts-links/1` document: `add_part`, `remove_part`,
  `set_field`, `add_link`, `remove_link`, `move` (containment), `mark` and `unmark` (a place a
  search may vary), `set_border`.
- A pack's own edit verbs are declared in its manifest with an adapter: agenticspec's `wrap` and
  `replicate` are such verbs.
- A `metaspec-patch/1` is an ordered list of edits. It is applied as a whole or refused as a
  whole, and it is one revision.
- The agent has four tools: `query`, `apply` (with `dry_run`), `check`, `simulate`. They are the
  server's commands, also offered as an MCP server (`metaspec mcp`), so an agent session outside
  the studio works on the same draft.

A draft is a base document (its digest) and its patches. It lives in the studio's state, not in
the pack, until the user saves it: then metaspec writes the document file with the edits and
nothing else changed, comments included. An edit that cannot be written that way is refused.

### Server

- The studio's own nouns are an ESS system, `metaspec`: pack, document, draft, revision, patch,
  finding, run, variant, agent session, tool call. Its views and commands are the server's API,
  generated; a channel carries changes to the browser. The UI is modelled in `ess-ui/1` and
  held by `ess ui check`. This is modelled before the server is written.
- Pack data is derived from the files and cached by the digest of its sources. Drafts, runs and
  agent sessions are in a store in the user's state directory, one per pack.
- Layout runs in the server; the browser draws scenes. The same layout code produces the scenes
  of the docs build.

## Private packs

A pack of one real organization is private. metaspec is public. The rules:

- metaspec's repository holds no content of any real pack: no block, no document, no finding.
  It holds the manifests of the public packs it knows (`packs/`, and only there). Its fixtures
  are two invented packs with different vocabulary (see "Proof of generality").
- A private pack's manifest is never in metaspec's repository: it lives outside it (with the
  owner of the private pack, never in the pack itself) and is given by its path at run time
  (`--manifest <file>`).
- `visibility: private` in the manifest: `docs build` marks every page as private and refuses a
  publication target; `serve` shows a private marker in the header.
- `extends`: a private pack's manifest names the public pack it builds on and adds documents,
  document paths (a type path) and presentation. The name resolves to `packs/<name>.yaml`, or to
  a manifest given at run time (`--extends <file>`). The public pack never names the private
  one. The studio loads both and shows which pack a document comes from.
- The studio makes no network request of its own: fonts and scripts are served from the binary.
- The agent channel sends what the agent reads to a model provider. For a private pack it is
  off until the user turns it on for that pack.
- Drafts, runs and the cache of a private pack are in the user's state directory, never in the
  metaspec checkout.
- metaspec's gate carries the same guard as orgspec's: a deny list outside the repository,
  checked before commit and push.

## UX rules and gates

Taken from the rules agenticspec's site already holds, as the floor:

- Layout: no sideways page scroll, nothing past the viewport except code, no cut text without a
  title, 0 serious or critical accessibility violations (axe, WCAG 2 A to 2.2 AA), both themes.
- Viewports that gate: 1440×900 and 1280×800. Measured and reported: 1024×768, 768×1024,
  390×844.
- A ratchet: a baseline of counts per scenario; a better value is taken into the baseline, a
  worse one fails. A closed finding without a regression test fails.
- Tests draw from fixtures, never from a running server per test. A test is killed at 60 s and
  the test step ends within 120 s.
- A graph is readable without colour: a block's family is also an icon and a label; a finding
  is also a mark and a row in the list.
- Every scene, table and filter state has a URL.

## Proof of generality

metaspec must not become agenticspec's UI with a second skin, nor orgspec's. It is the top-level
application over all spec packs: whatever it can do with one pack it must do with every pack it
knows (agenticspec, orgspec, and cloudspec once it has blocks). Four gates:

- **Two fixture packs** in `fixtures/packs/`, both invented and small, with different words,
  different containment (`in` against `per`), different KPI field names and a tool written as a
  shell script that answers from files in a format of its own, a different shape per pack.
  Their manifests are metaspec's, `fixtures/manifests/<name>.yaml`; a fixture pack directory
  holds nothing of metaspec (`scripts/check-pack-dirs.sh`). Every test of the generator, the
  layout and the components runs on both.
- **No pack vocabulary in the code.** A check fails when a block name, link name, rule id or
  KPI id of a real pack or of a fixture pack appears under `crates/` or `web/*/src`. The
  manifests (`packs/`, `fixtures/manifests/`) are where those words belong and are not searched.
- **No feature for one pack only.** A feature that one pack's tool supports is offered for
  every pack through the manifest's adapter. What a pack lacks for it is written down in "Gaps
  per pack", per pack, not worked around in the code.
- **The acceptance test of each wave** loads both real packs from their checkouts
  (`metaspec pack check <name> --pack <dir>`, manifests from `packs/`) and compares counts with
  what each pack's own gate prints; no pack directory it is given holds a manifest of metaspec.

## Gaps per pack

What each known pack lacks for a feature metaspec offers. A gap is closed in the pack (its own
tool, its own documents) or in a neutral format, never by code for one pack.

- **orgspec** (L3). Its defaults add links the generic reader cannot see: a finding on such a
  link is a row, not a mark, until an adapter for `graph` reads the links its checker adds (no
  broken design or example needs it today). No `presentation` yet.
- **agenticspec** (L3). Its manifest reads `agenticspec sim check --json <architecture>`
  (`agenticspec-check/1`). A file the tool cannot read is answered on stderr with no JSON, so
  it is shown as a failed check, not a refusal: the adapter's `error` has nothing to point at.
  The pack has no broken architectures, so its rules page lists no rule; the acceptance proves
  the mapping on the invented cases of `fixtures/answers/agenticspec/`. The KPIs of an
  architecture as a whole (its catalogue's block `architecture`, read from the document before
  any simulation) come with an accepted architecture's answer at `/kpis` and are read by the
  adapter's `kpis`; a refused architecture's answer has none, so its page has no section
  "KPIs". Not shown: the answer's `single_points` (the parts the architecture cannot lose: a
  list of part ids with no neutral field; a mark per part would need findings that are not
  violations), `nested` and `ingresses`; and the comparison of every architecture on these KPIs
  in one sortable table that agenticspec's catalog index has (the class index of metaspec lists
  checks, not values). Sections of its own pages at its main with no counterpart yet (parity):
  "Policies", "Contract" (the contract `agenticspec sim contract` derives), "Nested
  architectures" and "Packs and fields". Its simulator's result
  waits for `simulate`
  (wave 7). The gaps of its architectures listed in "Decided in wave 2" stand: two blocks the
  documents use are no block of `blocks/`, a link `over` a part has no neutral field, part
  fields that name other parts are fields only, and `architectures/ladder/` is outside the
  manifest's glob. metaspec reads one KPI catalogue, `kpis/catalogue.yaml` (the manifest's
  `kpis.path`); the KPI catalogues of its packs and fields (`packs/*/kpis.yaml`,
  `fields/*/kpis.yaml`, 33 KPIs at its main) are not read.
- **cloudspec**. A planning store only, no blocks: no manifest yet. `packs/cloudspec.yaml` is
  added once it has blocks.

## What moves and what stays

- From agenticspec into metaspec: the generator's reading of the compiled model and its page
  writers; the layout and the SVG of the renderer; the five React components. Each becomes
  generic on the way in. agenticspec keeps its simulator, its architecture check, its session
  application and its hand-written pages. It gains nothing of metaspec: its manifest and the
  adapters of its tool are in metaspec's `packs/agenticspec.yaml`.
- agenticspec's generator and renderer are deleted from agenticspec when metaspec produces the
  same pages for it: the list of pages and their sections is the parity test.
- orgspec gains nothing of metaspec: its manifest, with the adapter of `orgspec check --json`, is
  metaspec's `packs/orgspec.yaml`. Its scratch viewer is not kept. Where its site lives is wave 3's
  question.
- `@beyond10x/docs-system` stays what it is: the site contract, the tokens and the landing page.
- `ess-ui-templates` draws any ESS model as one page with its lifecycles played. metaspec links
  to that page from a block where a pack builds it; whether its lifecycle drawing replaces
  metaspec's own is open (Q7).

## What metaspec does not do

- It does not check a document, simulate anything or know a rule. The pack's tool does.
- It does not author ESS. The blocks are written in the pack with the ESS tools.
- It does not publish. A pack's own workflow publishes its site.
- It is not the session application of agenticspec; a pack may link its own applications from
  the pack screen.

## Repository layout

```text
design/                 this document
schema/                 JSON Schemas of the neutral formats
scripts/                the vocabulary check, the pack directory check, the acceptance script,
                        the planning-only path list, the recorder of a known pack's answers
packs/                  the manifests of the public packs metaspec knows, `<name>.yaml`
spec/                   the ESS system `metaspec` and its ess-ui model
crates/metaspec-core/   manifest, pack loading, the neutral model
crates/metaspec-layout/ graph layout to a scene
crates/metaspec-docs/   the generator
crates/metaspec-server/ serve, the store, the tool runner
crates/metaspec/        the CLI: pack check, docs build, serve, trust, mcp
web/ui/                 @beyond10x/metaspec-ui (React)
web/docusaurus/         @beyond10x/metaspec-docusaurus
web/studio/             the studio (React, Vite)
fixtures/packs/         two invented packs, nothing of metaspec in them
fixtures/manifests/     their manifests, `<name>.yaml`
fixtures/expected/      what each fixture pack's check must count
fixtures/answers/       recorded answers of the known packs' tools on invented documents
ux/                     adapter, baseline, rounds of the UI loop
```

## Build order

Each wave ends with its acceptance on both real packs and both fixture packs.

1. **Contract.** The manifest format, the schemas, the two fixture packs, `metaspec pack check`.
   Acceptance: both real packs load from a manifest written in their checkouts; the counts of
   blocks, layers, KPIs and documents equal what each pack's gate prints.
2. **Graph.** The `parts-links/1` reader, layout, scenes, a static page per document with the
   findings of the tool. Acceptance: every example of orgspec and every architecture of
   agenticspec is drawn; every broken design of orgspec shows its rule on the part it names.
3. **Docs.** The generator, the component package, the Docusaurus preset. Acceptance: orgspec
   has a site; agenticspec's site is generated by metaspec with the parity list green.
4. **Model of the studio.** The ESS system `metaspec` and its `ess-ui/1` model, validated.
5. **Studio, read-only.** `serve`: pack, reference and document screens, live on file change.
6. **Drafts.** Generic edits, patches, revisions, comparison, save.
7. **Runs.** `simulate` and `search`, results, variants. agenticspec first; orgspec when it has
   a simulator.
8. **Agent.** The four tools, the agent screen, `metaspec mcp`.

## Decided in wave 1

What the build of wave 1 decided where this design was open or wrong. The open questions Q1 to
Q8 below kept their defaults.

- `contain` is `{ in: <field> }` or `{ per: <field> }`, exactly one key; the value names the
  part's field. The first draft wrote `{ field: in }` for orgspec, which the schema refuses.
  Both keys at once since 2026-11-28 ("Decided: containment by `in` and `per` at once").
- `visibility` is required: a manifest that forgets it is refused rather than published.
- `extends` is the name of a pack, not a path. Where that pack is checked out is given when
  metaspec is started, so a manifest names no path outside its pack.
- The KPI catalogue keeps its rows in a list `kpis`. `kpis.fields` defaults to `measure` and
  `modes`.
- A block is a domain of the compiled system, the domain of identifiers included. Its layer is
  the numbered directory, directly under `blocks.path`, of its file in the system's
  `ess-inputs.yaml` specification list.
- A block that the KPI catalogue names and the blocks do not have is a planned block: counted
  (`counts.planned_blocks`), not a warning.
- `pack check` runs nothing of the pack, also not `describe`. It says whether `tool.run`'s
  program can be started: a first element with a slash is a path inside the pack to an
  executable file, any other is looked up on PATH. `describe` is a verb every tool answers;
  `tool.verbs` lists the others (`graph`, `check`, `simulate`, `search`, `apply`). Since
  2026-11-28 there is no `describe`, and `tool.verbs` maps each verb to its adapter.
- The level a pack is at is the highest one its manifest declares (`level` in the report).
- The palette has twelve names: blue, violet, green, teal, amber, orange, red, pink, cyan, lime,
  brown, slate.
- The manifest is validated against its JSON Schema under `schema/` when it is read; the
  schema is the contract, not a copy of the code.
- orgspec's manifest has no `presentation`: the families of the first draft named
  `decision_class`, which is no block of orgspec, and every block in a family would be a choice
  of orgspec's authors. Without it a block takes the colour of its layer. It has no `tool` until
  orgspec answers a check (wave 2).
- The words of a pack come from `metaspec pack words <dir>`: block and layer names, link names
  (`carries` of documents and views, keys of a `links` mapping in a document), KPI ids (the
  catalogue and `kpis` lists in documents) and rule ids (`rules` lists in documents).
  `scripts/check-vocabulary.sh` finds a word with an underscore, hyphen or digit as a whole word
  anywhere under `crates/` and `web/src/`, and a word of letters only as a whole string literal
  (`"word"`): block names such as `tool` or `kpi` are common nouns, and metaspec's own code uses
  them; a table of a pack's block names in the code is a set of string literals.
- A rule id that a pack keeps only in its tool's source is not found by `pack words`.
  agenticspec has seven such rule ids today; its tool's `describe` will list them when
  agenticspec answers the protocol.
- The acceptance compares with the packs' own tooling: `check-layers.sh` (blocks, and layers
  from `--graph`), `check-kpis.sh` (KPIs), `orgspec check --json` and
  `orgspec type check --json` (orgspec's documents), and agenticspec's generated pages
  (`docs/architectures/`, `website/src/generated/blocks.json`) for its architectures and layers.

## Decided in wave 2

What the build of wave 2 decided where this design was open or wrong. Q1 to Q8 kept their
defaults.

- The reader (`metaspec-core::graph`, `schema/metaspec-graph-1.schema.json`): a part's `fields`
  are every field but `block`, a marked place read as its value; `kind` is the part's field
  `kind` when it is one text; `inside` or `per` is the field `contain` names. A link's id is
  `<from>-><to>:<carries>`, `?` for a missing end or name, `#<n>` appended to the n-th repeat:
  the same text orgspec's checker names a link with, so a finding's subject needs no table.
  `border` keeps the names as written, and `markers` of the graph holds the marked places
  outside parts and links (agenticspec's `policies`).
- A view without `blocks` shows every part, without `carries` every link. A link is shown when
  both its ends are shown or are no part at all; a name no part has is drawn as a dashed ghost
  box, so a finding on such a link is drawn too.
- Nothing of a document is hidden: when the views of a class leave a part or a link out, or the
  class declares none, metaspec adds the view "All parts and links" (`rows`, `implicit: true`).
- `rows` puts the highest layer on top. A view may say `rows: family` (new optional manifest
  field, default `layer`): a block in a family takes the family's row, ranked by the lowest
  layer of its blocks. A part that exists once per instance of another part (`per`) is drawn
  inside that part's box; a part that sits `in` another keeps its own row and says "in X".
  Names no part has get the bottom row, parts of a block the pack does not have the row above.
- `nested` draws both `in` and `per` inside the container. Parts flow left to right and wrap;
  an edge is a straight segment between box borders, and its label takes the first place along
  or beside it that covers no box text. Labels can still cross a line in a dense design; `rows`
  is the layout that guarantees no overlap.
- Lifted from agenticspec's renderer and made generic: rows with label rows between them,
  barycentre sweeps keeping the order with the fewest crossings, placement by isotonic
  regression, text widths by character class. Not lifted: the table of block names, channel
  cards, chips of a run.
- The palette's values: per colour a fill and a stroke per theme; text on a fill at least 4.5:1,
  a stroke on the page and on its fill at least 3:1, in both themes (`palette.rs`, held by a
  test). A block in no family takes the colour of its layer's position, in palette order.
  metaspec draws 17 icon names (`icons.rs`); any other icon name is drawn as its first letter.
- A finding's number is its position in the tool's answer. It is a mark `!<n>` at the top right
  of its part's box or at the left of its link's label (filled: gated; outlined: not gated), the
  marked link is dashed, and the row `<n>` of the findings list names the views it is drawn in.
  A finding about the document, or about a part or link the graph does not have, is a row only.
- Trust: `trust.json` in `$XDG_STATE_HOME/metaspec` (default `~/.local/state/metaspec`) maps the
  resolved pack directory to `sha256:` of the JSON of the manifest's `tool` section. Trust
  belongs to the directory, not to the manifest file: a second manifest with the same `tool`
  section is trusted too.
- A run: working directory the pack, stdin closed, its own process group, killed with all it
  started when it passes the deadline (`metaspec render --deadline`, default 30 s). The answer is
  validated against `metaspec-findings/1`; another version is refused with both named. The run
  is not cut off from the network yet, and it inherits the environment.
- `metaspec render --pack <dir> [--manifest <file>] --out <dir>` writes `index.html` and
  `<class>/<file stem>.html` per `parts-links/1` document. A page is one file: CSS and SVG
  inline, no font, no script that runs, no request; its data is `metaspec-document/1` in a
  `<script type="application/json">` element. A `records/1` class is listed on the index with
  its count; its pages come with the docs (wave 3). The drawing keeps its size and scrolls
  inside its frame rather than shrinking text.
- orgspec answers `check` with a subcommand of its own (`check <design>`); its manifest runs
  orgspec's built binary (Q8: metaspec builds nothing; `pack check` says when the binary
  is missing). Reversed on 2026-11-28: see "Decided: reversed the dependency". A violation's
  subject is `part:` when it names a part, `link:` when it names a link the design wrote or a
  default added, else `document` with the name kept at the start of the message (a name the
  border lists, the organization). A refused type is a gated finding about the document.
- orgspec's defaults add links the generic reader cannot see; a finding on such a link is a row,
  not a mark. orgspec's `graph` verb closes that; no broken design or example needs it today.
- The format keys `value`, `vary` and `message` are also words of real packs (a layer of orgspec,
  a link of agenticspec). metaspec reads them through typed structures, so the vocabulary check
  stays strict.
- Gaps of agenticspec's architectures, left to agenticspec: two blocks the documents use
  (`pool`, `schedule`) are no block of `blocks/`, so their parts are drawn as "block the pack
  does not have"; a link `over` a part (a channel) has no neutral field and is a field only;
  part fields that name other parts (`egress`, `acts_for`) are fields only; the architectures
  under `architectures/ladder/` are outside the manifest's glob.

## Decided: reversed the dependency (2026-11-28)

Ordered by the owner on 2026-11-28: a pack must know nothing of metaspec; metaspec may know the
packs. Until then orgspec carried the manifest in its own tree and a subcommand (`check`,
`describe`) that answered metaspec's protocol. What changed:

- The manifests of the public packs are metaspec's: `packs/orgspec.yaml` (restored from the
  manifest in orgspec's tree) and `packs/agenticspec.yaml` (the draft that was kept outside both
  repositories). They are embedded in the binary when it is built, so a pack is named by its
  name (`metaspec pack check orgspec --pack <dir>`, `metaspec pack known`). Any other manifest is
  given by its path (`--manifest <file>`). metaspec no longer reads a manifest from a pack
  directory; nothing published used it, so there is no fallback.
- The tool protocol (`describe`, `check` in `metaspec-findings/1`, exit 2 with `{error}`) is
  gone, and `metaspec-tool/1` with it. A manifest declares its verbs, each with an adapter that
  runs the pack's own command and maps its own answer (see "The tool adapter"). orgspec's
  adapter reads `orgspec check --json`: findings from `refusals` (each a gated finding about the
  document, the refused subject before the message) and `violations` (`rule`, `gate`,
  `message`; a subject with `->` is a link id, any other a part id, and an id the drawn graph
  does not have is about the document with the id before the message). The result is the same
  as the subcommand's: the 30 broken designs show the same rule ids as `orgspec check --json`,
  27 of 27 findings with a part or link subject are marked.
- The manifest format is `metaspec-pack/1`: `tool.verbs` changed from a list of names to a
  mapping from a verb to its adapter, which a reader of the first manifest format cannot read.
  A manifest in another format or version is refused with both named.
- Trust stays the digest of the manifest's `tool` section, adapters included.
- `extends` names a pack metaspec knows or one whose manifest is given (`--extends <file>`);
  `pack check` warns when it resolves to neither (`extends_not_found`) or to another pack.
- The fixture packs model the same rule: their manifests are `fixtures/manifests/<name>.yaml`,
  their directories hold nothing of metaspec, and their tools answer in two shapes of their own.
  `scripts/check-pack-dirs.sh` fails when a fixture pack, or a pack directory the acceptance is
  given, holds a file named like a manifest of metaspec.
- Added the same day by the owner: metaspec is the top-level application over all spec packs.
  No feature is built for one pack only; a feature one pack's tool supports is offered for
  every pack through the adapter, and each pack's gaps are listed ("Gaps per pack"). cloudspec
  is the third known pack; its manifest comes when it has blocks.

## Decided in wave 3

What the build of wave 3 decided where this design was open or wrong. Q1, Q2 and Q4 kept their
defaults. Rebased onto "Decided: reversed the dependency": the docs build takes the pack like
every other command.

- `metaspec docs build [NAME | --manifest <file>] --pack <dir> --out <dir> [--check]
  [--deadline <s>]` writes four kinds of file: `docs/<id>.mdx` (front matter, imports, Markdown, component tags),
  `pages/<id>.json` (`metaspec-page/1`: id, kind, front matter, the components the page draws and
  their data), `site.json` (`metaspec-site/1`: counts, page list, sidebar, presentation table) and
  `palette.css` (the colour classes `sa-c-<name>` and the page variables, light and dark). The
  output is the same bytes for the same pack and tool answers; `--check` prints `changed:`,
  `missing:` and `not owned:` lines and exits 1 when there is one. A write refuses a directory
  that holds files and no `site.json` of a docs build, and removes the files it no longer owns.
- A page's MDX imports its own JSON, `site.json` and the components from
  `@beyond10x/metaspec-ui`, and writes a tag per drawing:
  `<GraphView data={page.data["scene-1"]} presentation={site.presentation} />`. No MDX
  component table is swizzled, and a page is readable as Markdown without the components.
- Page ids: `index` (the pack: what it holds and its counts), `blocks/index` (layer map),
  `blocks/<N>-<layer>/<block>` (or `blocks/no-layer/<block>`), `blocks/planned`, `kpis`,
  `rules`, and per class `<class>/index` plus `<class>/<file stem>`. A class whose name is one
  of `blocks`, `kpis`, `rules`, `index`, `documents` gets `documents/<class>`. Repeated stems get
  `-2`, `-3`; a stem `index` gets `index-document`.
- A block's file comes from `ess-inputs.yaml`, and a block's domain is the longest domain prefix
  of a name (agenticspec's generator took two segments). The layer map is agenticspec's layout
  made generic: the lowest layer is drawn as a band only when it holds one block that every other
  block depends on; up to 5,040 orders per row are searched, above that barycentre sweeps.
- KPIs are grouped by the value of the manifest's `measure` field and filtered by measure,
  status, gate and block; the filter state is the URL query, so a filtered view is a link.
- A document page shows the check sentence, one section per view (`GraphView`), the findings
  (`FindingsList`), parts, links, border and marked places as tables, and `ResultSections` only
  when the manifest declares the verb `simulate` or `search` (`metaspec-pack/1` declares only
  `check` so far, so no page draws it yet). With no result committed beside the document it
  says so; the docs build runs no simulation. A `records/1` document is one `RecordTable` per
  table of its class.
- New field of `metaspec-pack/1`, `source_file_url` (optional, `https://...{path}`): the front
  matter's `source_url` of each page, so the docs-system header links the file a page was
  written from. It joins `/2` before any release of `/2`, so the version stays.
  `packs/orgspec.yaml` and `packs/agenticspec.yaml` set it to the file on the pack's `main`
  branch; the fixture manifests leave it out.
- A pack that is not public is refused by the preset unless the site's `url` is localhost or
  127.0.0.1, and every page of such a pack carries a banner that says so.
- `@beyond10x/metaspec-docusaurus` exports `withMetaspecSite(config, {out, siteDir,
  routeBasePath})`: one docs plugin instance (id `metaspec`, `numberPrefixParser: false` so
  `1-shore` stays in the route) over `<out>/docs` with the sidebar of `site.json`, and a plugin
  that loads the components' CSS and `palette.css` and passes Docusaurus's `Link` and the route
  base to the components (`configure`). Paths in the plugin options are relative to the site, so
  no machine path reaches the bundle. The site then wraps the config with docs-system's
  `withProductSite`. The packages are private and built to `dist/` (Docusaurus does not compile
  TypeScript under `node_modules`); a site outside `web/` installs them with
  `npm install --no-save --install-links <dir>`, which copies rather than links, so React is
  loaded once.
- A site needs a `browserslist` like agenticspec's: without one, Docusaurus's CSS step raises
  the specificity of Infima's defaults over docs-system's colours and links fail contrast.
- The fixture packs' sites are committed (`fixtures/expected/<pack>/site`), compared byte for
  byte by a test (`METASPEC_BLESS=1` rewrites them) and by `task check:docs`; the vitest suite of
  `web/ui` draws every component tag of every committed page. A real pack's site is generated at
  build time and not committed.
- A pack carries nothing of metaspec ("Decided: reversed the dependency"). A real pack's site is
  built from metaspec's side: `metaspec docs build orgspec --pack <orgspec>` and the site shell
  of `web/site` over the output (`METASPEC_SITE_OUT`). Where a real pack's site shell lives in
  metaspec (for example `sites/`) is the next step's decision. This replaces "A pack's
  `website/` holds its identity" above.
- The Playwright and axe-core gates (both fixture sites, 1440x900 and 1280x800, both themes,
  no serious or critical violation, no sideways scroll) are the lane `task ux`, outside
  `task check`: building two sites does not fit the 120 s step with the rest of the gate.
- Parity with agenticspec (`scripts/parity.sh`, outside `task check`): every block page, the
  layer map, planned blocks, every architecture page, the KPI catalogue with all rows and every
  component have a counterpart. Missing: the `Policies` section of an architecture page (the
  marked places outside parts and links are in the graph, `markers`, but not drawn as a section
  of their own). Expected missing: the catalog and every section of a run result.

## Decided for the meta model (wave 4, 2026-11-28)

The owner's decisions of 2026-11-28 and what writing `spec/` decided. The ESS system is
`metaspec`.

- A pack knows nothing of metaspec. metaspec holds the manifests (and the adapters that map a
  pack's own tool output) of the public packs it knows, `packs/<name>.yaml`; a private pack's
  manifest is given by path at run time.
- metaspec is the top-level application over all packs: what it does for one pack it offers for
  every pack, and the features a pack cannot use yet are listed per pack.
- The meta model of a pack lives in metaspec as the ESS system `spec/` (chosen over a format
  owned by ESS and over a repository of its own). It has two groups of domains: the pack's
  meta model (`packs`, `blocks`, `documents`, `checks`) and the studio (`drafts`, `runs`,
  `agents`), plus `quality`: measures of every pack's KPI catalogue and specification, their
  values per pack and report, and a baseline that takes a better value and fails a worse one.
- `spec/ess-inputs.yaml` pins its ess release (`requires: ess 0.99.11`); every ess command runs
  inside `spec/`, where the pin applies. `task check` runs `check:spec`
  (`scripts/check-spec-conform.sh`): validate with `--strict-requires`, then the conformance
  suite on the interpreted target, synthesized once per digest of the sources under
  `target/spec-conform/`. `spec/known-failing.txt` lists the scenarios that fail because the
  synthesized scenario is wrong.
- Pack data is read from files: those entities are `provided: true` and no command creates them.
  What the pack's tool answers enters through commands of the tool runner (`RecordCheck`,
  `RecordFinding`, `EndRun`, `RecordVariant`).
- The level of adoption is a field of the pack held to the manifest by six invariants (the
  rule of "Decided in wave 1"); ESS has no conditional value, so it is checked, not computed.
- The generic edits are the variants of one `Edit` union carried by a patch; `apply` with
  `dry_run` is two commands, `ApplyPatch` and `DryRunPatch`, because ess 0.99.9 answers no typed
  response from a branch that writes nothing beside one that creates a revision.
- A variant is written as a patch (the search uses the same operation set) and is accepted when
  a revision takes it; it keeps one state.
- The `ess-ui/1` model of the screens is not written yet: `ess ui check` needs a component
  reached over the network and refuses the reads of the domains that have no command.

The 22 `UNMAPPED:` markers of the first pass, settled on the owner's rule "decide, don't ask"
with the author's recommendation (marker locations as in that pass; `spec/` holds none of them
now):

- D1 `packs.yaml:113` adapter: the manifest is metaspec-pack/1; `Pack.check_adapter` is a
  `CheckAdapter` (args, exit, at, error, valid test, counts, finding lists of pointer templates
  with a gate test and a subject rule), present exactly when `check` is a tool verb. Reason: it
  is what `crates/metaspec-core/src/adapter.rs` does. ESS form: `check` is the only verb with an
  adapter (the code has no other); there is no format name (the code maps onto
  metaspec-findings/1 only); the untagged `gate` and `subject` of the manifest are tagged unions,
  as ESS has no untagged union.
- D2 `packs.yaml:197` gap key: one gap per pack and feature, `kind` (screen, measure, verb) and
  `feature` (its id). Reason: a level is too coarse to say what a pack cannot use.
- D3 `blocks.yaml:70` dependency edges: a provided `Dependency` row (pack, from, to, count).
  Reason: an edge has its own count and the layering measure reads edges, not blocks.
- D4 `blocks.yaml:128` KPI columns: `direction` and `status` typed as the pack's words
  (`Optional<String>`), `gate` a `Boolean` (whether the KPI gates); `how` stays in `columns`.
  Reason: the KPI-quality measures read them, and every catalogue writes `gate` as a Boolean on
  every row: orgspec `kpis/catalogue.yaml` 62 `gate: false` and 49 `gate: true` (111 rows),
  agenticspec 91 false and 34 true (125 rows), and both fixture catalogues `true`/`false`.
- D5 `checks.yaml:84` check result: it references its document; a check of a draft's revision
  records one, with an optional `revision_id` (RecordCheck and CheckRevision, D17). Reason: a
  result is a record of a run, not a part of the document.
- D6 `checks.yaml:125` finding to rule: an optional `rule_id` reference beside the tool's text.
  Reason: a rule the pack's rules do not list must still be recorded.
- D7 `drafts.yaml:21` edit targets: a part by its name, a link by its id
  (`<from>-><to>:<carries>`, `#<n>` for repeats). Reason: the same ids the reader and findings
  use (wave 2).
- D8 `drafts.yaml:40` `set_field` reaches parts, links and the document's own keys. Reason: one
  edit for every value a subject names.
- D9 `drafts.yaml:100` pack edit verb: one `Edit` variant `pack_verb {verb, arguments: Json}`.
  Reason: metaspec passes a pack's arguments on and keeps no pack vocabulary.
- D10 `drafts.yaml:150` after save: `Saved` is final; undo is a new revision that reverts.
  Reason: revisions are only ever added, so side-by-side comparison keeps every step.
- D11 `drafts.yaml:175` revision findings and KPIs: stored with the revision (`findings`); the
  KPI values wait for a schema of metaspec-result/1. Reason: a comparison must not re-run the
  tool.
- D12 `drafts.yaml:347` accepted variant: only into the draft whose revision its run read; a
  variant of a run on the file goes into a draft of that document, opened first. Reason: a
  variant is a patch on that base. ESS form: an `external:` refusal (`other-draft`,
  `NotTheRunsDraft`). A typed guard over a copy of the run's draft on `Run` and `Variant`
  validated, but ess 0.99.9 synthesis then built no scenario of AcceptVariant (ESS-SYNTH-003)
  and left StartRun without a revision unwitnessed (ESS-SYNTH-020).
- D13 `runs.yaml:57` run result: no typed members until metaspec-result/1 has a schema (a
  comment). Reason: no contract to type it from.
- D14 `runs.yaml:103` variant KPI values: the same, with metaspec-result/1.
- D15 `agents.yaml:80` session's pack: carried by the draft (`Draft.pack`, read from the new
  `Document.pack` when the draft is opened); the session keeps only `draft_id`. Reason: one
  source for the pack, checked when the draft is opened.
- D16 `agents.yaml:104` tool call: one record written after the answer, one state. Reason: the
  answer is what is kept.
- D17 `agents.yaml:120` agent tools: `query` reads views and is no command; `check` is the
  command `CheckRevision` (agent and user), which records a `CheckResult` of the revision.
  Reason: a check of a draft is a run of the tool like any other. ESS form: only a check the tool
  answered is recorded (`answer: ran`); one it did not answer is the `external:` refusal
  `NoAnswer`. A recorded answer the runner supplies broke the invariant "`ran` has `valid`" on
  the interpreted target (ESS-CF-TARGET), and ESS has no conditional value.
- D18 `agents.yaml:243` consent: a channel per pack, as written. ESS form: the private-pack
  refusal stays `external:`, as ess 0.99.9 cannot guard the pack's visibility and its channel
  rows in one predicate.
- D19 `quality.yaml:38` directions: `higher_is_better` / `lower_is_better`. Reason: metaspec's
  own words; a pack's catalogue keeps its own.
- D20 `quality.yaml:89` metaspec is a pack with its own KPI catalogue; `Measure.kpi_id` is
  required. Reason: every measure is a KPI row.
- D21 `quality.yaml:150` ids behind a value: one `Evidence` row per id (`RecordEvidence`), with
  optional typed references to a KPI, rule, document, finding or block; the `behind` list is
  gone. Reason: ESS relates one field, not the elements of a list.
- D22 `quality.yaml:179` baseline: a file committed in metaspec, one per measure and pack.
  Reason: a ratchet is reviewed in the diff like the code it guards.

The design review of 2026-11-28 (`spec/hardening/design-review-1.md`) and what fixing it decided.
A rule ESS can declare is a typed guard or invariant where ess 0.99.9 synthesis writes a
scenario for it; where the typed form costs scenarios (a guard over a provided row's fields, or a
row-set guard that blocks the arrangement of another command), it is an `external:` refusal with
a comment in `spec/` that names the typed form and the refusal it got.

- D23 `Document.pack` is the pack the document comes from (the pack whose manifest's paths
  select its file), not the pack of its class: with `extends` a private pack's document of a
  public pack's class carries the private pack, so its drafts fall under the private-pack consent.
- D24 The agent-channel consent holds for every call of the agent, not only for StartSession:
  ApplyPatch, DryRunPatch, CheckRevision, StartRun and RecordToolCall carry the external refusal
  `agent-channel-off` (one `outcome_groups:` entry in `agents.yaml`; external for D18's reason).
  TurnOnChannel refuses a second channel of a pack (`ChannelExists`).
- D25 SaveDraft ends the open agent sessions of the draft it saves (an `affects:` move), and
  StartSession refuses a saved draft. Reason: the agent works on the open draft.
- D26 A pack has its resolved `directory`; `Pack.trust` references the Trust row of that
  directory (none: not trusted). TrustPack takes the pack, the directory and the digest the user
  was shown, and refuses another directory, a pack without a tool and a digest that is not the
  pack's (external: guards over the provided Pack's fields get no scenario, ESS-SYNTH-004).
  StartRun's `not-trusted` covers a trust of another digest (a changed tool section asks again);
  a tool without the verb is its own refusal `no-verb`.
- D27 RecordCheck records a check of the document file and takes no revision; a check of a
  revision is CheckRevision, which holds the revision to its draft (amends D5). A revision copies
  its draft's `document_id`, so StartRun refuses a revision that does not exist or belongs to
  another document (its `other-document` refusal gets no scenario from ess 0.99.9 synthesis,
  ESS-SYNTH-003). `DocumentChecked` and `RevisionChecked` are two events.
- D28 A private pack's baselines are in the user's state directory, keyed by the pack's name; a
  public or fixture pack's are committed in metaspec (owner, 2026-11-28; amends D22). A
  baseline's direction is the measure's, and one baseline per pack and measure is refused as a
  second (both external: the typed forms cost Ratchet's scenarios).
- D29 New rows the design names: `checks.RuleCount` (counts per rule, metaspec-findings/1),
  `runs.KpiValue` (a KPI value per document and per part of a finished run, metaspec-result/1;
  amends D13) and `blocks.Declaration` (entities with their lifecycles, commands with their
  refusals, events, views and types of a block, with source lines; metaspec-model/1). The best's
  and the variants' values are not KpiValue rows: D14 stands (a comment in `spec/`).
- D30 A measurement has an optional breakdown key (a rule id, a mutant class: the pack's data, no
  vocabulary of metaspec) and a `value` or a `gap_id` referencing the pack's Gap, replacing the
  `Reading` union; evidence has an optional `value`. A measure of another question than its
  report is refused (external: it reads the provided Measure).
- D31 A document class carries an optional `expect: invalid` for a class of broken examples
  (decided in the plan store); gate fidelity and refusal power read it.
- D32 A document has the `digest` of its file; a draft starts from it, and a check result records
  the digest of the text it checked and when (`checked_at`), so the newest result is a
  document's check status and a result of another digest is stale.
- D33 A draft of a records/1 document takes only the pack's edit verbs (`pack_verb`); a generic
  edit of it is refused as PatchRefused. The agent's StartRun refuses `search` (external
  `agent-search`: ESS has no guard over the caller's actor). `UntrustPack` is
  `metaspec trust --remove <name> --pack <dir>`.
- D34 Not declared, ESS limits, each with a comment in `spec/`: a pack with `extends` is private
  (the invariant validates, and the interpreted target then mints no provided Pack, ESS-CF-TARGET);
  one gap per pack and feature (no command creates a Gap, and a struct identity breaks the
  `gaps` relation, ESS-ENTITY-005); revisions numbered from 1 in order (no value source counts
  rows); a measurement's gap is its report's pack's and names its measure (a cross-row comparison
  of text); a measurement holds not both a value and a gap (synthesis then writes no scenario of
  RecordMeasurement, ESS-SYNTH-003).

## Decided: one site for every pack (2026-11-28)

The owner, on the orgspec site: "the header says orgspec all the time, but this is now about
metaspec", "or navigation". One site holds every pack; this replaces the site per pack of
"Decided in wave 3" (one docs plugin instance `metaspec`, the pack's title in the navbar,
`METASPEC_SITE_PACK`, `fixtures/expected/<pack>/site`).

- The title of the site and of every page's header is `metaspec`. The navbar has one item per
  pack, with the pack's title, that opens the pack's own sidebar.
- `metaspec docs build --pack <NAME|FILE>=<dir> [--pack ...] --out <dir> [--check]
  [--deadline <s>]` writes one site over every pack given: NAME is a pack metaspec knows, FILE a
  manifest (a path with a `/`, or ending in `.yaml` or `.yml`). The single-pack form
  `[NAME | --manifest <file>] --pack <dir> [--extends <file>]` stays and writes a site of that
  one pack, in the same layout. Two packs of one name are bad usage (exit 2).
- Layout of the output, per pack `<pack>/docs/<id>.mdx`, `<pack>/pages/<id>.json` and
  `<pack>/site.json` (`metaspec-site/1`, unchanged in shape); for the site `index.json`
  (`metaspec-index/1`, `schema/metaspec-index-1.schema.json`: the title and, per pack ordered by
  name, its identity, route `/<pack>`, the file of its part, its counts and the check of its
  documents), `_index/index.mdx` (the page `/`) and one `palette.css`. `_index` is a name no
  pack can take (`^[a-z][a-z0-9-]*$`), and a pages plugin over the output's root would also
  compile the packs' MDX. A write accepts a directory with an `index.json` of a docs build, or a
  `site.json` of the earlier layout, and refuses any other that holds files.
- A pack's pages are routed below `/<pack>`: the `slug` of each page in `site.json` and every
  `route` in the components' data carry the prefix, so the components link with no base of their
  own (`configure({base: ''})`). The MDX of the pages is byte for byte what the site per pack
  wrote; only the routes in the data changed.
- The check of a pack on `/`: per drawn document, `valid` when the tool ran and found it valid,
  `refused` when the tool refused it or found it invalid, else `not checked` (no tool, not
  trusted, timed out, failed). The pack is `refused` when one document is refused, `valid` when
  every drawn document was checked and none refused, else `not checked`. Records documents are
  not checked and not counted.
- The page `/` is Markdown, readable without components: one table row per pack with its title
  (a link to `/<pack>`), summary, blocks, documents, KPIs and the check with its counts.
- `@beyond10x/metaspec-docusaurus`: `withMetaspecSite(config, {out, siteDir})` adds one docs
  plugin instance per pack (id and route base the pack's name, its sidebar from its
  `site.json`), a pages plugin instance `metaspec-index` over `<out>/_index`, and the plugin of
  the styles and links; `packNavbarItems(index)` gives the navbar items (`docSidebar`), and
  `readIndex(out)` / `readSite(out, pack)` read the files. A site with a pack that is not public
  is refused unless its `url` is this machine.
- `web/site` builds the site of `METASPEC_SITE_OUT`, by default the committed
  `fixtures/expected/site`, the one site of both fixture packs, compared byte for byte by a test
  and by `task check:docs`. The UX lane builds it once (`web/site/build/fixtures`), serves it
  once, and checks `/` and every page of both packs; on each page the navbar title is metaspec
  and every pack is a navbar item, and `/` lists every pack with its counts and check.
- `scripts/acceptance.sh` builds the one site of both real packs (`target/acceptance/docs-site`,
  through `web/site` into `target/acceptance/site`): per pack its HTML pages equal its pages, and
  the site has its page `/`. `scripts/parity.sh` reads the pack's part, `<out>/<pack>`.

## Decided for the quality layout (2026-11-28)

- The quality layout (`story:kpi-quality-catalogue-shape`): `metaspec quality` with verbs
  `kpis`, `spec`, `ratchet`, and `render --quality <report.json>` (repeatable), wired once in
  `crates/metaspec/src/main.rs`; the arguments and the shared run of a question are in
  `crates/metaspec/src/quality/mod.rs`. `metaspec_core::quality::registry` maps each measure id
  to a `Module` (`file`, `measure`, `built`) in the file of the story that builds it; the module
  says itself which of its measures it builds, so a story that fills a stub changes its own file
  only. A measure reads an `Input` (the pack, the state directory of trust, the deadline) whose
  check of every document runs at most once (`Input::check_pass`, `quality/check_pass.rs`). A
  catalogue row of status `measured` whose entry is a stub (`measure_not_built`) or absent
  (`measure_not_registered`), and a registry measure without a row (`measure_without_row`), are
  refused before anything is measured.
- `completeness.fields`, decided where the story left it open: "a direction per mode" asks for a
  direction for every mode any row of the catalogue writes (the modes field of
  `kpis.fields.modes`); a catalogue that writes no mode asks for none. A gate value that is
  neither a number, `true`, `false` nor absent is no threshold metaspec can read and counts as
  incomplete. A share over nothing (no blocks, no KPI rows) is a gap, never 0.
- A name behind a value is never blank: a KPI row without an id, or with an empty or blank one,
  is named `#<index>` by its place; a block with a blank name (a `strip_prefix` that takes the
  whole domain) by its domain.
- The quality page (`story:kpi-quality-page`): `render --quality` writes `quality.html` from the
  reports given, one section per report in the order of the questions (`kpi_catalogue`, then
  `specification`), one row per measurement the report holds, so a later measure needs no change
  of the page. The page carries each report as it is, format `metaspec-quality/1`, in its own
  data element `metaspec-quality-<question>`; no new format. The index links the page only when
  a report is given (one `<nav aria-label="Pages">` line after `<main>`), so the index goldens
  stay as they are. A report that is unreadable, not of the format, of another pack, or a second
  report of one question is refused with exit 1 and no page; every render first removes a
  `quality.html` an earlier render left. The goldens read recorded reports
  `fixtures/quality-reports/<pack>/kpis.json`, never the live output of `quality kpis`; they are
  written again only when the page is meant to change.
- `validity.refusals` counts what `ess specify validate --format json` refuses: each entry of
  `problems` of a refusal (`compiled: false`, exit 1), written `[<code>] <subject>: <message>` and
  named `<code> <subject>`, and each entry of `scenario_refusals` of a report (`valid: false`,
  exit 1), named `<code> <scenario>` (its origin where it names no scenario). The `diagnostics` of
  a refusal are counted only when it lists no problem (named `<code> <span.path>`), so a refusal
  that names anything never counts 0. `validity.warnings` counts `warnings`, named `<code>
  <span.path>`. A name behind a value is never blank (`#<index>`).
- ESS completeness reads `completeness.counts`; a report without `completeness` owes nothing (ess
  leaves it out then), so the counts are 0. A refusal, and a report whose `completeness` lists
  `unsynthesizable`, make the three rows gaps (`needs: L0`) naming the refusal: nothing was
  synthesized, so a zero would be silent. Behind: `<code> <subject>` of each unscenarioed
  construct and each unanswered question (kind `finding`), each scenario held outside (kind
  `scenario`).
- `honesty` counts lines, as `grep -ra 'UNMAPPED:'` does: two markers on one line are one, and
  every regular file under `blocks.path` is read, hidden ones and files that are no YAML included;
  symbolic links are not followed. Behind: `<file>:<line>` relative to the pack (kind `marker`).
- `adoption_level` is `Manifest::level` as a number. The verbs metaspec offers are the fields of
  `manifest::Verbs`, each with the level that needs it (`OFFERED_VERBS`: `check`, L3); a verb the
  manifest does not declare is a gap of kind `verb` of the report, added by
  `static_measures::report` after the registry's run (the registry adds gaps of kind `measure`
  only). Found while building: `packs/agenticspec.yaml` declares `check` since 2026-11-28
  (b788cc3), so agenticspec has no verb gap; the acceptance shows the gap on a manifest of the run
  made from it without `tool`.
- The answer of `ess` (`quality/spec/static_measures/ess_answer.rs`): `ess <words> --path .
  <options>` run with the blocks directory as working directory, as ess asks: ess 0.99.11, the
  release `spec/ess-inputs.yaml` pins, refuses a run from the pack directory whose
  `blocks/ess-inputs.yaml` pins another release (exit 1, nothing on stdout), and from the blocks
  directory hands the run to the pinned release; `ess --version` runs in the same directory, so
  the cache key names the release that answers. Stdin is closed; after 600 s, or the seconds of
  `METASPEC_ESS_DEADLINE` (raised to 1 ms when below), the process group gets SIGTERM and 2 s
  later SIGKILL; the pipes are read on threads that keep what they read, and are read 200 ms more
  after SIGKILL, so a process that left the group (as a conformance target does) and holds a pipe
  does not keep the run: it lasts at most the deadline + 2.2 s. When ess exits and a process it
  left in its group keeps a pipe open, the pipes are read until the deadline, 2 s after the exit
  at most, and the group is then stopped the same way. The 600 s: a synthesis of agenticspec's
  blocks took 67 s, orgspec's 43 s, over the 30 s `--deadline` of the pack's tool, which does not
  apply to ess. A run that gives no answer is a gap naming the first five lines of ess's stderr,
  `note:` lines left out when it said more; within one `scoped` run the same request is not run
  again after a failure. Cached by the SHA-256 of `ess --version`, the words and options, and the
  path and content of every file under `blocks.path`, in `$XDG_CACHE_HOME/metaspec/ess` (an
  absolute path only), else `$HOME/.cache/metaspec/ess`, never in the checkout; nothing is cached
  without either, and a failed run is never cached. `METASPEC_ESS` names the program (default
  `ess` on PATH); `ess_answer::scoped` gives the program and cache for one run on the thread and
  asks each answer once in it. Later stories call `ess_answer::answer` with their own words and do
  not change the file.
- The stub `fixtures/ess/ess` (POSIX sh) answers `--version` with `ess stub` and `ess <words>
  <verb> [--options]` with `fixtures/ess/<pack>/<verb>.json`, the pack being the name of the
  directory above its working directory (the blocks directory); exit 2 naming the pack and verb
  when no answer is recorded, and exit 2 for a `--path` other than `.`, so a run that names
  another path fails the tests as it fails with the real ess. The recorded validate answers of
  harbor and orchard are invented in ess 0.99.2's shape (a report with one scenario refusal, one
  warning and completeness counts); the fixture packs' own blocks answer nothing owed.
- A pack word that is also a word of one of metaspec's own formats is metaspec's word, not a leak:
  `scripts/check-vocabulary.sh` leaves out every property name and every string enum or const
  value of `schema/metaspec-*.schema.json` (read with jq) before it searches, and its last line
  says how many it left out (`37 word(s) of 2 pack(s), 0 of them metaspec's own, none in the
  code`); `scripts/check-vocabulary.test.sh` holds both sides.

## Decided: containment by `in` and `per` at once (2026-11-28)

`story:contain-in-and-per` of `epic:cloudspec-pack`: cloudspec places one part `in` a part and
`per` another at once. This replaces "exactly one key" of wave 1 and "`inside` or `per`" of
wave 2.

- The manifest's `contain` is `{ in: <field> }`, `{ per: <field> }` or `{ in: <field>, per:
  <field> }` (`metaspec-pack/1`, at least one key). The single forms read as before.
- `metaspec-graph/1` is unchanged in shape: a part keeps `inside` and `per` both. `groups` stays
  one group per containing part and word: a part that holds parts by `in` and by `per` has an
  `in` group and a `per` group, the `in` group first, so `contain` stays `in` or `per`.
- `nested` draws a part with both inside its `in` container (its `per` container when the `in`
  one is not shown) and writes the other as a line of its box, `per <part>`. `rows` draws it
  inside its `per` host as before, and its line there ends `· in <part>`. A part with one of the
  two is drawn as before, so no existing scene changes.
- The table of parts on a document's page says `in <part>, per <part>`.
- The fixture pack harbor holds it: `contain: { in: within, per: one_per }` and the plan
  `harbors/lagoon.yaml`, whose berth sits in the quay and exists once per ferry.

## Open questions

Each with the default this design takes.

- Q1 The studio in React, because the docs components are React and are shared. agenticspec's
  session application stays Vue. Default: React.
- Q2 The component package lives in metaspec, not in docs-system. Default: metaspec.
- Q3 Layout in Rust in the server, scenes to the browser. Dragging a part in the editor may need
  the layout in the browser later (the same crate compiled to WebAssembly). Default: server.
- Q4 agenticspec moves to metaspec after parity, not before. Default: after.
- Q5 Selectors. agenticspec's epic selects parts with its query language. Default: edits name
  ids first; selectors come when that language is a crate of its own.
- Q6 Saving a draft keeps comments and order of the YAML file. Default: yes, and an edit that
  cannot be written that way is refused; the alternative is a pack tool's `apply` that writes
  the file.
- Q7 Lifecycle drawing: metaspec's own component or the page of `ess-ui-templates`. Default:
  own component for the diagram on a block page, a link to that page for playback.
- Q8 A tool that is not installed. Default: `tool.run` is an argv the manifest names, and
  `metaspec pack check` says when it cannot be started; metaspec builds nothing of a pack.
