---
format: aep.planning-md/3
id: review-result:gaps-acceptance-pass-1
kind: review-result
status: active
title: aep:plan-critic-acceptance, pass 1, cloudspec gap stories
relations:
- reviews: epic:cloudspec-pack
- reviews: story:cloudspec-warnings
- reviews: story:part-block-aliases
- reviews: story:extends-inherits
revision: 1
---
needs-revision

- story:part-block-aliases — the acceptance cannot be met for `groups.yaml`: its parts `payment-provider` (`block: payment_api`) and `mail-relay` (`block: mail_relay`) are neither aliased nor blocks of the pack, so the story's own scope leaves them in "block the pack does not have". Quote: "no part of the public cloudspec architectures `groups.yaml`, `node-pools.yaml` and `l4-media.yaml` lands in the group "block the pack does not have"". Cited at `.engineering/planning/story/part-block-aliases.md:105-107` and `git -C <private path> show origin/main:architectures/groups.yaml` lines 224-231. Reword it to name only the parts whose words are `group`, `node_pool` and `endpoints`, or state the expected remainder.
- story:part-block-aliases — "the existing pages of ... the real packs stay byte for byte" contradicts the story's own purpose: the cloudspec pages for `groups.yaml`, `node-pools.yaml` and `l4-media.yaml` must change when their parts move out of the slate group. No command or committed baseline for real-pack pages exists. Only the fixture pages have one, in `fixtures/expected/*/pages`. Quote at `part-block-aliases.md:107`. Name the real packs and pages that stay unchanged, and how to check it.
- story:extends-inherits — "with the same blocks and page as the manifest `packs/cloudspec.yaml` gives" does not say what is compared. The page data carries `pack`, `title` and `visibility` (`fixtures/expected/harbor/pages/plan/cove.html`: `"pack":"harbor","title":"Harbor","visibility":"public"`). An extending manifest is private and has its own name, so the pages cannot be equal byte for byte. The scope also says `documents` and `presentation` are not inherited. Quote at `.engineering/planning/story/extends-inherits.md:118-119`. Name the compared fields (for example graph, scenes, findings) or the compared bytes.
- story:extends-inherits — "the existing pages of the real packs stay byte for byte" names no baseline or command (`extends-inherits.md:119-120`). Name one.
- story:cloudspec-warnings — the acceptance cannot reach "`scripts/acceptance.sh` exits 0" within the scope as drafted. Two existing cloudspec rows compare the page only against `.violations`: `theirs_cloudspec` (`scripts/acceptance.sh:260-262`) and `named` (`scripts/acceptance.sh:335-341`). Once `/warnings` is mapped, the page for `journeys.yaml` carries `single_point_of_failure` findings, so `examples:rules-matched` and `examples:named-drawn` go red. The scope adds only "a cloudspec row" (`cloudspec-warnings.md:51`). The acceptance must say these rows now count warnings too, and the scope must name those edits. Quote at `cloudspec-warnings.md:73-76`.
- story:cloudspec-warnings — the `warning_list_` clause is trivially true. Harbor already declares two finding lists with `gate: false` (`fixtures/manifests/harbor.yaml:55-60`), and the story expects no Rust change. The test is new, so the filter fails before the work only because it matches no test. Only the `acceptance.sh` row shows the cloudspec change. Also, "under 5 s" is not enforced by the command. Quote at `cloudspec-warnings.md:75-77`. Either state what the test shows that the harbor manifest does not, or drop the clause.
- All three stories — each acceptance is one sentence joining two independent checks with "; and", so the nextest cases can pass while the `scripts/acceptance.sh` row fails. The aliases and extends acceptances add a third and fourth outcome (byte-for-byte pages; four separate cases). Cited at `cloudspec-warnings.md:73-77`, `part-block-aliases.md:102-107`, `extends-inherits.md:114-120`. Split these, or reduce each to the one outcome that tells the work from the world before it.

What I read: the three story bodies (`aep plan artifact show` for each, 3 of 3). I checked their citations against `adapter.rs`, `manifest.rs`, `pack.rs`, `paths.rs`, `main.rs` and `metaspec-layout/src/lib.rs`, plus `scripts/acceptance.sh`, `check-vocabulary.sh` and `.config/nextest.toml`. For cloudspec I read `architectures/{journeys,groups,node-pools,l4-media}.yaml`, `blocks/system.yaml`, and the `Violation` and `FileReport` structs and the journeys tests in `crates/cloudspec/src` and `tests/journeys/mod.rs`, all through `git show origin/main`.

The story citations I checked are accurate: `manifest.rs:43-44`, `pack.rs:105-148`, `main.rs:214-216`, `paths.rs:6-41`, `pack.rs:716-731`, `graph.rs:202`, and the `lib.rs` lookup miss. Cloudspec's `journeys.yaml` gives 3 warnings of `single_point_of_failure`. The `at: /0` pointer fits the array that `check --json` writes.

What I could not establish:
- Where in the scene or page data the group name of a part is read. I did not find its field, and the acceptances do not name one.
- Whether nextest 0.9 here fails on a filter that matches no test. I assumed it does (the `--no-tests=fail` default).
- Out of my lane: design — the alias is applied in both `graph.rs` and the style, which are two places for one rule. Parallel-safety — the stories share `manifest.rs`, `pack.rs`, `acceptance.sh` and `CHANGELOG.md` with the active view stories.

```findings
[
{"file":".engineering/planning/story/part-block-aliases.md","line":105,"category":"acceptance","severity":"blocker","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the acceptance says no part of groups.yaml lands in the group \"block the pack does not have\", but its parts payment-provider (block payment_api) and mail-relay (block mail_relay) are neither aliased nor blocks, so the story's own scope leaves them there and the acceptance cannot pass"},
{"file":".engineering/planning/story/part-block-aliases.md","line":107,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the existing pages of the real packs stay byte for byte contradicts the alias change, which must change the cloudspec pages of groups.yaml, node-pools.yaml and l4-media.yaml, and names no baseline or command for real-pack pages"},
{"file":".engineering/planning/story/extends-inherits.md","line":118,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the same blocks and page as the manifest packs/cloudspec.yaml gives does not say what is compared; page data carries pack, title and visibility, which differ for an extending (private, renamed) pack, and documents and presentation are not inherited"},
{"file":".engineering/planning/story/extends-inherits.md","line":120,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the existing pages of the real packs stay byte for byte names no baseline or command that shows it"},
{"file":".engineering/planning/story/cloudspec-warnings.md","line":73,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"scripts/acceptance.sh exits 0 cannot hold within the scope as drafted: theirs_cloudspec (acceptance.sh:260-262) and named (acceptance.sh:335-341) compare the page only against violations, so mapping /warnings turns examples:rules-matched and examples:named-drawn red; the scope names only a new cloudspec row"},
{"file":".engineering/planning/story/cloudspec-warnings.md","line":75,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the warning_list_ test passes on an invented manifest and the harbor manifest already declares two lists with gate false (fixtures/manifests/harbor.yaml:55-60), so the clause shows nothing the cloudspec change did; the under-5-s bound is not enforced by the command"},
{"file":".engineering/planning/story/cloudspec-warnings.md","line":73,"category":"acceptance","severity":"warning","verdict":"NEEDS-CHANGE","origin":"introduced","message":"the acceptance joins the acceptance.sh row and the nextest run with 'and', two independent outcomes, one of which can pass while the other fails; part-block-aliases.md:102-107 and extends-inherits.md:114-120 do the same and add further outcomes"}
]
```
