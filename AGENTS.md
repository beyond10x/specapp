# metaspec: rules for agent sessions

Read `design/metaspec.md` first. The rules of the folder above this repository apply; these add to
them.

## What this repository is

The user interface (docs site and studio) of every specification pack, the top-level application
over all of them. A pack is another repository. metaspec knows the packs; a pack knows nothing of
metaspec. metaspec describes a pack in a manifest of its own: `packs/<name>.yaml` for a public pack
it knows (agenticspec, orgspec; cloudspec once it has blocks), a file given at run time for any
other.

## Rules

- **metaspec knows the packs; a pack knows nothing of metaspec.** No file, format, command or
  word of metaspec is asked of a pack. A pack's manifest and the adapters of its tool live in
  metaspec (`packs/`), or, for a private pack, outside this repository. metaspec runs a pack's
  tool as the pack ships it and maps its own answer with the manifest's adapter. Owner's order,
  2026-11-28.
- **No feature for one pack only.** Whatever metaspec can do with one pack it must do with every
  pack it knows: a feature that one pack's tool supports is offered for every pack through the
  manifest's adapter, and what a pack lacks for it is listed per pack in "Gaps per pack" of
  `design/metaspec.md`. Owner's order, 2026-11-28.

- **No pack vocabulary in the code.** No block name, link name, rule id, KPI id, layer name or
  colour of any pack under `crates/` or `web/`. What a pack calls its things comes from its
  manifest (`packs/`, `fixtures/manifests/`, or a file given at run time) and its tool at run
  time; the manifests are not code. A change that needs such a word in the code is a gap of the
  manifest or of a neutral format: change the format in `design/metaspec.md` and `schema/`.
- **metaspec implements no rule of a pack.** A finding, a KPI value and a refusal come from the
  pack's tool. metaspec draws them.
- **Two fixture packs.** Every test of the generator, the layout and the components runs on both
  packs under `fixtures/packs/`. Both are invented. Their manifests are
  `fixtures/manifests/<name>.yaml`; a fixture pack directory holds nothing of metaspec, and its
  tool answers in a format of its own (`scripts/check-pack-dirs.sh`).
- **This repository is public.** No content of a real pack (no block, document, finding or other
  file of it), no company, product, person or host name, no path of a home directory. The one
  exception: the manifests of public packs under `packs/`, and only there. A private pack is
  loaded from its own checkout at run time with a manifest given by its path, and nothing of it
  is written here: no manifest, no fixture, no screenshot, no baseline, no test name.
- A session that has read a private pack sets `METASPEC_DENYLIST` to that organization's deny
  list and runs `scripts/check-public.sh` before it reports. The deny list is a file outside
  this repository; it is never committed here and no digest of it is published.
- **A format has a version and a schema.** A change that an older reader cannot read is a new
  version.
- The studio's nouns are modelled in ESS (`spec/`) before a server route or a screen is written.
- The test budget of the folder above applies to every runner here: a test is killed at 60 s,
  the test step ends within 120 s, tests draw from fixtures and start no server per test.
- No Python. Tools are Rust or POSIX sh.

## Delivery

- The remote is `github.com/beyond10x/metaspec`. Only the organization's bot writes a ref, and
  every commit has the bot as author and committer. Commit with
  `b10x-gates bot -- commit -F <file>`, push with `b10x-gates bot -- push`, open a pull request
  with `b10x-gates gh -- pr create`.
- `.github/workflows/shared-gates.yml` runs the organization's source gates, pinned by commit.
  `main` requires its check `common / Security and privacy`.
- `task check` is the gate: `check:public` (`scripts/check-public.sh`: home paths, e-mail
  addresses, IP addresses, and the terms of the deny list `METASPEC_DENYLIST` names, in file
  contents and file names; without `METASPEC_DENYLIST`, as in CI and on a build node, it skips
  the deny check and says so), script tests, `ess specify validate` of the fixture packs' blocks
  and `scripts/check-pack-dirs.sh` (no fixture pack holds anything of metaspec), `cargo fmt` and
  clippy with `-D warnings`, the tests under cargo-nextest (a test is killed at 60 s, the step
  fails over 120 s; `.config/nextest.toml`), `check:docs` (`metaspec docs build --manifest
  fixtures/manifests/<pack>.yaml --pack fixtures/packs/<pack> --check` against
  `fixtures/expected/<pack>/site`), `check:web` (tsc of `web/ui` and `web/docusaurus`, vitest of
  `web/ui` on the committed pages of both fixture packs; needs `task web:install`), and
  `scripts/check-vocabulary.sh` on the fixture packs (it searches `crates/`, `web/src` and
  `web/*/src`, not the manifests). A command of the gate that is no cargo call starts with
  `{{.SLOT}}`, which is `slot` where slot is installed and empty in CI.
- `task ux` (`scripts/ux.sh`) is the UX lane, outside `task check`: it builds both fixture sites
  and runs the Playwright and axe-core gates (every page, 1440x900 and 1280x800, both themes, no
  serious or critical violation, no sideways scroll; a test is killed at 60 s, the run at
  120 s). Run it after a change under `web/` or of the generated pages.
- `web/` is one npm workspace (`ui`, `docusaurus`, `site`). Its packages are private and never
  published. `@beyond10x/docs-system` is a git dependency of the workspace root, pinned at the
  commit agenticspec uses; `.npmrc` allows git for the root only. `npm ci` runs without slot;
  builds and tests run through it.
- A pack's site is built from here, never from the pack: `metaspec docs build [NAME | --manifest
  <file>] --pack <dir> --out <dir>` and a site shell over the output (`web/site` with
  `METASPEC_SITE_OUT` today). A pack carries no site shell, no dependency on these packages and
  no task of metaspec.
- `.github/workflows/gate.yml` runs `task check` (job `gate`) with the toolchain
  `rust-toolchain.toml` names, `ess` 0.99.2 and cargo-nextest 0.9.143. A pull request that
  changes only planning or prose files passes it without `task check`; the path list is
  `scripts/planning-only.sh`, and `scripts/planning-only.test.sh` holds both sides of it.
- `scripts/acceptance.sh <orgspec-dir> <agenticspec-dir>` is the acceptance of a wave: no pack
  directory given holds a manifest of metaspec (`scripts/check-pack-dirs.sh`); `metaspec pack
  check` on both fixture packs and both real packs (manifests from `packs/` by name), with the
  counts compared to what each pack's own tooling prints; `metaspec render` of every pack
  (pages = documents) with the tools trusted in a state directory of the run, and for every
  broken design and example of orgspec the rule ids of the page equal to
  `orgspec check --json` and every finding about a part or link marked; then the vocabulary
  check with all four packs; `metaspec docs build` of all four packs (block pages, KPI rows and
  document pages equal the counts of `pack check`; each fixture site equal to the committed
  one), the orgspec site through `web/site`, and the agenticspec parity summary
  (`scripts/parity.sh <agenticspec-dir> [<manifest>]`). It is not part of `task check` and not
  run in CI.
- Tests read `fixtures/expected/<pack>.json` for what a fixture pack must count, so no test
  names a word of a pack. The compiled model of a fixture's blocks is made once per digest of
  its sources under the target directory (`tests/cases/common.rs`).
- The scenes, pages and docs sites (`site/`) of the fixture packs are committed under
  `fixtures/expected/<pack>/` and compared byte for byte. A change of the layout or the page rewrites them with
  `METASPEC_BLESS=1 cargo nextest run`; the diff of those files is part of the review.
- A test or script that runs a pack's tool trusts it in a state directory of its own
  (`XDG_STATE_HOME`), never in the user's.

## Open decisions

The open questions Q1 to Q8 are at the end of `design/metaspec.md`, each with the default the
design takes.
