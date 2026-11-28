//! The report of one quality question over one pack, format `metaspec-quality/1`
//! (`schema/metaspec-quality-1.schema.json`). Both questions share it: `kpi_catalogue`
//! (`metaspec quality kpis`) and `specification` (`metaspec quality spec`).
//!
//! A report names the pack, the question and the digest of the sources the measures read, and
//! holds one measurement per measure of the question: the measure id, its unit and direction
//! from metaspec's catalogue, a reading and the names behind it. A reading is a value or a gap;
//! a gap reading is also listed among the report's gaps, of kind `measure` with the measure id
//! as its feature, so what a pack cannot supply is named and never a silent zero.

use crate::report::Issue;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

/// The format of a report.
pub const FORMAT: &str = "metaspec-quality/1";

/// The schema of the format.
pub const SCHEMA: &str = include_str!("../../../../schema/metaspec-quality-1.schema.json");

/// The gap reading of a measure metaspec does not build yet.
pub const NOT_BUILT: &str = "not built yet";

/// What a measure is about: a pack's KPI catalogue or its specification. The words are the
/// `measure` column of metaspec's catalogue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Question {
    KpiCatalogue,
    Specification,
}

impl Question {
    /// The word the catalogue and the report write.
    pub fn as_str(self) -> &'static str {
        match self {
            Question::KpiCatalogue => "kpi_catalogue",
            Question::Specification => "specification",
        }
    }
}

/// One report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityReport {
    pub format: String,
    pub pack: String,
    pub question: Question,
    /// `sha256:` and the digest of the sources the measures read.
    pub source_digest: String,
    pub measurements: Vec<Measurement>,
    pub gaps: Vec<Gap>,
}

/// The reading of one measure for the pack.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measurement {
    pub measure: String,
    /// The unit of the measure's row in metaspec's catalogue.
    pub unit: String,
    /// The direction of the measure's row in metaspec's catalogue.
    pub direction: String,
    pub reading: Reading,
    /// The names behind the value, as the pack writes them; empty for a gap.
    pub behind: Vec<Behind>,
}

/// A value, or the gap that says why the pack cannot supply one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Reading {
    Value { value: Number },
    Gap { gap: String },
}

/// One name behind a value: what it names (`block`, `kpi`, ...) and the name as the pack
/// writes it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Behind {
    pub kind: String,
    pub name: String,
}

impl Behind {
    pub fn new(kind: &str, name: impl Into<String>) -> Behind {
        Behind {
            kind: kind.to_string(),
            name: name.into(),
        }
    }
}

/// A feature metaspec offers that the pack cannot use: of kind `measure` for a gap reading.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gap {
    pub kind: String,
    pub feature: String,
    /// The level of adoption whose input the pack lacks (`L0` to `L5`); `None` when the gap is
    /// metaspec's own, a measure not built yet.
    pub needs: Option<String>,
    pub summary: String,
}

/// What a measure function answers; the runner adds the measure id, unit and direction.
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Value { value: Number, behind: Vec<Behind> },
    Gap { gap: String, needs: Option<String> },
}

impl Outcome {
    /// A count.
    pub fn count(n: usize, behind: Vec<Behind>) -> Outcome {
        Outcome::Value {
            value: Number::from(n as u64),
            behind,
        }
    }

    /// `part` over `whole`; when `whole` is 0 there is no share, and the outcome is the gap
    /// `empty` with the level `needs` whose input is missing.
    pub fn share(
        part: usize,
        whole: usize,
        behind: Vec<Behind>,
        needs: Option<&str>,
        empty: &str,
    ) -> Outcome {
        match Number::from_f64(part as f64 / whole as f64) {
            Some(value) if whole > 0 => Outcome::Value { value, behind },
            _ => Outcome::gap(needs, empty),
        }
    }

    /// A gap the pack has: `needs` is the level of adoption whose input it lacks.
    pub fn gap(needs: Option<&str>, gap: impl Into<String>) -> Outcome {
        Outcome::Gap {
            gap: gap.into(),
            needs: needs.map(String::from),
        }
    }

    /// The gap reading of a measure metaspec does not build yet.
    pub fn not_built() -> Outcome {
        Outcome::gap(None, NOT_BUILT)
    }
}

impl QualityReport {
    /// The report as lines for the terminal: one per measure, then the number of gaps.
    pub fn text(&self) -> String {
        let mut out = format!(
            "pack: {}\nquestion: {}\nsource: {}\n",
            self.pack,
            self.question.as_str(),
            self.source_digest
        );
        for m in &self.measurements {
            match &m.reading {
                Reading::Value { value } => {
                    out.push_str(&format!("{}: {value} {}", m.measure, m.unit));
                    if !m.behind.is_empty() {
                        let names: Vec<String> = m
                            .behind
                            .iter()
                            .map(|b| format!("{} {}", b.kind, b.name))
                            .collect();
                        out.push_str(&format!(
                            "; behind {}: {}",
                            m.behind.len(),
                            names.join(", ")
                        ));
                    }
                    out.push('\n');
                }
                Reading::Gap { gap } => out.push_str(&format!("{}: gap: {gap}\n", m.measure)),
            }
        }
        let gaps = self
            .measurements
            .iter()
            .filter(|m| matches!(m.reading, Reading::Gap { .. }))
            .count();
        out.push_str(&format!(
            "measured: {} value(s), {gaps} gap(s)\n",
            self.measurements.len() - gaps
        ));
        out
    }
}

/// Validates a report, already read into JSON, against the schema of `metaspec-quality/1`.
pub fn validate(value: &Value) -> Result<(), Vec<Issue>> {
    let schema: Value = serde_json::from_str(SCHEMA).expect("the quality schema is JSON");
    let validator = jsonschema::validator_for(&schema).expect("the quality schema compiles");
    let issues: Vec<Issue> = validator
        .iter_errors(value)
        .map(|error| {
            let at = error.instance_path().to_string();
            let at = if at.is_empty() { String::from("/") } else { at };
            Issue::new(
                "quality_report_invalid",
                "report",
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
