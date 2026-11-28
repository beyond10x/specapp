//! The quality page `metaspec render --quality <report.json>` writes beside index.html:
//! quality.html, from the reports `metaspec quality kpis --json` and `metaspec quality spec
//! --json` wrote. A stub, filled by `story:kpi-quality-page`; until then render exits 2 with
//! "not built yet" after its other pages.

use crate::render::Rendered;
use metaspec_core::pack::Pack;
use metaspec_core::quality::report::NOT_BUILT;
use std::path::{Path, PathBuf};

/// Why the quality page was not written: the exit code of render and a sentence.
pub struct Refusal {
    pub code: u8,
    pub message: String,
}

/// Writes quality.html into `out` from the given reports. Returns one line per page written.
pub fn write(
    _pack: &Pack,
    _rendered: &Rendered,
    _reports: &[PathBuf],
    _out: &Path,
) -> Result<Vec<String>, Refusal> {
    Err(Refusal {
        code: 2,
        message: format!("quality page: {NOT_BUILT}"),
    })
}
