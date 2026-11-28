//! `metaspec render`: one self-contained HTML page per document of a `parts-links/1` class,
//! with each view of the class drawn, the findings of the pack's tool, and an index page.
//!
//! Each page carries its data as one JSON document, format `metaspec-document/1`, in a
//! `<script type="application/json" id="metaspec-data">` element: the graph, the check, the
//! findings with the views they are drawn in, and the scenes. The data is built by
//! `metaspec_docs::document`, which the docs build shares.

use crate::{page, quality_page};
use metaspec_core::pack::Pack;
use std::path::Path;
use std::time::Duration;

pub use metaspec_docs::document::{DocumentPage, Drawn as Rendered, state_word};

/// Builds the pages of a loaded pack: reads every document of a `parts-links/1` class, runs
/// the tool's check where the pack is trusted, and lays out each view. A page's file is
/// `<class>/<file stem>.html`.
pub fn build(pack: &Pack, state: Option<&Path>, deadline: Duration) -> Rendered {
    let mut rendered =
        metaspec_docs::document::build(pack, state, deadline, &|class: &str| class.to_string());
    for class in &mut rendered.classes {
        for p in &mut class.pages {
            p.page = format!("{}.html", p.page);
        }
    }
    rendered
}

/// Writes the pages and the index into `out`. Returns one line per page for the terminal.
///
/// The quality page of an earlier render into `out` is removed first: the quality page is
/// written after these pages only from the reports of this render, so no page with an earlier
/// report's numbers stays beside an index that does not link it.
pub fn write(pack: &Pack, rendered: &Rendered, out: &Path) -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    std::fs::create_dir_all(out).map_err(|e| format!("{} cannot be made: {e}", out.display()))?;
    let quality = out.join(quality_page::FILE);
    match std::fs::remove_file(&quality) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
            return Err(format!("{} cannot be removed: {e}", quality.display()));
        }
        _ => {}
    }
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
    write_index(pack, rendered, out, None)?;
    Ok(lines)
}

/// Writes index.html into `out`; with `quality`, the file of the quality page, it links it.
pub fn write_index(
    pack: &Pack,
    rendered: &Rendered,
    out: &Path,
    quality: Option<&str>,
) -> Result<(), String> {
    let index = out.join("index.html");
    std::fs::write(&index, page::index(pack, rendered, quality))
        .map_err(|e| format!("{} cannot be written: {e}", index.display()))
}
