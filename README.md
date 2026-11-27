# specapp

One docs site and one studio for every specification pack.

A pack is a repository built like agenticspec or orgspec: an ESS system of building blocks in
numbered layers, a KPI catalogue, documents that compose the blocks into a graph of parts and
links, and a tool that checks such a document. A pack describes itself in one manifest,
`specapp.yaml`. specapp reads it and gives the pack:

- **Docs**: a static site with every block by layer, every lifecycle, the KPI catalogue and
  every document drawn as a graph with its check result.
- **Studio**: a local application with the same pages live, and work on a document: edit, check
  at every change, run, compare variants, keep drafts, together with an agent.

## Status

Waves 1 and 2 of 8 ("Contract", "Graph") are built: the manifest format `specapp-pack/1` with
its JSON Schema, two invented fixture packs, `specapp pack check`, and a static page per
document drawn with the findings of the pack's tool. There is no docs site and no studio yet.

```sh
cargo build -p specapp
target/debug/specapp pack check fixtures/packs/harbor
target/debug/specapp pack check <dir> --manifest <file> --json
target/debug/specapp trust fixtures/packs/harbor
target/debug/specapp render --pack fixtures/packs/harbor --out target/pages
```

`pack check` compiles the pack's blocks with `ess specify compile` (the `ess` CLI on PATH),
reads its KPI catalogue and documents, and prints counts and warnings. It runs nothing of the
pack. `render` writes one self-contained HTML page per drawn document (`target/pages/index.html`
lists them): each view of the document as a graph, and the findings of the pack's tool as
numbered marks and a list. The tool runs only after `specapp trust <dir>`, which records the
digest of the manifest's `tool` section in `~/.local/state/specapp`. `task check` is the gate;
`scripts/acceptance.sh` compares the counts and the drawn findings on real packs with what their
own tooling prints.

- [design/specapp.md](design/specapp.md): the design, the manifest, the neutral formats, the
  tool protocol, the build order and the open questions.
- [schema/](schema/): the JSON Schemas of the formats.
- [CHANGELOG.md](CHANGELOG.md): what changed.
- [AGENTS.md](AGENTS.md): rules for agent sessions that work here.

## License

Apache-2.0. See [LICENSE](LICENSE).
