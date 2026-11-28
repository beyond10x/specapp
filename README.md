# metaspec

The meta model of every specification pack in ESS, and one docs site and studio over all packs.

A pack is a repository built like agenticspec or orgspec: an ESS system of building blocks in
numbered layers, a KPI catalogue, documents that compose the blocks into a graph of parts and
links, and a tool that checks such a document. metaspec knows the packs; a pack knows nothing of
metaspec. metaspec keeps a manifest of each pack it knows under `packs/` (a private pack's
manifest stays outside this repository), runs the pack's own tool through an adapter in that
manifest, and gives the pack:

- **Docs**: a static site with every block by layer, every lifecycle, the KPI catalogue and
  every document drawn as a graph with its check result.
- **Studio**: a local application with the same pages live, and work on a document: edit, check
  at every change, run, compare variants, keep drafts, together with an agent.

## Status

Waves 1 to 3 of 8 ("Contract", "Graph", "Docs") are built: the manifest format `metaspec-pack/1`
with its JSON Schema, the manifests of the packs metaspec knows (`packs/`), two invented fixture
packs, `metaspec pack check`, a static page per document drawn with the findings of the pack's
own tool, and the docs site. There is no studio yet.

```sh
cargo build -p metaspec
target/debug/metaspec pack known
target/debug/metaspec pack check orgspec --pack <orgspec-checkout>
target/debug/metaspec pack check --manifest fixtures/manifests/harbor.yaml --pack fixtures/packs/harbor --json
target/debug/metaspec trust --manifest fixtures/manifests/harbor.yaml --pack fixtures/packs/harbor
target/debug/metaspec render --manifest fixtures/manifests/harbor.yaml --pack fixtures/packs/harbor --out target/pages
target/debug/metaspec docs build --pack fixtures/manifests/harbor.yaml=fixtures/packs/harbor --pack fixtures/manifests/orchard.yaml=fixtures/packs/orchard --out target/docs
```

`docs build` writes one docs site, titled metaspec, over every pack given with `--pack
<name or manifest>=<dir>`: each pack's pages under `/<pack>` (MDX with their data, `site.json`)
with its own navbar item and sidebar, and the page `/` that lists the packs with their counts and
check status (`index.json`); `--check` compares them with a directory. `docs build <name> --pack
<dir>` or `--manifest <file> --pack <dir>` writes a site of one pack. The site itself is Docusaurus on
`@beyond10x/docs-system` with the two packages of `web/`: `@beyond10x/metaspec-ui` (the
components) and `@beyond10x/metaspec-docusaurus` (the preset). To build and open the site of
the docs build above:

```sh
task web:install web:build
cd web/site
METASPEC_SITE_OUT=../../target/docs npx docusaurus build --out-dir build/local
npx docusaurus serve --dir build/local --host 127.0.0.1 --port 3000
```

A pack is named by its name when metaspec knows it (`packs/<name>.yaml`); any other pack, such
as a private one, is read with `--manifest <file>`. The pack directory is always `--pack`.
`pack check` compiles the pack's blocks with `ess specify compile` (the `ess` CLI on PATH),
reads its KPI catalogue and documents, and prints counts and warnings. It runs nothing of the
pack. `render` writes one self-contained HTML page per drawn document (`target/pages/index.html`
lists them): each view of the document as a graph, and the findings of the pack's tool as
numbered marks and a list. The tool runs only after `metaspec trust`, which records the digest of
the manifest's `tool` section in `~/.local/state/metaspec`; the manifest's adapter maps the
tool's own answer. `task check` is the gate; `scripts/acceptance.sh` compares the counts and the
drawn findings on real packs with what their own tooling prints.

- [design/metaspec.md](design/metaspec.md): the design, the manifest, the neutral formats, the
  tool adapter, the gaps per pack, the build order and the open questions.
- [schema/](schema/): the JSON Schemas of the formats.
- [CHANGELOG.md](CHANGELOG.md): what changed.
- [AGENTS.md](AGENTS.md): rules for agent sessions that work here.

## License

Apache-2.0. See [LICENSE](LICENSE).
