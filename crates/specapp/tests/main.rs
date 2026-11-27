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

// --- trust and render ---

/// A scratch directory under the target directory, empty.
fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("specapp-cli")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Runs specapp with `state` as XDG_STATE_HOME.
fn specapp_with(state: &Path, args: &[&std::ffi::OsStr]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_specapp"))
        .env("XDG_STATE_HOME", state)
        .args(args)
        .output()
        .expect("specapp starts")
}

fn render(state: &Path, pack: &Path, out: &Path, extra: &[&str]) -> Output {
    let mut args: Vec<&std::ffi::OsStr> = vec![
        "render".as_ref(),
        "--pack".as_ref(),
        pack.as_os_str(),
        "--out".as_ref(),
        out.as_os_str(),
    ];
    args.extend(extra.iter().map(|a| std::ffi::OsStr::new(*a)));
    specapp_with(state, &args)
}

fn trust(state: &Path, pack: &Path) -> Output {
    specapp_with(state, &["trust".as_ref(), pack.as_os_str()])
}

/// The JSON a page carries in its data element.
fn page_data(html: &str) -> Value {
    let start = "<script type=\"application/json\" id=\"specapp-data\">";
    let from = html.find(start).expect("a data element") + start.len();
    let to = from + html[from..].find("</script>").unwrap();
    serde_json::from_str(&html[from..to]).expect("the data is JSON")
}

fn schema(file: &str, value: &Value) -> Vec<String> {
    let text = std::fs::read_to_string(repo().join("schema").join(file)).unwrap();
    let schema: Value = serde_json::from_str(&text).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    validator
        .iter_errors(value)
        .map(|e| format!("{}: {e}", e.instance_path()))
        .collect()
}

/// Every file under `dir`, relative, sorted.
fn files(dir: &Path) -> Vec<String> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(root, &path, out);
            } else {
                out.push(
                    path.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }
    let mut out = Vec::new();
    walk(dir, dir, &mut out);
    out.sort();
    out
}

#[test]
fn render_of_a_trusted_fixture_pack_writes_the_committed_pages() {
    for dir in packs() {
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let state = scratch(&format!("state-{name}"));
        let out = scratch(&format!("pages-{name}"));
        assert_eq!(trust(&state, &dir).status.code(), Some(0), "{name}");
        let run = render(&state, &dir, &out, &[]);
        assert_eq!(
            run.status.code(),
            Some(0),
            "{name}: {}",
            String::from_utf8_lossy(&run.stderr)
        );
        let expected = repo().join(format!("fixtures/expected/{name}/pages"));
        let bless = std::env::var("SPECAPP_BLESS").as_deref() == Ok("1");
        if bless {
            let _ = std::fs::remove_dir_all(&expected);
        }
        let got = files(&out);
        for rel in &got {
            let text = std::fs::read_to_string(out.join(rel)).unwrap();
            assert!(
                !text.contains(&*repo().to_string_lossy()),
                "{name} {rel}: a path of this machine"
            );
            assert!(
                !text.contains("<link") && !text.contains("src=\""),
                "{name} {rel}: self-contained"
            );
            if bless {
                let path = expected.join(rel);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(&path, &text).unwrap();
            } else {
                let want = std::fs::read_to_string(expected.join(rel))
                    .unwrap_or_else(|e| panic!("{name} {rel}: {e} (SPECAPP_BLESS=1 writes it)"));
                assert!(
                    want == text,
                    "{name} {rel} differs from the committed page (SPECAPP_BLESS=1 rewrites it)"
                );
            }
        }
        assert_eq!(got, files(&expected), "{name}: the same files");
    }
}

#[test]
fn every_page_carries_its_data_with_each_finding_drawn_where_its_subject_is() {
    let mut drawn = 0;
    let mut link_findings = 0;
    for dir in packs() {
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let state = scratch(&format!("data-state-{name}"));
        let out = scratch(&format!("data-pages-{name}"));
        trust(&state, &dir);
        let run = render(&state, &dir, &out, &[]);
        let stdout = String::from_utf8_lossy(&run.stdout).into_owned();
        let pages: Vec<String> = files(&out)
            .into_iter()
            .filter(|f| f != "index.html")
            .collect();
        assert!(
            stdout.contains(&format!("render: {} page(s)", pages.len())),
            "{stdout}"
        );
        for rel in pages {
            let data = page_data(&std::fs::read_to_string(out.join(&rel)).unwrap());
            assert_eq!(
                schema("specapp-document-1.schema.json", &data),
                Vec::<String>::new(),
                "{rel}"
            );
            assert_eq!(
                schema("specapp-graph-1.schema.json", &data["graph"]),
                Vec::<String>::new()
            );
            assert_eq!(
                schema("specapp-findings-1.schema.json", &data["check"]["result"]),
                Vec::<String>::new()
            );
            for scene in data["scenes"].as_array().unwrap() {
                assert_eq!(
                    schema("specapp-scene-1.schema.json", scene),
                    Vec::<String>::new(),
                    "{rel}"
                );
            }
            assert_eq!(data["check"]["state"], "ran", "{rel}");
            for f in data["findings"].as_array().unwrap() {
                let subject = f["subject"].as_str().unwrap();
                let views = f["drawn_in"].as_array().unwrap();
                if subject == "document" {
                    assert!(views.is_empty());
                } else {
                    assert!(!views.is_empty(), "{rel}: {subject} is drawn");
                    drawn += 1;
                }
                if subject.starts_with("link:") {
                    link_findings += 1;
                }
            }
        }
    }
    assert!(drawn >= 4 && link_findings >= 2, "{drawn} {link_findings}");
}

#[test]
fn an_untrusted_pack_is_drawn_without_running_its_tool() {
    let dir = &packs()[0];
    let state = scratch("untrusted-state");
    let out = scratch("untrusted-pages");
    let run = render(&state, dir, &out, &[]);
    assert_eq!(run.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout
            .lines()
            .filter(|l| !l.starts_with("render:"))
            .all(|l| l.contains("check not_trusted")),
        "{stdout}"
    );
    for rel in files(&out).iter().filter(|f| *f != "index.html") {
        let data = page_data(&std::fs::read_to_string(out.join(rel)).unwrap());
        assert_eq!(data["check"]["state"], "not_trusted");
        assert_eq!(data["findings"], serde_json::json!([]));
    }
}

#[test]
fn a_check_over_the_deadline_is_killed_and_the_page_says_so() {
    let fixture = &packs()[0];
    let dir = scratch("slow-pack");
    copy(fixture, &dir);
    let text = std::fs::read_to_string(dir.join("specapp.yaml")).unwrap();
    let start = text.find("tool:").unwrap();
    let end = start + text[start..].find("\n\n").unwrap_or(text.len() - start);
    let text = format!(
        "{}tool:\n  run: [sh, slow.sh]\n  verbs: [check]{}",
        &text[..start],
        &text[end..]
    );
    std::fs::write(dir.join("specapp.yaml"), text).unwrap();
    std::fs::write(dir.join("slow.sh"), "#!/bin/sh\nsleep 30\n").unwrap();
    let state = scratch("slow-state");
    let out = scratch("slow-pages");
    assert_eq!(trust(&state, &dir).status.code(), Some(0));
    let begin = std::time::Instant::now();
    let run = render(&state, &dir, &out, &["--deadline", "0.2"]);
    assert_eq!(
        run.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(begin.elapsed().as_secs() < 20, "{:?}", begin.elapsed());
    let pages: Vec<String> = files(&out)
        .into_iter()
        .filter(|f| f != "index.html")
        .collect();
    assert!(!pages.is_empty());
    for rel in pages {
        let html = std::fs::read_to_string(out.join(&rel)).unwrap();
        assert_eq!(page_data(&html)["check"]["state"], "timed_out", "{rel}");
        assert!(html.contains("ran over its deadline"), "{rel}");
    }
}

#[test]
fn trust_refuses_a_manifest_without_a_tool_and_render_refuses_bad_usage() {
    let fixture = &packs()[0];
    let dir = scratch("toolless-pack");
    copy(fixture, &dir);
    let text = std::fs::read_to_string(dir.join("specapp.yaml")).unwrap();
    let start = text.find("tool:").unwrap();
    let end = start + text[start..].find("\n\n").unwrap_or(text.len() - start);
    std::fs::write(
        dir.join("specapp.yaml"),
        format!("{}{}", &text[..start], &text[end..]),
    )
    .unwrap();
    let state = scratch("toolless-state");
    assert_eq!(trust(&state, &dir).status.code(), Some(1));
    let out = scratch("toolless-pages");
    assert_eq!(
        render(&state, &dir, &out, &["--deadline", "0"])
            .status
            .code(),
        Some(2)
    );
    let run = render(&state, &dir, &out, &[]);
    assert_eq!(run.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&run.stdout).contains("check no_tool"));
}

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap().flatten() {
        let path = entry.path();
        let target = to.join(entry.file_name());
        if path.is_dir() {
            copy(&path, &target);
        } else {
            std::fs::copy(&path, &target).unwrap();
        }
    }
}
