//! The measures of the specification that read only `ess` outputs, the sources of the blocks
//! and the manifest:
//!
//! - `validity.refusals`: what `ess specify validate --format json` refuses: each entry of
//!   `problems` of a refusal (written `[<code>] <subject>: <message>`; a refusal that lists no
//!   problem counts its `diagnostics`), and each entry of `scenario_refusals` of a report;
//!   behind: `<code> <subject>` (a scenario refusal's subject is its scenario, else its origin;
//!   a diagnostic's is its `span.path`).
//! - `validity.warnings`: each entry of `warnings` of the same answer; behind:
//!   `<code> <span.path>`.
//! - `ess_completeness.unscenarioed`, `.unanswered`, `.outside`: `completeness.counts` of the
//!   same answer, 0 when a report carries no `completeness` (ess leaves it out when nothing is
//!   owed); behind: `<code> <subject>` of each construct without a scenario, each unanswered
//!   question, each scenario held outside. Blocks that do not validate, and a model that
//!   synthesis does not admit (`completeness.unsynthesizable`), make the three rows gaps naming
//!   the refusal: nothing was synthesized, so no count is a zero.
//! - `honesty`: lines with an open `UNMAPPED:` marker in the files under `blocks.path`, as
//!   `grep -r` counts them; behind: `<file>:<line>` relative to the pack (kind `marker`).
//! - `adoption_level`: the level of adoption `metaspec pack check` derives from the manifest,
//!   0 to 5 ([`Manifest::level`]). Each verb metaspec offers that the manifest does not declare
//!   is a gap of kind `verb` of the report, with the level that needs it ([`report`]).
//!
//! The answer of `ess` is the cached one of [`ess_answer`]. An `ess` that gives no answer makes
//! its five measures gaps naming why; honesty and the adoption level still measure.
//!
//! A name behind a value is never blank: an entry without a code or subject is named
//! `#<index>` by its place.

pub mod ess_answer;

use crate::manifest::{CheckAdapter, Manifest, Verbs};
use crate::pack::Pack;
use crate::paths::{resolved_inside, shown};
use crate::quality::catalogue::Catalogue;
use crate::quality::registry::{self, Input, Module};
use crate::quality::report::{Behind, Gap, Outcome, QualityReport, Question, Reading};
use crate::report::Issue;
use serde_json::{Number, Value};

/// The measures of this file.
pub const MODULE: Module = Module {
    file: "spec/static_measures.rs",
    measure,
    built,
};

const VALIDITY_REFUSALS: &str = "validity.refusals";
const VALIDITY_WARNINGS: &str = "validity.warnings";
const UNSCENARIOED: &str = "ess_completeness.unscenarioed";
const UNANSWERED: &str = "ess_completeness.unanswered";
const OUTSIDE: &str = "ess_completeness.outside";
const HONESTY: &str = "honesty";
const ADOPTION_LEVEL: &str = "adoption_level";

/// The marker of an open question in a specification source.
const MARKER: &[u8] = b"UNMAPPED:";

/// Every verb metaspec offers, with the level of adoption that needs it.
pub const OFFERED_VERBS: &[(&str, &str)] = &[("check", "L3")];

fn built(id: &str) -> bool {
    [
        VALIDITY_REFUSALS,
        VALIDITY_WARNINGS,
        UNSCENARIOED,
        UNANSWERED,
        OUTSIDE,
        HONESTY,
        ADOPTION_LEVEL,
    ]
    .contains(&id)
}

/// The report of the specification question: every measure of metaspec's catalogue whose
/// question is `specification`, and the gaps of kind `verb` of the adoption level.
pub fn report(catalogue: &Catalogue, input: &Input<'_>) -> Result<QualityReport, Vec<Issue>> {
    let mut report = registry::run(Question::Specification, catalogue, input)?;
    let measured = report
        .measurements
        .iter()
        .any(|m| m.measure == ADOPTION_LEVEL && matches!(m.reading, Reading::Value { .. }));
    if measured {
        report.gaps.extend(verb_gaps(&input.pack.manifest));
    }
    Ok(report)
}

/// Each verb metaspec offers that the manifest does not declare, as a gap of kind `verb`.
pub fn verb_gaps(manifest: &Manifest) -> Vec<Gap> {
    let declared = manifest
        .tool
        .as_ref()
        .map(|t| t.verbs.names())
        .unwrap_or_default();
    OFFERED_VERBS
        .iter()
        .filter(|(verb, _)| !declared.contains(verb))
        .map(|(verb, level)| Gap {
            kind: String::from("verb"),
            feature: verb.to_string(),
            needs: Some(level.to_string()),
            summary: format!(
                "the manifest declares no adapter for the verb {verb} (tool.verbs.{verb}), which {level} needs"
            ),
        })
        .collect()
}

/// The verbs a manifest can declare. A verb added to [`Verbs`] breaks this literal, and
/// [`OFFERED_VERBS`] takes it with its level.
pub fn declarable_verbs() -> Vec<&'static str> {
    Verbs {
        check: Some(CheckAdapter::default()),
    }
    .names()
}

fn measure(id: &str, input: &Input<'_>) -> Outcome {
    let pack = input.pack;
    match id {
        HONESTY => honesty(pack),
        ADOPTION_LEVEL => adoption_level(&pack.manifest),
        VALIDITY_REFUSALS | VALIDITY_WARNINGS | UNSCENARIOED | UNANSWERED | OUTSIDE => {
            match ess_answer::validate(pack) {
                Ok(answer) => from_validate(id, &answer),
                Err(why) => Outcome::gap(
                    Some("L0"),
                    format!("ess specify validate gave no answer: {why}"),
                ),
            }
        }
        _ => Outcome::not_built(),
    }
}

/// One measure from the answer of `ess specify validate --format json`.
fn from_validate(id: &str, answer: &Value) -> Outcome {
    let refused = answer.get("compiled").and_then(Value::as_bool) == Some(false)
        || answer.get("problems").is_some_and(Value::is_array);
    if !refused && !answer.get("valid").is_some_and(Value::is_boolean) {
        return Outcome::gap(
            Some("L0"),
            "ess specify validate answered neither a report nor a refusal",
        );
    }
    match id {
        VALIDITY_REFUSALS => {
            let mut behind: Vec<Behind> = list(answer, "problems")
                .iter()
                .enumerate()
                .map(|(index, problem)| Behind::new("finding", problem_name(problem, index)))
                .collect();
            if refused && behind.is_empty() {
                behind = coded(list(answer, "diagnostics"));
            }
            let offset = behind.len();
            behind.extend(list(answer, "scenario_refusals").iter().enumerate().map(
                |(index, refusal)| {
                    let subject = text(refusal, "scenario").or_else(|| text(refusal, "origin"));
                    Behind::new(
                        "finding",
                        name_of(text(refusal, "code"), subject, offset + index),
                    )
                },
            ));
            Outcome::count(behind.len(), behind)
        }
        VALIDITY_WARNINGS => {
            let behind = coded(list(answer, "warnings"));
            Outcome::count(behind.len(), behind)
        }
        _ if refused => Outcome::gap(
            Some("L0"),
            format!("the blocks do not validate: {}", first_refusal(answer)),
        ),
        _ => completeness(id, answer),
    }
}

/// Diagnostics in ess's shape (`warnings`, the `diagnostics` of a refusal), each named
/// `<code> <span.path>`.
fn coded(diagnostics: &[Value]) -> Vec<Behind> {
    diagnostics
        .iter()
        .enumerate()
        .map(|(index, diagnostic)| {
            let path = diagnostic.pointer("/span/path").and_then(Value::as_str);
            Behind::new("finding", name_of(text(diagnostic, "code"), path, index))
        })
        .collect()
}

/// One of the three completeness counts of a report.
fn completeness(id: &str, answer: &Value) -> Outcome {
    let Some(owed) = answer.get("completeness") else {
        return Outcome::count(0, Vec::new());
    };
    let unsynthesizable = list(owed, "unsynthesizable");
    if let Some(first) = unsynthesizable.first() {
        let more = match unsynthesizable.len() {
            1 => String::new(),
            n => format!(" (and {} more)", n - 1),
        };
        return Outcome::gap(
            Some("L0"),
            format!(
                "conformance synthesis does not admit the model, so nothing was synthesized: {} {}: {}{more}",
                text(first, "reason").unwrap_or("?"),
                text(first, "path").unwrap_or("?"),
                text(first, "detail").unwrap_or("?"),
            ),
        );
    }
    let field = match id {
        UNSCENARIOED => "unscenarioed",
        UNANSWERED => "unanswered",
        _ => "outside",
    };
    let entries = list(owed, field);
    let behind: Vec<Behind> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| match field {
            "unscenarioed" => Behind::new(
                "finding",
                name_of(text(entry, "code"), text(entry, "subject"), index),
            ),
            "outside" => Behind::new("scenario", name_of(None, text(entry, "scenario"), index)),
            _ => Behind::new("finding", name_of(None, entry.as_str(), index)),
        })
        .collect();
    match owed
        .pointer(&format!("/counts/{field}"))
        .and_then(Value::as_u64)
    {
        Some(count) => Outcome::Value {
            value: Number::from(count),
            behind,
        },
        None => Outcome::count(entries.len(), behind),
    }
}

/// The first thing a refusal names: its first problem, else the text of its first diagnostic.
fn first_refusal(answer: &Value) -> String {
    list(answer, "problems")
        .iter()
        .find_map(Value::as_str)
        .or_else(|| {
            list(answer, "diagnostics")
                .iter()
                .find_map(|d| text(d, "message"))
        })
        .unwrap_or("ess refused them and named no problem")
        .to_string()
}

/// `<code> <subject>` of a problem written `[<code>] <subject>: <message>`; the problem as
/// written when it is not.
fn problem_name(problem: &Value, index: usize) -> String {
    let Some(written) = problem.as_str() else {
        return format!("#{index}");
    };
    let parsed = written.strip_prefix('[').and_then(|rest| {
        let (code, rest) = rest.split_once("] ")?;
        let subject = rest.split_once(": ").map_or(rest, |(subject, _)| subject);
        Some((code, subject))
    });
    match parsed {
        Some((code, subject)) => name_of(Some(code), Some(subject), index),
        None => name_of(None, Some(written), index),
    }
}

/// `<code> <subject>`, or whichever of them is set and not blank; `#<index>` for neither.
fn name_of(code: Option<&str>, subject: Option<&str>, index: usize) -> String {
    fn set(s: Option<&str>) -> Option<&str> {
        s.map(str::trim).filter(|s| !s.is_empty())
    }
    match (set(code), set(subject)) {
        (Some(code), Some(subject)) => format!("{code} {subject}"),
        (Some(one), None) | (None, Some(one)) => one.to_string(),
        (None, None) => format!("#{index}"),
    }
}

fn list<'a>(value: &'a Value, field: &str) -> &'a [Value] {
    value
        .get(field)
        .and_then(Value::as_array)
        .map_or(&[], Vec::as_slice)
}

fn text<'a>(value: &'a Value, field: &str) -> Option<&'a str> {
    value.get(field).and_then(Value::as_str)
}

/// The lines with a marker in the files under the blocks.
fn honesty(pack: &Pack) -> Outcome {
    let blocks = match resolved_inside(&pack.root, &pack.root.join(&pack.manifest.blocks.path)) {
        Ok(blocks) => blocks,
        Err(why) => {
            return Outcome::gap(Some("L0"), format!("the blocks cannot be read: {why}"));
        }
    };
    let mut behind = Vec::new();
    for file in ess_answer::files(&blocks) {
        let Ok(bytes) = std::fs::read(&file) else {
            continue;
        };
        let name = shown(&pack.root, &file);
        for (index, line) in bytes.split(|b| *b == b'\n').enumerate() {
            if line.windows(MARKER.len()).any(|w| w == MARKER) {
                behind.push(Behind::new("marker", format!("{name}:{}", index + 1)));
            }
        }
    }
    Outcome::count(behind.len(), behind)
}

/// The level of the manifest as a number, 0 to 5.
fn adoption_level(manifest: &Manifest) -> Outcome {
    let level = manifest.level();
    match level.trim_start_matches('L').parse::<usize>() {
        Ok(n) => Outcome::count(n, Vec::new()),
        Err(_) => Outcome::gap(
            None,
            format!("metaspec derives no level number from {level}"),
        ),
    }
}
