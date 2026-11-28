//! Every path a manifest names stays inside the pack; the manifest lies outside it.

use super::common::{codes, compiled, copy_dir, load_changed, manifest_of, packs, scratch};
use metaspec_core::paths::relative_inside;
use metaspec_core::{load, load_with};

#[test]
fn a_path_text_that_leaves_the_pack_is_refused() {
    for text in ["../x", "a/../../x", "/abs/x", "a/..", "a\\b", ""] {
        assert!(relative_inside(text).is_err(), "{text:?}");
    }
    for text in ["x", "a/b.yaml", "a/*.yaml", "./a/b", "a/b/"] {
        assert!(relative_inside(text).is_ok(), "{text:?}");
    }
}

#[test]
fn every_kind_of_manifest_path_that_leaves_the_pack_is_refused_before_anything_is_read() {
    let (_, dir) = packs().remove(0);
    let never = |_: &std::path::Path| -> Result<serde_json::Value, String> {
        panic!("nothing is compiled for a refused manifest")
    };
    let cases: [fn(&mut metaspec_core::Manifest); 4] = [
        |m| m.blocks.path = String::from("../blocks"),
        |m| m.kpis.as_mut().unwrap().path = String::from("/etc/hostname"),
        |m| m.documents[0].paths.push(String::from("../../*.yaml")),
        |m| m.tool.as_mut().unwrap().run[0] = String::from("../bin/program"),
    ];
    for change in cases {
        let mut manifest =
            metaspec_core::pack::read_manifest(&manifest_of(&dir)).expect("manifest");
        change(&mut manifest);
        let (report, pack) = load_with(&dir, manifest, &never);
        assert!(!report.valid);
        assert!(pack.is_none());
        assert_eq!(codes(&report.errors), vec!["path_leaves_pack"]);
    }
}

#[test]
fn a_symbolic_link_out_of_the_pack_is_refused() {
    let (name, fixture) = packs().remove(0);
    let pack = scratch("symlink-out").join(&name);
    std::fs::create_dir_all(&pack).unwrap();
    std::os::unix::fs::symlink(fixture.join("blocks"), pack.join("blocks")).unwrap();
    let (report, _) = load(&pack, &manifest_of(&pack), &compiled);
    assert!(!report.valid);
    assert!(
        codes(&report.errors).contains(&"path_leaves_pack"),
        "{:?}",
        report.errors
    );
}

#[test]
fn a_document_reached_through_a_link_out_of_the_pack_is_refused() {
    for (name, fixture) in packs() {
        let outside = scratch(&format!("outside-{name}"));
        std::fs::write(outside.join("stray.yaml"), "a: 1\n").unwrap();
        let dir = scratch(&format!("linked-{name}")).join(&name);
        copy_dir(&fixture, &dir);
        std::os::unix::fs::symlink(&outside, dir.join("linked")).unwrap();
        let (report, _) = load_changed(&dir, |m| {
            m.documents[0].paths.push(String::from("linked/*.yaml"));
        });
        assert!(!report.valid, "{name}");
        assert_eq!(codes(&report.errors), vec!["path_leaves_pack"], "{name}");
    }
}

#[test]
fn any_manifest_file_reads_the_pack() {
    for (name, dir) in packs() {
        let elsewhere = scratch(&format!("manifest-elsewhere-{name}"));
        let manifest = elsewhere.join("other.yaml");
        std::fs::copy(manifest_of(&dir), &manifest).unwrap();
        let (fixture, _) = load(&dir, &manifest_of(&dir), &compiled);
        let (other, _) = load(&dir, &manifest, &compiled);
        assert!(other.valid, "{name}: {:?}", other.errors);
        assert_eq!(fixture.counts, other.counts, "{name}");
    }
}

#[test]
fn a_missing_pack_or_manifest_is_refused() {
    let (_, dir) = packs().remove(0);
    let missing = scratch("missing").join("no-pack-here");
    let (report, _) = load(&missing, &missing.join("manifest.yaml"), &compiled);
    assert_eq!(codes(&report.errors), vec!["manifest_unreadable"]);
    let (report, _) = load(&missing, &manifest_of(&dir), &compiled);
    assert_eq!(codes(&report.errors), vec!["pack_missing"]);
}

#[test]
fn no_fixture_pack_holds_anything_of_metaspec() {
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else {
                out.push(path);
            }
        }
    }
    for (name, dir) in packs() {
        let mut files = Vec::new();
        walk(&dir, &mut files);
        assert!(!files.is_empty());
        for file in files {
            let text = std::fs::read_to_string(&file).unwrap_or_default();
            let shown = file.strip_prefix(&dir).unwrap().display().to_string();
            assert!(
                !shown.to_lowercase().contains("metaspec"),
                "{name}: {shown} is named after metaspec"
            );
            assert!(
                !text.to_lowercase().contains("metaspec"),
                "{name}: {shown} names metaspec"
            );
        }
    }
}
