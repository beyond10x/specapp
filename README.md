# specapp

One docs site and one studio for every specification pack.

A pack is a repository built like agenticspec or orgspec: an ESS system of building blocks in
numbered layers, a KPI catalogue, documents that compose the blocks into a graph of parts and
links, and a tool that checks such a document. specapp knows the packs; a pack knows nothing of
specapp. specapp keeps a manifest of each pack it knows under `packs/` (a private pack's
manifest stays outside this repository), runs the pack's own tool through an adapter in that
manifest, and gives the pack:

- **Docs**: a static site with every block by layer, every lifecycle, the KPI catalogue and
  every document drawn as a graph with its check result.
- **Studio**: a local application with the same pages live, and work on a document: edit, check
  at every change, run, compare variants, keep drafts, together with an agent.

## Status

Waves 1 and 2 of 8 ("Contract", "Graph") are built: the manifest format `specapp-pack/2` with
its JSON Schema, the manifests of the packs specapp knows (`packs/`), two invented fixture
packs, `specapp pack check`, and a static page per document drawn with the findings of the
pack's own tool. There is no docs site and no studio yet.

```sh
cargo build -p specapp
target/debug/specapp pack known
target/debug/specapp pack check orgspec --pack <orgspec-checkout>
target/debug/specapp pack check --manifest fixtures/manifests/harbor.yaml --pack fixtures/packs/harbor --json
target/debug/specapp trust --manifest fixtures/manifests/harbor.yaml --pack fixtures/packs/harbor
target/debug/specapp render --manifest fixtures/manifests/harbor.yaml --pack fixtures/packs/harbor --out target/pages
```

A pack is named by its name when specapp knows it (`packs/<name>.yaml`); any other pack, such
as a private one, is read with `--manifest <file>`. The pack directory is always `--pack`.
`pack check` compiles the pack's blocks with `ess specify compile` (the `ess` CLI on PATH),
reads its KPI catalogue and documents, and prints counts and warnings. It runs nothing of the
pack. `render` writes one self-contained HTML page per drawn document (`target/pages/index.html`
lists them): each view of the document as a graph, and the findings of the pack's tool as
numbered marks and a list. The tool runs only after `specapp trust`, which records the digest of
the manifest's `tool` section in `~/.local/state/specapp`; the manifest's adapter maps the
tool's own answer. `task check` is the gate; `scripts/acceptance.sh` compares the counts and the
drawn findings on real packs with what their own tooling prints.

- [design/specapp.md](design/specapp.md): the design, the manifest, the neutral formats, the
  tool adapter, the gaps per pack, the build order and the open questions.
- [schema/](schema/): the JSON Schemas of the formats.
- [CHANGELOG.md](CHANGELOG.md): what changed.
- [AGENTS.md](AGENTS.md): rules for agent sessions that work here.

## License

Apache-2.0. See [LICENSE](LICENSE).
