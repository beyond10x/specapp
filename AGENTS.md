# specapp: rules for agent sessions

Read `design/specapp.md` first. The rules of the folder above this repository apply; these add to
them.

## What this repository is

The user interface (docs site and studio) of every specification pack. A pack is another
repository; it describes itself in `specapp.yaml`.

## Rules

- **No pack vocabulary in the code.** No block name, link name, rule id, KPI id, layer name or
  colour of any pack under `crates/` or `web/`. What a pack calls its things comes from its
  manifest and its tool at run time. A change that needs such a word in the code is a gap of the
  manifest or of a neutral format: change the format in `design/specapp.md` and `schema/`.
- **specapp implements no rule of a pack.** A finding, a KPI value and a refusal come from the
  pack's tool. specapp draws them.
- **Two fixture packs.** Every test of the generator, the layout and the components runs on both
  packs under `fixtures/packs/`. Both are invented.
- **This repository is public.** No content of a real pack, no company, product, person or host
  name, no path of a home directory. A private pack is loaded from its own checkout at run time
  and nothing of it is written here: no fixture, no screenshot, no baseline, no test name.
- **A format has a version and a schema.** A change that an older reader cannot read is a new
  version.
- The studio's nouns are modelled in ESS (`spec/`) before a server route or a screen is written.
- The test budget of the folder above applies to every runner here: a test is killed at 60 s,
  the test step ends within 120 s, tests draw from fixtures and start no server per test.
- No Python. Tools are Rust or POSIX sh.

## Delivery

- The remote is `github.com/beyond10x/specapp`. Only the organization's bot writes a ref, and
  every commit has the bot as author and committer. Commit with
  `b10x-gates bot -- commit -F <file>`, push with `b10x-gates bot -- push`, open a pull request
  with `b10x-gates gh -- pr create`.
- `.github/workflows/shared-gates.yml` runs the organization's source gates, pinned by commit.
  `main` requires its check `common / Security and privacy`.
- There is no code and no gate of this repository's own yet. The first wave that adds code adds
  `Taskfile.yml` with `task check` and a workflow that runs it.

## Open decisions

The open questions Q1 to Q8 are at the end of `design/specapp.md`, each with the default the
design takes.
