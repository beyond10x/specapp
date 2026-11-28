//! Cases added by the adversarial pass on the catalogue loader: what the loader keeps of a row,
//! and an id the schema of the format refuses that the loader must not let through either.

use super::super::common::codes;
use metaspec_core::quality::catalogue::{self, Direction};
use std::collections::BTreeMap;

/// The schema of `metaspec-kpis/1` refuses an empty id (`minLength: 1`); the loader trims a
/// row's text, so an id of blanks becomes the empty id. The runner reads a measure by its id: a
/// row it cannot name must be refused, not loaded as the measure "".
#[test]
fn a_row_whose_id_is_only_blanks_is_refused() {
    let text = "format: metaspec-kpis/1\nkpis:\n  - id: \"   \"\n    block: quality\n    unit: share\n    direction: higher_is_better\n";
    match catalogue::parse(text) {
        Ok(loaded) => panic!(
            "a row with a blank id loaded as {:?}",
            loaded
                .measures
                .iter()
                .map(|m| m.id.as_str())
                .collect::<Vec<_>>()
        ),
        Err(issues) => assert!(!codes(&issues).is_empty()),
    }
}

/// Every column a row writes is kept as written: block, measure kind, summary, how, a direction
/// per mode (as a map, the way a catalogue writes it), gate, reads and status. No case of the
/// unit reads any of these after loading, so a loader that dropped them stayed green.
#[test]
fn a_row_keeps_every_column_it_writes() {
    let text = "format: metaspec-kpis/1\nkpis:\n  - id: m\n    block: quality\n    measure: kpi_catalogue\n    unit: share\n    summary: S.\n    how: H.\n    direction: lower_is_better\n    modes:\n      strict: higher_is_better\n      loose: lower_is_better\n    gate: true\n    reads: [e.one, e.two]\n    status: planned\n";
    let loaded = catalogue::parse(text).unwrap_or_else(|e| panic!("{e:?}"));
    let m = &loaded.measures[0];
    assert_eq!(m.block, "quality");
    assert_eq!(m.measure, "kpi_catalogue");
    assert_eq!(m.summary, "S.");
    assert_eq!(m.how, "H.");
    assert_eq!(m.direction, Direction::LowerIsBetter);
    let want: BTreeMap<String, String> = [
        ("loose".to_string(), "lower_is_better".to_string()),
        ("strict".to_string(), "higher_is_better".to_string()),
    ]
    .into_iter()
    .collect();
    assert_eq!(m.modes, want);
    assert_eq!(m.gate, Some(true));
    assert_eq!(m.reads, ["e.one", "e.two"]);
    assert_eq!(m.status, "planned");
}
