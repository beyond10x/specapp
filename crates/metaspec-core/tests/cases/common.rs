//! Helpers shared by the tests: the fixture packs, their expectations, and the compiled model
//! of a pack's blocks, made once per digest of its sources.

use metaspec_core::{Pack, Report, load, load_with};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::Command;

/// The root of the repository.
pub fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repository root")
}

/// Every fixture pack: its name and its directory. A fixture pack's manifest is metaspec's,
/// `fixtures/manifests/<name>.yaml`; the pack directory `fixtures/packs/<name>` holds nothing
/// of metaspec.
pub fn packs() -> Vec<(String, PathBuf)> {
    let dir = repo().join("fixtures/packs");
    let mut packs: Vec<(String, PathBuf)> = std::fs::read_dir(&dir)
        .expect("fixtures/packs")
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .map(|entry| {
            (
                entry.file_name().to_string_lossy().into_owned(),
                entry.path(),
            )
        })
        .collect();
    packs.sort();
    assert!(packs.len() >= 2, "two fixture packs at least");
    let mut manifests: Vec<String> = std::fs::read_dir(repo().join("fixtures/manifests"))
        .expect("fixtures/manifests")
        .flatten()
        .filter_map(|e| {
            e.file_name()
                .to_string_lossy()
                .strip_suffix(".yaml")
                .map(String::from)
        })
        .collect();
    manifests.sort();
    let names: Vec<String> = packs.iter().map(|(n, _)| n.clone()).collect();
    assert_eq!(names, manifests, "one manifest per fixture pack");
    packs
}

/// The manifest of a fixture pack, `fixtures/manifests/<name>.yaml`, where `<name>` is the
/// last component of the pack directory: a copy of a fixture pack keeps its name.
pub fn manifest_of(dir: &Path) -> PathBuf {
    let name = dir.file_name().expect("a pack directory").to_string_lossy();
    repo().join(format!("fixtures/manifests/{name}.yaml"))
}

/// What `fixtures/expected/<pack>.json` says about a fixture pack.
pub fn expected(name: &str) -> Value {
    let path = repo().join(format!("fixtures/expected/{name}.json"));
    let text = std::fs::read_to_string(&path).expect("an expectation per fixture pack");
    serde_json::from_str(&text).expect("the expectation is JSON")
}

/// A directory for one test's files, under the target directory, empty.
pub fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("metaspec-core")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

/// The compiled model of a blocks directory. `ess specify compile` runs once per digest of
/// `ess --version` and the files of the directory; the result is kept under the target
/// directory and read by every later test.
pub fn compiled(blocks: &Path) -> Result<Value, String> {
    let mut hasher = Sha256::new();
    // In the blocks directory, where ess_compile runs: the release that answers there.
    let version = Command::new("ess")
        .arg("--version")
        .current_dir(blocks)
        .output()
        .map_err(|error| format!("ess cannot be started: {error}"))?;
    hasher.update(&version.stdout);
    let mut files = Vec::new();
    collect(blocks, &mut files);
    files.sort();
    for file in &files {
        hasher.update(
            file.strip_prefix(blocks)
                .unwrap_or(file)
                .to_string_lossy()
                .as_bytes(),
        );
        hasher.update([0]);
        hasher.update(std::fs::read(file).unwrap_or_default());
        hasher.update([0]);
    }
    let digest: String = hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let cache = Path::new(env!("CARGO_TARGET_TMPDIR")).join("ess-compile");
    let path = cache.join(format!("{digest}.json"));
    if let Ok(text) = std::fs::read_to_string(&path)
        && let Ok(value) = serde_json::from_str(&text)
    {
        return Ok(value);
    }
    let value = metaspec_core::ess_compile(blocks)?;
    std::fs::create_dir_all(&cache).map_err(|e| e.to_string())?;
    let partial = cache.join(format!("{digest}.{}.tmp", std::process::id()));
    std::fs::write(&partial, value.to_string()).map_err(|e| e.to_string())?;
    std::fs::rename(&partial, &path).map_err(|e| e.to_string())?;
    Ok(value)
}

/// Copies a directory tree, files and directories only.
pub fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let path = entry.path();
        let target = to.join(entry.file_name());
        if path.is_dir() {
            copy_dir(&path, &target);
        } else {
            std::fs::copy(&path, &target).unwrap();
        }
    }
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// Loads a fixture pack with its manifest (`manifest_of`) and the cached compiler.
pub fn load_pack(dir: &Path) -> (Report, Option<Pack>) {
    load(dir, &manifest_of(dir), &compiled)
}

/// Loads a pack with a manifest changed by `change`.
pub fn load_changed(
    dir: &Path,
    change: impl FnOnce(&mut metaspec_core::Manifest),
) -> (Report, Option<Pack>) {
    let mut manifest = metaspec_core::pack::read_manifest(&manifest_of(dir)).expect("the manifest");
    change(&mut manifest);
    load_with(dir, manifest, &compiled)
}

/// Validates a JSON document against a schema of `schema/`, and lists the violations.
pub fn schema_errors(schema_file: &str, value: &Value) -> Vec<String> {
    let text =
        std::fs::read_to_string(repo().join("schema").join(schema_file)).expect("the schema file");
    let schema: Value = serde_json::from_str(&text).expect("the schema is JSON");
    let validator = jsonschema::validator_for(&schema).expect("the schema compiles");
    validator
        .iter_errors(value)
        .map(|e| format!("{}: {e}", e.instance_path()))
        .collect()
}

/// The codes of a list of issues.
pub fn codes(issues: &[metaspec_core::Issue]) -> Vec<&str> {
    issues.iter().map(|i| i.code.as_str()).collect()
}
