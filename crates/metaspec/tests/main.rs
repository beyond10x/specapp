//! The one integration-test binary of the metaspec command line. Each test runs the built
//! binary on the fixture packs; `pack check` compiles a fixture's blocks with `ess` (a few
//! milliseconds each). A fixture pack's manifest is `fixtures/manifests/<name>.yaml`, given
//! with `--manifest`; the pack directory is given with `--pack`.

use serde_json::Value;
use std::ffi::{OsStr, OsString};
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

/// The manifest of a fixture pack, by the last component of its directory: a copy keeps it.
fn manifest_of(dir: &Path) -> PathBuf {
    let name = dir.file_name().unwrap().to_string_lossy();
    repo().join(format!("fixtures/manifests/{name}.yaml"))
}

/// `--manifest <the fixture's manifest> --pack <dir>`.
fn pack_args(dir: &Path) -> Vec<OsString> {
    vec![
        "--manifest".into(),
        manifest_of(dir).into(),
        "--pack".into(),
        dir.into(),
    ]
}

fn metaspec<S: AsRef<OsStr>>(args: &[S]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_metaspec"))
        .args(args)
        .output()
        .expect("metaspec starts")
}

/// `metaspec <head> --manifest … --pack <dir> <tail>`.
fn on_pack(head: &[&str], dir: &Path, tail: &[&str]) -> Output {
    let mut args: Vec<OsString> = head.iter().map(OsString::from).collect();
    args.extend(pack_args(dir));
    args.extend(tail.iter().map(OsString::from));
    metaspec(&args)
}

fn schema_errors(value: &Value) -> Vec<String> {
    let text =
        std::fs::read_to_string(repo().join("schema/metaspec-pack-check-1.schema.json")).unwrap();
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
        let out = on_pack(&["pack", "check"], &dir, &["--json"]);
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
        assert_eq!(
            report.pointer("/tool/verbs"),
            Some(&serde_json::json!(["check"])),
            "{name}"
        );
    }
}

#[test]
fn pack_check_text_names_the_counts() {
    let dir = &packs()[0];
    let out = on_pack(&["pack", "check"], dir, &[]);
    assert_eq!(out.status.code(), Some(0));
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(text.lines().any(|l| l.starts_with("blocks: ")), "{text}");
    assert!(text.lines().any(|l| l.starts_with("kpis: ")), "{text}");
    assert!(text.lines().any(|l| l.starts_with("documents: ")), "{text}");
    assert!(text.ends_with("loaded: 0 errors, 0 warnings\n"), "{text}");
}

#[test]
fn a_refused_pack_exits_1_with_a_report_that_follows_the_schema() {
    let scratch = scratch("refused");
    let manifest = scratch.join("leaving.yaml");
    std::fs::write(
        &manifest,
        "format: metaspec-pack/1\npack: leaving\ntitle: Leaving\nvisibility: public\nblocks: { path: ../outside }\n",
    )
    .unwrap();
    let dir = &packs()[0];
    let out = metaspec(&[
        OsStr::new("pack"),
        "check".as_ref(),
        "--manifest".as_ref(),
        manifest.as_os_str(),
        "--pack".as_ref(),
        dir.as_os_str(),
        "--json".as_ref(),
    ]);
    assert_eq!(out.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["valid"], Value::Bool(false));
    assert_eq!(schema_errors(&report), Vec::<String>::new());
}

#[test]
fn bad_usage_exits_2() {
    let dir = &packs()[0];
    let cases: Vec<Vec<OsString>> = vec![
        vec!["pack".into(), "check".into()],
        vec!["pack".into(), "check".into(), "--pack".into(), dir.into()],
        vec![
            "pack".into(),
            "check".into(),
            "--manifest".into(),
            manifest_of(dir).into(),
        ],
    ];
    for args in cases {
        assert_eq!(metaspec(&args).status.code(), Some(2), "{args:?}");
    }
    let mut both = pack_args(dir);
    both.insert(0, "a-name".into());
    both.insert(0, "check".into());
    both.insert(0, "pack".into());
    assert_eq!(
        metaspec(&both).status.code(),
        Some(2),
        "a name and a manifest"
    );
}

#[test]
fn a_pack_is_named_by_a_known_name_and_an_unknown_name_is_bad_usage() {
    let out = metaspec(&["pack", "known"]);
    assert_eq!(out.status.code(), Some(0));
    let known: Vec<String> = String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut files: Vec<String> = std::fs::read_dir(repo().join("packs"))
        .unwrap()
        .flatten()
        .filter_map(|e| {
            e.file_name()
                .to_string_lossy()
                .strip_suffix(".yaml")
                .map(String::from)
        })
        .collect();
    files.sort();
    assert_eq!(known, files);

    let dir = &packs()[0];
    let out = metaspec(&[
        OsStr::new("pack"),
        "check".as_ref(),
        "no-such-pack".as_ref(),
        "--pack".as_ref(),
        dir.as_os_str(),
    ]);
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("knows no pack no-such-pack"), "{stderr}");
    assert!(
        known.iter().all(|k| stderr.contains(k.as_str())),
        "{stderr}"
    );

    // A known name reads the manifest under packs/, here against a fixture's directory, which
    // that manifest does not describe: metaspec reports with that manifest and says so.
    let out = metaspec(&[
        OsStr::new("pack"),
        "check".as_ref(),
        known[0].as_ref(),
        "--pack".as_ref(),
        dir.as_os_str(),
        "--json".as_ref(),
    ]);
    let report: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(schema_errors(&report), Vec::<String>::new());
    assert!(
        report["pack"] == known[0].as_str() || report["valid"] == false,
        "{report}"
    );
    assert!(
        !report["errors"].as_array().unwrap().is_empty()
            || !report["warnings"].as_array().unwrap().is_empty(),
        "{report}"
    );
}

#[test]
fn extends_resolves_a_known_pack_or_a_given_manifest() {
    let dirs = packs();
    let (base, dir) = (&dirs[0], &dirs[1]);
    let base_name = base.file_name().unwrap().to_string_lossy().into_owned();
    let scratch = scratch("extends").join(dir.file_name().unwrap());
    std::fs::create_dir_all(&scratch).unwrap();
    let text = std::fs::read_to_string(manifest_of(dir)).unwrap();
    let manifest = scratch.join("extending.yaml");
    std::fs::write(&manifest, format!("{text}extends: {base_name}\n")).unwrap();
    let check = |extends: Option<&Path>| {
        let mut args: Vec<OsString> = vec![
            "pack".into(),
            "check".into(),
            "--manifest".into(),
            manifest.clone().into(),
            "--pack".into(),
            dir.into(),
            "--json".into(),
        ];
        if let Some(path) = extends {
            args.push("--extends".into());
            args.push(path.into());
        }
        let out = metaspec(&args);
        assert_eq!(out.status.code(), Some(0));
        let report: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(report["extends"], Value::from(base_name.as_str()));
        report["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| w["code"].as_str().unwrap().to_string())
            .collect::<Vec<_>>()
    };
    assert_eq!(check(None), vec!["extends_not_found"]);
    assert_eq!(check(Some(&manifest_of(base))), Vec::<String>::new());
    assert_eq!(check(Some(&manifest_of(dir))), vec!["extends_other_pack"]);
}

#[test]
fn pack_words_prints_every_kind() {
    for dir in packs() {
        let out = on_pack(&["pack", "words"], &dir, &[]);
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
        .join("metaspec-cli")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Runs metaspec with `state` as XDG_STATE_HOME.
fn metaspec_with(state: &Path, args: &[OsString]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_metaspec"))
        .env("XDG_STATE_HOME", state)
        .args(args)
        .output()
        .expect("metaspec starts")
}

/// `metaspec render --manifest <manifest> --pack <pack> --out <out> <extra>`.
fn render_with(state: &Path, pack: &Path, manifest: &Path, out: &Path, extra: &[&str]) -> Output {
    let mut args: Vec<OsString> = vec![
        "render".into(),
        "--manifest".into(),
        manifest.into(),
        "--pack".into(),
        pack.into(),
        "--out".into(),
        out.into(),
    ];
    args.extend(extra.iter().map(OsString::from));
    metaspec_with(state, &args)
}

fn render(state: &Path, pack: &Path, out: &Path, extra: &[&str]) -> Output {
    render_with(state, pack, &manifest_of(pack), out, extra)
}

fn trust_with(state: &Path, pack: &Path, manifest: &Path) -> Output {
    metaspec_with(
        state,
        &[
            "trust".into(),
            "--manifest".into(),
            manifest.into(),
            "--pack".into(),
            pack.into(),
        ],
    )
}

fn trust(state: &Path, pack: &Path) -> Output {
    trust_with(state, pack, &manifest_of(pack))
}

/// The JSON a page carries in its data element.
fn page_data(html: &str) -> Value {
    let start = "<script type=\"application/json\" id=\"metaspec-data\">";
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
        let bless = std::env::var("METASPEC_BLESS").as_deref() == Ok("1");
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
                    .unwrap_or_else(|e| panic!("{name} {rel}: {e} (METASPEC_BLESS=1 writes it)"));
                assert!(
                    want == text,
                    "{name} {rel} differs from the committed page (METASPEC_BLESS=1 rewrites it)"
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
                schema("metaspec-document-1.schema.json", &data),
                Vec::<String>::new(),
                "{rel}"
            );
            assert_eq!(
                schema("metaspec-graph-1.schema.json", &data["graph"]),
                Vec::<String>::new()
            );
            assert_eq!(
                schema("metaspec-findings-1.schema.json", &data["check"]["result"]),
                Vec::<String>::new()
            );
            for scene in data["scenes"].as_array().unwrap() {
                assert_eq!(
                    schema("metaspec-scene-1.schema.json", scene),
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

/// A copy of the first fixture pack, under its own name, and a manifest beside it (outside
/// the pack) whose `tool` section is `tool`.
fn with_tool(name: &str, tool: &str) -> (PathBuf, PathBuf) {
    let fixture = &packs()[0];
    let root = scratch(name);
    let dir = root.join(fixture.file_name().unwrap());
    copy(fixture, &dir);
    let text = std::fs::read_to_string(manifest_of(fixture)).unwrap();
    let start = text.find("tool:").unwrap();
    let end = start + text[start..].find("\n\n").unwrap_or(text.len() - start);
    let manifest = root.join("manifest.yaml");
    std::fs::write(
        &manifest,
        format!("{}{tool}{}", &text[..start], &text[end..]),
    )
    .unwrap();
    (dir, manifest)
}

#[test]
fn a_check_over_the_deadline_is_killed_and_the_page_says_so() {
    let (dir, manifest) = with_tool(
        "slow-pack",
        "tool:\n  run: [sh, slow.sh]\n  verbs:\n    check: {}",
    );
    std::fs::write(dir.join("slow.sh"), "#!/bin/sh\nsleep 30\n").unwrap();
    let state = scratch("slow-state");
    let out = scratch("slow-pages");
    assert_eq!(trust_with(&state, &dir, &manifest).status.code(), Some(0));
    let begin = std::time::Instant::now();
    let run = render_with(&state, &dir, &manifest, &out, &["--deadline", "0.2"]);
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
    let (dir, manifest) = with_tool("toolless-pack", "");
    let state = scratch("toolless-state");
    assert_eq!(trust_with(&state, &dir, &manifest).status.code(), Some(1));
    let out = scratch("toolless-pages");
    assert_eq!(
        render_with(&state, &dir, &manifest, &out, &["--deadline", "0"])
            .status
            .code(),
        Some(2)
    );
    let run = render_with(&state, &dir, &manifest, &out, &[]);
    assert_eq!(run.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&run.stdout).contains("check no_tool"));
}

#[test]
fn a_tool_without_the_verb_check_is_drawn_without_a_check() {
    let (dir, manifest) = with_tool("verbless-pack", "tool:\n  run: [sh, absent.sh]");
    let state = scratch("verbless-state");
    assert_eq!(trust_with(&state, &dir, &manifest).status.code(), Some(0));
    let out = scratch("verbless-pages");
    let run = render_with(&state, &dir, &manifest, &out, &[]);
    assert_eq!(run.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&run.stdout).contains("check no_check"));
}

// --- docs build ---

/// `<manifest of the fixture>=<dir>`, a pack of a site of several packs.
fn site_pack(dir: &Path) -> OsString {
    let mut spec = OsString::from(manifest_of(dir));
    spec.push("=");
    spec.push(dir);
    spec
}

/// `metaspec docs build --pack <manifest>=<dir> ... --out <out> <extra>`: one site over the packs.
fn docs(state: &Path, packs: &[PathBuf], out: &Path, extra: &[&str]) -> Output {
    let mut args: Vec<OsString> = vec!["docs".into(), "build".into()];
    for dir in packs {
        args.push("--pack".into());
        args.push(site_pack(dir));
    }
    args.push("--out".into());
    args.push(out.into());
    args.extend(extra.iter().map(OsString::from));
    metaspec_with(state, &args)
}

/// Trusts every fixture pack in a state directory of its own.
fn trusted_state(name: &str) -> PathBuf {
    let state = scratch(&format!("docs-state-{name}"));
    for dir in packs() {
        assert_eq!(trust(&state, &dir).status.code(), Some(0), "{name}");
    }
    state
}

/// Builds the site of both trusted fixture packs into a scratch directory.
fn site_of(name: &str) -> PathBuf {
    let state = trusted_state(name);
    let out = scratch(&format!("docs-{name}"));
    let run = docs(&state, &packs(), &out, &[]);
    assert_eq!(
        run.status.code(),
        Some(0),
        "{name}: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    out
}

fn committed_site() -> PathBuf {
    repo().join("fixtures/expected/site")
}

#[test]
fn docs_build_of_both_trusted_fixture_packs_writes_the_committed_site() {
    let out = site_of("both");
    let expected = committed_site();
    let bless = std::env::var("METASPEC_BLESS").as_deref() == Ok("1");
    if bless {
        let _ = std::fs::remove_dir_all(&expected);
    }
    let got = files(&out);
    for rel in &got {
        let text = std::fs::read_to_string(out.join(rel)).unwrap();
        assert!(
            !text.contains(&*repo().to_string_lossy()),
            "{rel}: a path of this machine"
        );
        if bless {
            let path = expected.join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, &text).unwrap();
        } else {
            let want = std::fs::read_to_string(expected.join(rel))
                .unwrap_or_else(|e| panic!("{rel}: {e} (METASPEC_BLESS=1 writes it)"));
            assert!(
                want == text,
                "{rel} differs from the committed site (METASPEC_BLESS=1 rewrites it)"
            );
        }
    }
    assert_eq!(got, files(&expected), "the same files");
}

#[test]
fn the_one_pack_form_writes_that_pack_as_the_site_of_two_packs_holds_it() {
    let dir = &packs()[0];
    let name = dir.file_name().unwrap().to_string_lossy().into_owned();
    let state = trusted_state("one");
    let out = scratch("docs-one");
    let mut args: Vec<OsString> = vec!["docs".into(), "build".into()];
    args.extend(pack_args(dir));
    args.extend(["--out".into(), out.clone().into_os_string()]);
    let run = metaspec_with(&state, &args);
    assert_eq!(
        run.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let mine = files(&out.join(&name));
    assert_eq!(mine, files(&committed_site().join(&name)), "{name}");
    for rel in &mine {
        assert_eq!(
            std::fs::read(out.join(&name).join(rel)).unwrap(),
            std::fs::read(committed_site().join(&name).join(rel)).unwrap(),
            "{name} {rel}"
        );
    }
    let index: Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("index.json")).unwrap()).unwrap();
    assert_eq!(index["packs"].as_array().unwrap().len(), 1);
    assert_eq!(index["packs"][0]["pack"], name.as_str());
}

#[test]
fn every_page_the_index_and_each_pack_site_follow_their_schemas_and_the_counts_of_the_pack() {
    let root = committed_site();
    let index: Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("index.json")).unwrap()).unwrap();
    assert_eq!(
        schema("metaspec-index-1.schema.json", &index),
        Vec::<String>::new(),
        "index.json"
    );
    assert_eq!(index["title"], "metaspec");
    let landing = std::fs::read_to_string(root.join("_index/index.mdx")).unwrap();
    let entries = index["packs"].as_array().unwrap();
    assert_eq!(entries.len(), packs().len(), "one entry per fixture pack");
    for dir in packs() {
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let pack_root = root.join(&name);
        let site: Value =
            serde_json::from_str(&std::fs::read_to_string(pack_root.join("site.json")).unwrap())
                .unwrap();
        assert_eq!(
            schema("metaspec-site-1.schema.json", &site),
            Vec::<String>::new(),
            "{name}"
        );
        let entry = entries
            .iter()
            .find(|e| e["pack"] == name.as_str())
            .unwrap_or_else(|| panic!("{name} in index.json"));
        assert_eq!(entry["counts"], site["counts"], "{name}: counts");
        assert_eq!(entry["route"], format!("/{name}"), "{name}");
        assert!(
            landing.contains(&format!("]({})", entry["route"].as_str().unwrap())),
            "{name}: / links the pack"
        );
        for page in site["pages"].as_array().unwrap() {
            let slug = page["slug"].as_str().unwrap();
            assert!(
                slug == format!("/{name}") || slug.starts_with(&format!("/{name}/")),
                "{name}: {slug} is below /{name}"
            );
        }
        let mut kinds: std::collections::BTreeMap<String, usize> = Default::default();
        let mut pages = 0;
        for rel in files(&pack_root.join("pages")) {
            let page: Value = serde_json::from_str(
                &std::fs::read_to_string(pack_root.join("pages").join(&rel)).unwrap(),
            )
            .unwrap();
            assert_eq!(
                schema("metaspec-page-1.schema.json", &page),
                Vec::<String>::new(),
                "{name} {rel}"
            );
            let id = rel.trim_end_matches(".json");
            assert_eq!(page["id"], id, "{name} {rel}");
            assert!(
                pack_root.join("docs").join(format!("{id}.mdx")).is_file(),
                "{name} {id}.mdx"
            );
            *kinds
                .entry(page["kind"].as_str().unwrap().to_string())
                .or_default() += 1;
            pages += 1;
        }
        let report = on_pack(&["pack", "check"], &dir, &["--json"]);
        let report: Value = serde_json::from_slice(&report.stdout).unwrap();
        let counts = &report["counts"];
        let n = |k: &str| kinds.get(k).copied().unwrap_or(0) as u64;
        assert_eq!(
            n("block"),
            counts["blocks"].as_u64().unwrap(),
            "{name}: block pages"
        );
        assert_eq!(
            n("document") + n("record"),
            counts["documents"].as_u64().unwrap(),
            "{name}: document pages"
        );
        assert_eq!(
            n("class"),
            counts["document_classes"].as_u64().unwrap(),
            "{name}"
        );
        assert_eq!(site["counts"]["pages"].as_u64().unwrap(), pages, "{name}");
        let checked = &entry["check"];
        let sum = ["valid", "refused", "not_checked"]
            .iter()
            .map(|k| checked[k].as_u64().unwrap())
            .sum::<u64>();
        assert_eq!(
            sum,
            n("document"),
            "{name}: every drawn document is counted once"
        );
        let catalogue: Value = serde_json::from_str(
            &std::fs::read_to_string(pack_root.join("pages/kpis.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            catalogue["data"]["catalogue"]["kpis"]
                .as_array()
                .unwrap()
                .len() as u64,
            counts["kpis"].as_u64().unwrap(),
            "{name}: KPI rows"
        );
    }
}

#[test]
fn an_untrusted_site_says_its_documents_are_not_checked() {
    let state = scratch("docs-state-untrusted");
    let out = scratch("docs-untrusted");
    let run = docs(&state, &packs(), &out, &[]);
    assert_eq!(run.status.code(), Some(0));
    let index: Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("index.json")).unwrap()).unwrap();
    for entry in index["packs"].as_array().unwrap() {
        assert_eq!(entry["check"]["status"], "not checked", "{}", entry["pack"]);
        assert_eq!(entry["check"]["valid"], 0);
        assert_eq!(entry["check"]["refused"], 0);
    }
}

#[test]
fn docs_build_refuses_bad_pack_arguments_and_two_packs_of_one_name_with_2() {
    let state = scratch("docs-state-usage");
    let out = scratch("docs-usage");
    let dir = &packs()[0];
    let plain: Vec<OsString> = vec![
        "docs".into(),
        "build".into(),
        "--pack".into(),
        dir.into(),
        "--out".into(),
        out.clone().into(),
    ];
    let run = metaspec_with(&state, &plain);
    assert_eq!(run.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&run.stderr).contains("NAME=DIR"));
    let mut two: Vec<OsString> = vec!["docs".into(), "build".into()];
    two.extend(pack_args(dir));
    two.extend([
        "--pack".into(),
        dir.into(),
        "--out".into(),
        out.clone().into(),
    ]);
    assert_eq!(metaspec_with(&state, &two).status.code(), Some(2));
    let run = docs(&state, &[dir.clone(), dir.clone()], &out, &[]);
    assert_eq!(run.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&run.stderr).contains("two packs are named"));
    assert!(files(&out).is_empty(), "nothing written");
}

#[test]
fn docs_build_check_names_changed_missing_and_foreign_files_and_exits_1() {
    let name = packs()[0]
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let out = site_of("check");
    let state = trusted_state("check-again");
    let equal = docs(&state, &packs(), &out, &["--check"]);
    assert_eq!(equal.status.code(), Some(0));
    let again = docs(&state, &packs(), &out, &[]);
    assert!(String::from_utf8_lossy(&again.stdout).contains("0 written, 0 removed"));
    std::fs::write(out.join(&name).join("rules.json.extra"), "x").unwrap();
    std::fs::write(out.join(&name).join("site.json"), "{}").unwrap();
    std::fs::remove_file(out.join("palette.css")).unwrap();
    let run = docs(&state, &packs(), &out, &["--check"]);
    assert_eq!(run.status.code(), Some(1));
    let text = String::from_utf8_lossy(&run.stdout);
    for line in [
        format!("docs: changed: {name}/site.json"),
        String::from("docs: missing: palette.css"),
        format!("docs: not owned: {name}/rules.json.extra"),
    ] {
        assert!(text.contains(&line), "{line} in:\n{text}");
    }
}

#[test]
fn docs_build_refuses_a_directory_it_did_not_write_and_removes_its_stale_files() {
    let dir = packs()[0].clone();
    let state = scratch("docs-state-foreign");
    trust(&state, &dir);
    let foreign = scratch("docs-foreign");
    std::fs::write(foreign.join("notes.md"), "mine").unwrap();
    let run = docs(&state, std::slice::from_ref(&dir), &foreign, &[]);
    assert_eq!(run.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&run.stderr).contains("no index.json of a docs build"));
    assert_eq!(
        std::fs::read_to_string(foreign.join("notes.md")).unwrap(),
        "mine"
    );
    let own = scratch("docs-own");
    assert_eq!(
        docs(&state, std::slice::from_ref(&dir), &own, &[])
            .status
            .code(),
        Some(0)
    );
    std::fs::create_dir_all(own.join("gone")).unwrap();
    std::fs::write(own.join("gone/old.mdx"), "old").unwrap();
    let run = docs(&state, std::slice::from_ref(&dir), &own, &[]);
    assert_eq!(run.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&run.stdout).contains("docs: removed: gone/old.mdx"));
    assert!(!own.join("gone").exists());
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
