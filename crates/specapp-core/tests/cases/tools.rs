//! The tools of the fixture packs answer the tool protocol: `describe` in
//! `specapp-tool/1`, `check <document>` in `specapp-findings/1`, exit 2 with `{error}` on a
//! verb they do not know. Each tool is a POSIX sh script started with the pack as its working
//! directory.

use super::common::{load_pack, packs, schema_errors};
use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};

fn run(dir: &Path, argv: &[String], args: &[&str]) -> Output {
    Command::new(&argv[0])
        .args(&argv[1..])
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the fixture tool starts")
}

fn json(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("the tool prints JSON")
}

#[test]
fn every_fixture_tool_answers_describe_and_check_in_the_neutral_formats() {
    for (name, dir) in packs() {
        let pack = load_pack(&dir).1.expect("loads");
        let argv = pack
            .manifest
            .tool
            .as_ref()
            .expect("a fixture tool")
            .run
            .clone();

        let out = run(&dir, &argv, &["describe"]);
        assert!(out.status.success(), "{name}");
        assert_eq!(
            schema_errors("specapp-tool-1.schema.json", &json(&out)),
            Vec::<String>::new(),
            "{name}"
        );

        let mut valid = 0;
        let mut refused = 0;
        let mut findings = 0;
        for documents in &pack.documents {
            for document in documents {
                let out = run(&dir, &argv, &["check", &document.path]);
                assert!(out.status.success(), "{name}: {}", document.path);
                let result = json(&out);
                assert_eq!(
                    schema_errors("specapp-findings-1.schema.json", &result),
                    Vec::<String>::new(),
                    "{name}"
                );
                if result["valid"] == Value::Bool(true) {
                    valid += 1;
                } else {
                    refused += 1;
                }
                findings += result["findings"].as_array().unwrap().len();
            }
        }
        assert!(valid > 0, "{name}: a valid document");
        assert!(findings > 0 || refused > 0, "{name}: a finding");

        let out = run(&dir, &argv, &["no-such-verb"]);
        assert_eq!(out.status.code(), Some(2), "{name}");
        assert!(json(&out).get("error").is_some(), "{name}");
    }
}
