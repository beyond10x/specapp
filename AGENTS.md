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
- A session that has read a private pack sets `SPECAPP_DENYLIST` to that organization's deny
  list and runs `scripts/check-public.sh` before it reports. The deny list is a file outside
  this repository; it is never committed here and no digest of it is published.
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
- `task check` is the gate: `check:public` (`scripts/check-public.sh`: home paths, e-mail
  addresses, IP addresses, and the terms of the deny list `SPECAPP_DENYLIST` names, in file
  contents and file names; without `SPECAPP_DENYLIST`, as in CI and on a build node, it skips
  the deny check and says so), script tests, `ess specify validate` of the fixture packs' blocks,
  `cargo fmt` and clippy with `-D warnings`, the tests under cargo-nextest (a test is killed at
  60 s, the step fails over 120 s; `.config/nextest.toml`), and
  `scripts/check-vocabulary.sh` on the fixture packs.
- `.github/workflows/gate.yml` runs `task check` (job `gate`) with the toolchain
  `rust-toolchain.toml` names, `ess` 0.99.2 and cargo-nextest 0.9.143. A pull request that
  changes only planning or prose files passes it without `task check`; the path list is
  `scripts/planning-only.sh`, and `scripts/planning-only.test.sh` holds both sides of it.
- `scripts/acceptance.sh <orgspec-dir> <agenticspec-dir> <agenticspec-manifest>` is the
  acceptance of a wave: `specapp pack check` on both fixture packs and both real packs, with
  the counts compared to what each pack's own tooling prints; `specapp render` of every pack
  (pages = documents) with the tools trusted in a state directory of the run, and for every
  broken design and example of orgspec the rule ids of the page equal to
  `orgspec check --json` and every finding about a part or link marked; then the vocabulary
  check with all four packs. It is not part of `task check` and not run in CI. A real pack's
  manifest that lives outside this repository is passed with `--manifest`; it is never
  committed here.
- Tests read `fixtures/expected/<pack>.json` for what a fixture pack must count, so no test
  names a word of a pack. The compiled model of a fixture's blocks is made once per digest of
  its sources under the target directory (`tests/cases/common.rs`).
- The scenes and pages of the fixture packs are committed under `fixtures/expected/<pack>/`
  and compared byte for byte. A change of the layout or the page rewrites them with
  `SPECAPP_BLESS=1 cargo nextest run`; the diff of those files is part of the review.
- A test or script that runs a pack's tool trusts it in a state directory of its own
  (`XDG_STATE_HOME`), never in the user's.

## Open decisions

The open questions Q1 to Q8 are at the end of `design/specapp.md`, each with the default the
design takes.
