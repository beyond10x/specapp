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

Wave 1 of 8 ("Contract") is built: the manifest format `specapp-pack/1` with its JSON Schema,
two invented fixture packs, and `specapp pack check`. There is no docs site and no studio yet.

```sh
cargo build -p specapp
target/debug/specapp pack check fixtures/packs/harbor
target/debug/specapp pack check <dir> --manifest <file> --json
```

`pack check` compiles the pack's blocks with `ess specify compile` (the `ess` CLI on PATH),
reads its KPI catalogue and documents, and prints counts and warnings. It runs nothing of the
pack. `task check` is the gate; `scripts/acceptance.sh` compares the counts on real packs with
what their own tooling prints.

- [design/specapp.md](design/specapp.md): the design, the manifest, the neutral formats, the
  tool protocol, the build order and the open questions.
- [schema/](schema/): the JSON Schemas of the formats.
- [CHANGELOG.md](CHANGELOG.md): what changed.
- [AGENTS.md](AGENTS.md): rules for agent sessions that work here.

## License

Apache-2.0. See [LICENSE](LICENSE).
