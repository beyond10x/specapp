//! The edges of a class's `exclude` and of a link subject the adapter maps: a glob written with
//! `./`, `**`, an exclude that is no glob, and a link without `carries` named by a tool.

use super::common::{codes, copy_dir, load_changed, manifest_of, packs, scratch};
use metaspec_core::adapter::{self, Exit, Mapped};
use metaspec_core::graph;
use metaspec_core::manifest::{CheckAdapter, parse};
use serde_json::json;
use std::path::PathBuf;

/// A copy of the first fixture pack with a file `zz-left.side.yaml` that is no YAML document
/// beside the documents of its first class (and one in `<folder>/deeper/`). Returns the copy and
/// the directory of the first class.
fn pack_with_side_files(name: &str) -> (PathBuf, String) {
    let (pack, fixture) = packs().remove(0);
    let dir = scratch(&format!("exclude-edges-{name}")).join(&pack);
    copy_dir(&fixture, &dir);
    let manifest = metaspec_core::pack::read_manifest(&manifest_of(&dir)).expect("manifest");
    let pattern = manifest.documents[0].paths[0].clone();
    let (folder, _) = pattern.rsplit_once('/').expect("a glob inside a directory");
    std::fs::write(dir.join(folder).join("zz-left.side.yaml"), "{ not: [yaml\n").unwrap();
    std::fs::create_dir_all(dir.join(folder).join("deeper")).unwrap();
    std::fs::write(
        dir.join(folder).join("deeper").join("zz-low.side.yaml"),
        "{ not: [yaml\n",
    )
    .unwrap();
    (dir, folder.to_string())
}

fn read_of(pack: Option<metaspec_core::Pack>) -> Vec<String> {
    pack.expect("the pack loads").documents[0]
        .iter()
        .map(|d| d.path.clone())
        .collect()
}

fn minimal_class(extra: &str) -> String {
    format!(
        "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: {{ path: b }}\ndocuments:\n  - {{ class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n{extra} }}\n"
    )
}

/// The schema and the containment check take `./d/*.side.yaml` as a path inside the pack; it
/// names the same files as `d/*.side.yaml`. A manifest that writes it must either be refused or
/// leave those files unread, never be taken and then exclude nothing.
#[test]
fn exclude_with_a_leading_dot_slash_leaves_the_file_unread_or_is_refused() {
    assert!(
        parse(&minimal_class(", exclude: [./d/*.side.yaml]")).is_ok(),
        "the schema takes an exclude written with ./"
    );
    let (dir, folder) = pack_with_side_files("dot-slash-exclude");
    let (report, pack) = load_changed(&dir, |m| {
        m.documents[0].exclude = vec![format!("./{folder}/*.side.yaml")];
    });
    if report.valid {
        let read = read_of(pack);
        assert!(
            !read.contains(&format!("{folder}/zz-left.side.yaml")),
            "taken, and the file the exclude names is read: {read:?}; warnings {:?}",
            codes(&report.warnings)
        );
    }
}

/// The same with the `./` on `paths`: the exclude is matched against the whole path in the pack.
#[test]
fn paths_with_a_leading_dot_slash_still_honour_the_exclude() {
    let (dir, folder) = pack_with_side_files("dot-slash-paths");
    let (report, pack) = load_changed(&dir, |m| {
        m.documents[0].paths = vec![format!("./{folder}/*.yaml")];
        m.documents[0].exclude = vec![format!("{folder}/*.side.yaml")];
    });
    if report.valid {
        let read = read_of(pack);
        assert!(
            !read.iter().any(|p| p.ends_with("zz-left.side.yaml")),
            "taken, and the file the exclude names is read: {read:?}; warnings {:?}",
            codes(&report.warnings)
        );
    }
}

/// `**/` crosses directories in an exclude as it does in `paths`.
#[test]
fn exclude_with_double_star_reaches_every_directory() {
    let (dir, folder) = pack_with_side_files("double-star");
    let (report, pack) = load_changed(&dir, |m| {
        m.documents[0].paths = vec![format!("{folder}/**/*.yaml")];
        m.documents[0].exclude = vec![String::from("**/*.side.yaml")];
    });
    assert!(report.valid, "{:?}", report.errors);
    let read = read_of(pack);
    assert!(!read.iter().any(|p| p.ends_with(".side.yaml")), "{read:?}");
    assert!(!read.is_empty());
}

/// An exclude that is no glob is refused with the code a bad `paths` glob gets.
#[test]
fn exclude_that_is_no_glob_is_refused() {
    let (dir, _) = pack_with_side_files("no-glob");
    let (report, pack) = load_changed(&dir, |m| {
        m.documents[0].exclude = vec![String::from("d/[.yaml")];
    });
    assert!(!report.valid);
    assert!(pack.is_none());
    assert_eq!(codes(&report.errors), vec!["path_invalid"]);
}

/// A symbolic link that the exclude names and that points out of the pack is left unread, not
/// refused and not read.
#[test]
fn exclude_leaves_a_link_out_of_the_pack_unread() {
    let (dir, folder) = pack_with_side_files("symlink");
    let outside = dir.parent().unwrap().join("outside.yaml");
    std::fs::write(&outside, "n: outside\n").unwrap();
    std::os::unix::fs::symlink(&outside, dir.join(&folder).join("zz-out.side.yaml")).unwrap();
    let (report, pack) = load_changed(&dir, |m| {
        m.documents[0].exclude = vec![format!("{folder}/*.side.yaml")];
    });
    assert!(report.valid, "{:?}", report.errors);
    assert!(!read_of(pack).iter().any(|p| p.ends_with(".side.yaml")));
}

fn adapter_of(yaml: &str) -> CheckAdapter {
    let text = format!(
        "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: {{ path: b }}\ntool:\n  run: [t]\n  verbs:\n    check:\n{}",
        yaml.lines()
            .map(|l| format!("      {l}\n"))
            .collect::<String>()
    );
    parse(&text).unwrap().tool.unwrap().verbs.check.unwrap()
}

/// A tool that writes a link as `<from>-><to>:<carries>` writes a link without `carries` as
/// `p->q:` (the empty text). The document holds that link, so the finding is on the link, not
/// on the document.
#[test]
fn a_link_without_carries_named_by_the_tool_is_found_on_its_link() {
    let m = parse(&minimal_class("")).unwrap();
    let doc = json!({"n": "whole", "parts": {"p": {"block": "b"}, "q": {"block": "b"}},
        "links": [{"from": "p", "to": "q"}]});
    let g = graph::read(&m.documents[0], "d/x.yaml", &doc);
    let a = adapter_of(
        "findings:\n  - at: /items\n    rule: \"{/r}\"\n    gate: { at: /g }\n    message: \"{/m}\"\n    subject: { id: \"{/s}\", link_when_contains: \"->\", unknown: document }\n",
    );
    let answer = json!({"items": [{"r": "one", "g": true, "m": "on the link", "s": "p->q:"}]});
    let exit = Exit {
        code: Some(0),
        stdout: answer.to_string().into_bytes(),
        stderr: String::new(),
    };
    let f = match adapter::check(&a, &exit, Some(&g)) {
        Mapped::Findings(f) => f,
        other => panic!("{other:?}"),
    };
    assert!(
        f.findings[0].subject.starts_with("link:"),
        "the graph's link ids {:?}; the finding {:?}",
        g.links.iter().map(|l| l.id.as_str()).collect::<Vec<_>>(),
        f.findings[0]
    );
}
