//! `specapp render`: one self-contained HTML page per document of a `parts-links/1` class,
//! with each view of the class drawn, the findings of the pack's tool, and an index page.
//!
//! Each page carries its data as one JSON document, format `specapp-document/1`, in a
//! `<script type="application/json" id="specapp-data">` element: the graph, the check, the
//! findings with the views they are drawn in, and the scenes.

use crate::page;
use serde::Serialize;
use specapp_core::graph::{self, Graph};
use specapp_core::manifest::Shape;
use specapp_core::pack::Pack;
use specapp_core::tool::{self, Check, CheckState, Subject};
use specapp_layout::scene::Scene;
use specapp_layout::{Style, layout, views_of};
use std::collections::BTreeSet;
use std::path::Path;
use std::time::Duration;

/// The format of a page's data.
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
    /// The file of the page, relative to the output directory.
    pub page: String,
    pub check: Check,
    pub findings: Vec<Row>,
    pub graph: Graph,
    pub scenes: Vec<Scene>,
}

/// One document class on the index page.
pub struct ClassIndex {
    pub class: String,
    pub title: String,
    pub shape: Shape,
    pub documents: usize,
    pub pages: Vec<DocumentPage>,
}

/// Everything one render writes.
pub struct Rendered {
    pub classes: Vec<ClassIndex>,
}

/// Builds the pages of a loaded pack: reads every document of a `parts-links/1` class, runs
/// the tool's check where the pack is trusted, and lays out each view.
pub fn build(pack: &Pack, state: Option<&Path>, deadline: Duration) -> Rendered {
    let style = Style::of_pack(pack);
    let mut classes = Vec::new();
    for (class, documents) in pack.manifest.documents.iter().zip(&pack.documents) {
        let mut pages = Vec::new();
        let mut files: BTreeSet<String> = BTreeSet::new();
        if class.shape == Shape::PartsLinks {
            for document in documents {
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
                let stem = file_stem(&document.path);
                let mut file = format!("{}/{stem}.html", class.class);
                let mut n = 1;
                while files.contains(&file) {
                    n += 1;
                    file = format!("{}/{stem}-{n}.html", class.class);
                }
                files.insert(file.clone());
                pages.push(DocumentPage {
                    format: FORMAT,
                    pack: pack.manifest.pack.clone(),
                    title: pack.manifest.title.clone(),
                    visibility: match pack.manifest.visibility {
                        specapp_core::manifest::Visibility::Public => "public",
                        specapp_core::manifest::Visibility::Private => "private",
                    },
                    class: class.class.clone(),
                    class_title: class.title.clone(),
                    document: document.path.clone(),
                    name: graph.name.clone(),
                    page: file,
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
    Rendered { classes }
}

fn file_stem(path: &str) -> String {
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

/// Writes the pages and the index into `out`. Returns one line per page for the terminal.
pub fn write(pack: &Pack, rendered: &Rendered, out: &Path) -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    std::fs::create_dir_all(out).map_err(|e| format!("{} cannot be made: {e}", out.display()))?;
    for class in &rendered.classes {
        for p in &class.pages {
            let path = out.join(&p.page);
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)
                    .map_err(|e| format!("{} cannot be made: {e}", dir.display()))?;
            }
            std::fs::write(&path, page::document(p))
                .map_err(|e| format!("{} cannot be written: {e}", path.display()))?;
            let drawn = p.findings.iter().filter(|f| !f.drawn_in.is_empty()).count();
            lines.push(format!(
                "{} {}: {} view(s), check {}, {} finding(s), {} drawn",
                p.class,
                p.document,
                p.scenes.len(),
                state_word(p.check.state),
                p.findings.len(),
                drawn
            ));
        }
    }
    let index = out.join("index.html");
    std::fs::write(&index, page::index(pack, rendered))
        .map_err(|e| format!("{} cannot be written: {e}", index.display()))?;
    Ok(lines)
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
