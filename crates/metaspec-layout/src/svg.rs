//! A scene as one SVG element for a page. Colours are classes (`c-<name>`) and variables the
//! page's CSS sets per theme ([`crate::palette::css`]); the SVG has no colour value of its own.
//! Every box and edge has its identifiers on hover; a finding is a numbered mark that links to
//! its row in the page's list.

use crate::scene::Scene;
use crate::text::{esc, px};
use crate::{ICON, LABEL_H, LINE_H, PAD, icons, mark_width};
use std::fmt::Write as _;

/// The CSS of a scene's elements; the palette's CSS sets the variables.
pub const CSS: &str = "\
svg.scene{display:block;max-width:none;font-family:system-ui,-apple-system,'Segoe UI',Roboto,sans-serif}
svg.scene .band{fill:var(--band);stroke:none}
svg.scene .band-title{fill:var(--muted);font-size:11px;font-weight:600}
svg.scene .box>rect.body{fill:var(--fill);stroke:var(--stroke);stroke-width:1.5}
svg.scene .box.container>rect.body{fill:var(--bg)}
svg.scene .box.ghost>rect.body{fill:var(--bg);stroke-dasharray:5 3}
svg.scene .box>rect.edge-of-border{fill:none;stroke:var(--stroke);stroke-width:1;stroke-dasharray:2 2}
svg.scene .box.marked>rect.body{stroke:var(--finding);stroke-width:2.5}
svg.scene .label{fill:var(--fg);font-size:13px;font-weight:600}
svg.scene .child .label{font-size:11px;font-weight:500}
svg.scene .line{fill:var(--fg);font-size:11px}
svg.scene .icon{fill:none;stroke:var(--stroke);stroke-width:1.4;stroke-linecap:round;stroke-linejoin:round}
svg.scene .monogram{fill:var(--stroke);font-size:10px;font-weight:700}
svg.scene .edge>path{fill:none;stroke:var(--line);stroke-width:1.4}
svg.scene .edge.added>path{stroke-dasharray:6 4}
svg.scene .edge.marked>path{stroke:var(--finding);stroke-width:2.4;stroke-dasharray:8 3}
svg.scene .arrowhead{fill:var(--line)}
svg.scene .pill{fill:var(--bg);stroke:var(--line);stroke-width:1}
svg.scene .edge.marked .pill{stroke:var(--finding);stroke-width:1.6}
svg.scene .edge-label{fill:var(--fg);font-size:11px}
svg.scene .mark rect{fill:var(--finding)}
svg.scene .mark text{fill:var(--finding-fg);font-size:10px;font-weight:700}
svg.scene .mark.note rect{fill:var(--bg);stroke:var(--finding);stroke-width:1.5}
svg.scene .mark.note text{fill:var(--fg)}
";

/// The CSS of hubs and badges, inside the SVG of a scene that has them, so a page without hubs
/// stays as it was.
pub const HUB_CSS: &str = "svg.scene .box.hub>rect.body{stroke-width:3}svg.scene .hublink.marked .pill{stroke:var(--finding);stroke-width:1.6}";

fn curve(points: &[[f64; 2]]) -> String {
    let mut d = String::new();
    for (i, p) in points.iter().enumerate() {
        if i == 0 {
            let _ = write!(d, "M{} {}", px(p[0]), px(p[1]));
            continue;
        }
        let a = points[i - 1];
        if (a[0] - p[0]).abs() < 0.05 {
            let _ = write!(d, "L{} {}", px(p[0]), px(p[1]));
        } else {
            let my = (a[1] + p[1]) / 2.0;
            let _ = write!(
                d,
                "C{} {} {} {} {} {}",
                px(a[0]),
                px(my),
                px(p[0]),
                px(my),
                px(p[0]),
                px(p[1])
            );
        }
    }
    d
}

fn polyline(points: &[[f64; 2]]) -> String {
    let mut d = String::new();
    for (i, p) in points.iter().enumerate() {
        let _ = write!(
            d,
            "{}{} {}",
            if i == 0 { "M" } else { "L" },
            px(p[0]),
            px(p[1])
        );
    }
    d
}

/// The SVG of a scene. `prefix` makes its element ids unique in a page that holds several
/// scenes: a box is `<prefix>-b<n>`, an edge `<prefix>-e<n>`, in the order of the scene.
pub fn svg(scene: &Scene, prefix: &str) -> String {
    let mut out = String::new();
    let (w, h) = (px(scene.width), px(scene.height));
    let _ = write!(
        out,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"scene\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" role=\"img\" aria-labelledby=\"{p}-t {p}-d\">",
        p = esc(prefix)
    );
    let badges = if scene.badges.is_empty() {
        String::new()
    } else {
        format!(
            "; {} links to a hub, each a badge on its caller's box",
            scene.badges.len()
        )
    };
    let _ = write!(
        out,
        "<title id=\"{p}-t\">{} of {}</title><desc id=\"{p}-d\">{} boxes and {} edges, each edge labelled by what it carries{badges}; {} findings marked with their number. The tables below the drawing list the same parts, links and findings as text.</desc>",
        esc(&scene.view.title),
        esc(&scene.document),
        scene.boxes.len(),
        scene.edges.len(),
        scene.marks.len(),
        p = esc(prefix)
    );
    let _ = write!(
        out,
        "<defs><marker id=\"{}-arrow\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"7\" markerHeight=\"7\" orient=\"auto-start-reverse\"><path d=\"M0 0L10 5L0 10z\" class=\"arrowhead\"/></marker></defs>",
        esc(prefix)
    );
    if !scene.badges.is_empty() || scene.boxes.iter().any(|b| b.hub) {
        out.push_str("<style>");
        out.push_str(HUB_CSS);
        out.push_str("</style>");
    }

    if !scene.rows.is_empty() {
        out.push_str("<g class=\"bands\">");
        for band in &scene.rows {
            let _ = write!(
                out,
                "<rect class=\"band\" x=\"4\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"6\"/><text class=\"band-title\" x=\"12\" y=\"{}\">{}</text>",
                px(band.y),
                px(scene.width - 8.0),
                px(band.h),
                px(band.y + 16.0),
                esc(&band.title)
            );
        }
        out.push_str("</g>");
    }

    out.push_str("<g class=\"boxes\">");
    for (i, b) in scene.boxes.iter().enumerate() {
        let mut class = format!("box c-{}", b.colour);
        for (on, name) in [
            (b.ghost, "ghost"),
            (b.container, "container"),
            (b.parent.is_some() && b.lines.is_empty(), "child"),
            (!b.findings.is_empty(), "marked"),
            (b.hub, "hub"),
        ] {
            if on {
                class.push(' ');
                class.push_str(name);
            }
        }
        let _ = write!(
            out,
            "<g class=\"{class}\" id=\"{}-b{i}\" data-id=\"{}\"><title>{}</title>",
            esc(prefix),
            esc(&b.id),
            esc(&b.title)
        );
        if b.on_border {
            let _ = write!(
                out,
                "<rect class=\"edge-of-border\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"9\"/>",
                px(b.x - 3.0),
                px(b.y - 3.0),
                px(b.w + 6.0),
                px(b.h + 6.0)
            );
        }
        let _ = write!(
            out,
            "<rect class=\"body\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"6\"/>",
            px(b.x),
            px(b.y),
            px(b.w),
            px(b.h)
        );
        if b.parent.is_some() && b.lines.is_empty() {
            let _ = write!(
                out,
                "<text class=\"label\" x=\"{}\" y=\"{}\">{}</text>",
                px(b.x + 8.0),
                px(b.y + 16.0),
                esc(&b.label)
            );
        } else {
            let mut lx = b.x + PAD;
            if let Some(icon) = &b.icon {
                let (ix, iy) = (b.x + PAD, b.y + PAD);
                match icons::path(icon) {
                    Some(d) => {
                        let _ = write!(
                            out,
                            "<path class=\"icon\" transform=\"translate({} {}) scale({})\" d=\"{d}\"><title>{}</title></path>",
                            px(ix),
                            px(iy),
                            ICON / 14.0,
                            esc(icon)
                        );
                    }
                    None => {
                        let letter: String =
                            icon.chars().take(1).collect::<String>().to_uppercase();
                        let _ = write!(
                            out,
                            "<g><title>{}</title><circle class=\"icon\" cx=\"{}\" cy=\"{}\" r=\"7.5\"/><text class=\"monogram\" x=\"{}\" y=\"{}\" text-anchor=\"middle\">{}</text></g>",
                            esc(icon),
                            px(ix + 8.0),
                            px(iy + 8.0),
                            px(ix + 8.0),
                            px(iy + 11.5),
                            esc(&letter)
                        );
                    }
                }
                lx += ICON + 6.0;
            }
            let _ = write!(
                out,
                "<text class=\"label\" x=\"{}\" y=\"{}\">{}</text>",
                px(lx),
                px(b.y + PAD + 13.0),
                esc(&b.label)
            );
            for (k, line) in b.lines.iter().enumerate() {
                let _ = write!(
                    out,
                    "<text class=\"line\" x=\"{}\" y=\"{}\">{}</text>",
                    px(b.x + PAD),
                    px(b.y + PAD + LABEL_H + 11.0 + k as f64 * LINE_H),
                    esc(line)
                );
            }
        }
        out.push_str("</g>");
    }
    out.push_str("</g>");

    if !scene.badges.is_empty() {
        out.push_str("<g class=\"hublinks\">");
        for (i, b) in scene.badges.iter().enumerate() {
            let marks: f64 = b.findings.iter().map(|&n| mark_width(n) + 3.0).sum();
            let r = &b.at;
            let _ = write!(
                out,
                "<g class=\"hublink{}\" id=\"{}-h{i}\" data-id=\"{}\"><title>{}</title><rect class=\"pill\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"9\"/><text class=\"edge-label\" x=\"{}\" y=\"{}\" text-anchor=\"middle\">{}</text></g>",
                if b.findings.is_empty() { "" } else { " marked" },
                esc(prefix),
                esc(&b.id),
                esc(&b.title),
                px(r.x),
                px(r.y),
                px(r.w),
                px(r.h),
                px(r.x + marks + (r.w - marks) / 2.0),
                px(r.y + 13.0),
                esc(&b.label)
            );
        }
        out.push_str("</g>");
    }

    let curved = scene.view.layout == "rows";
    out.push_str("<g class=\"edges\">");
    for (i, e) in scene.edges.iter().enumerate() {
        let mut class = String::from("edge");
        if e.added_by.is_some() {
            class.push_str(" added");
        }
        if !e.findings.is_empty() {
            class.push_str(" marked");
        }
        let d = if curved {
            curve(&e.points)
        } else {
            polyline(&e.points)
        };
        let marks: f64 = e.findings.iter().map(|&n| mark_width(n) + 3.0).sum();
        let r = &e.label_at;
        let _ = write!(
            out,
            "<g class=\"{class}\" id=\"{}-e{i}\" data-id=\"{}\"><title>{}</title><path d=\"{d}\" marker-end=\"url(#{}-arrow)\"/><rect class=\"pill\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"9\"/><text class=\"edge-label\" x=\"{}\" y=\"{}\" text-anchor=\"middle\">{}</text></g>",
            esc(prefix),
            esc(&e.id),
            esc(&e.title),
            esc(prefix),
            px(r.x),
            px(r.y),
            px(r.w),
            px(r.h),
            px(r.x + marks + (r.w - marks) / 2.0),
            px(r.y + 13.0),
            esc(&e.label)
        );
    }
    out.push_str("</g>");

    out.push_str("<g class=\"marks\">");
    for m in &scene.marks {
        let class = if m.gate { "mark" } else { "mark note" };
        let text = format!("!{}", m.finding);
        let _ = write!(
            out,
            "<a href=\"#finding-{n}\"><g class=\"{class}\"><title>finding {n}: {} on {}{}</title><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"16\" rx=\"8\"/><text x=\"{}\" y=\"{}\" text-anchor=\"middle\">{}</text></g></a>",
            esc(&m.rule),
            esc(&m.subject),
            if m.gate { "" } else { " (not gated)" },
            px(m.x - m.w / 2.0),
            px(m.y - 8.0),
            px(m.w),
            px(m.x),
            px(m.y + 3.5),
            esc(&text),
            n = m.finding
        );
    }
    out.push_str("</g></svg>");
    out
}
