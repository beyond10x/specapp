//! Cases added by the adversarial pass on `story:kpi-quality-catalogue-shape`: a pack's KPI
//! row that the loader accepts must give a report that follows `metaspec-quality/1`, and the
//! schema must hold what its own descriptions say.

use super::super::common::{load_pack, packs};
use metaspec_core::Pack;
use metaspec_core::quality::catalogue;
use metaspec_core::quality::registry::{self, Input};
use metaspec_core::quality::report::{self, Question};
use serde_json::json;
use std::time::Duration;

fn first_pack() -> Pack {
    let (_, dir) = &packs()[0];
    let (loaded, pack) = load_pack(dir);
    pack.unwrap_or_else(|| panic!("{} loads: {:?}", dir.display(), loaded.errors))
}

/// A KPI row whose id is the empty string loads (no error, no warning of the loader), and it is
/// incomplete, so `completeness.fields` names it behind its value. The name must not break the
/// report: `metaspec quality kpis` validates its own report and exits 1 when it fails, so the
/// whole report of a pack that loads would be withheld for one row's id.
#[test]
fn adversary_a_kpi_row_with_an_empty_id_still_gives_a_report_that_follows_the_schema() {
    let mut pack = first_pack();
    let block = pack.blocks[0].name.clone();
    pack.kpis = vec![json!({"id": "", "block": block})];
    let catalogue = catalogue::embedded().expect("metaspec's catalogue loads");
    let input = Input::new(&pack, None, Duration::from_secs(5));
    let report = registry::run(Question::KpiCatalogue, &catalogue, &input)
        .unwrap_or_else(|issues| panic!("the KPI question runs: {issues:?}"));
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(
        report::validate(&value),
        Ok(()),
        "the report of a pack whose loader accepted every row follows metaspec-quality/1"
    );
}

/// The schema says `behind` is "empty for a gap reading", and the report module says the same;
/// a report with names behind a gap must be refused by the schema that says so.
#[test]
fn adversary_the_schema_refuses_names_behind_a_gap_reading() {
    let pack = first_pack();
    let catalogue = catalogue::embedded().expect("metaspec's catalogue loads");
    let input = Input::new(&pack, None, Duration::from_secs(5));
    let report = registry::run(Question::KpiCatalogue, &catalogue, &input).unwrap();
    let mut value = serde_json::to_value(&report).unwrap();
    let at = value["measurements"]
        .as_array()
        .unwrap()
        .iter()
        .position(|m| m["reading"]["kind"] == "gap")
        .expect("a fixture report has a gap reading (a measure not built yet)");
    value["measurements"][at]["behind"] = json!([{"kind": "kpi", "name": "k"}]);
    assert!(
        report::validate(&value).is_err(),
        "a gap reading with a name behind it passes the schema"
    );
}
