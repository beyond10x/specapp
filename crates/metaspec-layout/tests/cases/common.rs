//! Helpers: the fixture packs loaded with the compiled model made once per digest of the
//! blocks, every drawn document with its graph, the findings of its fixture tool, and its views.

use metaspec_core::graph::{self, Graph};
use metaspec_core::manifest::Shape;
use metaspec_core::pack::Pack;
use metaspec_core::tool::{self, Finding};
use metaspec_layout::{Style, ViewSpec, views_of};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repository root")
}

/// The compiled model of a blocks directory, once per digest of `ess --version` and its files.
pub fn compiled(blocks: &Path) -> Result<Value, String> {
    let mut hasher = Sha256::new();
    // In the blocks directory, where ess_compile runs: the release that answers there.
    let version = Command::new("ess")
        .arg("--version")
        .current_dir(blocks)
        .output()
        .map_err(|e| e.to_string())?;
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

/// Every fixture pack: its name and the loaded pack.
pub fn packs() -> Vec<(String, Pack)> {
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(repo().join("fixtures/packs"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    assert!(dirs.len() >= 2);
    dirs.into_iter()
        .map(|dir| {
            let name = dir.file_name().unwrap().to_string_lossy().into_owned();
            let manifest = repo().join(format!("fixtures/manifests/{name}.yaml"));
            let (report, pack) = metaspec_core::load(&dir, &manifest, &compiled);
            (name, pack.unwrap_or_else(|| panic!("{:?}", report.errors)))
        })
        .collect()
}

/// One drawn document of a fixture pack.
pub struct Drawn {
    pub pack: String,
    pub class: String,
    pub graph: Graph,
    pub findings: Vec<Finding>,
    pub views: Vec<ViewSpec>,
    pub style: Style,
}

/// Every `parts-links/1` document of both fixture packs with the findings its tool answers.
pub fn drawn() -> Vec<Drawn> {
    let mut out = Vec::new();
    for (name, pack) in packs() {
        let state = Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join("metaspec-layout-state")
            .join(&name);
        std::fs::create_dir_all(&state).unwrap();
        tool::trust(&state, &pack.root, pack.manifest.tool.as_ref().unwrap()).unwrap();
        let style = Style::of_pack(&pack);
        for (class, documents) in pack.manifest.documents.iter().zip(&pack.documents) {
            if class.shape != Shape::PartsLinks {
                continue;
            }
            for document in documents {
                let graph = graph::read(class, &document.path, document.value.as_ref().unwrap());
                let check = tool::check(
                    &pack,
                    &document.path,
                    Some(&graph),
                    Some(&state),
                    tool::DEADLINE,
                );
                assert_eq!(check.state, tool::CheckState::Ran, "{:?}", check.message);
                out.push(Drawn {
                    pack: name.clone(),
                    class: class.class.clone(),
                    views: views_of(class, &graph),
                    findings: check.findings().to_vec(),
                    graph,
                    style: style.clone(),
                });
            }
        }
    }
    out
}

pub fn schema_errors(schema_file: &str, value: &Value) -> Vec<String> {
    let text = std::fs::read_to_string(repo().join("schema").join(schema_file)).unwrap();
    let schema: Value = serde_json::from_str(&text).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    validator
        .iter_errors(value)
        .map(|e| format!("{}: {e}", e.instance_path()))
        .collect()
}

/// Compares `text` with the committed file `fixtures/expected/<rel>` byte for byte, or writes
/// it when `METASPEC_BLESS=1`.
pub fn golden(rel: &str, text: &str) {
    let path = repo().join("fixtures/expected").join(rel);
    if std::env::var("METASPEC_BLESS").as_deref() == Ok("1") {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, text).unwrap();
        return;
    }
    let want = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{rel}: {e} (METASPEC_BLESS=1 writes it)"));
    assert!(
        want == text,
        "{rel} differs from a fresh layout (METASPEC_BLESS=1 rewrites it)"
    );
}
