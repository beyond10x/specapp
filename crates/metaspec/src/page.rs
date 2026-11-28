//! The HTML of the pages `metaspec render` writes: self-contained (CSS and SVG inline, no font,
//! no script, no request), both themes by `prefers-color-scheme`, readable without colour.

use crate::render::{DocumentPage, Rendered, state_word};
use metaspec_core::manifest::Shape;
use metaspec_core::pack::Pack;
use metaspec_core::tool::CheckState;
use metaspec_docs::document::check_sentence;
use metaspec_layout::text::{esc, value};
use metaspec_layout::{icons, palette, svg};
use std::fmt::Write as _;

const CSS: &str = "\
*{box-sizing:border-box}
html{background:var(--bg);color:var(--fg)}
body{margin:0 auto;max-width:1240px;padding:16px 20px 48px;font:15px/1.5 system-ui,-apple-system,'Segoe UI',Roboto,sans-serif;overflow-wrap:break-word}
a{color:var(--link)}
h1{font-size:26px;margin:4px 0}
h2{font-size:19px;margin:28px 0 8px}
td code{overflow-wrap:anywhere}
code{font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;font-size:13px}
.crumbs,.path,.meta,.note{color:var(--muted);margin:2px 0}
.private{display:inline-block;border:1.5px solid var(--finding);color:var(--fg);border-radius:4px;padding:0 6px;font-size:13px;font-weight:600}
.state{border-left:4px solid var(--line);padding:4px 10px;margin:6px 0}
.state.bad{border-left-color:var(--finding)}
nav ul{list-style:none;padding:0;margin:8px 0;display:flex;flex-wrap:wrap;gap:6px 16px}
figure.drawing{margin:8px 0;overflow-x:auto;border:1px solid var(--line);border-radius:8px;padding:4px}
ul.legend{list-style:none;padding:0;margin:6px 0;display:flex;flex-wrap:wrap;gap:4px 18px;font-size:13px}
ul.legend svg{vertical-align:middle;margin-right:6px}
.swatch{fill:var(--fill);stroke:var(--stroke);stroke-width:1.5}
.table-wrap{overflow-x:auto}
table{border-collapse:collapse;width:100%;font-size:14px}
th,td{text-align:left;vertical-align:top;padding:4px 8px;border-bottom:1px solid var(--line)}
th{font-weight:600}
.badge{display:inline-block;min-width:26px;text-align:center;border-radius:8px;padding:0 6px;font-size:12px;font-weight:700;background:var(--finding);color:var(--finding-fg)}
.badge.note{background:var(--bg);color:var(--fg);border:1.5px solid var(--finding)}
tr:target td{outline:2px solid var(--link)}
";

/// The head of every page: the title, the palette of both themes and the CSS, inline.
pub(crate) fn head(title: &str) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\n<title>{}</title>\n<style>\n{}{}{}</style>\n</head>\n",
        esc(title),
        palette::css(),
        CSS,
        svg::CSS
    )
}

fn swatch(colour: &str, icon: Option<&str>) -> String {
    let mut out = format!(
        "<svg width=\"22\" height=\"18\" viewBox=\"0 0 22 18\" aria-hidden=\"true\" class=\"c-{}\"><rect class=\"swatch\" x=\"1\" y=\"1\" width=\"20\" height=\"16\" rx=\"4\"/>",
        esc(colour)
    );
    if let Some(name) = icon {
        match icons::path(name) {
            Some(d) => {
                let _ = write!(
                    out,
                    "<path style=\"fill:none;stroke:var(--stroke);stroke-width:1.4\" transform=\"translate(4 2)\" d=\"{d}\"/>"
                );
            }
            None => {
                let letter = name.chars().take(1).collect::<String>().to_uppercase();
                let _ = write!(
                    out,
                    "<text x=\"11\" y=\"13\" text-anchor=\"middle\" style=\"fill:var(--stroke);font-size:10px;font-weight:700\">{}</text>",
                    esc(&letter)
                );
            }
        }
    }
    out.push_str("</svg>");
    out
}

/// The page of one document.
pub fn document(p: &DocumentPage) -> String {
    let mut h = head(&format!("{} · {} · {}", p.name, p.class_title, p.title));
    h.push_str("<body>\n<header>\n");
    let _ = writeln!(
        h,
        "<p class=\"crumbs\"><a href=\"../index.html\">{}</a> / {}</p>",
        esc(&p.title),
        esc(&p.class_title)
    );
    if p.visibility == "private" {
        h.push_str("<p><span class=\"private\">private pack: not for publication</span></p>\n");
    }
    let _ = writeln!(
        h,
        "<h1>{}</h1>\n<p class=\"path\"><code>{}</code></p>\n</header>\n<main>",
        esc(&p.name),
        esc(&p.document)
    );

    let (sentence, bad) = check_sentence(p);
    let _ = writeln!(
        h,
        "<section id=\"check\" aria-labelledby=\"check-h\"><h2 id=\"check-h\">Check</h2><p class=\"state{}\">{}</p><p class=\"note\">A finding is a numbered mark on the part or link it names (filled: gated; outlined: not gated) and a row of the list below.</p></section>",
        if bad { " bad" } else { "" },
        esc(&sentence)
    );

    h.push_str("<nav aria-label=\"Views\"><ul>");
    for s in &p.scenes {
        let _ = write!(
            h,
            "<li><a href=\"#view-{}\">{}</a></li>",
            esc(&s.view.id),
            esc(&s.view.title)
        );
    }
    h.push_str("<li><a href=\"#findings\">Findings</a></li><li><a href=\"#parts\">Parts</a></li><li><a href=\"#links\">Links</a></li></ul></nav>\n");

    for s in &p.scenes {
        let _ = writeln!(
            h,
            "<section class=\"view\" id=\"view-{id}\" aria-labelledby=\"view-{id}-h\"><h2 id=\"view-{id}-h\">{}</h2>",
            esc(&s.view.title),
            id = esc(&s.view.id)
        );
        let edges = s.edges.len();
        let boxes = s.boxes.iter().filter(|b| !b.ghost).count();
        let ghosts = s.boxes.len() - boxes;
        let mut meta = format!(
            "Layout {}: {boxes} part(s), {edges} link(s), {} finding mark(s).",
            s.view.layout,
            s.marks.len()
        );
        if ghosts > 0 {
            meta.push_str(&format!(
                " {ghosts} dashed box(es) for names that links give and no part has."
            ));
        }
        if s.view.implicit {
            meta.push_str(
                " metaspec adds this view because the views of the class leave a part or link out.",
            );
        }
        let _ = writeln!(h, "<p class=\"meta\">{}</p>", esc(&meta));
        let _ = writeln!(
            h,
            "<figure class=\"drawing\">{}</figure>",
            svg::svg(s, &format!("v-{}", s.view.id))
        );
        h.push_str("<ul class=\"legend\" aria-label=\"Legend\">");
        for entry in &s.legend {
            let _ = write!(
                h,
                "<li>{}<strong>{}</strong>{}</li>",
                swatch(&entry.colour, entry.icon.as_deref()),
                esc(&entry.label),
                if entry.blocks.is_empty() {
                    String::new()
                } else {
                    format!(": {}", esc(&entry.blocks.join(", ")))
                }
            );
        }
        h.push_str("</ul>\n</section>\n");
    }

    h.push_str("<section id=\"findings\" aria-labelledby=\"findings-h\"><h2 id=\"findings-h\">Findings</h2>\n");
    if p.findings.is_empty() {
        h.push_str("<p>None.</p>\n");
    } else {
        h.push_str("<div class=\"table-wrap\"><table><thead><tr><th>#</th><th>Rule</th><th>Gate</th><th>Subject</th><th>Message</th><th>Drawn in</th></tr></thead><tbody>\n");
        for f in &p.findings {
            let drawn = if f.drawn_in.is_empty() {
                if f.subject == "document" {
                    String::from("the whole document")
                } else {
                    String::from("not drawn: no part or link of this id")
                }
            } else {
                f.drawn_in
                    .iter()
                    .map(|v| {
                        let title = p
                            .scenes
                            .iter()
                            .find(|s| &s.view.id == v)
                            .map_or(v.as_str(), |s| s.view.title.as_str());
                        format!("<a href=\"#view-{}\">{}</a>", esc(v), esc(title))
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let _ = writeln!(
                h,
                "<tr id=\"finding-{n}\"><td><span class=\"badge{}\">!{n}</span></td><td><code>{}</code></td><td>{}</td><td><code>{}</code></td><td>{}</td><td>{drawn}</td></tr>",
                if f.gate { "" } else { " note" },
                esc(&f.rule),
                if f.gate { "gate" } else { "not gated" },
                esc(&f.subject),
                esc(&f.message),
                n = f.n
            );
        }
        h.push_str("</tbody></table></div>\n");
    }
    h.push_str("</section>\n");

    h.push_str("<section id=\"parts\" aria-labelledby=\"parts-h\"><h2 id=\"parts-h\">Parts</h2><div class=\"table-wrap\"><table><thead><tr><th>Part</th><th>Block</th><th>Contained</th><th>Fields</th></tr></thead><tbody>\n");
    for part in &p.graph.parts {
        let contained = match (&part.inside, &part.per) {
            (Some(i), _) => format!("in {i}"),
            (_, Some(per)) => format!("per {per}"),
            _ => String::new(),
        };
        let fields: Vec<String> = part
            .fields
            .iter()
            .map(|(k, v)| format!("{k}: {}", value(v)))
            .collect();
        let _ = writeln!(
            h,
            "<tr><td><code>{}</code>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            esc(&part.id),
            if part.on_border {
                " (on the border)"
            } else {
                ""
            },
            esc(part.block.as_deref().unwrap_or("")),
            esc(&contained),
            esc(&fields.join("; "))
        );
    }
    h.push_str("</tbody></table></div></section>\n");

    h.push_str("<section id=\"links\" aria-labelledby=\"links-h\"><h2 id=\"links-h\">Links</h2><div class=\"table-wrap\"><table><thead><tr><th>Link</th><th>From</th><th>To</th><th>Carries</th><th>Fields</th></tr></thead><tbody>\n");
    for link in &p.graph.links {
        let fields: Vec<String> = link
            .fields
            .iter()
            .map(|(k, v)| format!("{k}: {}", value(v)))
            .collect();
        let _ = writeln!(
            h,
            "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            esc(&link.id),
            esc(link.from.as_deref().unwrap_or("")),
            esc(link.to.as_deref().unwrap_or("")),
            esc(link.carries.as_deref().unwrap_or("")),
            esc(&fields.join("; "))
        );
    }
    h.push_str("</tbody></table></div></section>\n");

    let mut marked: Vec<(String, String, String)> = Vec::new();
    for part in &p.graph.parts {
        for m in &part.markers {
            marked.push((
                format!("part {} {}", part.id, m.path),
                value(&m.value),
                value(&m.vary),
            ));
        }
    }
    for link in &p.graph.links {
        for m in &link.markers {
            marked.push((
                format!("link {} {}", link.id, m.path),
                value(&m.value),
                value(&m.vary),
            ));
        }
    }
    for m in &p.graph.markers {
        marked.push((m.path.clone(), value(&m.value), value(&m.vary)));
    }
    if !marked.is_empty() {
        h.push_str("<section id=\"marked\" aria-labelledby=\"marked-h\"><h2 id=\"marked-h\">Marked places</h2><div class=\"table-wrap\"><table><thead><tr><th>Place</th><th>Value</th><th>Varies over</th></tr></thead><tbody>\n");
        for (place, v, vary) in &marked {
            let _ = writeln!(
                h,
                "<tr><td><code>{}</code></td><td>{}</td><td>{}</td></tr>",
                esc(place),
                esc(v),
                esc(vary)
            );
        }
        h.push_str("</tbody></table></div></section>\n");
    }

    h.push_str("</main>\n");
    let data = serde_json::to_string(p)
        .unwrap_or_default()
        .replace('<', "\\u003c");
    let _ = writeln!(
        h,
        "<script type=\"application/json\" id=\"metaspec-data\">{data}</script>"
    );
    h.push_str("</body>\n</html>\n");
    h
}

/// The index page: every class, and for a drawn class every document with its check. With
/// `quality`, the file of the quality page, one line more links it; without, the index is the
/// same as before the quality page existed.
pub fn index(pack: &Pack, rendered: &Rendered, quality: Option<&str>) -> String {
    let title = &pack.manifest.title;
    let mut h = head(title);
    h.push_str("<body>\n<header>\n");
    if pack.manifest.visibility == metaspec_core::manifest::Visibility::Private {
        h.push_str("<p><span class=\"private\">private pack: not for publication</span></p>\n");
    }
    let _ = writeln!(h, "<h1>{}</h1>", esc(title));
    if let Some(summary) = &pack.manifest.summary {
        let _ = writeln!(h, "<p>{}</p>", esc(summary));
    }
    h.push_str("</header>\n<main>\n");
    if let Some(file) = quality {
        let _ = writeln!(
            h,
            "<nav aria-label=\"Pages\"><ul><li><a href=\"{}\">Quality</a></li></ul></nav>",
            esc(file)
        );
    }
    for class in &rendered.classes {
        let _ = writeln!(
            h,
            "<section id=\"class-{id}\" aria-labelledby=\"class-{id}-h\"><h2 id=\"class-{id}-h\">{}</h2>",
            esc(&class.title),
            id = esc(&class.class)
        );
        if class.shape != Shape::PartsLinks {
            let _ = writeln!(
                h,
                "<p class=\"note\">{} document(s) of shape {}; these pages draw only parts-links/1 documents.</p></section>",
                class.documents,
                class.shape.as_str()
            );
            continue;
        }
        h.push_str("<div class=\"table-wrap\"><table><thead><tr><th>Document</th><th>Check</th><th>Findings</th><th>Views</th></tr></thead><tbody>\n");
        for p in &class.pages {
            let gated = p.findings.iter().filter(|f| f.gate).count();
            let check = match (&p.check.state, &p.check.result) {
                (CheckState::Ran, Some(r)) if r.valid => String::from("valid"),
                (CheckState::Ran, _) => String::from("not valid"),
                (state, _) => format!("not checked ({})", state_word(*state)),
            };
            let _ = writeln!(
                h,
                "<tr><td><a href=\"{}\">{}</a><br><code>{}</code></td><td>{}</td><td>{} ({gated} gated)</td><td>{}</td></tr>",
                esc(&p.page),
                esc(&p.name),
                esc(&p.document),
                esc(&check),
                p.findings.len(),
                p.scenes.len()
            );
        }
        h.push_str("</tbody></table></div></section>\n");
    }
    h.push_str("</main>\n</body>\n</html>\n");
    h
}
