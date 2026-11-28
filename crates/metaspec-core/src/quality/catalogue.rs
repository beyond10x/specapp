//! metaspec's own KPI catalogue: one row per quality measure, in the format the packs' catalogues
//! are read into (`metaspec-kpis/1`, `schema/metaspec-kpis-1.schema.json`). The catalogue is
//! `kpis/catalogue.yaml` at the root of the repository, embedded when metaspec is built. The
//! quality runner reads the unit and the direction of each measure from it: a measure is data,
//! not code.
//!
//! The loader takes what the schema of the format takes, and refuses beyond it what the runner
//! cannot read, each text trimmed first: a row without id (`measure_without_id`, the row named
//! by its place, `measure:#<index>`), an id on two rows (`measure_duplicate`), a row without unit
//! (`measure_without_unit`) or direction (`measure_without_direction`), and a direction other
//! than metaspec's two words, `higher_is_better` and `lower_is_better` (`measure_direction`).
//! Every refusal is one issue; all are named.

use crate::report::Issue;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// The format of the catalogue.
pub const FORMAT: &str = "metaspec-kpis/1";

/// The schema of the format.
pub const SCHEMA: &str = include_str!("../../../../schema/metaspec-kpis-1.schema.json");

/// The text of metaspec's catalogue, `kpis/catalogue.yaml`, as it was when metaspec was built.
pub const TEXT: &str = include_str!("../../../../kpis/catalogue.yaml");

/// Which way a value of a measure is better; the ratchet reads it. metaspec's own words; a
/// pack's catalogue keeps its own words for `direction`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    HigherIsBetter,
    LowerIsBetter,
}

impl Direction {
    /// The word the catalogue writes.
    pub fn as_str(self) -> &'static str {
        match self {
            Direction::HigherIsBetter => "higher_is_better",
            Direction::LowerIsBetter => "lower_is_better",
        }
    }

    /// The direction a word of the catalogue names, if it names one.
    pub fn from_word(word: &str) -> Option<Direction> {
        [Direction::HigherIsBetter, Direction::LowerIsBetter]
            .into_iter()
            .find(|d| d.as_str() == word)
    }
}

/// One quality measure: a row of the catalogue.
#[derive(Debug, Clone, PartialEq)]
pub struct Measure {
    pub id: String,
    /// The block of metaspec's own specification the measure belongs to.
    pub block: String,
    /// The measure kind: the question the measure answers.
    pub measure: String,
    pub unit: String,
    pub summary: String,
    pub how: String,
    pub direction: Direction,
    /// A direction per mode; may be empty.
    pub modes: BTreeMap<String, String>,
    /// Whether the measure gates; `None` where the row sets no `gate`.
    pub gate: Option<bool>,
    pub reads: Vec<String>,
    pub status: String,
}

/// The measures of a catalogue, in the order written.
#[derive(Debug, Clone, PartialEq)]
pub struct Catalogue {
    pub measures: Vec<Measure>,
}

impl Catalogue {
    /// The measure with this id.
    pub fn get(&self, id: &str) -> Option<&Measure> {
        self.measures.iter().find(|m| m.id == id)
    }
}

/// metaspec's own catalogue, embedded when metaspec was built.
pub fn embedded() -> Result<Catalogue, Vec<Issue>> {
    parse(TEXT)
}

/// Reads a catalogue from its YAML text: the format, the schema, then the rows.
pub fn parse(text: &str) -> Result<Catalogue, Vec<Issue>> {
    let value: Value = serde_yaml_ng::from_str(text).map_err(|error| {
        vec![Issue::new(
            "catalogue_unreadable",
            "catalogue",
            format!("the catalogue is no YAML document: {error}"),
        )]
    })?;
    validate(&value)?;
    let rows = value
        .get("kpis")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut issues = Vec::new();
    let mut seen = BTreeSet::new();
    let mut measures = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let id = field(row, "id");
        let subject = if id.is_empty() {
            format!("measure:#{index}")
        } else {
            format!("measure:{id}")
        };
        if id.is_empty() {
            issues.push(Issue::new(
                "measure_without_id",
                &subject,
                "the row has no id",
            ));
        } else if !seen.insert(id.clone()) {
            issues.push(Issue::new(
                "measure_duplicate",
                &subject,
                format!("the id {id} is on more than one row"),
            ));
        }
        let unit = field(row, "unit");
        if unit.is_empty() {
            issues.push(Issue::new(
                "measure_without_unit",
                &subject,
                "the row has no unit",
            ));
        }
        let word = field(row, "direction");
        let direction = if word.is_empty() {
            issues.push(Issue::new(
                "measure_without_direction",
                &subject,
                "the row has no direction",
            ));
            None
        } else {
            let direction = Direction::from_word(&word);
            if direction.is_none() {
                issues.push(Issue::new(
                    "measure_direction",
                    &subject,
                    format!(
                        "the direction {word} is neither {} nor {}",
                        Direction::HigherIsBetter.as_str(),
                        Direction::LowerIsBetter.as_str()
                    ),
                ));
            }
            direction
        };
        let Some(direction) = direction else {
            continue;
        };
        measures.push(Measure {
            block: field(row, "block"),
            measure: field(row, "measure"),
            summary: field(row, "summary"),
            how: field(row, "how"),
            modes: modes(row.get("modes")),
            gate: row.get("gate").and_then(Value::as_bool),
            reads: row
                .get("reads")
                .and_then(Value::as_array)
                .map(|xs| {
                    xs.iter()
                        .filter_map(Value::as_str)
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default(),
            status: field(row, "status"),
            id,
            unit,
            direction,
        });
    }
    if issues.is_empty() {
        Ok(Catalogue { measures })
    } else {
        Err(issues)
    }
}

/// Validates a catalogue, already read into JSON, against the schema of `metaspec-kpis/1`.
pub fn validate(value: &Value) -> Result<(), Vec<Issue>> {
    if let Some(format) = value.get("format").and_then(Value::as_str)
        && format != FORMAT
    {
        return Err(vec![Issue::new(
            "catalogue_format",
            "catalogue",
            format!("the catalogue is in format {format}; this metaspec reads {FORMAT}"),
        )]);
    }
    let schema: Value = serde_json::from_str(SCHEMA).expect("the catalogue schema is JSON");
    let validator = jsonschema::validator_for(&schema).expect("the catalogue schema compiles");
    let issues: Vec<Issue> = validator
        .iter_errors(value)
        .map(|error| {
            let at = error.instance_path().to_string();
            let at = if at.is_empty() { String::from("/") } else { at };
            Issue::new(
                "catalogue_invalid",
                "catalogue",
                format!("at {at}: {error}"),
            )
        })
        .collect();
    if issues.is_empty() {
        Ok(())
    } else {
        Err(issues)
    }
}

/// A text field of a row, trimmed; empty where the row has none.
fn field(row: &Value, name: &str) -> String {
    row.get(name)
        .and_then(Value::as_str)
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

/// A direction per mode, from a map (as a catalogue writes it) or a list of `{mode, direction}`.
fn modes(value: Option<&Value>) -> BTreeMap<String, String> {
    match value {
        Some(Value::Object(map)) => map
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string()))
            .collect(),
        Some(Value::Array(list)) => list
            .iter()
            .map(|m| (field(m, "mode"), field(m, "direction")))
            .collect(),
        _ => BTreeMap::new(),
    }
}
