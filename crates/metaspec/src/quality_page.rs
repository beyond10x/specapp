//! The quality page `metaspec render --quality <report.json>` writes beside index.html:
//! quality.html, from the reports `metaspec quality kpis --json` and `metaspec quality spec
//! --json` wrote (format `metaspec-quality/1`). Render measures nothing itself.
//!
//! One section per report, in the order of the questions (`kpi_catalogue`, then
//! `specification`), whatever the order of the flags: one row per measurement the report holds,
//! with its value or gap, unit, direction and the names behind it, then the report's gaps. A
//! measure the page has never seen is shown like every other. The page follows the rules of
//! render's other pages (one file, CSS inline, no script that runs, no request) and carries each
//! report in a `<script type="application/json" id="metaspec-quality-<question>">`
//! element. With the page written, index.html is written again with a link to it.
//!
//! A report that cannot be read, is no report of the format, is the report of another pack, or
//! repeats a question another report given answers, is refused with exit 1; no quality page is
//! written then, and the index links none.

use crate::page;
use crate::render::{self, Rendered};
use metaspec_core::manifest::Visibility;
use metaspec_core::pack::Pack;
use metaspec_core::quality::report::{self, FORMAT, QualityReport, Question, Reading};
use metaspec_layout::text::esc;
use serde_json::Value;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// The file of the page, beside index.html.
pub const FILE: &str = "quality.html";

/// Why the quality page was not written: the exit code of render and a sentence.
pub struct Refusal {
    pub code: u8,
    pub message: String,
}

impl Refusal {
    fn new(message: String) -> Refusal {
        Refusal {
            code: 1,
            message: format!("quality page: {message}"),
        }
    }
}

/// Writes quality.html into `out` from the given reports, and index.html again with a link to
/// it. Returns one line per report and one for the page.
pub fn write(
    pack: &Pack,
    rendered: &Rendered,
    reports: &[PathBuf],
    out: &Path,
) -> Result<Vec<String>, Refusal> {
    let mut read = Vec::new();
    for path in reports {
        read.push(read_report(pack, path)?);
    }
    read.sort_by_key(|r| rank(r.question));
    if let Some(pair) = read.windows(2).find(|w| w[0].question == w[1].question) {
        return Err(Refusal::new(format!(
            "two reports of the question {}; give one report per question",
            pair[0].question.as_str()
        )));
    }
    let path = out.join(FILE);
    std::fs::write(&path, html(pack, &read))
        .map_err(|e| Refusal::new(format!("{} cannot be written: {e}", path.display())))?;
    render::write_index(pack, rendered, out, Some(FILE)).map_err(Refusal::new)?;
    let mut lines: Vec<String> = read
        .iter()
        .map(|r| {
            let gaps = gap_readings(r);
            format!(
                "quality {}: {} measure(s), {} value(s), {gaps} gap(s)",
                r.question.as_str(),
                r.measurements.len(),
                r.measurements.len() - gaps
            )
        })
        .collect();
    lines.push(format!(
        "render: {FILE} for {} report(s), linked from index.html",
        read.len()
    ));
    Ok(lines)
}

/// Reads one report: JSON, valid against the schema of the format, of this pack.
fn read_report(pack: &Pack, path: &Path) -> Result<QualityReport, Refusal> {
    let shown = path.display();
    let text = std::fs::read_to_string(path)
        .map_err(|e| Refusal::new(format!("{shown} cannot be read: {e}")))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|e| Refusal::new(format!("{shown} is not JSON: {e}")))?;
    report::validate(&value).map_err(|issues| {
        let why: Vec<String> = issues.into_iter().map(|i| i.message).collect();
        Refusal::new(format!(
            "{shown} is no report of format {FORMAT}: {}",
            why.join("; ")
        ))
    })?;
    let read: QualityReport = serde_json::from_value(value)
        .map_err(|e| Refusal::new(format!("{shown} is no report of format {FORMAT}: {e}")))?;
    if read.pack != pack.manifest.pack {
        return Err(Refusal::new(format!(
            "{shown} is the report of the pack {}, not of {}",
            read.pack, pack.manifest.pack
        )));
    }
    Ok(read)
}

/// The place of a question on the page.
fn rank(question: Question) -> u8 {
    match question {
        Question::KpiCatalogue => 0,
        Question::Specification => 1,
    }
}

/// The heading of a question's section.
fn heading(question: Question) -> &'static str {
    match question {
        Question::KpiCatalogue => "KPI catalogue",
        Question::Specification => "Specification",
    }
}

fn gap_readings(r: &QualityReport) -> usize {
    r.measurements
        .iter()
        .filter(|m| matches!(m.reading, Reading::Gap { .. }))
        .count()
}

/// The page of the reports, already in the order of their questions.
fn html(pack: &Pack, reports: &[QualityReport]) -> String {
    let title = &pack.manifest.title;
    let mut h = page::head(&format!("{title} · Quality"));
    h.push_str("<body>\n<header>\n");
    let _ = writeln!(
        h,
        "<p class=\"crumbs\"><a href=\"index.html\">{}</a> / Quality</p>",
        esc(title)
    );
    if pack.manifest.visibility == Visibility::Private {
        h.push_str("<p><span class=\"private\">private pack: not for publication</span></p>\n");
    }
    h.push_str("<h1>Quality</h1>\n<p class=\"note\">Each measure of a report of metaspec quality: its value with the names behind it, or the gap that says why the pack cannot supply one.</p>\n</header>\n<main>\n");
    h.push_str("<nav aria-label=\"Questions\"><ul>");
    for r in reports {
        let _ = write!(
            h,
            "<li><a href=\"#question-{}\">{}</a></li>",
            r.question.as_str(),
            heading(r.question)
        );
    }
    h.push_str("</ul></nav>\n");
    for r in reports {
        section(&mut h, r);
    }
    h.push_str("</main>\n");
    for r in reports {
        let data = serde_json::to_string(r)
            .unwrap_or_default()
            .replace('<', "\\u003c");
        let _ = writeln!(
            h,
            "<script type=\"application/json\" id=\"metaspec-quality-{}\">{data}</script>",
            r.question.as_str()
        );
    }
    h.push_str("</body>\n</html>\n");
    h
}

/// The section of one report.
fn section(h: &mut String, r: &QualityReport) {
    let q = r.question.as_str();
    let gaps = gap_readings(r);
    let _ = writeln!(
        h,
        "<section id=\"question-{q}\" aria-labelledby=\"question-{q}-h\"><h2 id=\"question-{q}-h\">{}</h2>",
        heading(r.question)
    );
    let _ = writeln!(
        h,
        "<p class=\"meta\">{} measure(s): {} value(s), {gaps} gap(s). Sources <code>{}</code>.</p>",
        r.measurements.len(),
        r.measurements.len() - gaps,
        esc(&r.source_digest)
    );
    h.push_str("<div class=\"table-wrap\"><table><thead><tr><th>Measure</th><th>Reading</th><th>Unit</th><th>Direction</th><th>Behind</th></tr></thead><tbody>\n");
    for m in &r.measurements {
        let reading = match &m.reading {
            Reading::Value { value } => value.to_string(),
            Reading::Gap { gap } => {
                let needs = r
                    .gaps
                    .iter()
                    .find(|g| g.kind == "measure" && g.feature == m.measure)
                    .and_then(|g| g.needs.as_deref())
                    .map(|level| format!(" (needs {})", esc(level)))
                    .unwrap_or_default();
                format!("<span class=\"badge note\">gap</span> {}{needs}", esc(gap))
            }
        };
        let _ = writeln!(
            h,
            "<tr id=\"measure-{id}\"><td><code>{id}</code></td><td>{reading}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            esc(&m.unit),
            esc(&m.direction.replace('_', " ")),
            behind(m),
            id = esc(&m.measure)
        );
    }
    h.push_str("</tbody></table></div>\n");
    if !r.gaps.is_empty() {
        let _ = writeln!(
            h,
            "<h3 id=\"question-{q}-gaps\">Gaps</h3><div class=\"table-wrap\"><table><thead><tr><th>Feature</th><th>Kind</th><th>Needs</th><th>Summary</th></tr></thead><tbody>"
        );
        for g in &r.gaps {
            let _ = writeln!(
                h,
                "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td>{}</td></tr>",
                esc(&g.feature),
                esc(&g.kind),
                esc(g.needs.as_deref().unwrap_or("metaspec's own")),
                esc(&g.summary)
            );
        }
        h.push_str("</tbody></table></div>\n");
    }
    h.push_str("</section>\n");
}

/// The names behind a value, by kind in the order the report first names each kind.
fn behind(m: &report::Measurement) -> String {
    if matches!(m.reading, Reading::Gap { .. }) {
        return String::new();
    }
    if m.behind.is_empty() {
        return String::from("none");
    }
    let mut kinds: Vec<(&str, Vec<&str>)> = Vec::new();
    for b in &m.behind {
        match kinds.iter_mut().find(|(k, _)| *k == b.kind) {
            Some((_, names)) => names.push(&b.name),
            None => kinds.push((&b.kind, vec![&b.name])),
        }
    }
    kinds
        .iter()
        .map(|(kind, names)| {
            let names: Vec<String> = names
                .iter()
                .map(|n| format!("<code>{}</code>", esc(n)))
                .collect();
            format!("{} ({}): {}", esc(kind), names.len(), names.join(", "))
        })
        .collect::<Vec<_>>()
        .join("<br>")
}
