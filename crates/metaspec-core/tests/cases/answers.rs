//! What the tools of the known packs answer, recorded. `fixtures/answers/<pack>/<class>/` holds
//! per case an invented document (`<case>.yaml`), what the pack's own tool printed and exited
//! with for its check (`<case>.stdout.json`, `<case>.exit`, written by
//! `scripts/record-answers.sh <pack> <pack-dir>`), and the `metaspec-findings/1` the adapter of
//! `packs/<pack>.yaml` reads from that answer (`<case>.findings.json`; `METASPEC_BLESS=1`
//! rewrites it). No pack's tool runs here, and no word of a pack is written here.

use super::common::{repo, schema_errors};
use metaspec_core::adapter::{self, Exit, Mapped};
use metaspec_core::graph;
use metaspec_core::known;
use metaspec_core::manifest::{Shape, parse};
use metaspec_core::tool::Subject;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// Every recorded case: its pack, its document class and the path of its document.
fn cases() -> Vec<(String, String, PathBuf)> {
    let mut cases = Vec::new();
    let root = repo().join("fixtures/answers");
    for pack in sorted(&root) {
        for class in sorted(&pack) {
            for file in sorted(&class) {
                let name = file.file_name().unwrap().to_string_lossy().into_owned();
                if name.ends_with(".yaml") {
                    cases.push((base(&pack), base(&class), file));
                }
            }
        }
    }
    cases
}

fn sorted(dir: &Path) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .flatten()
        .map(|e| e.path())
        .collect();
    entries.sort();
    entries
}

fn base(path: &Path) -> String {
    path.file_name().unwrap().to_string_lossy().into_owned()
}

fn beside(document: &Path, suffix: &str) -> PathBuf {
    let stem = document.file_stem().unwrap().to_string_lossy();
    document.with_file_name(format!("{stem}.{suffix}"))
}

/// Compares `text` with a committed file byte for byte, or writes it when `METASPEC_BLESS=1`.
fn golden(path: &Path, text: &str) {
    if std::env::var("METASPEC_BLESS").as_deref() == Ok("1") {
        std::fs::write(path, text).unwrap();
        return;
    }
    let want = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: {e} (METASPEC_BLESS=1 writes it)", path.display()));
    assert!(
        want == text,
        "{} differs from what the adapter reads (METASPEC_BLESS=1 rewrites it):\n{text}",
        path.display()
    );
}

#[test]
fn every_recorded_answer_of_a_known_pack_is_read_by_the_adapter_of_its_manifest() {
    let cases = cases();
    assert!(!cases.is_empty(), "fixtures/answers holds a recorded case");
    let mut packs: std::collections::BTreeMap<String, [usize; 4]> = Default::default();
    for (pack, class, document) in &cases {
        let at = document.strip_prefix(repo()).unwrap().display().to_string();
        let text = known::manifest(pack).unwrap_or_else(|| panic!("{at}: {pack} is no known pack"));
        let manifest = parse(text).unwrap_or_else(|e| panic!("{pack}: {e:?}"));
        let adapter = manifest
            .tool
            .as_ref()
            .and_then(|t| t.verbs.check.as_ref())
            .unwrap_or_else(|| panic!("{at}: packs/{pack}.yaml declares no check"));
        let spec = manifest
            .documents
            .iter()
            .find(|c| &c.class == class)
            .unwrap_or_else(|| panic!("{at}: packs/{pack}.yaml has no class {class}"));
        assert_eq!(spec.shape, Shape::PartsLinks, "{at}");

        let value: Value =
            serde_yaml_ng::from_str(&std::fs::read_to_string(document).unwrap()).unwrap();
        let g = graph::read(spec, &format!("{class}/{}", base(document)), &value);
        let code: i32 = std::fs::read_to_string(beside(document, "exit"))
            .unwrap_or_else(|e| panic!("{at}: no exit code: {e}"))
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("{at}: the exit code: {e}"));
        let stdout = std::fs::read(beside(document, "stdout.json"))
            .unwrap_or_else(|e| panic!("{at}: no answer: {e}"));
        let exit = Exit {
            code: Some(code),
            stdout,
            stderr: String::new(),
        };
        let result = match adapter::check(adapter, &exit, Some(&g)) {
            Mapped::Findings(f) => f,
            other => panic!("{at}: {other:?}"),
        };
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(
            schema_errors("metaspec-findings-1.schema.json", &json),
            Vec::<String>::new(),
            "{at}"
        );
        golden(
            &beside(document, "findings.json"),
            &format!("{}\n", serde_json::to_string_pretty(&json).unwrap()),
        );

        let tally = packs.entry(pack.clone()).or_default();
        if result.valid {
            tally[0] += 1;
        } else {
            tally[1] += 1;
        }
        for finding in &result.findings {
            match finding.subject() {
                Subject::Part(id) => {
                    assert!(g.part(id).is_some(), "{at}: the part {id} is drawn");
                    tally[2] += 1;
                }
                Subject::Link(id) => {
                    assert!(g.link(id).is_some(), "{at}: the link {id} is drawn");
                    tally[3] += 1;
                }
                Subject::Document => {}
            }
        }
    }
    for (pack, [valid, refused, parts, links]) in packs {
        assert!(valid > 0, "{pack}: a recorded case the tool accepts");
        assert!(refused > 0, "{pack}: a recorded case the tool refuses");
        assert!(parts > 0, "{pack}: a finding on a drawn part");
        assert!(links > 0, "{pack}: a finding on a drawn link");
    }
}
