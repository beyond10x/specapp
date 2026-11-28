# specapp: one docs site and one studio for every specification pack (draft 1, 2026-11-27)

Status: waves 1 ("Contract") and 2 ("Graph") are built: the manifest, the schemas, two fixture
packs, `specapp pack check`, the `parts-links/1` reader, the layout, `specapp trust` and
`specapp render` (a static page per document with the findings of the pack's tool). The build
order is at the end; "Decided in wave 1" and "Decided in wave 2" list what the builds decided
that this design had left open. On 2026-11-28 the owner reversed the dependency between specapp
and the packs: specapp knows the packs, a pack knows nothing of specapp ("Decided: reversed the
dependency"). Where an older section below says otherwise, that section wins.

## What it is

Several Beyond10x repositories have the same build: an ESS system of building blocks in numbered
layers, a KPI catalogue, documents that compose the blocks into a graph of parts and links, and a
tool that checks such a document and, in some of them, simulates it. agenticspec describes
agentic development this way. orgspec describes an organization this way.

specapp is the user interface of such a repository, written once:

- **Docs**: a static site. Every block by layer, every lifecycle, the KPI catalogue, every
  document drawn as a graph with its check result.
- **Studio**: a local application. The same pages live, plus work on a document: edit it, check
  it at every change, run it, compare variants, keep drafts, and do all of that together with an
  agent.

A repository that specapp can load is a **pack**. specapp describes a pack in one manifest of
its own; the pack knows nothing of specapp. specapp's code contains no word of any pack: no
block name, no layer name, no colour of a block, no rule id. The words are in the manifests.

specapp is the top-level application over all spec packs. The packs it knows today are
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

## The principle: specapp knows the packs; a pack knows nothing of specapp

1. specapp reads a pack through a manifest of its own and a small set of neutral formats. The
   manifest and its adapters are specapp's: the manifests of public packs are in specapp's
   `packs/`, a private pack's manifest stays outside this repository and is given by its path
   at run time. A pack holds no file of specapp, names no format of specapp and has no command
   for specapp; it never adapts to specapp. specapp adapts to the pack: the manifest states
   where the pack's things are, what they are called and how its own tool answers.
2. Reading costs nothing to adopt and executes nothing. Behaviour (check, run, apply) comes from
   the pack's own tool, run as the pack ships it and read by an adapter in the manifest (see
   "The tool adapter"), generic in specapp's code.
3. One model, one component library, two shells. The docs site is the static export of the
   read-only part of the studio. A page looks the same in both.
4. What specapp shows is what the pack's tool says. specapp implements no rule of any pack. A
   finding, a KPI value and a refusal are the tool's.
5. A private pack stays on the machine. See "Private packs".
6. One application for every pack. No feature of specapp is built for one pack only. A feature
   that one pack's tool supports is offered for every pack through the manifest's adapter, and
   what a pack lacks for it is listed as that pack's gap ("Gaps per pack").

## A pack

specapp holds the manifest of every public pack it knows, `packs/<name>.yaml`, format
`specapp-pack/2`; `specapp pack check <name> --pack <dir>` reads it by the pack's name. A pack
specapp does not know, such as a private pack, is read with a manifest given by its path:
`specapp pack check --manifest <file> --pack <dir>`. The pack directory is always given
separately; nothing in it is read as a manifest.

```yaml
format: specapp-pack/2
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
`contain: { per: per }` and one view, and no `tool` yet (see "Gaps per pack").

Rules of the manifest:

- It is specapp's file and lies outside the pack: under `packs/` for a public pack specapp
  knows, anywhere outside this repository for a private one. A pack directory holds no file
  named like it; `scripts/check-pack-dirs.sh` fails on one.
- Every path is inside the pack. A path that leaves it is refused.
- `presentation` is optional. Without it a block's row is the layer of its domain and its colour
  is the colour of its layer, from a palette of specapp. This is the one place a pack states a
  colour or an icon; a pack that needs none writes none.
- No number about the pack is written in the manifest or in any copy: counts are counted.
- `extends` names another pack: a pack specapp knows (`packs/<name>.yaml`), or one whose manifest
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
- **L5 edit.** The tool's `apply`, or specapp's generic edits: drafts, revisions, comparison,
  the agent channel.

orgspec today reaches L3: its manifest in specapp reads `orgspec check --json` through an
adapter. agenticspec reaches L2 today (see "Gaps per pack").

## Neutral formats

specapp's own formats. A pack's formats stay the pack's.

- `specapp-pack/2`: the manifest, with the adapters of the pack's tool. One producer: specapp's
  maintainers (`packs/`), or the owner of a private pack (a file outside this repository).
- `specapp-pack-check/1`: what `specapp pack check --json` prints. `valid`, `errors` and
  `warnings` (`{code, subject, message}`); for a pack that loads also its name, visibility,
  `extends`, the level it declares, `counts` (blocks, layers, KPIs, planned blocks, document
  classes, documents), the layers with their blocks, the documents per class, and the state of
  the tool (`none`, `found`, `not_found`). One producer: specapp.
- `specapp-model/1`: the blocks, from the compiled ESS model. Layers, blocks, dependency edges
  with counts, and per block its entities, lifecycles, commands, events, views, types and source
  lines. One producer: specapp, from `ess specify compile --format json`.
- `specapp-kpis/1`: the catalogue with neutral field names (`measure`, `modes`), the pack's
  words for their values kept as data, and the pack's own columns kept as extra columns.
- `specapp-graph/1`: one composed document. `parts[{id, block, kind, fields, inside, per,
  on_border, markers}]`, `links[{id, from, to, carries, fields, added_by}]`, `groups` from
  containment. `added_by` names a default of the pack that added a link the document did not
  write. Produced by specapp's reader for `parts-links/1`, or by an adapter of the tool's
  `graph`.
- `specapp-findings/1`: a check result. `valid`, and `findings[{rule, gate, subject, message}]`
  where `subject` is `part:<id>`, `link:<id>` or `document`. Counts per rule. Produced by
  specapp only: the adapter of the manifest maps the tool's own answer to it.
- `specapp-scene/1`: a laid-out view of a graph: boxes, groups, edge paths, labels, in
  coordinates. Produced by specapp's layout, deterministic for the same input. The docs site
  and the studio draw the same scene.
- `specapp-result/1`: a run. Baseline and best, KPI values per document and per part, the
  marked places, variants, a trace. agenticspec's `agenticspec-sim-result/1` is the model for
  it; an adapter of the manifest maps the tool's own result to it.
- `specapp-patch/1`: an ordered list of edits. See "Operations".
- `specapp-page/1`: one generated page: front matter, Markdown, and the data of its components.
- `specapp-document/1`: the data of one page `specapp render` writes (wave 2): the graph, the
  check, the findings with the views they are drawn in, the scenes. One producer: specapp.
- `specapp-trust/1`: the trust file in the user's state directory. One producer: specapp.

Every format has a JSON Schema in `schema/` and a version in its name. A manifest in a version
specapp does not know is refused with both versions named. A pack's tool answers in the pack's
own format; specapp never asks it for one of its own.

## The tool adapter

The pack's tool is a process the pack ships for its own use; it knows nothing of specapp. For
each verb specapp offers, the manifest's `tool.verbs` holds an adapter: how specapp runs the
tool and how it maps the tool's own answer to a neutral format. The mapping is declarative and
generic in specapp's code; no adapter names a word of a pack in the code. A verb the manifest
does not declare is not offered for that pack; nothing is asked of the tool (there is no
`describe`).

`check` (L3) is the verb built today. specapp runs `<tool.run> <args> <document>` in the pack
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
- `findings`: lists of findings, each `{at, rule, gate, message, subject}`, in the order the
  findings are shown. `rule` and `message` are templates: `{<pointer>}` stands for the text at
  that pointer in one entry. `gate` is `true`, `false` or a test of the entry. `subject` is
  `document`, or a rule: `id` (a template) and either `link_when_contains: <text>` (an id that
  contains the text is a link, any other a part) or `kind: <template>` with `kinds` (the
  template's value names `part`, `link` or `document`). `unknown: document` makes an id that the
  drawn graph does not have a finding about the document, with the id put before the message
  unless it is the document's name.

The result is `specapp-findings/1`, validated against its schema; an answer the adapter cannot
read is a failed check, shown as such. orgspec's adapter reads `orgspec check --json`; the two
fixture packs answer in two other shapes (a list with exit 1 for an invalid plan, and an object
with a verdict word and a kind field per finding), so the mapping is tested on both.

The verbs to come take the same form when they are built: `graph` (an adapter to
`specapp-graph/1`, where the generic reader is not enough, such as a pack whose defaults add
links), `simulate` and `search` (to `specapp-result/1`; specapp passes the budget and a
deadline, a run over the deadline is killed and shown as such), and `apply` (a
`specapp-patch/1` translated to the tool's own edit, or specapp's generic edits).

Running a pack's tool is running code from a repository. So:

- L0 to L2 execute nothing of the pack.
- specapp runs a tool only for a pack the user trusted: `specapp trust <name> --pack <dir>` (or
  `--manifest <file>`) records the digest of the manifest's `tool` section, adapters included.
  A changed `tool` section asks again.
- The tool gets the pack directory as its working directory and no network by default.

## Presentation

One table per pack, in the manifest, read by every part of specapp. It replaces the three copies
in agenticspec.

- A family groups blocks under one colour and one icon. Colours are names of a palette of
  specapp that passes the contrast rules in both themes; a pack never writes a colour value.
- A block in no family is drawn with the colour of its layer, and specapp says so in
  `specapp pack check`. No block is ever "other".
- A view of a document class says which blocks and which link names it shows and how it is laid
  out: `rows` (a part's row is its block's layer or its family's row) or `nested` (a part is
  drawn inside the part that contains it).
- Words: a pack may rename a block or a link name for display (`vocabulary`). The identifier
  stays visible on hover and in every link.

## Docs

`specapp docs build --pack <dir> --out <dir>` writes pages and data; `--check` compares bytes
with a fresh generation and lists files it does not own. No file of the output is edited by
hand.

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

Delivery: a Docusaurus preset, `@beyond10x/specapp-docusaurus`, on `@beyond10x/docs-system`. A
pack's `website/` holds its identity and its hand-written pages; the preset adds the generated
pages, the sidebar and the components. The docs build does not run a simulation: a page shows a
result that is committed beside the document, or none.

The components are a package, `@beyond10x/specapp-ui`: `LayerMap`, `BlockHeader`, `Lifecycle`,
`KpiCatalogue`, `GraphView` (draws a `specapp-scene/1` as SVG in the page, no frame),
`FindingsList`, `RecordTable`, `ResultSections`. They take data and a presentation table and
know no pack.

## Studio

`specapp serve --pack <dir> [--pack <dir>]` serves on the loopback address only.

### Screens

- **Pack**: the layer map, counts, the state of the pack (valid, warnings, what
  `specapp pack check` says), the documents with their check status.
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
- A `specapp-patch/1` is an ordered list of edits. It is applied as a whole or refused as a
  whole, and it is one revision.
- The agent has four tools: `query`, `apply` (with `dry_run`), `check`, `simulate`. They are the
  server's commands, also offered as an MCP server (`specapp mcp`), so an agent session outside
  the studio works on the same draft.

A draft is a base document (its digest) and its patches. It lives in the studio's state, not in
the pack, until the user saves it: then specapp writes the document file with the edits and
nothing else changed, comments included. An edit that cannot be written that way is refused.

### Server

- The studio's own nouns are an ESS system, `specapp`: pack, document, draft, revision, patch,
  finding, run, variant, agent session, tool call. Its views and commands are the server's API,
  generated; a channel carries changes to the browser. The UI is modelled in `ess-ui/1` and
  held by `ess ui check`. This is modelled before the server is written.
- Pack data is derived from the files and cached by the digest of its sources. Drafts, runs and
  agent sessions are in a store in the user's state directory, one per pack.
- Layout runs in the server; the browser draws scenes. The same layout code produces the scenes
  of the docs build.

## Private packs

A pack of one real organization is private. specapp is public. The rules:

- specapp's repository holds no content of any real pack: no block, no document, no finding.
  It holds the manifests of the public packs it knows (`packs/`, and only there). Its fixtures
  are two invented packs with different vocabulary (see "Proof of generality").
- A private pack's manifest is never in specapp's repository: it lives outside it (with the
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
  specapp checkout.
- specapp's gate carries the same guard as orgspec's: a deny list outside the repository,
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

specapp must not become agenticspec's UI with a second skin, nor orgspec's. It is the top-level
application over all spec packs: whatever it can do with one pack it must do with every pack it
knows (agenticspec, orgspec, and cloudspec once it has blocks). Four gates:

- **Two fixture packs** in `fixtures/packs/`, both invented and small, with different words,
  different containment (`in` against `per`), different KPI field names and a tool written as a
  shell script that answers from files in a format of its own, a different shape per pack.
  Their manifests are specapp's, `fixtures/manifests/<name>.yaml`; a fixture pack directory
  holds nothing of specapp (`scripts/check-pack-dirs.sh`). Every test of the generator, the
  layout and the components runs on both.
- **No pack vocabulary in the code.** A check fails when a block name, link name, rule id or
  KPI id of a real pack or of a fixture pack appears under `crates/` or `web/*/src`. The
  manifests (`packs/`, `fixtures/manifests/`) are where those words belong and are not searched.
- **No feature for one pack only.** A feature that one pack's tool supports is offered for
  every pack through the manifest's adapter. What a pack lacks for it is written down in "Gaps
  per pack", per pack, not worked around in the code.
- **The acceptance test of each wave** loads both real packs from their checkouts
  (`specapp pack check <name> --pack <dir>`, manifests from `packs/`) and compares counts with
  what each pack's own gate prints; no pack directory it is given holds a manifest of specapp.

## Gaps per pack

What each known pack lacks for a feature specapp offers. A gap is closed in the pack (its own
tool, its own documents) or in a neutral format, never by code for one pack.

- **orgspec** (L3). Its defaults add links the generic reader cannot see: a finding on such a
  link is a row, not a mark, until an adapter for `graph` reads the links its checker adds (no
  broken design or example needs it today). No `presentation` yet.
- **agenticspec** (L2). Its tool's check (`agenticspec-sim check <architecture>`) answers in
  text lines, not JSON, so no adapter for `check` can read it yet; the adapter reads JSON. A
  JSON answer of its own (any shape) closes the gap. Its simulator's result waits for `simulate`
  (wave 7). The gaps of its architectures listed in "Decided in wave 2" stand: two blocks the
  documents use are no block of `blocks/`, a link `over` a part has no neutral field, part
  fields that name other parts are fields only, and `architectures/ladder/` is outside the
  manifest's glob.
- **cloudspec**. A planning store only, no blocks: no manifest yet. `packs/cloudspec.yaml` is
  added once it has blocks.

## What moves and what stays

- From agenticspec into specapp: the generator's reading of the compiled model and its page
  writers; the layout and the SVG of the renderer; the five React components. Each becomes
  generic on the way in. agenticspec keeps its simulator, its architecture check, its session
  application and its hand-written pages. It gains nothing of specapp: its manifest and the
  adapters of its tool are in specapp's `packs/agenticspec.yaml`.
- agenticspec's generator and renderer are deleted from agenticspec when specapp produces the
  same pages for it: the list of pages and their sections is the parity test.
- orgspec gains nothing of specapp: its manifest, with the adapter of `orgspec check --json`, is
  specapp's `packs/orgspec.yaml`. Its scratch viewer is not kept. Where its site lives is wave 3's
  question.
- `@beyond10x/docs-system` stays what it is: the site contract, the tokens and the landing page.
- `ess-ui-templates` draws any ESS model as one page with its lifecycles played. specapp links
  to that page from a block where a pack builds it; whether its lifecycle drawing replaces
  specapp's own is open (Q7).

## What specapp does not do

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
                        the planning-only path list
packs/                  the manifests of the public packs specapp knows, `<name>.yaml`
spec/                   the ESS system `specapp` and its ess-ui model
crates/specapp-core/    manifest, pack loading, the neutral model
crates/specapp-layout/  graph layout to a scene
crates/specapp-docs/    the generator
crates/specapp-server/  serve, the store, the tool runner
crates/specapp/         the CLI: pack check, docs build, serve, trust, mcp
web/ui/                 @beyond10x/specapp-ui (React)
web/docusaurus/         @beyond10x/specapp-docusaurus
web/studio/             the studio (React, Vite)
fixtures/packs/         two invented packs, nothing of specapp in them
fixtures/manifests/     their manifests, `<name>.yaml`
fixtures/expected/      what each fixture pack's check must count
ux/                     adapter, baseline, rounds of the UI loop
```

## Build order

Each wave ends with its acceptance on both real packs and both fixture packs.

1. **Contract.** `specapp-pack/1`, the schemas, the two fixture packs, `specapp pack check`.
   Acceptance: both real packs load from a manifest written in their checkouts; the counts of
   blocks, layers, KPIs and documents equal what each pack's gate prints.
2. **Graph.** The `parts-links/1` reader, layout, scenes, a static page per document with the
   findings of the tool. Acceptance: every example of orgspec and every architecture of
   agenticspec is drawn; every broken design of orgspec shows its rule on the part it names.
3. **Docs.** The generator, the component package, the Docusaurus preset. Acceptance: orgspec
   has a site; agenticspec's site is generated by specapp with the parity list green.
4. **Model of the studio.** The ESS system `specapp` and its `ess-ui/1` model, validated.
5. **Studio, read-only.** `serve`: pack, reference and document screens, live on file change.
6. **Drafts.** Generic edits, patches, revisions, comparison, save.
7. **Runs.** `simulate` and `search`, results, variants. agenticspec first; orgspec when it has
   a simulator.
8. **Agent.** The four tools, the agent screen, `specapp mcp`.

## Decided in wave 1

What the build of wave 1 decided where this design was open or wrong. The open questions Q1 to
Q8 below kept their defaults.

- `contain` is `{ in: <field> }` or `{ per: <field> }`, exactly one key; the value names the
  part's field. The first draft wrote `{ field: in }` for orgspec, which the schema refuses.
- `visibility` is required: a manifest that forgets it is refused rather than published.
- `extends` is the name of a pack, not a path. Where that pack is checked out is given when
  specapp is started, so a manifest names no path outside its pack.
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
- The manifest is validated against `schema/specapp-pack-1.schema.json` when it is read; the
  schema is the contract, not a copy of the code.
- orgspec's manifest has no `presentation`: the families of the first draft named
  `decision_class`, which is no block of orgspec, and every block in a family would be a choice
  of orgspec's authors. Without it a block takes the colour of its layer. It has no `tool` until
  `orgspec specapp check` exists (wave 2).
- The words of a pack come from `specapp pack words <dir>`: block and layer names, link names
  (`carries` of documents and views, keys of a `links` mapping in a document), KPI ids (the
  catalogue and `kpis` lists in documents) and rule ids (`rules` lists in documents).
  `scripts/check-vocabulary.sh` finds a word with an underscore, hyphen or digit as a whole word
  anywhere under `crates/` and `web/src/`, and a word of letters only as a whole string literal
  (`"word"`): block names such as `tool` or `kpi` are common nouns, and specapp's own code uses
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

- The reader (`specapp-core::graph`, `schema/specapp-graph-1.schema.json`): a part's `fields`
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
  class declares none, specapp adds the view "All parts and links" (`rows`, `implicit: true`).
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
  specapp draws 17 icon names (`icons.rs`); any other icon name is drawn as its first letter.
- A finding's number is its position in the tool's answer. It is a mark `!<n>` at the top right
  of its part's box or at the left of its link's label (filled: gated; outlined: not gated), the
  marked link is dashed, and the row `<n>` of the findings list names the views it is drawn in.
  A finding about the document, or about a part or link the graph does not have, is a row only.
- Trust: `trust.json` in `$XDG_STATE_HOME/specapp` (default `~/.local/state/specapp`) maps the
  resolved pack directory to `sha256:` of the JSON of the manifest's `tool` section. Trust
  belongs to the directory, not to the manifest file: a second manifest with the same `tool`
  section is trusted too.
- A run: working directory the pack, stdin closed, its own process group, killed with all it
  started when it passes the deadline (`specapp render --deadline`, default 30 s). The answer is
  validated against `specapp-findings/1`; another version is refused with both named. The run
  is not cut off from the network yet, and it inherits the environment.
- `specapp render --pack <dir> [--manifest <file>] --out <dir>` writes `index.html` and
  `<class>/<file stem>.html` per `parts-links/1` document. A page is one file: CSS and SVG
  inline, no font, no script that runs, no request; its data is `specapp-document/1` in a
  `<script type="application/json">` element. A `records/1` class is listed on the index with
  its count; its pages come with the docs (wave 3). The drawing keeps its size and scrolls
  inside its frame rather than shrinking text.
- orgspec answers `check` with `orgspec specapp check <design>`; its manifest runs
  `target/debug/orgspec specapp` (Q8: specapp builds nothing; `pack check` says when the binary
  is missing). Reversed on 2026-11-28: see "Decided: reversed the dependency". A violation's
  subject is `part:` when it names a part, `link:` when it names a link the design wrote or a
  default added, else `document` with the name kept at the start of the message (a name the
  border lists, the organization). A refused type is a gated finding about the document.
- orgspec's defaults add links the generic reader cannot see; a finding on such a link is a row,
  not a mark. orgspec's `graph` verb closes that; no broken design or example needs it today.
- The format keys `value`, `vary` and `message` are also words of real packs (a layer of orgspec,
  a link of agenticspec). specapp reads them through typed structures, so the vocabulary check
  stays strict.
- Gaps of agenticspec's architectures, left to agenticspec: two blocks the documents use
  (`pool`, `schedule`) are no block of `blocks/`, so their parts are drawn as "block the pack
  does not have"; a link `over` a part (a channel) has no neutral field and is a field only;
  part fields that name other parts (`egress`, `acts_for`) are fields only; the architectures
  under `architectures/ladder/` are outside the manifest's glob.

## Decided: reversed the dependency (2026-11-28)

Ordered by the owner on 2026-11-28: a pack must know nothing of specapp; specapp may know the
packs. Until then orgspec carried `specapp.yaml` and a subcommand `orgspec specapp check |
describe` that answered specapp's protocol. What changed:

- The manifests of the public packs are specapp's: `packs/orgspec.yaml` (restored from orgspec's
  `specapp.yaml`) and `packs/agenticspec.yaml` (the draft that was kept outside both
  repositories). They are embedded in the binary when it is built, so a pack is named by its
  name (`specapp pack check orgspec --pack <dir>`, `specapp pack known`). Any other manifest is
  given by its path (`--manifest <file>`). specapp no longer reads `specapp.yaml` from a pack
  directory; nothing published used it, so there is no fallback.
- The tool protocol (`describe`, `check` in `specapp-findings/1`, exit 2 with `{error}`) is
  gone, and `specapp-tool/1` with it. A manifest declares its verbs, each with an adapter that
  runs the pack's own command and maps its own answer (see "The tool adapter"). orgspec's
  adapter reads `orgspec check --json`: findings from `refusals` (each a gated finding about the
  document, the refused subject before the message) and `violations` (`rule`, `gate`,
  `message`; a subject with `->` is a link id, any other a part id, and an id the drawn graph
  does not have is about the document with the id before the message). The result is the same
  as the subcommand's: the 30 broken designs show the same rule ids as `orgspec check --json`,
  27 of 27 findings with a part or link subject are marked.
- The manifest format is `specapp-pack/2`: `tool.verbs` changed from a list of names to a
  mapping from a verb to its adapter, which a reader of `/1` cannot read. A `/1` manifest is
  refused with both versions named.
- Trust stays the digest of the manifest's `tool` section, adapters included.
- `extends` names a pack specapp knows or one whose manifest is given (`--extends <file>`);
  `pack check` warns when it resolves to neither (`extends_not_found`) or to another pack.
- The fixture packs model the same rule: their manifests are `fixtures/manifests/<name>.yaml`,
  their directories hold nothing of specapp, and their tools answer in two shapes of their own.
  `scripts/check-pack-dirs.sh` fails when a fixture pack, or a pack directory the acceptance is
  given, holds a file named like a manifest of specapp.
- Added the same day by the owner: specapp is the top-level application over all spec packs.
  No feature is built for one pack only; a feature one pack's tool supports is offered for
  every pack through the adapter, and each pack's gaps are listed ("Gaps per pack"). cloudspec
  is the third known pack; its manifest comes when it has blocks.

## Open questions

Each with the default this design takes.

- Q1 The studio in React, because the docs components are React and are shared. agenticspec's
  session application stays Vue. Default: React.
- Q2 The component package lives in specapp, not in docs-system. Default: specapp.
- Q3 Layout in Rust in the server, scenes to the browser. Dragging a part in the editor may need
  the layout in the browser later (the same crate compiled to WebAssembly). Default: server.
- Q4 agenticspec moves to specapp after parity, not before. Default: after.
- Q5 Selectors. agenticspec's epic selects parts with its query language. Default: edits name
  ids first; selectors come when that language is a crate of its own.
- Q6 Saving a draft keeps comments and order of the YAML file. Default: yes, and an edit that
  cannot be written that way is refused; the alternative is a pack tool's `apply` that writes
  the file.
- Q7 Lifecycle drawing: specapp's own component or the page of `ess-ui-templates`. Default:
  own component for the diagram on a block page, a link to that page for playback.
- Q8 A tool that is not installed. Default: `tool.run` is an argv the manifest names, and
  `specapp pack check` says when it cannot be started; specapp builds nothing of a pack.
