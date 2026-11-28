//! A manifest that `extends` a pack inherits the `blocks`, `kpis` and `tool` it leaves out, read
//! in the base pack's directory (`--base-pack`). The packs are the invented pair under
//! `fixtures/extends/`: a base pack and a private pack that extends it, each `<pack>/` beside its
//! manifest `manifests/<pack>.yaml`. No test names a word of them: the extending pack is the one
//! whose manifest has `extends`.

use super::common::{codes, compiled, copy_dir, repo, scratch};
use metaspec_core::manifest::{Kpis, Manifest};
use metaspec_core::pack::{BaseArgs, load_extending, read_manifest};
use metaspec_core::tool::{self, CheckState};
use metaspec_core::{Pack, Report, load};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// The base pack and the pack that extends it: each directory and manifest file.
struct Pair {
    base_dir: PathBuf,
    base_manifest: PathBuf,
    dir: PathBuf,
    manifest: PathBuf,
}

/// The pair under `fixtures/extends/`.
fn pair() -> Pair {
    pair_in(&repo().join("fixtures/extends"))
}

/// The pair in `root`, laid out as `fixtures/extends/`.
fn pair_in(root: &Path) -> Pair {
    let mut base = None;
    let mut extending = None;
    for entry in std::fs::read_dir(root.join("manifests")).expect("the manifests of the pair") {
        let path = entry.unwrap().path();
        let manifest = read_manifest(&path).expect("a manifest of the pair");
        let dir = root.join(&manifest.pack);
        if manifest.extends.is_some() {
            assert!(
                extending.replace((dir, path)).is_none(),
                "one extending pack"
            );
        } else {
            assert!(base.replace((dir, path)).is_none(), "one base pack");
        }
    }
    let (base_dir, base_manifest) = base.expect("a base pack");
    let (dir, manifest) = extending.expect("an extending pack");
    Pair {
        base_dir,
        base_manifest,
        dir,
        manifest,
    }
}

/// A copy of the pair under a scratch directory of the test.
fn pair_copy(name: &str) -> Pair {
    let root = scratch(name);
    copy_dir(&repo().join("fixtures/extends"), &root);
    pair_in(&root)
}

fn base_args(pair: &Pair) -> BaseArgs<'_> {
    BaseArgs {
        manifest: Some(&pair.base_manifest),
        dir: Some(&pair.base_dir),
    }
}

/// Loads the extending pack with its base.
fn load_extending_pack(pair: &Pair) -> (Report, Option<Pack>) {
    load_with(
        pair,
        read_manifest(&pair.manifest).unwrap(),
        base_args(pair),
    )
}

fn load_with(pair: &Pair, manifest: Manifest, base: BaseArgs) -> (Report, Option<Pack>) {
    load_extending(&pair.dir, manifest, base, &compiled)
}

/// Loads the base pack alone, with its own manifest.
fn load_base(pair: &Pair) -> Pack {
    let (report, pack) = load(&pair.base_dir, &pair.base_manifest, &compiled);
    pack.unwrap_or_else(|| panic!("the base pack loads: {:?}", report.errors))
}

fn loads(loaded: (Report, Option<Pack>)) -> (Report, Pack) {
    let (report, pack) = loaded;
    match pack {
        Some(pack) => (report, pack),
        None => panic!("refused: {:?}", report.errors),
    }
}

/// The base manifest's YAML changed by `change`, written beside the pair; its path.
fn base_variant(pair: &Pair, name: &str, change: impl FnOnce(&mut Value)) -> PathBuf {
    let text = std::fs::read_to_string(&pair.base_manifest).unwrap();
    let mut value: Value = serde_yaml_ng::from_str(&text).unwrap();
    change(&mut value);
    let path = scratch(name).join("base.yaml");
    std::fs::write(&path, serde_yaml_ng::to_string(&value).unwrap()).unwrap();
    path
}

fn canonical(path: &Path) -> PathBuf {
    path.canonicalize().unwrap()
}

#[test]
fn inherit_blocks() {
    let pair = pair();
    let base = load_base(&pair);
    let (report, pack) = loads(load_extending_pack(&pair));
    assert!(read_manifest(&pair.manifest).unwrap().blocks.is_none());
    assert!(report.valid);
    assert_eq!(pack.blocks, base.blocks);
    assert!(!pack.blocks.is_empty());
    assert_eq!(pack.model, base.model);
    assert_eq!(
        serde_json::to_value(pack.manifest.blocks()).unwrap(),
        serde_json::to_value(base.manifest.blocks()).unwrap(),
        "the section is inherited whole, aliases included"
    );
    assert!(!pack.manifest.blocks().aliases.is_empty());
    assert_eq!(pack.blocks_root(), base.root.as_path());
    assert_eq!(pack.root, canonical(&pair.dir));
    let counts = report.counts.as_ref().unwrap();
    let base_report = load(&pair.base_dir, &pair.base_manifest, &compiled).0;
    let base_counts = base_report.counts.as_ref().unwrap();
    assert_eq!(
        (counts.blocks, counts.layers),
        (base_counts.blocks, base_counts.layers)
    );
    // documents are never inherited: the extending pack's are its own
    let own = read_manifest(&pair.manifest).unwrap();
    assert_eq!(pack.manifest.documents.len(), own.documents.len());
    for document in pack.documents.iter().flatten() {
        assert!(
            pack.root.join(&document.path).is_file(),
            "{}",
            document.path
        );
    }
    assert_eq!(report.extends.as_deref(), own.extends.as_deref());
}

#[test]
fn inherit_kpis() {
    let pair = pair();
    let base = load_base(&pair);
    assert!(!base.kpis.is_empty());
    let (_, pack) = loads(load_extending_pack(&pair));
    assert_eq!(pack.kpis, base.kpis, "an omitted kpis is the base's");
    assert_eq!(pack.kpis_root(), base.root.as_path());

    // A manifest that declares kpis reads its own catalogue, whole.
    let own_dir = pair.dir.join("kpis");
    let file = std::fs::read_dir(&own_dir)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .find(|p| p.extension().is_some_and(|e| e == "yaml"))
        .expect("a catalogue of the extending pack");
    let catalogue: Value =
        serde_yaml_ng::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
    let mut manifest = read_manifest(&pair.manifest).unwrap();
    manifest.kpis = Some(Kpis {
        path: format!("kpis/{}", file.file_name().unwrap().to_string_lossy()),
        fields: Default::default(),
    });
    let (report, pack) = loads(load_with(&pair, manifest, base_args(&pair)));
    assert_eq!(&Value::Array(pack.kpis.clone()), &catalogue["kpis"]);
    assert_ne!(pack.kpis, base.kpis);
    assert_eq!(pack.kpis_root(), pack.root.as_path());
    assert_eq!(report.counts.unwrap().kpis, pack.kpis.len());
}

#[test]
fn inherit_tool_runs_in_base_dir() {
    let pair = pair_copy("inherit-tool");
    let base = load_base(&pair);
    let (report, pack) = loads(load_extending_pack(&pair));
    let tool = pack.manifest.tool.clone().expect("the inherited tool");
    let base_tool = base.manifest.tool.clone().unwrap();
    // The base's tool section, with the base directory it runs in (D66).
    assert_eq!(
        serde_json::to_value((&tool.run, &tool.verbs)).unwrap(),
        serde_json::to_value((&base_tool.run, &base_tool.verbs)).unwrap()
    );
    assert!(base_tool.base_dir.is_none());
    assert_eq!(
        tool.base_dir.as_deref(),
        Some(base.root.to_string_lossy().as_ref())
    );
    assert_ne!(tool::digest(&tool), tool::digest(&base_tool));
    assert_eq!(pack.tool_root(), base.root.as_path());
    let program = tool.run.iter().find(|a| a.contains('/')).unwrap();
    assert!(
        !pack.root.join(program).exists(),
        "the extending pack has no tool"
    );
    assert_eq!(
        report.tool.as_ref().unwrap().state,
        metaspec_core::report::ToolState::Found
    );

    let state = scratch("inherit-tool-state");
    tool::trust(&state, &pack.root, &tool).unwrap();
    let document = &pack.documents[0][0].path;
    let check = tool::check(&pack, document, None, Some(&state), tool::DEADLINE);
    assert_eq!(check.state, CheckState::Ran, "{:?}", check.message);
    let messages: Vec<&str> = check
        .findings()
        .iter()
        .map(|f| f.message.as_str())
        .collect();
    assert_eq!(
        messages,
        vec![
            base.root.to_string_lossy().as_ref(),
            pack.root.join(document).to_string_lossy().as_ref()
        ],
        "the stub prints its working directory and the document it was given"
    );
}

#[test]
fn inherit_alias_target_checked_against_base_blocks() {
    let pair = pair();
    let base = load_base(&pair);
    let block = base.blocks.last().unwrap().name.clone();
    let word = String::from("an-alias-word");
    let good = base_variant(&pair, "inherit-alias-good", |v| {
        v["blocks"]["aliases"] = serde_json::json!({ &word: &block });
    });
    let args = BaseArgs {
        manifest: Some(&good),
        dir: Some(&pair.base_dir),
    };
    let (_, pack) = loads(load_with(
        &pair,
        read_manifest(&pair.manifest).unwrap(),
        args,
    ));
    assert_eq!(pack.manifest.blocks().aliases.get(&word), Some(&block));

    let bad = base_variant(&pair, "inherit-alias-bad", |v| {
        v["blocks"]["aliases"] = serde_json::json!({ &word: "no-block-of-the-base" });
    });
    let args = BaseArgs {
        manifest: Some(&bad),
        dir: Some(&pair.base_dir),
    };
    let (report, pack) = load_with(&pair, read_manifest(&pair.manifest).unwrap(), args);
    assert!(pack.is_none());
    assert!(
        report
            .errors
            .iter()
            .any(|e| e.code == "manifest_invalid" && e.subject == format!("alias:{word}")),
        "{:?}",
        report.errors
    );
}

#[test]
fn inherit_without_base_pack_is_refused() {
    let pair = pair();
    let args = BaseArgs {
        manifest: Some(&pair.base_manifest),
        dir: None,
    };
    let (report, pack) = load_with(&pair, read_manifest(&pair.manifest).unwrap(), args);
    assert!(pack.is_none());
    assert!(!report.valid);
    assert_eq!(codes(&report.errors), vec!["base_pack_missing"]);
    assert!(
        report.errors[0].message.contains("--base-pack"),
        "{:?}",
        report.errors
    );
}

/// The refusal of a path of the extending pack that leaves it, and its message.
fn leaving_extending_pack() -> (Pair, Vec<metaspec_core::Issue>) {
    let pair = pair_copy("inherit-leave-extending");
    let mut manifest = read_manifest(&pair.manifest).unwrap();
    manifest.documents[0]
        .paths
        .push(String::from("../x/*.yaml"));
    let (report, pack) = load_with(&pair, manifest, base_args(&pair));
    assert!(pack.is_none());
    let mut errors = report.errors;

    // A document reached through a link out of the extending pack.
    let lamps = pair
        .dir
        .join(&read_manifest(&pair.manifest).unwrap().documents[0].paths[0]);
    let lamps = lamps.parent().unwrap().to_path_buf();
    std::fs::remove_dir_all(&lamps).unwrap();
    let outside = scratch("inherit-leave-extending-outside");
    copy_dir(
        &repo()
            .join("fixtures/extends")
            .join(pair.dir.file_name().unwrap())
            .join(lamps.file_name().unwrap()),
        &outside,
    );
    std::os::unix::fs::symlink(&outside, &lamps).unwrap();
    let (report, pack) = load_extending_pack(&pair);
    assert!(pack.is_none());
    errors.extend(report.errors);
    (pair, errors)
}

/// The refusals of the base pack's blocks and catalogue reached through links out of it.
fn leaving_base_pack() -> (Pair, Vec<metaspec_core::Issue>) {
    let fixture = pair();
    let base = read_manifest(&fixture.base_manifest).unwrap();
    let mut errors = Vec::new();
    for (name, section) in [
        (
            "inherit-leave-base-blocks",
            base.blocks.as_ref().unwrap().path.clone(),
        ),
        (
            "inherit-leave-base-kpis",
            base.kpis.as_ref().unwrap().path.clone(),
        ),
    ] {
        let pair = pair_copy(name);
        let top = section.split('/').next().unwrap().to_string();
        std::fs::remove_dir_all(pair.base_dir.join(&top)).unwrap();
        std::os::unix::fs::symlink(fixture.base_dir.join(&top), pair.base_dir.join(&top)).unwrap();
        let (report, pack) = load_extending_pack(&pair);
        assert!(pack.is_none(), "{name}");
        assert!(
            codes(&report.errors).contains(&"path_leaves_pack"),
            "{name}: {:?}",
            report.errors
        );
        errors.extend(report.errors);
    }
    (fixture, errors)
}

#[test]
fn inherit_path_leaving_extending_pack_is_refused() {
    let (_, errors) = leaving_extending_pack();
    assert_eq!(
        codes(&errors),
        vec!["path_leaves_pack", "path_leaves_pack"],
        "{errors:?}"
    );
}

#[test]
fn inherit_path_leaving_base_pack_is_refused() {
    let (_, errors) = leaving_base_pack();
    assert_eq!(errors.len(), 2, "{errors:?}");
    assert!(
        errors.iter().all(|e| e.code == "path_leaves_pack"),
        "{errors:?}"
    );
}

#[test]
fn inherit_refusal_names_the_pack() {
    let (pair, errors) = leaving_extending_pack();
    let own = read_manifest(&pair.manifest).unwrap().pack;
    let base = read_manifest(&pair.base_manifest).unwrap().pack;
    for e in &errors {
        assert!(e.message.contains(&format!("of the pack {own}")), "{e:?}");
        assert!(!e.message.contains(&format!("base pack {base}")), "{e:?}");
    }
    let (_, errors) = leaving_base_pack();
    for e in &errors {
        assert!(
            e.message.contains(&format!("of the base pack {base}")),
            "{e:?}"
        );
    }
}

#[test]
fn inherit_base_that_extends_is_refused() {
    let pair = pair();
    let chained = base_variant(&pair, "inherit-chain", |v| {
        v["extends"] = Value::from("another-base");
    });
    let args = BaseArgs {
        manifest: Some(&chained),
        dir: Some(&pair.base_dir),
    };
    let (report, pack) = load_with(&pair, read_manifest(&pair.manifest).unwrap(), args);
    assert!(pack.is_none());
    assert_eq!(
        codes(&report.errors),
        vec!["extends_chain"],
        "{:?}",
        report.errors
    );
    assert!(
        report.errors[0].message.contains("another-base"),
        "{:?}",
        report.errors
    );
}

#[test]
fn inherit_trust_digest_covers_base_tool() {
    let pair = pair();
    let state = scratch("inherit-trust-state");
    let (_, pack) = loads(load_extending_pack(&pair));
    let tool = pack.manifest.tool.clone().unwrap();
    tool::trust(&state, &pack.root, &tool).unwrap();
    assert!(tool::trusted(&state, &pack.root, &tool));

    // The base's tool section changes: the extending pack's tool is untrusted until trusted again.
    let changed = base_variant(&pair, "inherit-trust-changed", |v| {
        v["tool"]["verbs"]["check"]["args"]
            .as_array_mut()
            .unwrap()
            .push(Value::from("--again"));
    });
    let args = BaseArgs {
        manifest: Some(&changed),
        dir: Some(&pair.base_dir),
    };
    let (_, again) = loads(load_with(
        &pair,
        read_manifest(&pair.manifest).unwrap(),
        args,
    ));
    let new_tool = again.manifest.tool.clone().unwrap();
    assert_ne!(tool::digest(&tool), tool::digest(&new_tool));
    assert!(!tool::trusted(&state, &again.root, &new_tool));
    let check = tool::check(
        &again,
        &again.documents[0][0].path,
        None,
        Some(&state),
        tool::DEADLINE,
    );
    assert_eq!(check.state, CheckState::NotTrusted);
    tool::trust(&state, &again.root, &new_tool).unwrap();
    assert!(tool::trusted(&state, &again.root, &new_tool));
}

/// A copy of the pair whose extending pack holds a copy of the base's blocks, and the extending
/// manifest that writes them as its own `blocks` section.
fn pair_with_own_blocks(name: &str) -> (Pair, Manifest) {
    let pair = pair_copy(name);
    let base = read_manifest(&pair.base_manifest).unwrap();
    let blocks = base.blocks.clone().unwrap();
    copy_dir(
        &pair.base_dir.join(&blocks.path),
        &pair.dir.join(&blocks.path),
    );
    let mut manifest = read_manifest(&pair.manifest).unwrap();
    manifest.blocks = Some(blocks);
    (pair, manifest)
}

#[test]
fn inherit_own_blocks_without_base_pack_loads_as_before() {
    let (pair, manifest) = pair_with_own_blocks("inherit-own-blocks");
    let base = load_base(&pair);
    let args = BaseArgs {
        manifest: Some(&pair.base_manifest),
        dir: None,
    };
    let (report, pack) = loads(load_with(&pair, manifest.clone(), args));
    assert!(report.valid);
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    assert!(pack.base.is_none());
    assert!(pack.manifest.tool.is_none() && pack.kpis.is_empty());
    assert_eq!(pack.blocks_root(), pack.root.as_path());

    // With the base's directory, the omitted kpis and tool are the base's; the blocks stay own.
    let (_, pack) = loads(load_with(&pair, manifest, base_args(&pair)));
    let inherited = pack.base.as_ref().unwrap().inherited;
    assert!(!inherited.blocks && inherited.kpis && inherited.tool);
    assert_eq!(pack.blocks_root(), pack.root.as_path());
    assert_eq!(pack.kpis, base.kpis);
    assert_eq!(pack.tool_root(), base.root.as_path());
}

#[test]
fn inherit_base_that_extends_refuses_only_an_inheriting_manifest() {
    let (pair, mut manifest) = pair_with_own_blocks("inherit-chain-own");
    let base = read_manifest(&pair.base_manifest).unwrap();
    let chained = base_variant(&pair, "inherit-chain-own-base", |v| {
        v["extends"] = Value::from("another-base");
    });
    let args = BaseArgs {
        manifest: Some(&chained),
        dir: Some(&pair.base_dir),
    };
    // It omits kpis and tool, so it inherits them: refused.
    let (report, pack) = load_with(&pair, manifest.clone(), args);
    assert!(pack.is_none());
    assert_eq!(codes(&report.errors), vec!["extends_chain"]);

    // It writes all three sections: it inherits nothing and loads as before.
    let own_kpis = std::fs::read_dir(pair.dir.join("kpis"))
        .unwrap()
        .flatten()
        .next()
        .unwrap()
        .file_name();
    manifest.kpis = Some(Kpis {
        path: format!("kpis/{}", own_kpis.to_string_lossy()),
        fields: Default::default(),
    });
    manifest.tool = base.tool.clone();
    let (report, pack) = loads(load_with(&pair, manifest, args));
    assert!(report.valid);
    assert!(pack.base.is_none());
    assert!(
        report.warnings.iter().all(|w| w.code != "extends_chain"),
        "{:?}",
        report.warnings
    );
}

#[test]
fn inherit_base_pack_without_extends_warns_base_pack_unused() {
    let pair = pair();
    let args = BaseArgs {
        manifest: None,
        dir: Some(&pair.dir),
    };
    let manifest = read_manifest(&pair.base_manifest).unwrap();
    let (report, pack) = load_extending(&pair.base_dir, manifest, args, &compiled);
    let pack = pack.unwrap_or_else(|| panic!("refused: {:?}", report.errors));
    assert!(pack.base.is_none());
    assert_eq!(codes(&report.warnings), vec!["base_pack_unused"]);
}
