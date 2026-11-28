//! The KPI question (`metaspec quality kpis`): the layout every later story fills (the registry
//! of every measure of metaspec's catalogue, its stubs and the check of the registry against the
//! catalogue), the report format `metaspec-quality/1`, and the measures that read only the KPI
//! catalogue and the block list: `coverage.blocks`, `completeness.fields` and `planned`.
//!
//! The goldens are `fixtures/expected/<pack>/quality/<measure>.json`, one measurement each;
//! `METASPEC_BLESS=1` rewrites them. No word of a pack is written here: the catalogues the tests
//! write use invented ids and the blocks of the loaded pack.

use super::super::common::{load_pack, packs, repo, schema_errors};
use metaspec_core::Pack;
use metaspec_core::quality::catalogue::{self, Catalogue};
use metaspec_core::quality::registry::{self, Input, Module};
use metaspec_core::quality::report::{self, NOT_BUILT, Outcome, Question, Reading};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::Path;
use std::time::Duration;

/// The measures this story builds.
const BUILT_HERE: [&str; 3] = ["coverage.blocks", "completeness.fields", "planned"];

fn embedded() -> Catalogue {
    catalogue::embedded().expect("metaspec's catalogue loads")
}

/// The first fixture pack, loaded.
fn first_pack() -> Pack {
    let (_, dir) = &packs()[0];
    pack_of(dir)
}

fn pack_of(dir: &Path) -> Pack {
    let (report, pack) = load_pack(dir);
    pack.unwrap_or_else(|| panic!("{} loads: {:?}", dir.display(), report.errors))
}

fn measure(id: &str, pack: &Pack) -> Outcome {
    let input = Input::new(pack, None, Duration::from_secs(5));
    let module = registry::entry(id).unwrap_or_else(|| panic!("{id} is in the registry"));
    (module.measure)(id, &input)
}

/// The report of the KPI question on a pack.
fn kpi_report(pack: &Pack) -> report::QualityReport {
    let input = Input::new(pack, None, Duration::from_secs(5));
    registry::run(Question::KpiCatalogue, &embedded(), &input)
        .unwrap_or_else(|issues| panic!("the KPI question runs: {issues:?}"))
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

/// The measurement of one measure on every fixture pack against its golden.
fn golden_of(id: &str) {
    for (name, dir) in packs() {
        let report = kpi_report(&pack_of(&dir));
        let measurement = report
            .measurements
            .iter()
            .find(|m| m.measure == id)
            .unwrap_or_else(|| panic!("{name}: the report holds {id}"));
        assert!(
            matches!(measurement.reading, Reading::Value { .. }),
            "{name}: {id} is a value on a fixture pack: {measurement:?}"
        );
        let text = serde_json::to_string_pretty(measurement).unwrap() + "\n";
        golden(
            &repo().join(format!("fixtures/expected/{name}/quality/{id}.json")),
            &text,
        );
    }
}

fn subjects(issues: &[metaspec_core::Issue], code: &str) -> BTreeSet<String> {
    issues
        .iter()
        .filter(|i| i.code == code)
        .map(|i| i.subject.clone())
        .collect()
}

// --- the layout ---

#[test]
fn layout_the_registry_maps_each_of_the_40_ids_of_the_catalogue_to_a_function_or_a_stub() {
    let ids: Vec<String> = embedded().measures.iter().map(|m| m.id.clone()).collect();
    assert_eq!(ids.len(), 40, "metaspec's catalogue has 40 measures");
    for id in &ids {
        assert!(registry::entry(id).is_some(), "{id} has no registry entry");
    }
    let mapped: BTreeSet<&str> = registry::REGISTRY.iter().map(|(id, _)| *id).collect();
    let rows: BTreeSet<&str> = ids.iter().map(String::as_str).collect();
    assert_eq!(
        mapped, rows,
        "the registry maps exactly the catalogue's ids"
    );
    assert_eq!(
        registry::REGISTRY.len(),
        40,
        "each id is mapped once in the registry"
    );
    for id in BUILT_HERE {
        let module = registry::entry(id).unwrap();
        assert!((module.built)(id), "{id} is built");
        assert_eq!(module.file, "kpis/catalogue_measures.rs", "{id}");
    }
}

#[test]
fn layout_each_stub_measure_returns_the_gap_reading_not_built_yet() {
    let pack = first_pack();
    let mut stubs = 0;
    for (id, module) in registry::REGISTRY {
        if (module.built)(id) {
            continue;
        }
        stubs += 1;
        match measure(id, &pack) {
            Outcome::Gap { gap, needs } => {
                assert_eq!(gap, NOT_BUILT, "{id} in {}", module.file);
                assert_eq!(
                    needs, None,
                    "{id}: a measure not built yet is metaspec's gap"
                );
            }
            other => panic!("{id} in {} is a stub and measured {other:?}", module.file),
        }
    }
    assert_eq!(NOT_BUILT, "not built yet");
    assert!(stubs > 0, "the layout leaves stubs for the later stories");
}

#[test]
fn layout_a_catalogue_with_a_measured_row_whose_entry_is_a_stub_is_refused_naming_the_row() {
    // metaspec's own catalogue with every row `measured`: refused, naming exactly the rows
    // whose registry entry is a stub.
    let text = catalogue::TEXT.replace("status: planned", "status: measured");
    let all = catalogue::parse(&text).expect("the changed catalogue loads");
    assert!(all.measures.iter().all(|m| m.status == "measured"));
    let stubs: BTreeSet<String> = registry::REGISTRY
        .iter()
        .filter(|(id, module)| !(module.built)(id))
        .map(|(id, _)| format!("measure:{id}"))
        .collect();
    let issues = registry::check(&all).expect_err("a measured stub is refused");
    assert_eq!(subjects(&issues, "measure_not_built"), stubs);
    for issue in &issues {
        assert!(
            issue
                .message
                .contains(issue.subject.trim_start_matches("measure:")),
            "{issue:?} names the row"
        );
    }

    // A catalogue the test writes, with a registry the test writes.
    let one = catalogue::parse(
        "format: metaspec-kpis/1\nkpis:\n  - id: m\n    block: quality\n    measure: kpi_catalogue\n    unit: count\n    direction: lower_is_better\n    status: measured\n",
    )
    .expect("the test's catalogue loads");
    let issues = registry::check_with(&one, &[("m", &STUB)]).expect_err("refused");
    assert_eq!(
        subjects(&issues, "measure_not_built"),
        BTreeSet::from([String::from("measure:m")])
    );
    assert!(registry::check_with(&one, &[("m", &BUILT)]).is_ok());

    // The same row with no registry entry at all is refused too.
    let issues = registry::check_with(&one, &[]).expect_err("refused");
    assert_eq!(
        subjects(&issues, "measure_not_registered"),
        BTreeSet::from([String::from("measure:m")])
    );

    // metaspec's catalogue as committed (every row planned) passes.
    assert_eq!(registry::check(&embedded()), Ok(()));
}

#[test]
fn layout_a_registry_measure_with_no_catalogue_row_is_refused() {
    let one = catalogue::parse(
        "format: metaspec-kpis/1\nkpis:\n  - id: m\n    block: quality\n    measure: kpi_catalogue\n    unit: count\n    direction: lower_is_better\n    status: planned\n",
    )
    .expect("the test's catalogue loads");
    let issues = registry::check_with(&one, &[("m", &BUILT), ("n", &STUB)]).expect_err("refused");
    assert_eq!(
        subjects(&issues, "measure_without_row"),
        BTreeSet::from([String::from("measure:n")])
    );
    assert_eq!(issues.len(), 1, "{issues:?}");
}

fn stub_measure(_id: &str, _input: &Input<'_>) -> Outcome {
    Outcome::not_built()
}

fn built_measure(_id: &str, _input: &Input<'_>) -> Outcome {
    Outcome::count(1, Vec::new())
}

const STUB: Module = Module {
    file: "test stub",
    measure: stub_measure,
    built: |_| false,
};

const BUILT: Module = Module {
    file: "test function",
    measure: built_measure,
    built: |_| true,
};

// --- the goldens ---

#[test]
fn golden_coverage_blocks() {
    golden_of("coverage.blocks");
}

#[test]
fn golden_completeness_fields() {
    golden_of("completeness.fields");
}

#[test]
fn golden_planned() {
    golden_of("planned");
}

// --- the report ---

#[test]
fn the_report_holds_each_measure_of_the_kpi_question_once_in_catalogue_order_and_follows_its_schema()
 {
    let want: Vec<String> = embedded()
        .measures
        .iter()
        .filter(|m| m.measure == "kpi_catalogue")
        .map(|m| m.id.clone())
        .collect();
    assert_eq!(want.len(), 10);
    for (name, dir) in packs() {
        let pack = pack_of(&dir);
        let report = kpi_report(&pack);
        let value = serde_json::to_value(&report).unwrap();
        assert_eq!(
            schema_errors("metaspec-quality-1.schema.json", &value),
            Vec::<String>::new(),
            "{name}"
        );
        assert_eq!(report::validate(&value), Ok(()), "{name}");
        assert_eq!(value["format"], "metaspec-quality/1");
        assert_eq!(value["pack"], json!(pack.manifest.pack), "{name}");
        assert_eq!(value["question"], "kpi_catalogue");
        let got: Vec<&str> = report
            .measurements
            .iter()
            .map(|m| m.measure.as_str())
            .collect();
        assert_eq!(got, want, "{name}");
        for m in &report.measurements {
            let row = embedded();
            let row = row.get(&m.measure).unwrap();
            assert_eq!(m.unit, row.unit, "{name} {}", m.measure);
            assert_eq!(m.direction, row.direction.as_str(), "{name} {}", m.measure);
        }
        // Each gap reading is a gap of kind `measure` with the measure id as its feature.
        let gap_readings: Vec<&str> = report
            .measurements
            .iter()
            .filter(|m| matches!(m.reading, Reading::Gap { .. }))
            .map(|m| m.measure.as_str())
            .collect();
        let gaps: Vec<&str> = report
            .gaps
            .iter()
            .inspect(|g| assert_eq!(g.kind, "measure"))
            .map(|g| g.feature.as_str())
            .collect();
        assert_eq!(gaps, gap_readings, "{name}");
        // The same sources give the same digest.
        assert_eq!(report.source_digest, kpi_report(&pack).source_digest);
        assert!(report.source_digest.starts_with("sha256:"));
    }
}

#[test]
fn a_report_that_breaks_the_schema_is_refused_by_validate() {
    let report = kpi_report(&first_pack());
    let mut value = serde_json::to_value(&report).unwrap();
    value["measurements"][0]["reading"] = json!({"kind": "value"});
    let issues = report::validate(&value).expect_err("a value reading without value");
    assert!(issues.iter().all(|i| i.code == "quality_report_invalid"));
}

#[test]
fn a_pack_without_a_kpis_section_has_a_gap_naming_it_for_each_measure() {
    let mut pack = first_pack();
    pack.manifest.kpis = None;
    pack.kpis.clear();
    for id in BUILT_HERE {
        match measure(id, &pack) {
            Outcome::Gap { gap, needs } => {
                assert!(gap.contains("kpis"), "{id}: {gap}");
                assert_eq!(needs.as_deref(), Some("L1"), "{id}");
            }
            other => panic!("{id} without a catalogue: {other:?}"),
        }
    }
}

/// The first pack with its catalogue replaced by `rows`; the modes field is the manifest's.
fn with_rows(rows: Vec<Value>) -> Pack {
    let mut pack = first_pack();
    pack.kpis = rows;
    pack
}

fn modes_field(pack: &Pack) -> String {
    pack.manifest.kpis.as_ref().unwrap().fields.modes.clone()
}

fn value_of(outcome: &Outcome) -> f64 {
    match outcome {
        Outcome::Value { value, .. } => value.as_f64().unwrap(),
        other => panic!("a value: {other:?}"),
    }
}

fn behind_of(outcome: &Outcome) -> Vec<(String, String)> {
    match outcome {
        Outcome::Value { behind, .. } => behind
            .iter()
            .map(|b| (b.kind.clone(), b.name.clone()))
            .collect(),
        other => panic!("a value: {other:?}"),
    }
}

#[test]
fn completeness_reads_the_gate_column_as_written_and_a_direction_for_every_mode() {
    let pack = first_pack();
    let modes = modes_field(&pack);
    let block = pack.blocks[0].name.clone();
    let row = |id: &str, extra: Value| {
        let mut row = json!({
            "id": id, "block": block, "unit": "count", "direction": "down",
            modes.clone(): {"wet": "down", "dry": "up"},
        });
        for (k, v) in extra.as_object().unwrap() {
            if v.is_null() {
                row.as_object_mut().unwrap().remove(k);
            } else {
                row[k] = v.clone();
            }
        }
        row
    };
    let pack = with_rows(vec![
        row("k_plain", json!({})),
        row("k_gate_false", json!({"gate": false})),
        row("k_threshold", json!({"gate": 3})),
        row("k_gate_true", json!({"gate": true})),
        row("k_no_unit", json!({"unit": null})),
        row("k_blank_direction", json!({"direction": " "})),
        row("k_one_mode", json!({modes.clone(): {"wet": "down"}})),
        row(
            "k_empty_mode",
            json!({modes.clone(): {"wet": "down", "dry": ""}}),
        ),
    ]);
    let outcome = measure("completeness.fields", &pack);
    assert_eq!(value_of(&outcome), 3.0 / 8.0);
    let behind: Vec<String> = behind_of(&outcome)
        .into_iter()
        .inspect(|(kind, _)| assert_eq!(kind, "kpi"))
        .map(|(_, name)| name)
        .collect();
    assert_eq!(
        behind,
        [
            "k_gate_true",
            "k_no_unit",
            "k_blank_direction",
            "k_one_mode",
            "k_empty_mode"
        ]
    );

    // A catalogue that writes no mode asks for none.
    let pack = with_rows(vec![
        json!({"id": "k", "block": block, "unit": "count", "direction": "down"}),
    ]);
    assert_eq!(value_of(&measure("completeness.fields", &pack)), 1.0);

    // An empty catalogue has no share.
    let pack = with_rows(Vec::new());
    assert!(matches!(
        measure("completeness.fields", &pack),
        Outcome::Gap { .. }
    ));
}

/// Every name the built measures put behind a value comes from the pack: a KPI id, a block
/// name. None of them may be blank, or the report breaks its own schema and `quality kpis`
/// withholds it. A KPI row without an id, or with an empty or blank one, is named by its
/// place, `#<index>`; a block whose name is blank (a `strip_prefix` that takes the whole domain)
/// by its domain.
#[test]
fn a_blank_kpi_id_or_block_name_is_named_by_its_place_or_domain_and_the_report_holds() {
    let mut pack = first_pack();
    pack.blocks[1].name = String::new();
    let domain = pack.blocks[1].domain.clone();
    pack.kpis = vec![
        json!({"id": ""}),
        json!({"id": "  ", "block": "not_a_block_of_the_pack"}),
        json!({"id": "\t"}),
        json!({"id": "k", "block": pack.blocks[0].name}),
    ];
    let outcome = measure("coverage.blocks", &pack);
    assert!(
        behind_of(&outcome).contains(&(String::from("block"), domain.clone())),
        "{outcome:?}"
    );
    let outcome = measure("completeness.fields", &pack);
    let names: Vec<String> = behind_of(&outcome).into_iter().map(|(_, n)| n).collect();
    assert_eq!(names, ["#0", "#1", "#2", "k"]);
    let outcome = measure("planned", &pack);
    let names: Vec<String> = behind_of(&outcome).into_iter().map(|(_, n)| n).collect();
    assert_eq!(names, ["#1"]);
    let value = serde_json::to_value(kpi_report(&pack)).unwrap();
    assert_eq!(report::validate(&value), Ok(()));
}

#[test]
fn coverage_and_planned_read_the_block_column_against_the_blocks_of_the_pack() {
    let pack = first_pack();
    assert!(
        pack.blocks.len() >= 3,
        "the first fixture pack has 3 blocks at least"
    );
    let names: Vec<String> = pack.blocks.iter().map(|b| b.name.clone()).collect();
    let pack = with_rows(vec![
        json!({"id": "k1", "block": names[0]}),
        json!({"id": "k2", "block": names[0]}),
        json!({"id": "k3", "block": names[2]}),
        json!({"id": "k4", "block": "not_a_block_of_the_pack"}),
        json!({"id": "k5"}),
    ]);
    let coverage = measure("coverage.blocks", &pack);
    assert_eq!(value_of(&coverage), 2.0 / names.len() as f64);
    let uncovered: Vec<(String, String)> = names
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != 0 && *i != 2)
        .map(|(_, n)| (String::from("block"), n.clone()))
        .collect();
    assert_eq!(behind_of(&coverage), uncovered);

    let planned = measure("planned", &pack);
    assert_eq!(value_of(&planned), 1.0);
    assert_eq!(
        behind_of(&planned),
        [(String::from("kpi"), String::from("k4"))]
    );
    match &planned {
        Outcome::Value { value, .. } => assert!(value.is_u64(), "a count is an integer"),
        _ => unreachable!(),
    }

    // No KPI: no block is covered, and none is planned.
    let pack = with_rows(Vec::new());
    assert_eq!(value_of(&measure("coverage.blocks", &pack)), 0.0);
    assert_eq!(value_of(&measure("planned", &pack)), 0.0);
}
