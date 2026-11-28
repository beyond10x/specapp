//! Every measure of metaspec's catalogue, mapped to the function that measures it, in the
//! module file of the story that builds it. A module not built yet is a stub: each of its
//! measures answers the gap reading "not built yet". A module says itself which of its
//! measures it builds, so a story that fills a stub changes its own file only.
//!
//! The registry is checked against the catalogue before anything is measured: a row of status
//! `measured` whose entry is a stub (`measure_not_built`) or absent (`measure_not_registered`)
//! is refused, naming the row, and so is a measure the registry maps with no row in the
//! catalogue (`measure_without_row`).

use super::catalogue::Catalogue;
use super::check_pass::{self, CheckPass};
use super::kpis::{catalogue_measures, grounding, rules, values};
use super::report::{FORMAT, Gap, Measurement, Outcome, QualityReport, Question, Reading};
use super::spec::{conformance, drift, layering, mutation, refusal, static_measures};
use crate::pack::Pack;
use crate::report::Issue;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::cell::OnceCell;
use std::path::Path;
use std::time::Duration;

/// What every measure reads: the loaded pack, the state directory where the user trusts a
/// pack's tool, and how long one run of the tool may take. The check of every document is run
/// at most once per input ([`Input::check_pass`]).
pub struct Input<'a> {
    pub pack: &'a Pack,
    pub state: Option<&'a Path>,
    pub deadline: Duration,
    check_pass: OnceCell<Result<CheckPass, String>>,
}

impl<'a> Input<'a> {
    pub fn new(pack: &'a Pack, state: Option<&'a Path>, deadline: Duration) -> Input<'a> {
        Input {
            pack,
            state,
            deadline,
            check_pass: OnceCell::new(),
        }
    }

    /// The check of every document of the pack, run on the first call only.
    pub fn check_pass(&self) -> &Result<CheckPass, String> {
        self.check_pass.get_or_init(|| check_pass::run(self))
    }
}

/// One module file of measures.
pub struct Module {
    /// The file, relative to `quality/`.
    pub file: &'static str,
    /// Measures one of the module's measures, by its id.
    pub measure: fn(&str, &Input<'_>) -> Outcome,
    /// Whether the module builds the measure; `false` for a stub.
    pub built: fn(&str) -> bool,
}

/// A registry: each measure id with its module.
pub type Registry = [(&'static str, &'static Module)];

/// The registry of metaspec: every id of its catalogue, in the order of the catalogue.
pub const REGISTRY: &Registry = &[
    ("coverage.blocks", &catalogue_measures::MODULE),
    ("coverage.rules", &rules::MODULE),
    ("coverage.lifecycles", &grounding::MODULE),
    ("grounding", &grounding::MODULE),
    ("completeness.fields", &catalogue_measures::MODULE),
    ("alive", &values::MODULE),
    ("discrimination", &values::MODULE),
    ("gate_fidelity", &rules::MODULE),
    ("redundancy", &values::MODULE),
    ("planned", &catalogue_measures::MODULE),
    ("validity.refusals", &static_measures::MODULE),
    ("validity.warnings", &static_measures::MODULE),
    ("layering", &layering::MODULE),
    ("ess_completeness.unscenarioed", &static_measures::MODULE),
    ("ess_completeness.unanswered", &static_measures::MODULE),
    ("ess_completeness.outside", &static_measures::MODULE),
    ("conformance.synthesized", &conformance::MODULE),
    ("conformance.passed", &conformance::MODULE),
    ("conformance.known_failing", &conformance::MODULE),
    ("conformance.skipped", &conformance::MODULE),
    ("mutation.score", &mutation::MODULE),
    ("mutation.from-drop", &mutation::MODULE),
    ("mutation.transition-to", &mutation::MODULE),
    ("mutation.guard-boundary", &mutation::MODULE),
    ("mutation.sets-retarget", &mutation::MODULE),
    ("mutation.guard-negate", &mutation::MODULE),
    ("mutation.guard-connective", &mutation::MODULE),
    ("mutation.error-swap", &mutation::MODULE),
    ("mutation.emit-drop", &mutation::MODULE),
    ("mutation.order-flip", &mutation::MODULE),
    ("mutation.sets-drop", &mutation::MODULE),
    ("mutation.precedence-swap", &mutation::MODULE),
    ("mutation.emit-swap", &mutation::MODULE),
    ("mutation.grant-drop", &mutation::MODULE),
    ("mutation.grant-add", &mutation::MODULE),
    ("honesty", &static_measures::MODULE),
    ("refusal_power", &refusal::MODULE),
    ("document_health", &refusal::MODULE),
    ("adoption_level", &static_measures::MODULE),
    ("drift", &drift::MODULE),
];

/// The module of a measure in metaspec's registry.
pub fn entry(id: &str) -> Option<&'static Module> {
    lookup(REGISTRY, id)
}

fn lookup(registry: &Registry, id: &str) -> Option<&'static Module> {
    registry.iter().find(|(i, _)| *i == id).map(|(_, m)| *m)
}

/// Checks metaspec's registry against a catalogue.
pub fn check(catalogue: &Catalogue) -> Result<(), Vec<Issue>> {
    check_with(catalogue, REGISTRY)
}

/// Checks a registry against a catalogue: every `measured` row has a built entry, and every
/// entry has a row.
pub fn check_with(catalogue: &Catalogue, registry: &Registry) -> Result<(), Vec<Issue>> {
    let mut issues = Vec::new();
    for row in catalogue.measures.iter().filter(|m| m.status == "measured") {
        let subject = format!("measure:{}", row.id);
        match lookup(registry, &row.id) {
            None => issues.push(Issue::new(
                "measure_not_registered",
                &subject,
                format!(
                    "the row {} is measured, and no function of metaspec measures it",
                    row.id
                ),
            )),
            Some(module) if !(module.built)(&row.id) => issues.push(Issue::new(
                "measure_not_built",
                &subject,
                format!(
                    "the row {} is measured, and its function in {} is a stub",
                    row.id, module.file
                ),
            )),
            Some(_) => {}
        }
    }
    for (id, module) in registry {
        if catalogue.get(id).is_none() {
            issues.push(Issue::new(
                "measure_without_row",
                format!("measure:{id}"),
                format!(
                    "{} measures {id}, which has no row in the catalogue",
                    module.file
                ),
            ));
        }
    }
    if issues.is_empty() {
        Ok(())
    } else {
        Err(issues)
    }
}

/// Measures every row of the catalogue that answers `question`, in the order of the catalogue,
/// with metaspec's registry.
pub fn run(
    question: Question,
    catalogue: &Catalogue,
    input: &Input<'_>,
) -> Result<QualityReport, Vec<Issue>> {
    run_with(question, catalogue, REGISTRY, input)
}

/// Measures every row of the catalogue that answers `question` with a registry, after the
/// registry is checked against the catalogue.
pub fn run_with(
    question: Question,
    catalogue: &Catalogue,
    registry: &Registry,
    input: &Input<'_>,
) -> Result<QualityReport, Vec<Issue>> {
    check_with(catalogue, registry)?;
    let mut measurements = Vec::new();
    let mut gaps = Vec::new();
    for row in catalogue
        .measures
        .iter()
        .filter(|m| m.measure == question.as_str())
    {
        let outcome = match lookup(registry, &row.id) {
            Some(module) => (module.measure)(&row.id, input),
            None => Outcome::not_built(),
        };
        let (reading, behind) = match outcome {
            Outcome::Value { value, behind } => (Reading::Value { value }, behind),
            Outcome::Gap { gap, needs } => {
                gaps.push(Gap {
                    kind: String::from("measure"),
                    feature: row.id.clone(),
                    needs,
                    summary: gap.clone(),
                });
                (Reading::Gap { gap }, Vec::new())
            }
        };
        measurements.push(Measurement {
            measure: row.id.clone(),
            unit: row.unit.clone(),
            direction: row.direction.as_str().to_string(),
            reading,
            behind,
        });
    }
    Ok(QualityReport {
        format: FORMAT.to_string(),
        pack: input.pack.manifest.pack.clone(),
        question,
        source_digest: source_digest(input.pack),
        measurements,
        gaps,
    })
}

/// `sha256:` and the digest of what the measures read of a pack: the manifest, the compiled
/// blocks, the rows of the KPI catalogue and the documents (path and content).
pub fn source_digest(pack: &Pack) -> String {
    let documents: Vec<Value> = pack
        .documents
        .iter()
        .flatten()
        .map(|d| json!([d.path, d.value]))
        .collect();
    let sources = json!([pack.manifest, pack.model, pack.kpis, documents]);
    let digest = Sha256::digest(sources.to_string().as_bytes());
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    format!("sha256:{hex}")
}
