---
format: aep.planning-md/3
id: dependency-blocker:cloudspec-architectures-dir
kind: dependency-blocker
status: cleared
title: cloudspec has no architectures/ directory; its documents live under fixtures/cloudspec/
relations:
- blocks: story:cloudspec-manifest
revision: 4
transitions:
- {from: "open", to: "cleared", at: "2026-11-28T14:18:57Z", actor: "human:operator", revision: 4}
---
Cleared by cloudspec's story:architectures-directory (cloudspec's plan, 2026-11-28): hand-written architectures move to `architectures/<name>.yaml`, broken ones to `architectures/broken/<rule_id>.yaml`, with a test that every rule has one broken file.

Cleared on 2026-11-28: cloudspec#17 merged as e79e07a (on cloudspec main). `architectures/` holds
the architectures and 28 entries under `architectures/broken/`. Note for story:cloudspec-manifest:
`architectures/` also holds files that are not architectures (`*.binding.yaml`,
`*.contract.yaml`), so the manifest's document paths must leave them out or give them classes of
their own.
