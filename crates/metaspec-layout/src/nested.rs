//! The layout `nested`: a part is drawn inside the part that contains it (`in` or `per`).
//! A part that has both is drawn inside its `in` container, and its box says `per <part>`.
//!
//! The parts of one container, and the parts that no shown part contains, flow left to right
//! in the order of the document and wrap at a fixed width. An edge is a straight segment between
//! the borders of its two boxes; an edge between a part and a part that contains it runs from
//! the inner box up to the head of the outer one. A label that would overlap an earlier one
//! moves down until it is free.

use crate::scene::{self, Edge, Rect, Scene, SceneBox, SceneView};
use crate::{Card, Selection, Style, card, ghost_card, label_width, scene_box};
use metaspec_core::graph::Graph;
use std::collections::BTreeMap;

const MARGIN: f64 = 16.0;
const GAP: f64 = 24.0;
const TOP_GAP: f64 = 48.0;
const INNER_MAX: f64 = 560.0;
const TOP_MAX: f64 = 1100.0;
const LABEL_H: f64 = 18.0;

struct Node {
    part: Option<usize>,
    id: String,
    card: Card,
    children: Vec<usize>,
    w: f64,
    h: f64,
    x: f64,
    y: f64,
}

/// Rows of a flow: items of `widths` left to right, wrapped before `max`.
fn flow(widths: &[f64], gap: f64, max: f64) -> Vec<Vec<usize>> {
    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut used = 0.0;
    for (i, &w) in widths.iter().enumerate() {
        match rows.last_mut() {
            Some(row) if used + gap + w <= max => {
                row.push(i);
                used += gap + w;
            }
            _ => {
                rows.push(vec![i]);
                used = w;
            }
        }
    }
    rows
}

pub fn layout(
    graph: &Graph,
    style: &Style,
    sel: &Selection,
    marked: &BTreeMap<&str, Vec<usize>>,
) -> Scene {
    let parts = &graph.parts;
    let index = |name: &str| sel.parts.iter().position(|&i| parts[i].id == name);

    // Nodes: the shown parts, then the names no part has.
    let mut nodes: Vec<Node> = Vec::new();
    for &i in &sel.parts {
        nodes.push(Node {
            part: Some(i),
            id: parts[i].id.clone(),
            card: card(&parts[i], style, &[]),
            children: Vec::new(),
            w: 0.0,
            h: 0.0,
            x: 0.0,
            y: 0.0,
        });
    }
    for name in &sel.ghosts {
        nodes.push(Node {
            part: None,
            id: name.clone(),
            card: ghost_card(name),
            children: Vec::new(),
            w: 0.0,
            h: 0.0,
            x: 0.0,
            y: 0.0,
        });
    }

    // The container of each shown part: the part it sits `in`, else the part it exists `per`;
    // a cycle of containment is cut where it closes.
    let mut parent: Vec<Option<usize>> = (0..nodes.len())
        .map(|k| {
            let p = &parts[nodes[k].part?];
            let shown = |holder: Option<&str>| index(holder?).filter(|&h| h != k);
            shown(p.inside.as_deref()).or_else(|| shown(p.per.as_deref()))
        })
        .collect();
    for k in 0..nodes.len() {
        let mut cur = parent[k];
        let mut steps = 0;
        while let Some(c) = cur {
            if c == k || steps > nodes.len() {
                parent[k] = None;
                break;
            }
            cur = parent[c];
            steps += 1;
        }
    }
    // A part that sits in one part and exists per another says which of the two its box is
    // not drawn inside.
    for k in 0..nodes.len() {
        let Some(i) = nodes[k].part else { continue };
        let p = &parts[i];
        let (Some(inside), Some(per)) = (&p.inside, &p.per) else {
            continue;
        };
        let drawn_in = parent[k].map(|h| nodes[h].id.as_str());
        let mut noted = Vec::new();
        if drawn_in != Some(inside.as_str()) {
            noted.push(format!("in {inside}"));
        }
        if drawn_in != Some(per.as_str()) {
            noted.push(format!("per {per}"));
        }
        nodes[k].card = card(p, style, &noted);
    }
    for (k, holder) in parent.iter().enumerate() {
        if let Some(p) = *holder {
            nodes[p].children.push(k);
        }
    }
    let roots: Vec<usize> = (0..nodes.len()).filter(|&k| parent[k].is_none()).collect();

    // Sizes, inner boxes first.
    fn size(nodes: &mut Vec<Node>, k: usize) {
        let children = nodes[k].children.clone();
        if children.is_empty() {
            nodes[k].w = nodes[k].card.w;
            nodes[k].h = nodes[k].card.h;
            return;
        }
        for &c in &children {
            size(nodes, c);
        }
        let widths: Vec<f64> = children.iter().map(|&c| nodes[c].w).collect();
        let rows = flow(&widths, GAP, INNER_MAX);
        let mut inner_w: f64 = 0.0;
        let mut inner_h = 0.0;
        for (r, row) in rows.iter().enumerate() {
            let w: f64 =
                row.iter().map(|&i| widths[i]).sum::<f64>() + GAP * (row.len() as f64 - 1.0);
            inner_w = inner_w.max(w);
            inner_h += row
                .iter()
                .map(|&i| nodes[children[i]].h)
                .fold(0.0, f64::max);
            if r > 0 {
                inner_h += GAP;
            }
        }
        nodes[k].w = nodes[k].card.w.max(inner_w + 2.0 * crate::PAD).ceil();
        nodes[k].h = nodes[k].card.h + inner_h + crate::PAD;
    }
    fn place(nodes: &mut Vec<Node>, k: usize, x: f64, y: f64) {
        nodes[k].x = x;
        nodes[k].y = y;
        let children = nodes[k].children.clone();
        if children.is_empty() {
            return;
        }
        let widths: Vec<f64> = children.iter().map(|&c| nodes[c].w).collect();
        let mut cy = y + nodes[k].card.h;
        for row in flow(&widths, GAP, INNER_MAX) {
            let mut cx = x + crate::PAD;
            let mut tallest: f64 = 0.0;
            for &i in &row {
                let c = children[i];
                place(nodes, c, cx, cy);
                cx += nodes[c].w + GAP;
                tallest = tallest.max(nodes[c].h);
            }
            cy += tallest + GAP;
        }
    }
    for &r in &roots {
        size(&mut nodes, r);
    }
    let widths: Vec<f64> = roots.iter().map(|&r| nodes[r].w).collect();
    let mut y = MARGIN;
    for row in flow(&widths, TOP_GAP, TOP_MAX) {
        let mut x = MARGIN;
        let mut tallest: f64 = 0.0;
        for &i in &row {
            let r = roots[i];
            place(&mut nodes, r, x, y);
            x += nodes[r].w + TOP_GAP;
            tallest = tallest.max(nodes[r].h);
        }
        y += tallest + TOP_GAP;
    }

    // Boxes, a container before its parts.
    let mut boxes: Vec<SceneBox> = Vec::new();
    fn emit(
        nodes: &[Node],
        k: usize,
        graph: &Graph,
        style: &Style,
        parent: Option<&str>,
        out: &mut Vec<SceneBox>,
    ) {
        let n = &nodes[k];
        let part = n.part.map(|i| &graph.parts[i]);
        let mut b = scene_box(part, &n.id, &n.card, style, (n.x, n.y, n.w, n.h));
        b.parent = parent.map(String::from);
        b.container = !n.children.is_empty();
        out.push(b);
        for &c in &n.children {
            emit(nodes, c, graph, style, Some(&n.id), out);
        }
    }
    for &r in &roots {
        emit(&nodes, r, graph, style, None, &mut boxes);
    }

    // Edges.
    let node_of = |name: &str| -> Option<usize> {
        match index(name) {
            Some(k) => Some(k),
            None => nodes.iter().position(|n| n.part.is_none() && n.id == name),
        }
    };
    let ancestor = |a: usize, b: usize| -> bool {
        let mut cur = parent[b];
        while let Some(c) = cur {
            if c == a {
                return true;
            }
            cur = parent[c];
        }
        false
    };
    let clip = |k: usize, from: (f64, f64), to: (f64, f64)| -> (f64, f64) {
        let n = &nodes[k];
        let (dx, dy) = (to.0 - from.0, to.1 - from.1);
        let tx = if dx.abs() > 1e-9 {
            (n.w / 2.0) / dx.abs()
        } else {
            f64::INFINITY
        };
        let ty = if dy.abs() > 1e-9 {
            (n.h / 2.0) / dy.abs()
        } else {
            f64::INFINITY
        };
        let t = tx.min(ty).min(1.0);
        (from.0 + t * dx, from.1 + t * dy)
    };
    let centre = |k: usize| (nodes[k].x + nodes[k].w / 2.0, nodes[k].y + nodes[k].h / 2.0);
    let mut labels: Vec<Rect> = Vec::new();
    // The text of every box: a whole box without parts, the head of a container.
    let obstacles: Vec<Rect> = nodes
        .iter()
        .map(|n| Rect {
            x: n.x,
            y: n.y,
            w: n.w,
            h: if n.children.is_empty() { n.h } else { n.card.h },
        })
        .collect();
    let mut edges: Vec<Edge> = Vec::new();
    let mut right = nodes.iter().map(|n| n.x + n.w).fold(2.0 * MARGIN, f64::max);
    let mut bottom = nodes.iter().map(|n| n.y + n.h).fold(2.0 * MARGIN, f64::max);
    for &li in &sel.links {
        let link = &graph.links[li];
        let (Some(a), Some(b)) = (
            node_of(link.from.as_deref().unwrap_or("?")),
            node_of(link.to.as_deref().unwrap_or("?")),
        ) else {
            continue;
        };
        let label = crate::link_label(link, style);
        let empty = Vec::new();
        let w = label_width(&label, marked.get(link.id.as_str()).unwrap_or(&empty));
        let pts: Vec<(f64, f64)> = if a == b {
            let n = &nodes[a];
            let (x, y) = (n.x + n.w, n.y + n.h / 3.0);
            let d = n.h / 3.0;
            vec![(x, y), (x + 22.0, y), (x + 22.0, y + d), (x, y + d)]
        } else if ancestor(a, b) || ancestor(b, a) {
            let (inner, outer) = if ancestor(a, b) { (b, a) } else { (a, b) };
            let cx = nodes[inner].x + nodes[inner].w / 2.0;
            let pair = vec![
                (cx, nodes[inner].y),
                (cx, nodes[outer].y + nodes[outer].card.h - 6.0),
            ];
            if inner == a {
                pair
            } else {
                pair.into_iter().rev().collect()
            }
        } else {
            let (ca, cb) = (centre(a), centre(b));
            vec![clip(a, ca, cb), clip(b, cb, ca)]
        };
        // Where the label may go, best first: along the segment, then beside it. The first
        // place that covers no box text and no earlier label is taken.
        let candidates: Vec<(f64, f64)> = if a == b {
            vec![(pts[1].0 + w / 2.0 + 4.0, (pts[1].1 + pts[2].1) / 2.0)]
        } else {
            let (p, q) = (pts[0], pts[pts.len() - 1]);
            let mut out = Vec::new();
            for t in [0.5, 0.35, 0.65, 0.2, 0.8] {
                let (x, y) = (p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1));
                out.push((x, y));
            }
            for t in [0.5, 0.35, 0.65] {
                let (x, y) = (p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1));
                out.push((x, y - LABEL_H - 2.0));
                out.push((x, y + LABEL_H + 2.0));
            }
            let mx = (p.0 + q.0) / 2.0;
            let above = nodes[a].y.min(nodes[b].y) - LABEL_H / 2.0 - 3.0;
            let below =
                (nodes[a].y + nodes[a].h).max(nodes[b].y + nodes[b].h) + LABEL_H / 2.0 + 3.0;
            out.push((mx, above));
            out.push((mx, below));
            out
        };
        let place = |(x, y): (f64, f64)| Rect {
            x: (x - w / 2.0).max(2.0),
            y: (y - LABEL_H / 2.0).max(2.0),
            w,
            h: LABEL_H,
        };
        let meets = |r: &Rect, o: &Rect| {
            r.x < o.x + o.w && o.x < r.x + r.w && r.y < o.y + o.h && o.y < r.y + r.h
        };
        let free = |r: &Rect| !obstacles.iter().chain(labels.iter()).any(|o| meets(r, o));
        let mut rect = candidates
            .iter()
            .map(|&c| place(c))
            .find(|r| free(r))
            .unwrap_or_else(|| place(candidates[0]));
        let mut tries = 0;
        while labels.iter().any(|o| meets(&rect, o)) && tries < 12 {
            rect.y += LABEL_H + 2.0;
            tries += 1;
        }
        labels.push(rect);
        right = right.max(rect.x + rect.w);
        bottom = bottom.max(rect.y + rect.h);
        for p in &pts {
            right = right.max(p.0);
        }
        edges.push(Edge {
            id: link.id.clone(),
            from: link.from.clone().unwrap_or_else(|| String::from("?")),
            to: link.to.clone().unwrap_or_else(|| String::from("?")),
            carries: link.carries.clone(),
            label,
            points: pts
                .iter()
                .map(|p| [scene::round(p.0), scene::round(p.1)])
                .collect(),
            label_at: Rect {
                x: scene::round(rect.x),
                y: scene::round(rect.y),
                w: scene::round(rect.w),
                h: scene::round(rect.h),
            },
            added_by: link.added_by.clone(),
            findings: Vec::new(),
            title: crate::link_title(link),
        });
    }

    Scene {
        format: String::new(),
        document: String::new(),
        view: SceneView {
            id: String::new(),
            title: String::new(),
            layout: String::new(),
            implicit: false,
        },
        width: scene::round((right + MARGIN).ceil()),
        height: scene::round((bottom + MARGIN).ceil()),
        rows: Vec::new(),
        boxes,
        edges,
        marks: Vec::new(),
        legend: Vec::new(),
    }
}
