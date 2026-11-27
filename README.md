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

Design only. Nothing is built.

- [design/specapp.md](design/specapp.md): the design, the manifest, the neutral formats, the
  tool protocol, the build order and the open questions.
- [AGENTS.md](AGENTS.md): rules for agent sessions that work here.

## License

Apache-2.0. See [LICENSE](LICENSE).
