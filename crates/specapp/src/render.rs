//! `specapp render`: one self-contained HTML page per document of a `parts-links/1` class,
//! with each view of the class drawn, the findings of the pack's tool, and an index page.
//!
//! Each page carries its data as one JSON document, format `specapp-document/1`, in a
//! `<script type="application/json" id="specapp-data">` element: the graph, the check, the
//! findings with the views they are drawn in, and the scenes. The data is built by
//! `specapp_docs::document`, which the docs build shares.

use crate::page;
use specapp_core::pack::Pack;
use std::path::Path;
use std::time::Duration;

pub use specapp_docs::document::{DocumentPage, Drawn as Rendered, state_word};

/// Builds the pages of a loaded pack: reads every document of a `parts-links/1` class, runs
/// the tool's check where the pack is trusted, and lays out each view. A page's file is
/// `<class>/<file stem>.html`.
pub fn build(pack: &Pack, state: Option<&Path>, deadline: Duration) -> Rendered {
    let mut rendered =
        specapp_docs::document::build(pack, state, deadline, &|class: &str| class.to_string());
    for class in &mut rendered.classes {
        for p in &mut class.pages {
            p.page = format!("{}.html", p.page);
        }
    }
    rendered
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
