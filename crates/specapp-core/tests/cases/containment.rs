//! Every path a manifest names stays inside the pack; a manifest may lie outside it.

use super::common::{codes, compiled, copy_dir, load_changed, packs, scratch};
use specapp_core::paths::relative_inside;
use specapp_core::{load, load_with};

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
    let cases: [fn(&mut specapp_core::Manifest); 4] = [
        |m| m.blocks.path = String::from("../blocks"),
        |m| m.kpis.as_mut().unwrap().path = String::from("/etc/hostname"),
        |m| m.documents[0].paths.push(String::from("../../*.yaml")),
        |m| m.tool.as_mut().unwrap().run[0] = String::from("../bin/program"),
    ];
    for change in cases {
        let mut manifest =
            specapp_core::pack::read_manifest(&dir.join("specapp.yaml")).expect("manifest");
        change(&mut manifest);
        let (report, pack) = load_with(&dir, manifest, &never);
        assert!(!report.valid);
        assert!(pack.is_none());
        assert_eq!(codes(&report.errors), vec!["path_leaves_pack"]);
    }
}

#[test]
fn a_symbolic_link_out_of_the_pack_is_refused() {
    let (_, fixture) = packs().remove(0);
    let pack = scratch("symlink-out");
    std::fs::copy(fixture.join("specapp.yaml"), pack.join("specapp.yaml")).unwrap();
    std::os::unix::fs::symlink(fixture.join("blocks"), pack.join("blocks")).unwrap();
    let (report, _) = load(&pack, None, &compiled);
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
        let dir = scratch(&format!("linked-{name}"));
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
fn a_manifest_outside_the_pack_reads_the_pack() {
    for (name, dir) in packs() {
        let elsewhere = scratch(&format!("manifest-outside-{name}"));
        let manifest = elsewhere.join("other.yaml");
        std::fs::copy(dir.join("specapp.yaml"), &manifest).unwrap();
        let (inside, _) = load(&dir, None, &compiled);
        let (outside, _) = load(&dir, Some(&manifest), &compiled);
        assert!(outside.valid, "{name}: {:?}", outside.errors);
        assert_eq!(inside.counts, outside.counts, "{name}");
    }
}

#[test]
fn a_missing_pack_or_manifest_is_refused() {
    let missing = scratch("missing").join("no-pack-here");
    let (report, _) = load(&missing, None, &compiled);
    assert_eq!(codes(&report.errors), vec!["manifest_unreadable"]);
}
