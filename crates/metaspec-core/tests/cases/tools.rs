//! The tools of the fixture packs answer in their packs' own formats and know nothing of
//! metaspec; the adapter in each fixture manifest maps every answer to `metaspec-findings/1`.
//! The two tools answer in different shapes. The adapter's options are held on answers written
//! here.

use super::common::{load_pack, packs, schema_errors, scratch};
use metaspec_core::adapter::{self, Exit, Mapped};
use metaspec_core::graph::{self, Graph};
use metaspec_core::manifest::{CheckAdapter, Shape, parse};
use metaspec_core::tool::{self, CheckState};
use serde_json::{Value, json};
use std::path::Path;
use std::process::Command;

/// Runs the pack's tool as the adapter does, and returns its stdout as JSON.
fn raw(dir: &Path, run: &[String], adapter: &CheckAdapter, document: &str) -> (i32, Value) {
    let out = Command::new(if run[0].contains('/') {
        dir.join(&run[0]).to_string_lossy().into_owned()
    } else {
        run[0].clone()
    })
    .args(&run[1..])
    .args(&adapter.args)
    .arg(document)
    .current_dir(dir)
    .output()
    .expect("the fixture tool starts");
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(
        !text.contains("metaspec"),
        "the tool knows nothing of metaspec"
    );
    (
        out.status.code().unwrap(),
        serde_json::from_str(&text).expect("the tool prints JSON"),
    )
}

#[test]
fn every_fixture_tool_answers_in_its_own_shape_and_the_adapter_maps_it() {
    let mut shapes = Vec::new();
    for (name, dir) in packs() {
        let pack = load_pack(&dir).1.expect("loads");
        let spec = pack.manifest.tool.as_ref().expect("a fixture tool");
        let adapter = spec.verbs.check.as_ref().expect("an adapter for check");
        let state = scratch(&format!("tools-{name}"));
        tool::trust(&state, &pack.root, spec).unwrap();
        let (mut valid, mut invalid, mut findings) = (0, 0, 0);
        for (class, documents) in pack.manifest.documents.iter().zip(&pack.documents) {
            if class.shape != Shape::PartsLinks {
                continue;
            }
            for document in documents {
                let (code, answer) = raw(&dir, &spec.run, adapter, &document.path);
                assert!(
                    adapter.exit_codes().contains(&code),
                    "{name}: {}",
                    document.path
                );
                assert!(answer.get("format").is_none() && answer.get("findings").is_none());
                shapes.push((name.clone(), answer.is_array()));
                let g = graph::read(class, &document.path, document.value.as_ref().unwrap());
                let check = tool::check(
                    &pack,
                    &document.path,
                    Some(&g),
                    Some(&state),
                    tool::DEADLINE,
                );
                assert_eq!(check.state, CheckState::Ran, "{name}: {:?}", check.message);
                let result = serde_json::to_value(check.result.as_ref().unwrap()).unwrap();
                assert_eq!(
                    schema_errors("metaspec-findings-1.schema.json", &result),
                    Vec::<String>::new(),
                    "{name}"
                );
                if result["valid"] == Value::Bool(true) {
                    valid += 1;
                } else {
                    invalid += 1;
                }
                findings += check.findings().len();
            }
        }
        assert!(valid > 0, "{name}: a valid document");
        assert!(findings > 0 || invalid > 0, "{name}: a finding");
    }
    let kinds: std::collections::BTreeSet<bool> = shapes.iter().map(|(_, a)| *a).collect();
    assert_eq!(kinds.len(), 2, "one tool answers a list, another an object");
}

#[test]
fn a_document_the_tool_cannot_read_is_refused_with_the_tools_own_text() {
    for (name, dir) in packs() {
        let pack = load_pack(&dir).1.expect("loads");
        let state = scratch(&format!("tools-refused-{name}"));
        tool::trust(&state, &pack.root, pack.manifest.tool.as_ref().unwrap()).unwrap();
        let check = tool::check(&pack, "nowhere.yaml", None, Some(&state), tool::DEADLINE);
        assert_eq!(
            check.state,
            CheckState::Refused,
            "{name}: {:?}",
            check.message
        );
        assert!(!check.message.unwrap_or_default().is_empty());
    }
}

// --- the adapter's options, on answers written here ---

fn adapter_of(yaml: &str) -> CheckAdapter {
    let text = format!(
        "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: {{ path: b }}\ntool:\n  run: [t]\n  verbs:\n    check:\n{}",
        yaml.lines()
            .map(|l| format!("      {l}\n"))
            .collect::<String>()
    );
    parse(&text)
        .unwrap_or_else(|e| panic!("{e:?}"))
        .tool
        .unwrap()
        .verbs
        .check
        .unwrap()
}

fn exit(code: i32, answer: Value) -> Exit {
    Exit {
        code: Some(code),
        stdout: answer.to_string().into_bytes(),
        stderr: String::new(),
    }
}

/// A graph with the part `p` and the link `p->q:c`, named `whole`.
fn small_graph() -> Graph {
    let m = parse(
        "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: { path: b }\ndocuments:\n  - { class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n }\n",
    )
    .unwrap();
    let doc = json!({"n": "whole", "parts": {"p": {"block": "b"}}, "links": [{"from": "p", "to": "q", "carries": "c"}]});
    graph::read(&m.documents[0], "d/x.yaml", &doc)
}

fn findings(mapped: Mapped) -> metaspec_core::tool::Findings {
    match mapped {
        Mapped::Findings(f) => f,
        other => panic!("{other:?}"),
    }
}

const LINKS: &str = "findings:\n  - at: /items\n    rule: \"{/r}\"\n    gate: { at: /g }\n    message: \"{/m} ({/n})\"\n    subject: { id: \"{/s}\", link_when_contains: \"->\", unknown: document }\n";

#[test]
fn templates_tests_and_the_subject_rule_map_each_entry() {
    let a = adapter_of(LINKS);
    let answer = json!({"items": [
        {"r": "one", "g": true, "m": "on a part", "n": 1, "s": "p"},
        {"r": "two", "g": false, "m": "on a link", "n": 2, "s": "p->q:c"},
        {"r": "three", "g": false, "m": "on a name", "n": 3, "s": "elsewhere"},
        {"r": "four", "g": false, "m": "on the whole", "n": 4, "s": "whole"},
        {"r": "five", "g": false, "m": "on no link", "n": 5, "s": "p->z:c"}
    ]});
    let g = small_graph();
    let f = findings(adapter::check(&a, &exit(0, answer.clone()), Some(&g)));
    let got: Vec<(&str, bool, &str, &str)> = f
        .findings
        .iter()
        .map(|f| {
            (
                f.rule.as_str(),
                f.gate,
                f.subject.as_str(),
                f.message.as_str(),
            )
        })
        .collect();
    assert_eq!(
        got,
        vec![
            ("one", true, "part:p", "on a part (1)"),
            ("two", false, "link:p->q:c", "on a link (2)"),
            ("three", false, "document", "elsewhere: on a name (3)"),
            ("four", false, "document", "on the whole (4)"),
            ("five", false, "document", "p->z:c: on no link (5)"),
        ]
    );
    assert!(
        !f.valid,
        "without `valid`, a gated finding makes it invalid"
    );
    assert_eq!(f.counts, None);

    let f = findings(adapter::check(&a, &exit(0, answer), None));
    assert_eq!(f.findings[2].subject, "part:elsewhere", "no graph: kept");
}

#[test]
fn a_kind_field_valid_by_equality_and_counts_are_read() {
    let a = adapter_of(
        "at: /0\nvalid: { at: /verdict, equals: fine }\ncounts: /tally\nfindings:\n  - at: /notes\n    rule: \"{/code}\"\n    gate: { at: /level, equals: hard }\n    message: \"{/text}\"\n    subject: { id: \"{/on}\", kind: \"{/what}\", kinds: { piece: part, wire: link } }\n  - at: /absent\n    rule: x\n    gate: true\n    message: x\n    subject: document\n",
    );
    let answer = json!([{"verdict": "fine", "tally": {"a": 1, "b": 0}, "notes": [
        {"code": "a", "level": "soft", "text": "t", "on": "p", "what": "piece"},
        {"code": "b", "level": "hard", "text": "u", "on": "p->q:c", "what": "wire"},
        {"code": "c", "level": "soft", "text": "v", "on": "p", "what": "other"}
    ]}]);
    let f = findings(adapter::check(&a, &exit(0, answer), None));
    assert!(
        f.valid,
        "valid is the tool's word, even with a gated finding"
    );
    let subjects: Vec<&str> = f.findings.iter().map(|f| f.subject.as_str()).collect();
    assert_eq!(subjects, vec!["part:p", "link:p->q:c", "document"]);
    let gates: Vec<bool> = f.findings.iter().map(|f| f.gate).collect();
    assert_eq!(gates, vec![false, true, false]);
    assert_eq!(
        f.counts.unwrap().into_iter().collect::<Vec<_>>(),
        vec![(String::from("a"), 1), (String::from("b"), 0)]
    );
}

#[test]
fn exit_codes_errors_and_wrong_answers_are_told_apart() {
    let a = adapter_of(&format!("exit: [0, 1]\nerror: /error\n{LINKS}"));
    let ok = json!({"items": []});
    assert!(
        matches!(adapter::check(&a, &exit(1, ok.clone()), None), Mapped::Findings(f) if f.valid)
    );
    assert!(matches!(
        adapter::check(&a, &exit(3, ok.clone()), None),
        Mapped::Failed(why) if why.contains("exit 3")
    ));
    assert_eq!(
        adapter::check(&a, &exit(2, json!({"error": "cannot read it"})), None),
        Mapped::Refused(String::from("cannot read it"))
    );
    let noise = Exit {
        code: Some(0),
        stdout: b"not json".to_vec(),
        stderr: String::new(),
    };
    assert!(matches!(
        adapter::check(&a, &noise, None),
        Mapped::Failed(_)
    ));
    let signal = Exit {
        code: None,
        stdout: Vec::new(),
        stderr: String::from("killed"),
    };
    assert!(
        matches!(adapter::check(&a, &signal, None), Mapped::Failed(why) if why.contains("signal"))
    );
    for wrong in [
        json!({"items": "not a list"}),
        json!({"items": [{"r": "x", "g": "yes", "m": "", "n": 1, "s": "p"}]}),
        json!({"items": [{"g": true, "m": "", "n": 1, "s": "p"}]}),
        json!({"items": [{"r": "", "g": true, "m": "", "n": 1, "s": "p"}]}),
    ] {
        assert!(
            matches!(
                adapter::check(&a, &exit(0, wrong.clone()), None),
                Mapped::Failed(_)
            ),
            "{wrong}"
        );
    }
    let counted = adapter_of("counts: /c\n");
    assert!(matches!(
        adapter::check(&counted, &exit(0, json!({"c": {"a": -1}})), None),
        Mapped::Failed(_)
    ));
    assert!(matches!(
        adapter::check(&counted, &exit(0, json!({})), None),
        Mapped::Findings(f) if f.counts.is_none() && f.valid
    ));
}

#[test]
fn a_template_keeps_text_and_fills_numbers_and_booleans() {
    let v = json!({"a": "x", "n": 2, "b": true, "deep": {"k": "y"}});
    assert_eq!(adapter::template("plain", &v).unwrap(), "plain");
    assert_eq!(
        adapter::template("{/a}-{/n}-{/b}-{/deep/k} {x}", &v).unwrap(),
        "x-2-true-y {x}"
    );
    assert!(adapter::template("{/missing}", &v).is_err());
    assert!(adapter::template("{/a", &v).is_err());
}
