//! The specification question (`metaspec quality spec`) with the measures that read only `ess`
//! outputs, the sources of the blocks and the manifest: `validity.refusals`,
//! `validity.warnings`, `ess_completeness.unscenarioed`, `.unanswered`, `.outside`, `honesty`
//! and `adoption_level`.
//!
//! `ess` is the stub `fixtures/ess/ess`: it prints the answer recorded under
//! `fixtures/ess/<pack>/<verb>.json`, so no test runs synthesis. Its answers are cached in a
//! directory of the test under the target directory, never in the user's cache. The goldens are
//! `fixtures/expected/<pack>/quality/<measure>.json`, one measurement each, measured on the
//! test's own copy of each fixture pack with one `UNMAPPED:` marker in a block file;
//! `METASPEC_BLESS=1` rewrites them. No word of a pack is written here: names come from the
//! recorded answers and the loaded pack.

use super::super::common::{copy_dir, load_changed, load_pack, packs, repo, scratch};
use metaspec_core::Pack;
use metaspec_core::quality::catalogue;
use metaspec_core::quality::registry::{self, Input};
use metaspec_core::quality::report::{self, Measurement, QualityReport, Question, Reading};
use metaspec_core::quality::spec::static_measures::{
    self,
    ess_answer::{self, Ess},
};
use serde_json::{Value, json};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// The measures this story builds, in the order of metaspec's catalogue.
const BUILT_HERE: [&str; 7] = [
    "validity.refusals",
    "validity.warnings",
    "ess_completeness.unscenarioed",
    "ess_completeness.unanswered",
    "ess_completeness.outside",
    "honesty",
    "adoption_level",
];

const COMPLETENESS: [(&str, &str); 3] = [
    ("ess_completeness.unscenarioed", "unscenarioed"),
    ("ess_completeness.unanswered", "unanswered"),
    ("ess_completeness.outside", "outside"),
];

/// The line the tests add to a block file of their copy of a pack.
const MARKER_LINE: &str = "# UNMAPPED: an open question this test writes";

/// The stub of ess.
fn stub() -> PathBuf {
    repo().join("fixtures/ess/ess")
}

/// The stub with its recorded answers, and a cache under `dir`.
fn stub_ess(dir: &Path) -> Ess {
    Ess::new(stub(), Some(dir.join("cache")))
}

/// The recorded `ess specify validate --format json` answer of a fixture pack.
fn answer_of(name: &str) -> Value {
    let path = repo().join(format!("fixtures/ess/{name}/validate.json"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).expect("the recorded answer is JSON")
}

/// Every file under `dir`, sorted, relative to `root` with forward slashes.
fn files_under(root: &Path, dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                out.push(
                    path.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }
    out.sort();
    out
}

/// A copy of a fixture pack in the test's scratch directory `<test>/<name>`, keeping its name,
/// with [`MARKER_LINE`] appended to the first YAML file under its blocks (sorted). Returns the
/// copy and the marker's place, `<file>:<line>` relative to the pack.
fn marked_copy(test: &str, name: &str, dir: &Path) -> (PathBuf, String) {
    let copy = scratch(test).join(name);
    copy_dir(dir, &copy);
    let pack = pack_of(&copy);
    let blocks = copy.join(&pack.manifest.blocks.path);
    let file = files_under(&copy, &blocks)
        .into_iter()
        .find(|f| f.ends_with(".yaml") && f.matches('/').count() >= 2)
        .expect("a block file in a layer directory");
    let path = copy.join(&file);
    let mut text = std::fs::read_to_string(&path).unwrap();
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str(MARKER_LINE);
    text.push('\n');
    let line = text.lines().count();
    std::fs::write(&path, text).unwrap();
    (copy, format!("{file}:{line}"))
}

fn pack_of(dir: &Path) -> Pack {
    let (report, pack) = load_pack(dir);
    pack.unwrap_or_else(|| panic!("{} loads: {:?}", dir.display(), report.errors))
}

/// The report of the specification question on a pack with `ess`.
fn spec_report(pack: &Pack, ess: Ess) -> QualityReport {
    let catalogue = catalogue::embedded().expect("metaspec's catalogue loads");
    let input = Input::new(pack, None, Duration::from_secs(5));
    ess_answer::scoped(ess, || static_measures::report(&catalogue, &input))
        .unwrap_or_else(|issues| panic!("the specification question runs: {issues:?}"))
}

fn measurement<'a>(report: &'a QualityReport, id: &str) -> &'a Measurement {
    report
        .measurements
        .iter()
        .find(|m| m.measure == id)
        .unwrap_or_else(|| panic!("the report holds {id}"))
}

fn value(m: &Measurement) -> f64 {
    match &m.reading {
        Reading::Value { value } => value.as_f64().unwrap(),
        Reading::Gap { gap } => panic!("{} is a gap: {gap}", m.measure),
    }
}

fn gap(m: &Measurement) -> String {
    match &m.reading {
        Reading::Gap { gap } => gap.clone(),
        Reading::Value { value } => panic!("{} is the value {value}, not a gap", m.measure),
    }
}

fn behind(m: &Measurement) -> Vec<(String, String)> {
    m.behind
        .iter()
        .map(|b| (b.kind.clone(), b.name.clone()))
        .collect()
}

/// Compares `text` with a committed file byte for byte, or writes it when `METASPEC_BLESS=1`.
fn golden(path: &Path, text: &str) {
    if std::env::var("METASPEC_BLESS").as_deref() == Ok("1") {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
        return;
    }
    let want = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: {e} (METASPEC_BLESS=1 writes it)", path.display()));
    assert!(
        want == text,
        "{} differs from the measurement (METASPEC_BLESS=1 rewrites it):\n{text}",
        path.display()
    );
}

/// A directory of answers of the test's own: a copy of the stub, and `answer` as the validate
/// answer of the pack `name`.
fn own_answers(dir: &Path, name: &str, answer: &Value) -> PathBuf {
    let answers = dir.join("ess");
    std::fs::create_dir_all(answers.join(name)).unwrap();
    std::fs::copy(stub(), answers.join("ess")).unwrap();
    std::fs::write(
        answers.join(name).join("validate.json"),
        serde_json::to_string_pretty(answer).unwrap(),
    )
    .unwrap();
    answers.join("ess")
}

// --- the layout ---

#[test]
fn static_builds_its_seven_measures_in_its_own_file() {
    for id in BUILT_HERE {
        let module = registry::entry(id).unwrap_or_else(|| panic!("{id} is registered"));
        assert!((module.built)(id), "{id} is built");
        assert_eq!(module.file, "spec/static_measures.rs", "{id}");
    }
}

#[test]
fn static_offers_each_verb_a_manifest_can_declare_with_the_level_that_needs_it() {
    let offered: Vec<&str> = static_measures::OFFERED_VERBS
        .iter()
        .map(|(verb, _)| *verb)
        .collect();
    assert_eq!(offered, static_measures::declarable_verbs());
    for (verb, level) in static_measures::OFFERED_VERBS {
        assert!(
            ["L0", "L1", "L2", "L3", "L4", "L5"].contains(level),
            "{verb}: {level}"
        );
    }
}

// --- every measure on both fixture packs ---

#[test]
fn static_the_report_follows_the_schema_and_each_measure_equals_its_golden_on_both_fixture_packs() {
    for (name, dir) in packs() {
        let test = format!("spec-golden-{name}");
        let (copy, _) = marked_copy(&test, &name, &dir);
        let pack = pack_of(&copy);
        let report = spec_report(&pack, stub_ess(&scratch(&format!("{test}-ess"))));
        assert_eq!(report.question, Question::Specification);
        assert_eq!(report.pack, pack.manifest.pack);
        let value = serde_json::to_value(&report).unwrap();
        assert_eq!(report::validate(&value), Ok(()), "{name}");
        for id in BUILT_HERE {
            let m = measurement(&report, id);
            assert!(
                matches!(m.reading, Reading::Value { .. }),
                "{name}: {id} is a value on a fixture pack: {m:?}"
            );
            let text = serde_json::to_string_pretty(m).unwrap() + "\n";
            golden(
                &repo().join(format!("fixtures/expected/{name}/quality/{id}.json")),
                &text,
            );
        }
    }
}

#[test]
fn validity_counts_the_one_refusal_and_the_one_warning_of_the_recorded_answer_with_code_and_subject()
 {
    for (name, dir) in packs() {
        let answer = answer_of(&name);
        let refusals = answer["scenario_refusals"].as_array().unwrap();
        let warnings = answer["warnings"].as_array().unwrap();
        assert_eq!(
            (refusals.len(), warnings.len()),
            (1, 1),
            "{name}: the recorded answer has one refusal and one warning"
        );
        let pack = pack_of(&dir);
        let report = spec_report(&pack, stub_ess(&scratch(&format!("spec-validity-{name}"))));
        let m = measurement(&report, "validity.refusals");
        assert_eq!(value(m), 1.0, "{name}");
        let refusal = &refusals[0];
        assert_eq!(
            behind(m),
            [(
                String::from("finding"),
                format!(
                    "{} {}",
                    refusal["code"].as_str().unwrap(),
                    refusal["scenario"].as_str().unwrap()
                )
            )],
            "{name}"
        );
        let m = measurement(&report, "validity.warnings");
        assert_eq!(value(m), 1.0, "{name}");
        let warning = &warnings[0];
        assert_eq!(
            behind(m),
            [(
                String::from("finding"),
                format!(
                    "{} {}",
                    warning["code"].as_str().unwrap(),
                    warning["span"]["path"].as_str().unwrap()
                )
            )],
            "{name}"
        );
    }
}

#[test]
fn completeness_counts_equal_the_counts_of_the_answer_and_name_the_constructs() {
    for (name, dir) in packs() {
        let answer = answer_of(&name);
        let pack = pack_of(&dir);
        let report = spec_report(
            &pack,
            stub_ess(&scratch(&format!("spec-completeness-{name}"))),
        );
        for (id, list) in COMPLETENESS {
            let m = measurement(&report, id);
            let count = answer["completeness"]["counts"][list].as_u64().unwrap();
            assert_eq!(value(m), count as f64, "{name}: {id}");
            let names: Vec<(String, String)> = answer["completeness"][list]
                .as_array()
                .unwrap()
                .iter()
                .map(|entry| match list {
                    "unscenarioed" => (
                        String::from("finding"),
                        format!(
                            "{} {}",
                            entry["code"].as_str().unwrap(),
                            entry["subject"].as_str().unwrap()
                        ),
                    ),
                    "outside" => (
                        String::from("scenario"),
                        entry["scenario"].as_str().unwrap().to_string(),
                    ),
                    _ => (String::from("finding"), entry.as_str().unwrap().to_string()),
                })
                .collect();
            assert_eq!(behind(m), names, "{name}: {id}");
        }
        let total: u64 = COMPLETENESS
            .iter()
            .map(|(_, l)| answer["completeness"]["counts"][l].as_u64().unwrap())
            .sum();
        assert!(total > 0, "{name}: the recorded answer owes something");
    }
}

#[test]
fn completeness_absent_from_a_valid_answer_is_nothing_owed() {
    let (name, dir) = &packs()[0];
    let work = scratch("spec-nothing-owed");
    let mut answer = answer_of(name);
    let object = answer.as_object_mut().unwrap();
    object.remove("completeness");
    object.remove("warnings");
    object.remove("scenario_refusals");
    object.insert(String::from("valid"), json!(true));
    let program = own_answers(&work, name, &answer);
    let report = spec_report(&pack_of(dir), Ess::new(program, None));
    for id in BUILT_HERE.iter().take(5) {
        let m = measurement(&report, id);
        assert_eq!(value(m), 0.0, "{id}");
        assert!(m.behind.is_empty(), "{id}");
    }
}

// --- refused blocks and an ess that does not answer ---

#[test]
fn refused_blocks_count_their_problems_and_make_the_completeness_rows_gaps_naming_the_refusal() {
    let (name, dir) = &packs()[0];
    let work = scratch("spec-refused");
    let refused = json!({
        "compiled": false,
        "files_read": 3,
        "problems": [
            "[undeclared_reference] entity x.y.Z.fields.a: `x.y.NoSuchId` is not a declared type",
            "[type_mismatch] commands.x.y.Do.outcomes.done.sets.a: no conversion is declared"
        ],
        "diagnostics": []
    });
    let program = own_answers(&work, name, &refused);
    let report = spec_report(&pack_of(dir), Ess::new(program, Some(work.join("cache"))));
    let m = measurement(&report, "validity.refusals");
    assert_eq!(value(m), 2.0);
    assert_eq!(
        behind(m),
        [
            (
                String::from("finding"),
                String::from("undeclared_reference entity x.y.Z.fields.a")
            ),
            (
                String::from("finding"),
                String::from("type_mismatch commands.x.y.Do.outcomes.done.sets.a")
            ),
        ]
    );
    assert_eq!(value(measurement(&report, "validity.warnings")), 0.0);
    for (id, _) in COMPLETENESS {
        let reading = gap(measurement(&report, id));
        assert!(
            reading.contains("do not validate") && reading.contains("`x.y.NoSuchId`"),
            "{id}: {reading}"
        );
        let listed = report
            .gaps
            .iter()
            .find(|g| g.feature == id)
            .unwrap_or_else(|| panic!("{id} is listed among the gaps"));
        assert_eq!(listed.kind, "measure");
        assert_eq!(listed.needs.as_deref(), Some("L0"));
    }
    let json = serde_json::to_value(&report).unwrap();
    assert_eq!(report::validate(&json), Ok(()));

    // A refusal that lists no problem counts its diagnostics, never 0, and the gap names the
    // text of the first. The diagnostic is the recorded warning, made an error.
    let mut diagnostic = answer_of(name)["warnings"][0].clone();
    diagnostic["severity"] = json!("error");
    let diagnosed = json!({
        "compiled": false,
        "files_read": 3,
        "problems": [],
        "diagnostics": [diagnostic.clone()]
    });
    let program = own_answers(&work.join("diagnosed"), name, &diagnosed);
    let report = spec_report(&pack_of(dir), Ess::new(program, None));
    let m = measurement(&report, "validity.refusals");
    assert_eq!(value(m), 1.0);
    let code = diagnostic["code"].as_str().unwrap();
    let path = diagnostic["span"]["path"].as_str().unwrap();
    assert_eq!(
        behind(m),
        [(String::from("finding"), format!("{code} {path}"))]
    );
    let text = diagnostic["message"].as_str().unwrap();
    let reading = gap(measurement(&report, "ess_completeness.unscenarioed"));
    assert!(reading.contains(text), "{reading}");
}

#[test]
fn completeness_of_a_model_synthesis_does_not_admit_is_a_gap_never_a_zero() {
    let (name, dir) = &packs()[0];
    let work = scratch("spec-unsynthesizable");
    let mut answer = answer_of(name);
    answer["completeness"] = json!({
        "unscenarioed": [],
        "outside": [],
        "unanswered": [],
        "unsynthesizable": [{"reason": "recursive_type", "path": "types.x.y.T", "detail": "it holds itself"}],
        "counts": {"unscenarioed": 0, "outside": 0, "unanswered": 0, "unsynthesizable": 1}
    });
    let program = own_answers(&work, name, &answer);
    let report = spec_report(&pack_of(dir), Ess::new(program, None));
    for (id, _) in COMPLETENESS {
        let reading = gap(measurement(&report, id));
        assert!(
            reading.contains("recursive_type") && reading.contains("types.x.y.T"),
            "{id}: {reading}"
        );
    }
    assert_eq!(value(measurement(&report, "validity.warnings")), 1.0);
}

#[test]
fn an_ess_that_cannot_be_started_or_has_no_answer_is_a_gap_and_the_other_measures_still_measure() {
    let (_, dir) = &packs()[0];
    let pack = pack_of(dir);
    let work = scratch("spec-no-ess");
    let missing = work.join("no-such-ess");
    let report = spec_report(&pack, Ess::new(missing, Some(work.join("cache"))));
    for id in BUILT_HERE.iter().take(5) {
        let reading = gap(measurement(&report, id));
        assert!(reading.contains("cannot be started"), "{id}: {reading}");
    }
    assert_eq!(value(measurement(&report, "honesty")), 0.0);
    value(measurement(&report, "adoption_level"));
    assert!(!work.join("cache").exists(), "a failed run is not cached");

    // The stub, with no answer recorded for this pack: its line on stderr is the gap.
    let program = own_answers(&work, "a-pack-of-no-test", &json!({}));
    let report = spec_report(&pack, Ess::new(program, None));
    let reading = gap(measurement(&report, "validity.refusals"));
    assert!(reading.contains("no answer recorded"), "{reading}");
}

// --- honesty ---

#[test]
fn honesty_counts_each_line_with_an_unmapped_marker_under_blocks_path_and_names_file_and_line() {
    let (name, dir) = &packs()[0];
    let (copy, first) = marked_copy("spec-honesty", name, dir);
    let pack = pack_of(&copy);
    let blocks = copy.join(&pack.manifest.blocks.path);
    // Two markers on one line are one line, as grep counts; a word without the colon is no
    // marker; a hidden file and a file that is no YAML are read; a file outside the blocks is
    // not.
    std::fs::create_dir_all(blocks.join("notes/deep")).unwrap();
    std::fs::write(
        blocks.join("notes/deep/open.md"),
        "first\nUNMAPPED: one UNMAPPED: two\nUNMAPPED without a colon\n",
    )
    .unwrap();
    std::fs::write(blocks.join(".hidden"), "UNMAPPED: hidden\n").unwrap();
    std::fs::write(copy.join("outside.yaml"), "# UNMAPPED: not a block\n").unwrap();
    let report = spec_report(&pack, Ess::new(stub(), None));
    let m = measurement(&report, "honesty");
    let prefix = &pack.manifest.blocks.path;
    let mut want = vec![
        (String::from("marker"), first.clone()),
        (
            String::from("marker"),
            format!("{prefix}/notes/deep/open.md:2"),
        ),
        (String::from("marker"), format!("{prefix}/.hidden:1")),
    ];
    want.sort_by(|a, b| a.1.cmp(&b.1));
    assert_eq!(behind(m), want);
    assert_eq!(value(m), 3.0);
}

// --- adoption level ---

#[test]
fn adoption_level_is_the_level_pack_check_derives_and_each_verb_not_declared_is_a_gap_of_kind_verb()
{
    let (name, dir) = &packs()[0];
    let work = scratch("spec-adoption");
    let ess = || Ess::new(stub(), Some(work.join("cache")));
    let verb_gaps = |report: &QualityReport| -> Vec<(String, Option<String>)> {
        report
            .gaps
            .iter()
            .filter(|g| g.kind == "verb")
            .map(|g| (g.feature.clone(), g.needs.clone()))
            .collect()
    };

    // The committed manifest declares check: L3, no verb gap.
    let pack = pack_of(dir);
    assert_eq!(pack.manifest.level(), "L3", "{name}");
    let report = spec_report(&pack, ess());
    let m = measurement(&report, "adoption_level");
    assert_eq!(value(m), 3.0);
    assert!(m.behind.is_empty());
    assert_eq!(verb_gaps(&report), []);

    // Each step down, the value follows Manifest::level, which `pack check` prints as `level`.
    type Change = fn(&mut metaspec_core::Manifest);
    let changes: [(Change, f64); 3] = [
        (|m| m.tool = None, 2.0),
        (
            |m| {
                m.tool = None;
                m.documents.clear();
            },
            1.0,
        ),
        (
            |m| {
                m.tool = None;
                m.documents.clear();
                m.kpis = None;
            },
            0.0,
        ),
    ];
    for (change, level) in changes {
        let (loaded, pack) = load_changed(dir, change);
        let pack = pack.unwrap_or_else(|| panic!("loads: {:?}", loaded.errors));
        let report = spec_report(&pack, ess());
        assert_eq!(value(measurement(&report, "adoption_level")), level);
        assert_eq!(
            loaded.level.map(|l| l.trim_start_matches('L')),
            Some(level.to_string().as_str()),
            "the level of pack check"
        );
        assert_eq!(
            verb_gaps(&report),
            [(String::from("check"), Some(String::from("L3")))],
            "check is the verb a manifest without a tool does not declare"
        );
        let listed = report.gaps.iter().find(|g| g.kind == "verb").unwrap();
        assert!(listed.summary.contains("check"), "{listed:?}");
        let value = serde_json::to_value(&report).unwrap();
        assert_eq!(report::validate(&value), Ok(()));
    }
}

// --- the cache ---

#[test]
fn the_answer_is_cached_by_the_digest_of_the_blocks_in_the_cache_directory_and_read_back() {
    let (name, dir) = &packs()[0];
    let work = scratch("spec-cache");
    let copy = work.join(name);
    copy_dir(dir, &copy);
    let before = files_under(&copy, &copy);
    let cache = work.join("cache");
    let pack = pack_of(&copy);
    let first = spec_report(&pack, Ess::new(stub(), Some(cache.clone())));
    let cached = files_under(&cache, &cache);
    assert_eq!(cached.len(), 1, "one answer cached: {cached:?}");
    assert!(cached[0].ends_with(".json"), "{cached:?}");
    assert_eq!(
        files_under(&copy, &copy),
        before,
        "nothing is written into the pack"
    );

    // Another ess of the same version with another answer: the cached answer is read.
    let mut other = answer_of(name);
    let warning = other["warnings"][0].clone();
    other["warnings"] = json!([warning.clone(), warning.clone(), warning]);
    let program = own_answers(&work.join("other"), name, &other);
    let again = spec_report(&pack, Ess::new(program.clone(), Some(cache.clone())));
    assert_eq!(
        measurement(&again, "validity.warnings"),
        measurement(&first, "validity.warnings")
    );

    // A changed block file is another digest: ess answers again.
    let blocks = copy.join(&pack.manifest.blocks.path);
    let file = files_under(&copy, &blocks)
        .into_iter()
        .find(|f| f.ends_with(".yaml"))
        .unwrap();
    let mut text = std::fs::read_to_string(copy.join(&file)).unwrap();
    text.push_str("# a comment this test adds\n");
    std::fs::write(copy.join(&file), text).unwrap();
    let pack = pack_of(&copy);
    let changed = spec_report(&pack, Ess::new(program, Some(cache.clone())));
    assert_eq!(value(measurement(&changed, "validity.warnings")), 3.0);
    assert_eq!(files_under(&cache, &cache).len(), 2);

    // Without a cache directory nothing is written and the measures still measure.
    let none = spec_report(&pack, Ess::new(stub(), None));
    assert_eq!(value(measurement(&none, "validity.warnings")), 1.0);
}

#[test]
fn the_cache_directory_is_the_users_cache_never_the_checkout() {
    let xdg = OsStr::new("/var/cache-of-a-test");
    let home = OsStr::new("/srv/home-of-a-test");
    assert_eq!(
        ess_answer::cache_dir(Some(xdg), Some(home)),
        Some(PathBuf::from("/var/cache-of-a-test/metaspec/ess"))
    );
    assert_eq!(
        ess_answer::cache_dir(None, Some(home)),
        Some(PathBuf::from("/srv/home-of-a-test/.cache/metaspec/ess"))
    );
    assert_eq!(
        ess_answer::cache_dir(Some(OsStr::new("")), Some(home)),
        Some(PathBuf::from("/srv/home-of-a-test/.cache/metaspec/ess"))
    );
    assert_eq!(ess_answer::cache_dir(None, None), None);
    assert_eq!(ess_answer::PROGRAM_VAR, "METASPEC_ESS");
}

// --- the deadline ---

#[test]
fn an_ess_that_runs_over_its_deadline_is_killed_and_is_a_gap() {
    use std::os::unix::fs::PermissionsExt;
    let (_, dir) = &packs()[0];
    let work = scratch("spec-deadline");
    let program = work.join("ess");
    std::fs::write(
        &program,
        "#!/bin/sh\nif [ \"${1-}\" = --version ]; then echo 'ess slow'; exit 0; fi\nsleep 30\n",
    )
    .unwrap();
    std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).unwrap();
    let started = std::time::Instant::now();
    let ess = Ess::new(program, Some(work.join("cache"))).with_deadline(Duration::from_secs(1));
    let report = spec_report(&pack_of(dir), ess);
    let reading = gap(measurement(&report, "validity.refusals"));
    assert!(reading.contains("ran over 1 s and was killed"), "{reading}");
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "killed at the deadline, not after the sleep: {:?}",
        started.elapsed()
    );
    assert!(!work.join("cache").exists(), "a killed run is not cached");
}

#[test]
fn the_deadline_is_600_s_or_the_seconds_of_metaspec_ess_deadline() {
    assert_eq!(ess_answer::DEADLINE_VAR, "METASPEC_ESS_DEADLINE");
    assert_eq!(ess_answer::DEADLINE, Duration::from_secs(600));
    assert_eq!(ess_answer::deadline_of(None), Duration::from_secs(600));
    assert_eq!(
        ess_answer::deadline_of(Some(OsStr::new("2.5"))),
        Duration::from_millis(2500)
    );
    assert_eq!(ess_answer::DEADLINE_FLOOR, Duration::from_millis(1));
    for below in ["0.0005", "1e-12"] {
        assert_eq!(
            ess_answer::deadline_of(Some(OsStr::new(below))),
            Duration::from_millis(1),
            "{below:?} is raised to the floor"
        );
    }
    for bad in ["", "0", "-3", "soon", "inf", "NaN"] {
        assert_eq!(
            ess_answer::deadline_of(Some(OsStr::new(bad))),
            Duration::from_secs(600),
            "{bad:?}"
        );
    }
    assert_eq!(
        Ess::new("ess", None).deadline,
        Duration::from_secs(600),
        "a new Ess has the default deadline"
    );
}

// --- the stub ---

#[test]
fn the_stub_prints_the_answer_recorded_for_the_pack_above_its_working_directory() {
    for (name, dir) in packs() {
        let out = Command::new(stub())
            .args(["specify", "validate", "--path", ".", "--format", "json"])
            .current_dir(dir.join("blocks"))
            .output()
            .expect("the stub starts");
        assert_eq!(out.status.code(), Some(0), "{name}");
        let printed: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(printed, answer_of(&name), "{name}");
    }
    let dir = &packs()[0].1;
    let version = Command::new(stub())
        .arg("--version")
        .current_dir(dir.join("blocks"))
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&version.stdout), "ess stub\n");
    let unknown = Command::new(stub())
        .args(["verify", "conform", "no-such-verb", "--path", "."])
        .current_dir(dir.join("blocks"))
        .output()
        .unwrap();
    assert_eq!(unknown.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&unknown.stderr).contains("no answer recorded"),
        "{unknown:?}"
    );
}
