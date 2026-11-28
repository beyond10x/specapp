//! Every path a manifest names stays inside the pack; the manifest lies outside it.

use super::common::{
    codes, compiled, copy_dir, load_changed, load_pack, manifest_of, packs, scratch,
};
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
        |m| m.blocks.as_mut().unwrap().path = String::from("../blocks"),
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

/// A copy of a fixture pack with two files beside the documents of its first class: one its
/// glob matches only (`zz-kept.yaml`, a copy of a document) and one that is no YAML document
/// (`zz-left.side.yaml`). Returns the copy and the directory of the first class.
fn pack_with_side_file(
    test: &str,
    name: &str,
    fixture: &std::path::Path,
) -> (std::path::PathBuf, String) {
    let dir = scratch(&format!("exclude-{test}-{name}")).join(name);
    copy_dir(fixture, &dir);
    let manifest = metaspec_core::pack::read_manifest(&manifest_of(&dir)).expect("manifest");
    let pattern = manifest.documents[0].paths[0].clone();
    let (folder, _) = pattern.rsplit_once('/').expect("a glob inside a directory");
    let first = std::fs::read_dir(dir.join(folder))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "yaml"))
        .min()
        .expect("a document of the first class");
    std::fs::copy(&first, dir.join(folder).join("zz-kept.yaml")).unwrap();
    std::fs::write(dir.join(folder).join("zz-left.side.yaml"), "{ not: [yaml\n").unwrap();
    (dir, folder.to_string())
}

#[test]
fn exclude_leaves_a_matched_file_unread_and_reads_the_rest() {
    for (name, fixture) in packs() {
        let (_, base) = load_pack(&fixture);
        let base = base.expect("the fixture pack loads").documents[0].len();
        let (dir, folder) = pack_with_side_file("unread", &name, &fixture);

        let (report, pack) = load_changed(&dir, |_| {});
        let read: Vec<String> = pack.unwrap().documents[0]
            .iter()
            .map(|d| d.path.clone())
            .collect();
        assert_eq!(
            read.len(),
            base + 2,
            "{name}: without exclude both are read"
        );
        assert!(
            codes(&report.warnings).contains(&"document_unreadable"),
            "{name}: {:?}",
            report.warnings
        );

        let (report, pack) = load_changed(&dir, |m| {
            m.documents[0].exclude = vec![format!("{folder}/*.side.yaml")];
        });
        assert!(report.valid, "{name}: {:?}", report.errors);
        let read: Vec<String> = pack.unwrap().documents[0]
            .iter()
            .map(|d| d.path.clone())
            .collect();
        assert_eq!(read.len(), base + 1, "{name}: {read:?}");
        assert!(read.contains(&format!("{folder}/zz-kept.yaml")), "{name}");
        assert!(
            !read.contains(&format!("{folder}/zz-left.side.yaml")),
            "{name}"
        );
        assert!(
            !codes(&report.warnings).contains(&"document_unreadable"),
            "{name}: the excluded file is not read: {:?}",
            report.warnings
        );
    }
}

#[test]
fn exclude_matches_within_one_directory_and_leaves_the_others() {
    let (name, fixture) = packs().remove(0);
    let (dir, folder) = pack_with_side_file("one-directory", &name, &fixture);
    let (_, pack) = load_changed(&dir, |m| {
        m.documents[0].exclude = vec![String::from("*.side.yaml")];
    });
    let read: Vec<String> = pack.unwrap().documents[0]
        .iter()
        .map(|d| d.path.clone())
        .collect();
    assert!(
        read.contains(&format!("{folder}/zz-left.side.yaml")),
        "a glob without a directory does not reach into {folder}/: {read:?}"
    );
}

#[test]
fn exclude_that_leaves_the_pack_is_refused_before_anything_is_read() {
    let (_, dir) = packs().remove(0);
    let never = |_: &std::path::Path| -> Result<serde_json::Value, String> {
        panic!("nothing is compiled for a refused manifest")
    };
    let mut manifest = metaspec_core::pack::read_manifest(&manifest_of(&dir)).expect("manifest");
    manifest.documents[0].exclude = vec![String::from("../*.yaml")];
    let (report, pack) = load_with(&dir, manifest, &never);
    assert!(!report.valid);
    assert!(pack.is_none());
    assert_eq!(codes(&report.errors), vec!["path_leaves_pack"]);
}

#[test]
fn exclude_that_matches_no_file_is_named() {
    let (name, fixture) = packs().remove(0);
    let (dir, folder) = pack_with_side_file("dead", &name, &fixture);
    let dead = format!("{folder}/*.nothing.yaml");
    let (report, pack) = load_changed(&dir, |m| {
        m.documents[0].exclude = vec![format!("{folder}/*.side.yaml"), dead.clone()];
    });
    assert!(report.valid, "{:?}", report.errors);
    assert!(pack.is_some());
    let dead_ones: Vec<&metaspec_core::Issue> = report
        .warnings
        .iter()
        .filter(|w| w.code == "exclude_matches_nothing")
        .collect();
    assert_eq!(dead_ones.len(), 1, "{:?}", report.warnings);
    assert_eq!(dead_ones[0].subject, format!("path:{dead}"));
}

#[test]
fn exclude_that_leaves_no_file_says_so_and_not_that_the_paths_match_none() {
    let (name, fixture) = packs().remove(0);
    let (dir, _) = pack_with_side_file("all-left-out", &name, &fixture);
    let (report, _) = load_changed(&dir, |m| {
        m.documents[0].exclude = m.documents[0].paths.clone();
    });
    let empty: Vec<&metaspec_core::Issue> = report
        .warnings
        .iter()
        .filter(|w| w.code == "class_empty")
        .collect();
    assert_eq!(empty.len(), 1, "{:?}", report.warnings);
    assert!(
        empty[0].message.contains("exclude"),
        "{:?}",
        empty[0].message
    );
    assert!(
        !empty[0].message.contains("match no file"),
        "{:?}",
        empty[0].message
    );
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
