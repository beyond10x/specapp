//! The data of one drawn document, format `specapp-document/1`: the graph, the check of the
//! pack's tool, the findings with the views they are drawn in, and the scenes. `specapp render`
//! writes it into its HTML pages; `specapp docs build` into its generated pages.

use serde::Serialize;
use specapp_core::graph::{self, Graph};
use specapp_core::manifest::{Shape, Visibility};
use specapp_core::pack::Pack;
use specapp_core::tool::{self, Check, CheckState, Subject};
use specapp_layout::scene::Scene;
use specapp_layout::{Style, layout, views_of};
use std::collections::BTreeSet;
use std::path::Path;
use std::time::Duration;

/// The format of a document's data.
pub const FORMAT: &str = "specapp-document/1";

/// One finding as a page lists it.
#[derive(Debug, Clone, Serialize)]
pub struct Row {
    /// Its number, from 1, as its marks show it.
    pub n: usize,
    pub rule: String,
    pub gate: bool,
    pub subject: String,
    pub message: String,
    /// The views whose scene has a mark of it.
    pub drawn_in: Vec<String>,
}

/// The data of one document's page.
#[derive(Debug, Clone, Serialize)]
pub struct DocumentPage {
    pub format: &'static str,
    pub pack: String,
    pub title: String,
    pub visibility: &'static str,
    pub class: String,
    pub class_title: String,
    pub document: String,
    pub name: String,
    /// The file of the page relative to the output directory, without its extension.
    pub page: String,
    pub check: Check,
    pub findings: Vec<Row>,
    pub graph: Graph,
    pub scenes: Vec<Scene>,
}

/// One document class with the pages of its drawn documents.
pub struct ClassIndex {
    pub class: String,
    pub title: String,
    pub shape: Shape,
    pub documents: usize,
    pub pages: Vec<DocumentPage>,
}

/// Every drawn document of a pack.
pub struct Drawn {
    pub classes: Vec<ClassIndex>,
}

/// The word of a pack's visibility.
pub fn visibility(pack: &Pack) -> &'static str {
    match pack.manifest.visibility {
        Visibility::Public => "public",
        Visibility::Private => "private",
    }
}

/// The file stem of a path, with every character but letters, digits, `-` and `_` as `-`.
pub fn file_stem(path: &str) -> String {
    let file = path.rsplit('/').next().unwrap_or(path);
    let stem = file.rsplit_once('.').map_or(file, |(s, _)| s);
    stem.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

/// The page names of the documents of one class: `<dir>/<stem>`, `-<n>` appended to a repeat.
pub fn page_names(dir: &str, paths: &[&str]) -> Vec<String> {
    let mut taken: BTreeSet<String> = BTreeSet::new();
    let mut out = Vec::new();
    for path in paths {
        let stem = file_stem(path);
        let mut name = format!("{dir}/{stem}");
        let mut n = 1;
        while taken.contains(&name) || stem == "index" && n == 1 {
            n += 1;
            name = format!("{dir}/{stem}-{n}");
        }
        taken.insert(name.clone());
        out.push(name);
    }
    out
}

/// Reads every document of a `parts-links/1` class, runs the tool's check where the pack is
/// trusted, and lays out each view. `dir_of` names the directory of a class's pages.
pub fn build(
    pack: &Pack,
    state: Option<&Path>,
    deadline: Duration,
    dir_of: &dyn Fn(&str) -> String,
) -> Drawn {
    let style = Style::of_pack(pack);
    let mut classes = Vec::new();
    for (class, documents) in pack.manifest.documents.iter().zip(&pack.documents) {
        let mut pages = Vec::new();
        if class.shape == Shape::PartsLinks {
            let readable: Vec<_> = documents.iter().filter(|d| d.value.is_some()).collect();
            let paths: Vec<&str> = readable.iter().map(|d| d.path.as_str()).collect();
            let names = page_names(&dir_of(&class.class), &paths);
            for (document, page) in readable.into_iter().zip(names) {
                let Some(value) = &document.value else {
                    continue;
                };
                let graph = graph::read(class, &document.path, value);
                let check = tool::check(pack, &document.path, Some(&graph), state, deadline);
                let findings = check.findings().to_vec();
                let scenes: Vec<Scene> = views_of(class, &graph)
                    .iter()
                    .map(|view| layout(&graph, view, &style, &findings))
                    .collect();
                let rows = findings
                    .iter()
                    .enumerate()
                    .map(|(i, f)| Row {
                        n: i + 1,
                        rule: f.rule.clone(),
                        gate: f.gate,
                        subject: f.subject.clone(),
                        message: f.message.clone(),
                        drawn_in: scenes
                            .iter()
                            .filter(|s| match f.subject() {
                                Subject::Document => false,
                                _ => s.marks.iter().any(|m| m.finding == i + 1),
                            })
                            .map(|s| s.view.id.clone())
                            .collect(),
                    })
                    .collect();
                pages.push(DocumentPage {
                    format: FORMAT,
                    pack: pack.manifest.pack.clone(),
                    title: pack.manifest.title.clone(),
                    visibility: visibility(pack),
                    class: class.class.clone(),
                    class_title: class.title.clone(),
                    document: document.path.clone(),
                    name: graph.name.clone(),
                    page,
                    check,
                    findings: rows,
                    graph,
                    scenes,
                });
            }
        }
        classes.push(ClassIndex {
            class: class.class.clone(),
            title: class.title.clone(),
            shape: class.shape,
            documents: documents.len(),
            pages,
        });
    }
    Drawn { classes }
}

/// The state of a check in one word, as the terminal and the pages say it.
pub fn state_word(state: CheckState) -> &'static str {
    match state {
        CheckState::Ran => "ran",
        CheckState::NoTool => "no_tool",
        CheckState::NoCheck => "no_check",
        CheckState::NotTrusted => "not_trusted",
        CheckState::TimedOut => "timed_out",
        CheckState::Refused => "refused",
        CheckState::Failed => "failed",
    }
}

/// The check of a page in one sentence, and whether it is bad news.
pub fn check_sentence(p: &DocumentPage) -> (String, bool) {
    let gated = p.findings.iter().filter(|f| f.gate).count();
    let other = p.findings.len() - gated;
    match (p.check.state, &p.check.result) {
        (CheckState::Ran, Some(result)) if result.valid => (
            format!(
                "Valid. The pack's tool reports {} finding(s), {gated} gated, {other} not gated.",
                p.findings.len()
            ),
            false,
        ),
        (CheckState::Ran, _) => (
            format!(
                "Not valid. The pack's tool reports {} finding(s), {gated} gated, {other} not gated.",
                p.findings.len()
            ),
            true,
        ),
        (CheckState::TimedOut, _) => (
            format!(
                "Not checked: {}.",
                p.check.message.as_deref().unwrap_or("the check timed out")
            ),
            true,
        ),
        (state, _) => (
            format!(
                "Not checked ({}): {}.",
                state_word(state),
                p.check.message.as_deref().unwrap_or("")
            ),
            matches!(state, CheckState::Refused | CheckState::Failed),
        ),
    }
}
