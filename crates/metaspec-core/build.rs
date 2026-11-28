//! Embeds the manifests of the public packs metaspec knows, `packs/<name>.yaml`, so that a
//! pack can be named instead of its manifest's path. The table is generated into OUT_DIR; no
//! pack's name is written in the source.

use std::path::Path;

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let dir = Path::new(&root).join("../../packs");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut entries: Vec<(String, String)> = Vec::new();
    if let Ok(read) = std::fs::read_dir(&dir) {
        for entry in read.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("yaml") {
                continue;
            }
            let Some(name) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let full = path.canonicalize().expect("a manifest under packs/");
            println!("cargo:rerun-if-changed={}", full.display());
            entries.push((name.to_string(), full.to_string_lossy().into_owned()));
        }
    }
    entries.sort();
    let mut out =
        String::from("/// The manifests under packs/: per pack, its name and its YAML text.\n");
    out.push_str("pub const KNOWN: &[(&str, &str)] = &[\n");
    for (name, path) in &entries {
        out.push_str(&format!("    ({name:?}, include_str!({path:?})),\n"));
    }
    out.push_str("];\n");
    let target = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR")).join("known.rs");
    std::fs::write(target, out).expect("OUT_DIR/known.rs");
}
