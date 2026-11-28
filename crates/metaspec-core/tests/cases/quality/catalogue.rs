//! metaspec's own KPI catalogue, `kpis/catalogue.yaml` (format `metaspec-kpis/1`): one row per
//! quality measure, and the loader that refuses a row the quality runner cannot read.

use super::super::common::{codes, repo, schema_errors};
use metaspec_core::quality::catalogue::{self, Direction};
use serde_json::Value;
use std::collections::BTreeMap;

/// The measure ids of the KPI question, as the epic fixes them under "Measures".
const KPI_QUESTION: &str = "
    coverage.blocks coverage.rules coverage.lifecycles grounding completeness.fields alive
    discrimination gate_fidelity redundancy planned
";

/// The measure ids of the specification question, as the epic fixes them under "Measures": one
/// `mutation.<class>` per mutation class of `ess verify conform mutate --class`.
const SPECIFICATION_QUESTION: &str = "
    validity.refusals validity.warnings layering
    ess_completeness.unscenarioed ess_completeness.unanswered ess_completeness.outside
    conformance.synthesized conformance.passed conformance.known_failing conformance.skipped
    mutation.score
    mutation.from-drop mutation.transition-to mutation.guard-boundary mutation.sets-retarget
    mutation.guard-negate mutation.guard-connective mutation.error-swap mutation.emit-drop
    mutation.order-flip mutation.sets-drop mutation.precedence-swap mutation.emit-swap
    mutation.grant-drop mutation.grant-add
    honesty refusal_power document_health adoption_level drift
";

fn ids(list: &str) -> Vec<&str> {
    list.split_whitespace().collect()
}

fn committed() -> String {
    std::fs::read_to_string(repo().join("kpis/catalogue.yaml")).expect("kpis/catalogue.yaml")
}

/// A catalogue of one row: `id: m`, with the given fields after it.
fn one_row(fields: &str) -> String {
    format!("format: metaspec-kpis/1\nkpis:\n  - id: m\n    block: quality\n{fields}")
}

#[test]
fn the_committed_catalogue_holds_each_measure_id_of_both_epics_once() {
    let kpi = ids(KPI_QUESTION);
    let spec = ids(SPECIFICATION_QUESTION);
    assert_eq!(kpi.len(), 10, "the KPI question fixes 10 measures");
    assert_eq!(
        spec.len(),
        30,
        "the specification question fixes 30 measures"
    );
    let value: Value = serde_yaml_ng::from_str(&committed()).expect("the catalogue is YAML");
    let rows = value["kpis"].as_array().expect("a list `kpis`");
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for row in rows {
        let id = row["id"].as_str().expect("every row has an id");
        *seen.entry(id.to_string()).or_default() += 1;
    }
    let twice: Vec<&String> = seen
        .iter()
        .filter(|(_, n)| **n > 1)
        .map(|(id, _)| id)
        .collect();
    assert_eq!(twice, Vec::<&String>::new(), "an id on more than one row");
    let mut want: Vec<String> = kpi.iter().chain(&spec).map(|s| s.to_string()).collect();
    want.sort();
    let have: Vec<String> = seen.into_keys().collect();
    assert_eq!(have, want);
    assert_eq!(rows.len(), 40);
}

#[test]
fn the_embedded_catalogue_is_the_committed_file_and_loads() {
    assert_eq!(catalogue::TEXT, committed());
    let loaded = catalogue::embedded().unwrap_or_else(|e| panic!("{e:?}"));
    assert_eq!(loaded.measures.len(), 40);
    let lower = ids("
        validity.refusals validity.warnings layering ess_completeness.unscenarioed
        ess_completeness.unanswered ess_completeness.outside conformance.known_failing
        conformance.skipped honesty redundancy planned drift
    ");
    for m in &loaded.measures {
        let want = if lower.contains(&m.id.as_str()) {
            Direction::LowerIsBetter
        } else {
            Direction::HigherIsBetter
        };
        assert_eq!(m.direction, want, "{}", m.id);
        assert!(!m.unit.is_empty(), "{}", m.id);
        assert_eq!(m.gate, None, "{}: no threshold is invented", m.id);
    }
    let first = loaded.get("coverage.blocks").expect("a measure by its id");
    assert_eq!(first.direction, Direction::HigherIsBetter);
    assert!(loaded.get("no.such.measure").is_none());
}

#[test]
fn the_catalogue_follows_the_schema_of_its_format_as_the_kpi_pages_do() {
    let value: Value = serde_yaml_ng::from_str(&committed()).unwrap();
    assert_eq!(value["format"], catalogue::FORMAT);
    assert_eq!(
        schema_errors("metaspec-kpis-1.schema.json", &value),
        Vec::<String>::new(),
        "kpis/catalogue.yaml"
    );
    let site = repo().join("fixtures/expected/site");
    let mut pages = 0;
    for entry in std::fs::read_dir(&site).unwrap().flatten() {
        let path = entry.path().join("pages/kpis.json");
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let page: Value = serde_json::from_str(&text).unwrap();
        let data = &page["data"]["catalogue"];
        assert_eq!(data["format"], catalogue::FORMAT);
        assert_eq!(
            schema_errors("metaspec-kpis-1.schema.json", data),
            Vec::<String>::new(),
            "{}",
            path.display()
        );
        pages += 1;
    }
    assert!(pages >= 2, "a KPI page per fixture pack");
}

#[test]
fn a_row_with_unit_and_one_of_the_two_words_loads() {
    for (word, want) in [
        ("higher_is_better", Direction::HigherIsBetter),
        ("lower_is_better", Direction::LowerIsBetter),
    ] {
        let text = one_row(&format!("    unit: share\n    direction: {word}\n"));
        let loaded = catalogue::parse(&text).unwrap_or_else(|e| panic!("{word}: {e:?}"));
        assert_eq!(loaded.measures.len(), 1);
        assert_eq!(loaded.measures[0].direction, want);
        assert_eq!(loaded.measures[0].unit, "share");
        assert_eq!(want.as_str(), word);
    }
}

#[test]
fn a_duplicate_id_is_refused() {
    let text = format!(
        "{}  - id: m\n    block: quality\n    unit: count\n    direction: lower_is_better\n",
        one_row("    unit: share\n    direction: higher_is_better\n")
    );
    let issues = catalogue::parse(&text).unwrap_err();
    assert_eq!(codes(&issues), ["measure_duplicate"]);
    assert_eq!(issues[0].subject, "measure:m");
}

#[test]
fn a_row_whose_id_is_blank_after_trimming_is_refused_by_its_place() {
    let row = "    block: quality\n    unit: share\n    direction: higher_is_better\n";
    let text = format!(
        "format: metaspec-kpis/1\nkpis:\n  - id: m\n{row}  - id: \"   \"\n{row}  - id: \"\\t\"\n    block: quality\n    direction: up\n"
    );
    let issues = catalogue::parse(&text).unwrap_err();
    assert_eq!(
        codes(&issues),
        [
            "measure_without_id",
            "measure_without_id",
            "measure_without_unit",
            "measure_direction"
        ],
        "two blank ids are no duplicate of each other, and the row's other refusals are named"
    );
    assert_eq!(issues[0].subject, "measure:#1");
    assert_eq!(issues[1].subject, "measure:#2");
    assert_eq!(issues[2].subject, "measure:#2");
}

#[test]
fn a_row_without_unit_is_refused() {
    for fields in [
        "    direction: higher_is_better\n",
        "    unit: \"\"\n    direction: higher_is_better\n",
        "    unit: \"  \"\n    direction: higher_is_better\n",
    ] {
        let issues = catalogue::parse(&one_row(fields)).unwrap_err();
        assert_eq!(codes(&issues), ["measure_without_unit"], "{fields}");
        assert_eq!(issues[0].subject, "measure:m");
    }
}

#[test]
fn a_row_without_direction_is_refused() {
    for fields in [
        "    unit: share\n",
        "    unit: share\n    direction: \"\"\n",
    ] {
        let issues = catalogue::parse(&one_row(fields)).unwrap_err();
        assert_eq!(codes(&issues), ["measure_without_direction"], "{fields}");
        assert_eq!(issues[0].subject, "measure:m");
    }
}

#[test]
fn a_direction_outside_the_two_words_is_refused() {
    for word in ["higher", "lower", "Higher_Is_Better", "up"] {
        let text = one_row(&format!("    unit: share\n    direction: {word}\n"));
        let issues = catalogue::parse(&text).unwrap_err();
        assert_eq!(codes(&issues), ["measure_direction"], "{word}");
        assert_eq!(issues[0].subject, "measure:m");
        assert!(issues[0].message.contains(word), "{}", issues[0].message);
    }
}

#[test]
fn every_refusal_of_a_catalogue_is_named_not_only_the_first() {
    let text = "format: metaspec-kpis/1\nkpis:\n  - id: a\n    direction: higher_is_better\n  - id: b\n    unit: share\n  - id: b\n    unit: share\n    direction: up\n";
    let issues = catalogue::parse(text).unwrap_err();
    assert_eq!(
        codes(&issues),
        [
            "measure_without_unit",
            "measure_without_direction",
            "measure_duplicate",
            "measure_direction"
        ]
    );
}

#[test]
fn a_catalogue_in_another_format_or_shape_is_refused() {
    let other = catalogue::parse("format: metaspec-kpis/2\nkpis: []\n").unwrap_err();
    assert_eq!(codes(&other), ["catalogue_format"]);
    assert!(other[0].message.contains("metaspec-kpis/2"));
    let shape = catalogue::parse("format: metaspec-kpis/1\nkpis: {}\n").unwrap_err();
    assert_eq!(codes(&shape), ["catalogue_invalid"]);
    let yaml = catalogue::parse("format: [\n").unwrap_err();
    assert_eq!(codes(&yaml), ["catalogue_unreadable"]);
}
