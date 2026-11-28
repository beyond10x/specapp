//! The adapter of a verb: what the manifest declares about a pack's tool, applied to one run.
//!
//! A pack's tool answers in the pack's own format. The manifest says where in that answer the
//! result of a document is, how it says valid, which lists hold findings and how one entry maps
//! to a finding (`crate::manifest::CheckAdapter`). This module applies that mapping; it knows
//! no pack.

use crate::graph::Graph;
use crate::manifest::{
    CheckAdapter, FindingList, Gate, SubjectKind, SubjectMap, SubjectRule, Test, Unknown,
};
use crate::tool::{FINDINGS_FORMAT, Finding, Findings, findings as validate_findings};
use serde_json::Value;
use std::collections::BTreeMap;

/// What one run of the tool came to, before the adapter reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exit {
    /// The exit code; none when a signal ended the run.
    pub code: Option<i32>,
    pub stdout: Vec<u8>,
    /// The first line of stderr, for a message.
    pub stderr: String,
}

/// What the adapter read from a run.
#[derive(Debug, Clone, PartialEq)]
pub enum Mapped {
    /// The result of the document, in `specapp-findings/1`.
    Findings(Findings),
    /// The tool refused the document with this text (the adapter's `error`).
    Refused(String),
    /// The answer is not what the adapter declares.
    Failed(String),
}

/// Reads a run of the tool's check with the manifest's adapter. `graph` is the document as
/// specapp drew it; a subject rule with `unknown: document` looks its ids up in it.
pub fn check(adapter: &CheckAdapter, exit: &Exit, graph: Option<&Graph>) -> Mapped {
    let answer: Option<Value> = serde_json::from_slice(&exit.stdout).ok();
    let result = answer.as_ref().and_then(|a| a.pointer(&adapter.at));
    if let (Some(result), Some(error)) = (result, &adapter.error)
        && let Some(text) = result.pointer(error).and_then(Value::as_str)
    {
        return Mapped::Refused(text.to_string());
    }
    let code = exit
        .code
        .map_or(String::from("a signal"), |c| format!("exit {c}"));
    if !exit.code.is_some_and(|c| adapter.exit_codes().contains(&c)) {
        let tail = if exit.stderr.is_empty() {
            String::new()
        } else {
            format!(": {}", exit.stderr)
        };
        return Mapped::Failed(format!(
            "the tool ended with {code}, which the adapter names as no answer{tail}"
        ));
    }
    let Some(result) = result else {
        let why = if answer.is_none() {
            String::from("no JSON on stdout")
        } else {
            format!("nothing at {:?}", adapter.at)
        };
        return Mapped::Failed(format!("the tool ended with {code} and {why}"));
    };
    match read(adapter, result, graph) {
        Ok(findings) => {
            match validate_findings(&serde_json::to_value(&findings).unwrap_or_default()) {
                Ok(findings) => Mapped::Findings(findings),
                Err(why) => Mapped::Failed(why),
            }
        }
        Err(why) => Mapped::Failed(why),
    }
}

fn read(adapter: &CheckAdapter, result: &Value, graph: Option<&Graph>) -> Result<Findings, String> {
    let mut findings = Vec::new();
    for list in &adapter.findings {
        let entries = match result.pointer(&list.at) {
            None | Some(Value::Null) => continue,
            Some(Value::Array(entries)) => entries,
            Some(_) => return Err(format!("the answer has no list at {:?}", list.at)),
        };
        for (n, entry) in entries.iter().enumerate() {
            findings.push(
                finding(list, entry, graph)
                    .map_err(|why| format!("entry {n} of the list at {:?}: {why}", list.at))?,
            );
        }
    }
    let valid = match &adapter.valid {
        Some(test) => truth(test, result)?,
        None => !findings.iter().any(|f| f.gate),
    };
    let counts = match &adapter.counts {
        None => None,
        Some(at) => match result.pointer(at) {
            None | Some(Value::Null) => None,
            Some(Value::Object(map)) => {
                let mut counts = BTreeMap::new();
                for (rule, n) in map {
                    let n = n
                        .as_u64()
                        .ok_or_else(|| format!("the count of {rule} at {at:?} is no count"))?;
                    counts.insert(rule.clone(), n);
                }
                Some(counts)
            }
            Some(_) => return Err(format!("the answer has no mapping of counts at {at:?}")),
        },
    };
    Ok(Findings {
        format: String::from(FINDINGS_FORMAT),
        valid,
        findings,
        counts,
    })
}

fn finding(list: &FindingList, entry: &Value, graph: Option<&Graph>) -> Result<Finding, String> {
    let rule = template(&list.rule, entry)?;
    let gate = match &list.gate {
        Gate::Always(gate) => *gate,
        Gate::When(test) => truth(test, entry)?,
    };
    let message = template(&list.message, entry)?;
    let (subject, message) = match &list.subject {
        SubjectMap::Document(_) => (String::from("document"), message),
        SubjectMap::Rule(rule) => subject(rule, entry, message, graph)?,
    };
    Ok(Finding {
        rule,
        gate,
        subject,
        message,
    })
}

fn subject(
    rule: &SubjectRule,
    entry: &Value,
    message: String,
    graph: Option<&Graph>,
) -> Result<(String, String), String> {
    let id = template(&rule.id, entry)?;
    let kind = if id.is_empty() {
        SubjectKind::Document
    } else if let Some(kind) = &rule.kind {
        let value = template(kind, entry)?;
        rule.kinds
            .get(&value)
            .copied()
            .unwrap_or(SubjectKind::Document)
    } else if let Some(text) = &rule.link_when_contains {
        if id.contains(text.as_str()) {
            SubjectKind::Link
        } else {
            SubjectKind::Part
        }
    } else {
        SubjectKind::Part
    };
    let known = |kind: SubjectKind| match (graph, kind) {
        (Some(g), SubjectKind::Part) => g.parts.iter().any(|p| p.id == id),
        (Some(g), SubjectKind::Link) => g.links.iter().any(|l| l.id == id),
        _ => true,
    };
    if rule.unknown == Unknown::Document && kind != SubjectKind::Document && !known(kind) {
        let named = graph.is_some_and(|g| g.name == id);
        let message = if named {
            message
        } else {
            format!("{id}: {message}")
        };
        return Ok((String::from("document"), message));
    }
    Ok(match kind {
        SubjectKind::Part => (format!("part:{id}"), message),
        SubjectKind::Link => (format!("link:{id}"), message),
        SubjectKind::Document => (String::from("document"), message),
    })
}

/// The truth of a test: the boolean at its pointer, or whether the value there equals
/// `equals`.
fn truth(test: &Test, value: &Value) -> Result<bool, String> {
    let found = value.pointer(&test.at);
    match (&test.equals, found) {
        (Some(want), found) => Ok(found == Some(want)),
        (None, Some(Value::Bool(b))) => Ok(*b),
        (None, _) => Err(format!("no true or false at {:?}", test.at)),
    }
}

/// Fills a template: `{<JSON pointer>}` stands for the text at that pointer in `value` (a
/// number or a boolean is written out); any other character is kept.
pub fn template(text: &str, value: &Value) -> Result<String, String> {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("{/") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let Some(end) = after.find('}') else {
            return Err(format!("the template {text:?} has an open brace"));
        };
        let pointer = &after[..end];
        match value.pointer(pointer) {
            Some(Value::String(s)) => out.push_str(s),
            Some(Value::Number(n)) => out.push_str(&n.to_string()),
            Some(Value::Bool(b)) => out.push_str(if *b { "true" } else { "false" }),
            _ => return Err(format!("no text at {pointer:?}")),
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    Ok(out)
}
