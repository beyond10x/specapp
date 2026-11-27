//! The layout `rows`: a part's row is the layer of its block (or its family's row), the highest
//! layer on top.
//!
//! Between two rows of boxes lies a row of labels. An edge that crosses rows passes through one
//! label of the width its text needs, in the label row nearest its middle, and through a narrow
//! gap in every other row it crosses. An edge between two boxes of one row, or from a box to
//! itself, goes up to its label in the label row above. Items of a row are ordered by
//! barycentre sweeps (the order with the fewest crossings is kept) and placed by isotonic
//! regression (pool adjacent violators), so no two items of a row overlap.
//!
//! The idea and the placement are lifted from agenticspec's renderer and made generic: rows come
//! from the layers of the pack, not from a table of block names.

use crate::scene::{self, Band, Edge, Rect, Scene, SceneBox, SceneView};
use crate::{Card, PAD, Selection, Style, ViewSpec, card, ghost_card, label_width, scene_box};
use specapp_core::graph::Graph;
use specapp_core::manifest::RowsBy;
use std::collections::BTreeMap;

const MARGIN: f64 = 16.0;
const TITLE_H: f64 = 20.0;
const LAYER_GAP: f64 = 8.0;
const LABEL_H: f64 = 18.0;
const DUMMY_W: f64 = 8.0;
const CHILD_H: f64 = 24.0;
const CHILD_GAP: f64 = 4.0;
const PORT_GAP: f64 = 18.0;

struct Node {
    part: Option<usize>,
    id: String,
    card: Card,
    children: Vec<usize>,
    head_h: f64,
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Card,
    Label(usize),
    Dummy,
}

struct Item {
    kind: Kind,
    layer: usize,
    w: f64,
    h: f64,
    x: f64,
}

struct Route {
    link: usize,
    /// Items from the upper (or first) card to the lower (or last) card.
    chain: Vec<usize>,
    label: usize,
    /// Whether the chain runs from the link's `from` to its `to`.
    forward: bool,
    /// Both ends on the top side: an edge within one row.
    level: bool,
}

/// Pool adjacent violators: the non-decreasing sequence nearest to `target` in weighted least
/// squares.
pub fn pava(target: &[f64], weight: &[f64]) -> Vec<f64> {
    let mut blocks: Vec<(f64, f64, usize)> = Vec::new();
    for (i, &t) in target.iter().enumerate() {
        blocks.push((t * weight[i], weight[i], 1));
        while blocks.len() > 1 {
            let n = blocks.len();
            let (s1, w1, c1) = blocks[n - 2];
            let (s2, w2, c2) = blocks[n - 1];
            if s1 / w1 > s2 / w2 {
                blocks.truncate(n - 2);
                blocks.push((s1 + s2, w1 + w2, c1 + c2));
            } else {
                break;
            }
        }
    }
    let mut out = Vec::with_capacity(target.len());
    for (s, w, c) in blocks {
        out.extend(std::iter::repeat_n(s / w, c));
    }
    out
}

pub fn layout(
    graph: &Graph,
    view: &ViewSpec,
    style: &Style,
    sel: &Selection,
    marked: &BTreeMap<&str, Vec<usize>>,
) -> Scene {
    let parts = &graph.parts;
    let index = |name: &str| sel.parts.iter().copied().find(|&i| parts[i].id == name);

    // A part that exists once per instance of another shown part is drawn inside it.
    let mut host: BTreeMap<usize, usize> = BTreeMap::new();
    for &i in &sel.parts {
        let mut cur = i;
        for _ in 0..8 {
            match parts[cur].per.as_deref().and_then(index) {
                Some(next) if next != i && next != cur => cur = next,
                _ => break,
            }
        }
        if cur != i {
            host.insert(i, cur);
        }
    }
    let top = |i: usize| host.get(&i).copied().unwrap_or(i);

    // Boxes: in the order the links first name them, then the rest in the document's order,
    // then the names no part has.
    let mut order: Vec<usize> = Vec::new();
    for &li in &sel.links {
        for end in [&graph.links[li].from, &graph.links[li].to] {
            if let Some(i) = end.as_deref().and_then(index).map(top)
                && !order.contains(&i)
            {
                order.push(i);
            }
        }
    }
    for &i in &sel.parts {
        if !host.contains_key(&i) && !order.contains(&i) {
            order.push(i);
        }
    }
    let mut nodes: Vec<Node> = Vec::new();
    for &i in &order {
        let part = &parts[i];
        let mut noted = Vec::new();
        if let Some(inside) = &part.inside {
            noted.push(format!("in {inside}"));
        }
        if let Some(per) = &part.per
            && !host.contains_key(&i)
        {
            noted.push(format!("per {per}"));
        }
        let mut c = card(part, style, &noted);
        let children: Vec<usize> = sel
            .parts
            .iter()
            .copied()
            .filter(|c| host.get(c) == Some(&i))
            .collect();
        let head_h = c.h;
        for &k in &children {
            let w =
                crate::text::width(&child_label(graph, style, k), 11.0, false) + 16.0 + 2.0 * PAD;
            c.w = c.w.max(w.ceil());
            c.h += CHILD_H + CHILD_GAP;
        }
        if !children.is_empty() {
            c.h += 2.0;
        }
        nodes.push(Node {
            part: Some(i),
            id: part.id.clone(),
            card: c,
            children,
            head_h,
        });
    }
    for name in &sel.ghosts {
        let c = ghost_card(name);
        let head_h = c.h;
        nodes.push(Node {
            part: None,
            id: name.clone(),
            card: c,
            children: Vec::new(),
            head_h,
        });
    }
    let node_of = |name: &str| -> Option<usize> {
        match index(name) {
            Some(i) => nodes.iter().position(|n| n.part == Some(top(i))),
            None => nodes.iter().position(|n| n.part.is_none() && n.id == name),
        }
    };

    // Rows, highest first.
    let row_key = |n: &Node| -> (i64, String) {
        let Some(i) = n.part else {
            return (-2, String::from("names no part has"));
        };
        let s = style.block(parts[i].block.as_deref());
        if view.rows == RowsBy::Family
            && let Some(family) = &s.family
        {
            return (s.family_rank.map_or(-1, i64::from), family.clone());
        }
        match &s.layer {
            Some((number, name)) => (i64::from(*number), crate::layer_title(*number, name)),
            None => (-1, s.group.clone()),
        }
    };
    let mut keys: Vec<(i64, String)> = nodes.iter().map(row_key).collect();
    keys.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    keys.dedup();
    let row_of: Vec<usize> = nodes
        .iter()
        .map(|n| {
            let k = row_key(n);
            keys.iter().position(|r| *r == k).unwrap_or(0)
        })
        .collect();
    let n_layers = 2 * keys.len() + 1;

    // Items: the boxes, then the labels and gaps of the edges.
    let mut items: Vec<Item> = nodes
        .iter()
        .enumerate()
        .map(|(k, n)| Item {
            kind: Kind::Card,
            layer: 2 * row_of[k] + 1,
            w: n.card.w,
            h: n.card.h,
            x: 0.0,
        })
        .collect();
    let mut up: Vec<Vec<usize>> = vec![Vec::new(); items.len()];
    let mut down: Vec<Vec<usize>> = vec![Vec::new(); items.len()];
    let mut routes: Vec<Route> = Vec::new();
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
        let marks = marked.get(link.id.as_str()).unwrap_or(&empty);
        let (la, lb) = (items[a].layer, items[b].layer);
        let push = |items: &mut Vec<Item>, kind: Kind, layer: usize, w: f64, h: f64| {
            items.push(Item {
                kind,
                layer,
                w,
                h,
                x: 0.0,
            });
            items.len() - 1
        };
        if la == lb {
            let l = push(
                &mut items,
                Kind::Label(routes.len()),
                la - 1,
                label_width(&label, marks),
                LABEL_H,
            );
            up.resize(items.len(), Vec::new());
            down.resize(items.len(), Vec::new());
            for c in [a, b] {
                up[c].push(l);
                down[l].push(c);
            }
            routes.push(Route {
                link: li,
                chain: vec![a, l, b],
                label: l,
                forward: true,
                level: true,
            });
            continue;
        }
        let (u, d, forward) = if la < lb { (a, b, true) } else { (b, a, false) };
        let (lu, ld) = (items[u].layer, items[d].layer);
        let gaps = (ld - lu) / 2;
        let label_layer = lu + 1 + 2 * ((gaps - 1) / 2);
        let mut chain = vec![u];
        let mut label_item = 0;
        for layer in lu + 1..ld {
            let it = if layer == label_layer {
                let it = push(
                    &mut items,
                    Kind::Label(routes.len()),
                    layer,
                    label_width(&label, marks),
                    LABEL_H,
                );
                label_item = it;
                it
            } else {
                push(&mut items, Kind::Dummy, layer, DUMMY_W, 0.0)
            };
            chain.push(it);
        }
        chain.push(d);
        up.resize(items.len(), Vec::new());
        down.resize(items.len(), Vec::new());
        for w in chain.windows(2) {
            down[w[0]].push(w[1]);
            up[w[1]].push(w[0]);
        }
        routes.push(Route {
            link: li,
            chain,
            label: label_item,
            forward,
            level: false,
        });
    }

    // Order within each layer: barycentre sweeps, keeping the order with the fewest crossings.
    let mut layers: Vec<Vec<usize>> = vec![Vec::new(); n_layers];
    for (it, item) in items.iter().enumerate() {
        layers[item.layer].push(it);
    }
    let mut pos = vec![0.0; items.len()];
    let renumber = |layers: &Vec<Vec<usize>>, pos: &mut Vec<f64>| {
        for layer in layers {
            for (k, &it) in layer.iter().enumerate() {
                pos[it] = k as f64;
            }
        }
    };
    renumber(&layers, &mut pos);
    let crossings = |layers: &Vec<Vec<usize>>, pos: &[f64]| -> usize {
        let mut total = 0;
        for layer in layers {
            let segs: Vec<(f64, f64)> = layer
                .iter()
                .flat_map(|&a| down[a].iter().map(move |&b| (a, b)))
                .map(|(a, b)| (pos[a], pos[b]))
                .collect();
            for i in 0..segs.len() {
                for j in i + 1..segs.len() {
                    if (segs[i].0 - segs[j].0) * (segs[i].1 - segs[j].1) < 0.0 {
                        total += 1;
                    }
                }
            }
        }
        total
    };
    let mut best = layers.clone();
    let mut best_cross = crossings(&layers, &pos);
    for _ in 0..8 {
        for downward in [true, false] {
            let seq: Vec<usize> = if downward {
                (1..n_layers).collect()
            } else {
                (0..n_layers.saturating_sub(1)).rev().collect()
            };
            for li in seq {
                let (near, far) = if downward { (&up, &down) } else { (&down, &up) };
                let mut keyed: Vec<(f64, f64, usize)> = layers[li]
                    .iter()
                    .map(|&it| {
                        let ns = if near[it].is_empty() {
                            &far[it]
                        } else {
                            &near[it]
                        };
                        let k = if ns.is_empty() {
                            pos[it]
                        } else {
                            ns.iter().map(|&n| pos[n]).sum::<f64>() / ns.len() as f64
                        };
                        (k, pos[it], it)
                    })
                    .collect();
                keyed.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
                layers[li] = keyed.into_iter().map(|(_, _, it)| it).collect();
                renumber(&layers, &mut pos);
            }
            let c = crossings(&layers, &pos);
            if c < best_cross {
                best_cross = c;
                best = layers.clone();
            }
        }
    }
    let layers = best;

    // Horizontal placement.
    let kinds: Vec<Kind> = items.iter().map(|it| it.kind).collect();
    let gap = |a: usize, b: usize| -> f64 {
        match (kinds[a], kinds[b]) {
            (Kind::Card, Kind::Card) => 32.0,
            (Kind::Card, _) | (_, Kind::Card) => 18.0,
            _ => 10.0,
        }
    };
    for layer in &layers {
        let mut x = 0.0;
        for (k, &it) in layer.iter().enumerate() {
            if k > 0 {
                x += gap(layer[k - 1], it);
            }
            items[it].x = x + items[it].w / 2.0;
            x += items[it].w;
        }
    }
    for _ in 0..40 {
        let seq: Vec<usize> = (0..n_layers).chain((0..n_layers).rev()).collect();
        for li in seq {
            let layer = &layers[li];
            if layer.is_empty() {
                continue;
            }
            let mut offs = vec![0.0; layer.len()];
            for k in 1..layer.len() {
                let (a, b) = (layer[k - 1], layer[k]);
                offs[k] = offs[k - 1] + (items[a].w + items[b].w) / 2.0 + gap(a, b);
            }
            let target: Vec<f64> = layer
                .iter()
                .zip(&offs)
                .map(|(&it, o)| {
                    let ns: Vec<usize> = up[it].iter().chain(down[it].iter()).copied().collect();
                    let want = if ns.is_empty() {
                        items[it].x
                    } else {
                        ns.iter().map(|&n| items[n].x).sum::<f64>() / ns.len() as f64
                    };
                    want - o
                })
                .collect();
            let weight: Vec<f64> = layer
                .iter()
                .map(|&it| match items[it].kind {
                    Kind::Card => 1.0,
                    Kind::Label(_) => 1.5,
                    Kind::Dummy => 2.0,
                })
                .collect();
            let placed = pava(&target, &weight);
            for (k, &it) in layer.iter().enumerate() {
                items[it].x = placed[k] + offs[k];
            }
        }
    }
    let left = items
        .iter()
        .map(|it| it.x - it.w / 2.0)
        .fold(f64::INFINITY, f64::min);
    let shift = if left.is_finite() { MARGIN - left } else { 0.0 };
    for it in &mut items {
        it.x = (it.x + shift).round();
    }
    let titles = keys
        .iter()
        .map(|(_, t)| crate::text::width(t, 11.0, true) + 2.0 * MARGIN + 8.0)
        .fold(0.0, f64::max);
    let width = items
        .iter()
        .map(|it| it.x + it.w / 2.0 + MARGIN)
        .fold(titles.max(2.0 * MARGIN), f64::max)
        .ceil();

    // Vertical placement.
    let mut top_of = vec![0.0; n_layers];
    let mut h_of = vec![0.0; n_layers];
    let mut y = MARGIN;
    for li in 0..n_layers {
        let tallest = layers[li].iter().map(|&it| items[it].h).fold(0.0, f64::max);
        let h = if li % 2 == 1 {
            TITLE_H + tallest
        } else if layers[li].is_empty() {
            if li == 0 || li == n_layers - 1 {
                0.0
            } else {
                10.0
            }
        } else {
            tallest.max(10.0)
        };
        top_of[li] = y;
        h_of[li] = h;
        y += h + if h > 0.0 { LAYER_GAP } else { 0.0 };
    }
    let height = (y + MARGIN).ceil();
    let card_y = |it: usize| top_of[items[it].layer] + TITLE_H;
    let centre_y = |it: usize| top_of[items[it].layer] + h_of[items[it].layer] / 2.0;

    // Ports: the ends of edges on one side of a box, spread in the order of where they go.
    let mut ends: Vec<(usize, bool, f64, usize, usize)> = Vec::new(); // card, top side, toward, route, end
    for (ri, r) in routes.iter().enumerate() {
        let first = r.chain[0];
        let last = r.chain[r.chain.len() - 1];
        if r.level {
            ends.push((first, true, items[r.label].x, ri, 0));
            ends.push((last, true, items[r.label].x, ri, 1));
        } else {
            ends.push((first, false, items[r.chain[1]].x, ri, 0));
            ends.push((last, true, items[r.chain[r.chain.len() - 2]].x, ri, 1));
        }
    }
    let mut port: BTreeMap<(usize, usize), f64> = BTreeMap::new();
    for (ci, item) in items.iter().enumerate().take(nodes.len()) {
        for side in [true, false] {
            let mut mine: Vec<&(usize, bool, f64, usize, usize)> =
                ends.iter().filter(|e| e.0 == ci && e.1 == side).collect();
            mine.sort_by(|a, b| a.2.total_cmp(&b.2).then(a.3.cmp(&b.3)).then(a.4.cmp(&b.4)));
            let m = mine.len();
            let span = if m > 1 {
                (item.w - 24.0).min(PORT_GAP * (m as f64 - 1.0))
            } else {
                0.0
            };
            for (j, e) in mine.iter().enumerate() {
                let x = if m > 1 {
                    item.x - span / 2.0 + span * j as f64 / (m as f64 - 1.0)
                } else {
                    item.x
                };
                port.insert((e.3, e.4), x);
            }
        }
    }

    // Boxes.
    let mut boxes: Vec<SceneBox> = Vec::new();
    for (ci, n) in nodes.iter().enumerate() {
        let x = items[ci].x - n.card.w / 2.0;
        let y = card_y(ci);
        let part = n.part.map(|i| &parts[i]);
        let mut b = scene_box(part, &n.id, &n.card, style, (x, y, n.card.w, n.card.h));
        b.container = !n.children.is_empty();
        boxes.push(b);
        for (k, &c) in n.children.iter().enumerate() {
            let child = &parts[c];
            let text = Card {
                label: child_label(graph, style, c),
                lines: Vec::new(),
                title: card(child, style, &[format!("per {}", n.id)]).title,
                w: n.card.w - 2.0 * PAD,
                h: CHILD_H,
            };
            let cy = y + n.head_h - 2.0 + k as f64 * (CHILD_H + CHILD_GAP);
            let mut b = scene_box(
                Some(child),
                &child.id,
                &text,
                style,
                (x + PAD, cy, text.w, CHILD_H),
            );
            b.parent = Some(n.id.clone());
            boxes.push(b);
        }
    }

    // Edges.
    let mut edges: Vec<Edge> = Vec::new();
    for (ri, r) in routes.iter().enumerate() {
        let link = &graph.links[r.link];
        let first = r.chain[0];
        let last = r.chain[r.chain.len() - 1];
        let mut pts: Vec<[f64; 2]> = Vec::new();
        let p0 = port[&(ri, 0)];
        let p1 = port[&(ri, 1)];
        if r.level {
            pts.push([p0, card_y(first)]);
            pts.push([items[r.label].x, centre_y(r.label)]);
            pts.push([p1, card_y(last)]);
        } else {
            pts.push([p0, card_y(first) + items[first].h]);
            for &it in &r.chain[1..r.chain.len() - 1] {
                let x = items[it].x;
                match items[it].kind {
                    Kind::Label(_) => pts.push([x, centre_y(it)]),
                    _ => {
                        let l = items[it].layer;
                        pts.push([x, top_of[l]]);
                        pts.push([x, top_of[l] + h_of[l]]);
                    }
                }
            }
            pts.push([p1, card_y(last)]);
        }
        if !r.forward {
            pts.reverse();
        }
        let label = &items[r.label];
        let cy = centre_y(r.label);
        edges.push(Edge {
            id: link.id.clone(),
            from: link.from.clone().unwrap_or_else(|| String::from("?")),
            to: link.to.clone().unwrap_or_else(|| String::from("?")),
            carries: link.carries.clone(),
            label: crate::link_label(link, style),
            points: pts
                .iter()
                .map(|p| [scene::round(p[0]), scene::round(p[1])])
                .collect(),
            label_at: Rect {
                x: scene::round(label.x - label.w / 2.0),
                y: scene::round(cy - label.h / 2.0),
                w: scene::round(label.w),
                h: scene::round(label.h),
            },
            added_by: link.added_by.clone(),
            findings: Vec::new(),
            title: crate::link_title(link),
        });
    }

    let rows = keys
        .iter()
        .enumerate()
        .map(|(r, (_, title))| {
            let l = 2 * r + 1;
            Band {
                title: title.clone(),
                y: scene::round(top_of[l] - 4.0),
                h: scene::round(h_of[l] + 8.0),
            }
        })
        .collect();

    Scene {
        format: String::new(),
        document: String::new(),
        view: SceneView {
            id: String::new(),
            title: String::new(),
            layout: String::new(),
            implicit: false,
        },
        width: scene::round(width),
        height: scene::round(height),
        rows,
        boxes,
        edges,
        marks: Vec::new(),
        legend: Vec::new(),
    }
}

/// The one line of a part drawn inside its host: its id and block.
fn child_label(graph: &Graph, style: &Style, part: usize) -> String {
    let p = &graph.parts[part];
    match &p.block {
        Some(b) => format!("{} · {}", p.id, style.word(b)),
        None => p.id.clone(),
    }
}
