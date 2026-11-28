//! Adversarial cases of a manifest that `extends` a pack and inherits its tool: what the trust of
//! an inherited tool is bound to, and whether a tool path of the base pack that a link carries
//! out of it is run. The packs are the invented pair under `fixtures/extends/`, copied to a
//! scratch directory; no case names a word of them.

use super::common::{compiled, copy_dir, repo, scratch};
use metaspec_core::pack::{BaseArgs, load_extending, read_manifest};
use metaspec_core::tool::{self, CheckState};
use metaspec_core::{Pack, load};
use std::path::{Path, PathBuf};

/// The base pack and the pack that extends it in `root`, laid out as `fixtures/extends/`:
/// `(base dir, base manifest, extending dir, extending manifest)`.
fn pair_in(root: &Path) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let mut base = None;
    let mut extending = None;
    for entry in std::fs::read_dir(root.join("manifests")).unwrap().flatten() {
        let path = entry.path();
        let manifest = read_manifest(&path).unwrap();
        let dir = root.join(&manifest.pack);
        if manifest.extends.is_some() {
            extending = Some((dir, path));
        } else {
            base = Some((dir, path));
        }
    }
    let (base_dir, base_manifest) = base.unwrap();
    let (dir, manifest) = extending.unwrap();
    (base_dir, base_manifest, dir, manifest)
}

fn load_extending_with_base(
    dir: &Path,
    manifest: &Path,
    base_manifest: &Path,
    base_dir: &Path,
) -> Pack {
    let (report, pack) = load_extending(
        dir,
        read_manifest(manifest).unwrap(),
        BaseArgs {
            manifest: Some(base_manifest),
            dir: Some(base_dir),
        },
        &compiled,
    );
    pack.unwrap_or_else(|| panic!("refused: {:?}", report.errors))
}

/// The program of the base's tool, the first element of `tool.run` with a slash.
fn tool_program(base_manifest: &Path) -> String {
    read_manifest(base_manifest)
        .unwrap()
        .tool
        .unwrap()
        .run
        .into_iter()
        .find(|a| a.contains('/'))
        .expect("a tool path inside the base pack")
}

/// Writes an executable POSIX sh program at `path` that answers like the base's tool, with the
/// one note `marker` instead of its own.
fn write_answering_program(path: &Path, marker: &str) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::remove_file(path);
    std::fs::write(
        path,
        format!(
            "#!/bin/sh\nprintf '{{\"verdict\": \"lit\", \"notes\": [{{\"rule\": \"{marker}\", \"text\": \"{marker}\"}}]}}\\n'\n"
        ),
    )
    .unwrap();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
}

/// Trust belongs to the directory (design, "Trust"): a pack's own tool trusted in one directory
/// is not trusted in another. An inherited tool runs from the base pack's directory, which
/// `--base-pack` gives on every run. Trusting the extending pack with one base directory must
/// not make the same tool section trusted when it is read in another base directory, whose
/// program is other code.
#[test]
fn inherit_trust_is_bound_to_the_base_directory() {
    let root = scratch("adversary-inherit-trust-base-dir");
    copy_dir(&repo().join("fixtures/extends"), &root);
    let (base_dir, base_manifest, dir, manifest) = pair_in(&root);

    // A second checkout of the base pack whose tool program is other code.
    let other_base = root.join("other-checkout-of-the-base");
    copy_dir(&base_dir, &other_base);
    let program = tool_program(&base_manifest);
    write_answering_program(&other_base.join(&program), "other-code-ran");

    let state = scratch("adversary-inherit-trust-base-dir-state");
    let trusted_pack = load_extending_with_base(&dir, &manifest, &base_manifest, &base_dir);
    let tool = trusted_pack.manifest.tool.clone().unwrap();
    tool::trust(&state, &trusted_pack.root, &tool).unwrap();
    let document = trusted_pack.documents[0][0].path.clone();
    let first = tool::check(&trusted_pack, &document, None, Some(&state), tool::DEADLINE);
    assert_eq!(first.state, CheckState::Ran, "{:?}", first.message);

    let other = load_extending_with_base(&dir, &manifest, &base_manifest, &other_base);
    let check = tool::check(&other, &document, None, Some(&state), tool::DEADLINE);
    let rules: Vec<&str> = check.findings().iter().map(|f| f.rule.as_str()).collect();
    assert_eq!(
        check.state,
        CheckState::NotTrusted,
        "the tool of another base directory ran without being trusted; its findings: {rules:?}"
    );
}

/// A tool path of the base pack that a symbolic link carries out of it: the loader says the
/// tool cannot be started (`tool_not_started`, "no file inside the pack"), and the check must
/// not run the program outside the base pack all the same ("none of the base's leaves the base").
#[test]
fn inherit_tool_linked_out_of_the_base_pack_is_not_run() {
    let root = scratch("adversary-inherit-tool-link");
    copy_dir(&repo().join("fixtures/extends"), &root);
    let (base_dir, base_manifest, dir, manifest) = pair_in(&root);
    let program = tool_program(&base_manifest);
    let outside = scratch("adversary-inherit-tool-link-outside").join("program");
    write_answering_program(&outside, "outside-the-base-ran");
    let inside = base_dir.join(&program);
    std::fs::remove_file(&inside).unwrap();
    std::os::unix::fs::symlink(&outside, &inside).unwrap();

    let (report, pack) = load_extending(
        &dir,
        read_manifest(&manifest).unwrap(),
        BaseArgs {
            manifest: Some(&base_manifest),
            dir: Some(&base_dir),
        },
        &compiled,
    );
    let pack = pack.unwrap_or_else(|| panic!("refused: {:?}", report.errors));
    assert!(
        report.warnings.iter().any(|w| w.code == "tool_not_started"),
        "{:?}",
        report.warnings
    );
    let state = scratch("adversary-inherit-tool-link-state");
    let tool = pack.manifest.tool.clone().unwrap();
    tool::trust(&state, &pack.root, &tool).unwrap();
    let document = pack.documents[0][0].path.clone();
    let check = tool::check(&pack, &document, None, Some(&state), tool::DEADLINE);
    let rules: Vec<&str> = check.findings().iter().map(|f| f.rule.as_str()).collect();
    assert_ne!(
        check.state,
        CheckState::Ran,
        "a program outside the base pack ran; its findings: {rules:?}"
    );
}

/// The same link in a pack's own tool, without `extends`: shows whether the run of a program a
/// link carries out of the pack is older than inheritance.
#[test]
fn own_tool_linked_out_of_its_pack_is_not_run() {
    let root = scratch("adversary-own-tool-link");
    copy_dir(&repo().join("fixtures/extends"), &root);
    let (base_dir, base_manifest, _, _) = pair_in(&root);
    let program = tool_program(&base_manifest);
    let outside = scratch("adversary-own-tool-link-outside").join("program");
    write_answering_program(&outside, "outside-the-pack-ran");
    let inside = base_dir.join(&program);
    std::fs::remove_file(&inside).unwrap();
    std::os::unix::fs::symlink(&outside, &inside).unwrap();

    let (report, pack) = load(&base_dir, &base_manifest, &compiled);
    let pack = pack.unwrap_or_else(|| panic!("refused: {:?}", report.errors));
    assert!(
        report.warnings.iter().any(|w| w.code == "tool_not_started"),
        "{:?}",
        report.warnings
    );
    let state = scratch("adversary-own-tool-link-state");
    let tool = pack.manifest.tool.clone().unwrap();
    tool::trust(&state, &pack.root, &tool).unwrap();
    let document = pack.documents[0][0].path.clone();
    let check = tool::check(&pack, &document, None, Some(&state), tool::DEADLINE);
    let rules: Vec<&str> = check.findings().iter().map(|f| f.rule.as_str()).collect();
    assert_ne!(
        check.state,
        CheckState::Ran,
        "a program outside the pack ran; its findings: {rules:?}"
    );
}
