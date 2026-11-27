//! The one integration-test binary of the specapp command line. Each test runs the built
//! binary on the fixture packs; `pack check` compiles a fixture's blocks with `ess` (a few
//! milliseconds each).

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn packs() -> Vec<PathBuf> {
    let mut packs: Vec<PathBuf> = std::fs::read_dir(repo().join("fixtures/packs"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    packs.sort();
    packs
}

fn specapp(args: &[&std::ffi::OsStr]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_specapp"))
        .args(args)
        .output()
        .expect("specapp starts")
}

fn schema_errors(value: &Value) -> Vec<String> {
    let text =
        std::fs::read_to_string(repo().join("schema/specapp-pack-check-1.schema.json")).unwrap();
    let schema: Value = serde_json::from_str(&text).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    validator
        .iter_errors(value)
        .map(|e| format!("{}: {e}", e.instance_path()))
        .collect()
}

#[test]
fn pack_check_json_follows_the_schema_and_exits_0_for_every_fixture_pack() {
    for dir in packs() {
        let out = specapp(&[
            "pack".as_ref(),
            "check".as_ref(),
            dir.as_os_str(),
            "--json".as_ref(),
        ]);
        assert_eq!(
            out.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&out.stdout)
        );
        let report: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(schema_errors(&report), Vec::<String>::new());
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let want: Value = serde_json::from_str(
            &std::fs::read_to_string(repo().join(format!("fixtures/expected/{name}.json")))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(report["counts"], want["counts"], "{name}");
    }
}

#[test]
fn pack_check_text_names_the_counts() {
    let dir = &packs()[0];
    let out = specapp(&["pack".as_ref(), "check".as_ref(), dir.as_os_str()]);
    assert_eq!(out.status.code(), Some(0));
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(text.lines().any(|l| l.starts_with("blocks: ")), "{text}");
    assert!(text.lines().any(|l| l.starts_with("kpis: ")), "{text}");
    assert!(text.lines().any(|l| l.starts_with("documents: ")), "{text}");
    assert!(text.ends_with("loaded: 0 errors, 0 warnings\n"), "{text}");
}

#[test]
fn a_refused_pack_exits_1_with_a_report_that_follows_the_schema() {
    let scratch = Path::new(env!("CARGO_TARGET_TMPDIR")).join("specapp-cli-refused");
    let _ = std::fs::remove_dir_all(&scratch);
    std::fs::create_dir_all(&scratch).unwrap();
    let manifest = scratch.join("leaving.yaml");
    std::fs::write(
        &manifest,
        "format: specapp-pack/1\npack: leaving\ntitle: Leaving\nvisibility: public\nblocks: { path: ../outside }\n",
    )
    .unwrap();
    let dir = &packs()[0];
    let out = specapp(&[
        "pack".as_ref(),
        "check".as_ref(),
        dir.as_os_str(),
        "--manifest".as_ref(),
        manifest.as_os_str(),
        "--json".as_ref(),
    ]);
    assert_eq!(out.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["valid"], Value::Bool(false));
    assert_eq!(schema_errors(&report), Vec::<String>::new());
}

#[test]
fn bad_usage_exits_2() {
    let out = specapp(&["pack".as_ref(), "check".as_ref()]);
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn pack_words_prints_every_kind() {
    for dir in packs() {
        let out = specapp(&["pack".as_ref(), "words".as_ref(), dir.as_os_str()]);
        assert_eq!(out.status.code(), Some(0));
        let text = String::from_utf8(out.stdout).unwrap();
        for kind in ["block", "layer", "link", "kpi_id", "rule"] {
            assert!(
                text.lines().any(|l| l.split(' ').next() == Some(kind)),
                "{}: no {kind}",
                dir.display()
            );
        }
        assert!(text.lines().all(|l| l.split(' ').count() == 2));
    }
}
