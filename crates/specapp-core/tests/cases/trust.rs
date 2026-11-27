//! Trust and the run of a tool: specapp runs a pack's tool only when the user trusted that
//! `tool` section; a run over its deadline is killed; an answer in an unknown format or shape
//! is refused.

use super::common::{copy_dir, load_pack, packs, schema_errors, scratch};
use specapp_core::tool::{self, Answer, CheckState};
use std::time::{Duration, Instant};

#[test]
fn an_untrusted_pack_runs_nothing_and_a_trusted_one_answers() {
    for (name, dir) in packs() {
        let pack = load_pack(&dir).1.expect("loads");
        let state = scratch(&format!("trust-{name}"));
        let document = &pack.documents[0][0].path;
        let check = tool::check(&pack, document, Some(&state), tool::DEADLINE);
        assert_eq!(check.state, CheckState::NotTrusted, "{name}");
        assert!(check.result.is_none());
        let check = tool::check(&pack, document, None, tool::DEADLINE);
        assert_eq!(
            check.state,
            CheckState::NotTrusted,
            "{name}: no state directory"
        );

        let spec = pack.manifest.tool.as_ref().unwrap();
        tool::trust(&state, &pack.root, spec).expect("trusted");
        let file: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(state.join(tool::TRUST_FILE)).unwrap())
                .unwrap();
        assert_eq!(
            schema_errors("specapp-trust-1.schema.json", &file),
            Vec::<String>::new()
        );
        let check = tool::check(&pack, document, Some(&state), tool::DEADLINE);
        assert_eq!(check.state, CheckState::Ran, "{name}: {:?}", check.message);

        let mut changed = spec.clone();
        changed.run.push(String::from("--more"));
        assert!(!tool::trusted(&state, &pack.root, &changed), "{name}");
        assert_ne!(tool::digest(spec), tool::digest(&changed));
    }
}

#[test]
fn a_run_over_its_deadline_is_killed_with_what_it_started() {
    let dir = scratch("sleeper");
    std::fs::write(dir.join("slow.sh"), "#!/bin/sh\nsleep 20 &\nsleep 20\n").unwrap();
    let start = Instant::now();
    let answer = tool::run(
        &dir,
        &[String::from("sh"), String::from("slow.sh")],
        &["check", "x"],
        Duration::from_millis(300),
    );
    assert_eq!(answer, Answer::TimedOut(Duration::from_millis(300)));
    assert!(
        start.elapsed() < Duration::from_secs(5),
        "{:?}",
        start.elapsed()
    );
}

#[test]
fn exit_2_is_a_refusal_and_other_answers_fail() {
    let dir = scratch("answers");
    let argv = |script: &str| vec![String::from("sh"), String::from(script)];
    std::fs::write(
        dir.join("refuse.sh"),
        "echo '{\"error\": \"no such document\"}'; exit 2\n",
    )
    .unwrap();
    std::fs::write(dir.join("noise.sh"), "echo not json\n").unwrap();
    std::fs::write(dir.join("crash.sh"), "echo '{}'; exit 3\n").unwrap();
    let d = Duration::from_secs(10);
    assert_eq!(
        tool::run(&dir, &argv("refuse.sh"), &[], d),
        Answer::Refused(String::from("no such document"))
    );
    assert!(matches!(
        tool::run(&dir, &argv("noise.sh"), &[], d),
        Answer::Failed(_)
    ));
    assert!(matches!(
        tool::run(&dir, &argv("crash.sh"), &[], d),
        Answer::Failed(_)
    ));
    assert!(matches!(
        tool::run(&dir, &[String::from("no-such-program-anywhere")], &[], d),
        Answer::Failed(_)
    ));
}

#[test]
fn an_answer_in_another_version_or_shape_is_refused() {
    let other = serde_json::json!({"format": "specapp-findings/2", "valid": true, "findings": []});
    let why = tool::findings(&other).unwrap_err();
    assert!(
        why.contains("specapp-findings/2") && why.contains("specapp-findings/1"),
        "{why}"
    );
    let mut bad = serde_json::to_value(tool::Findings {
        format: String::from(tool::FINDINGS_FORMAT),
        valid: true,
        findings: vec![tool::Finding {
            rule: String::from("r"),
            gate: true,
            subject: String::from("part:p"),
            message: String::new(),
        }],
        counts: None,
    })
    .unwrap();
    assert!(tool::findings(&bad).is_ok());
    bad["findings"][0]["subject"] = serde_json::json!("elsewhere");
    assert!(tool::findings(&bad).is_err());
}

#[test]
fn a_tool_that_answers_wrongly_is_shown_as_failed() {
    let (name, fixture) = packs().remove(0);
    let dir = scratch(&format!("wrong-{name}"));
    copy_dir(&fixture, &dir);
    let pack = load_pack(&dir).1.expect("loads");
    let program = &pack.manifest.tool.as_ref().unwrap().run;
    let script = program.iter().find(|a| a.contains('/')).unwrap();
    std::fs::write(
        dir.join(script),
        "#!/bin/sh\necho '{\"format\": \"specapp-findings/1\", \"valid\": 3}'\n",
    )
    .unwrap();
    let state = scratch(&format!("wrong-state-{name}"));
    tool::trust(&state, &pack.root, pack.manifest.tool.as_ref().unwrap()).unwrap();
    let check = tool::check(
        &pack,
        &pack.documents[0][0].path,
        Some(&state),
        tool::DEADLINE,
    );
    assert_eq!(check.state, CheckState::Failed, "{:?}", check.message);
}
