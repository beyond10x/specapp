//! Collapsed regions: a view with `collapse` draws each collapsed region as one box with the
//! number of parts inside, rolls the links across its border up by far end and name with a
//! count, and is followed by the view `<view>/<region>` of the region and its parts. On the
//! fixture view with `collapse: { blocks: [...] }` and on hand-written graphs.

use super::common::drawn;
use metaspec_core::graph::{self, Graph};
use metaspec_core::manifest::{self, Collapse, DocumentClass, Layout};
use metaspec_core::tool::Finding;
use metaspec_layout::scene::{Scene, SceneBox};
use metaspec_layout::{Style, ViewSpec, layout, select, svg, views_of};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

/// A class of an invented pack with `contain: { in: in, per: per }` and the views `views`.
fn class(views: &str) -> DocumentClass {
    let text = format!(
        "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: {{ path: b }}\ndocuments:\n  - {{ class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, contain: {{ in: in, per: per }}, views: [{views}] }}\n"
    );
    let m = manifest::parse(&text).unwrap_or_else(|e| panic!("{e:?}"));
    m.documents[0].clone()
}

fn graph_of(
    class: &DocumentClass,
    parts: serde_json::Value,
    links: serde_json::Value,
    aliases: &[(&str, &str)],
) -> Graph {
    let doc = json!({ "n": "x", "parts": parts, "links": links });
    let aliases: BTreeMap<String, String> = aliases
        .iter()
        .map(|(w, b)| (w.to_string(), b.to_string()))
        .collect();
    graph::read_with_aliases(class, "d/x.yaml", &doc, &aliases)
}

fn view_of<'a>(views: &'a [ViewSpec], id: &str) -> &'a ViewSpec {
    views
        .iter()
        .find(|v| v.id == id)
        .unwrap_or_else(|| panic!("no view {id} in {:?}", ids(views)))
}

fn ids(views: &[ViewSpec]) -> Vec<&str> {
    views.iter().map(|v| v.id.as_str()).collect()
}

/// The boxes of the part `id` (no ghost).
fn boxes_of<'a>(s: &'a Scene, id: &str) -> Vec<&'a SceneBox> {
    s.boxes.iter().filter(|b| b.id == id && !b.ghost).collect()
}

fn one<'a>(s: &'a Scene, id: &str) -> &'a SceneBox {
    let found = boxes_of(s, id);
    assert_eq!(found.len(), 1, "{}: one box {id}", s.view.id);
    found[0]
}

fn part_ids(s: &Scene) -> BTreeSet<&str> {
    s.boxes
        .iter()
        .filter(|b| !b.ghost)
        .map(|b| b.id.as_str())
        .collect()
}

fn layouts(views: &str) -> Vec<(Layout, DocumentClass)> {
    [Layout::Rows, Layout::Nested]
        .into_iter()
        .map(|l| (l, class(&views.replace("LAYOUT", l.as_str()))))
        .collect()
}

/// The parts inside `region` through `in`, among the parts the view shows.
fn inside(g: &Graph, shown: &BTreeSet<&str>, region: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for p in &g.parts {
        let mut cur = p.inside.as_deref();
        let mut steps = 0;
        while let Some(h) = cur {
            if !shown.contains(h) || steps > g.parts.len() {
                break;
            }
            if h == region {
                out.insert(p.id.clone());
                break;
            }
            cur = g.part(h).and_then(|q| q.inside.as_deref());
            steps += 1;
        }
    }
    out
}

#[test]
fn region_collapse_by_block() {
    // The fixture view: every region of a listed block is one box, no region of another block
    // is collapsed.
    let docs = drawn();
    let mut collapsed = 0;
    for d in &docs {
        for view in &d.views {
            let Some(Collapse::Blocks(listed)) = &view.collapse else {
                continue;
            };
            let s = layout(&d.graph, view, &d.style, &d.findings);
            let plain = ViewSpec {
                collapse: None,
                ..view.clone()
            };
            let shown = select(&d.graph, &plain);
            let shown: BTreeSet<&str> = shown
                .parts
                .iter()
                .map(|&i| d.graph.parts[i].id.as_str())
                .collect();
            let regions: BTreeSet<&str> = d
                .graph
                .parts
                .iter()
                .filter(|p| shown.contains(p.id.as_str()))
                .filter_map(|p| p.inside.as_deref())
                .filter(|h| shown.contains(h))
                .collect();
            let listed_block = |id: &str| {
                d.graph
                    .part(id)
                    .and_then(|p| p.block.as_ref())
                    .is_some_and(|b| listed.blocks.contains(b))
            };
            for &r in &regions {
                let within_listed = regions
                    .iter()
                    .any(|&o| o != r && listed_block(o) && inside(&d.graph, &shown, o).contains(r));
                if listed_block(r) && !within_listed {
                    let b = one(&s, r);
                    let c = b.collapsed.as_ref().unwrap_or_else(|| {
                        panic!("{} {}: {r} is collapsed", d.graph.document, view.id)
                    });
                    let parts = inside(&d.graph, &shown, r);
                    assert_eq!(c.parts, parts.len(), "{r}");
                    for p in &parts {
                        assert!(boxes_of(&s, p).is_empty(), "{r}: {p} is inside its box");
                    }
                    collapsed += 1;
                } else if !listed_block(r) {
                    for b in boxes_of(&s, r) {
                        assert!(b.collapsed.is_none(), "{r} is of another block");
                    }
                }
            }
        }
    }
    assert!(
        collapsed >= 2,
        "the fixture view collapses a region in two documents: {collapsed}"
    );

    // An invented graph with regions of two blocks, one of them named by an alias.
    for (l, c) in layouts("{ id: v, title: V, layout: LAYOUT, collapse: { blocks: [kilo] } }") {
        let g = graph_of(
            &c,
            json!({
                "yard": { "block": "zone" },
                "y1": { "block": "lima", "in": "yard" },
                "y2": { "block": "lima", "in": "yard" },
                "dock": { "block": "lima" },
                "d1": { "block": "mike", "in": "dock" },
                "free": { "block": "mike" },
            }),
            json!([
                { "from": "y1", "to": "free", "carries": "x" },
                { "from": "d1", "to": "free", "carries": "x" },
            ]),
            &[("zone", "kilo")],
        );
        let views = views_of(&c, &g);
        let s = layout(&g, view_of(&views, "v"), &Style::default(), &[]);
        let yard = one(&s, "yard");
        assert_eq!(yard.block.as_deref(), Some("kilo"), "{l:?}: after aliases");
        assert_eq!(yard.collapsed.as_ref().map(|c| c.parts), Some(2), "{l:?}");
        assert_eq!(
            part_ids(&s),
            ["yard", "dock", "d1", "free"].into(),
            "{l:?}: the region of block lima is drawn with its part"
        );
        assert!(one(&s, "dock").collapsed.is_none(), "{l:?}");
    }
}

#[test]
fn region_collapsed_box_carries_its_part_count() {
    let parts = json!({
        "r": { "block": "kilo" },
        "s": { "block": "lima", "in": "r" },
        "t": { "block": "lima", "in": "s" },
        "u": { "block": "lima", "in": "r" },
        "v": { "block": "lima" },
    });
    let links = json!([{ "from": "t", "to": "v", "carries": "x" }]);
    for (collapse, region, count, drawn) in [
        ("[r]", "r", 3, vec!["r", "v"]),
        ("all", "r", 3, vec!["r", "v"]),
        ("[s]", "s", 1, vec!["r", "s", "u", "v"]),
    ] {
        let views = format!("{{ id: v, title: V, layout: LAYOUT, collapse: {collapse} }}");
        for (l, c) in layouts(&views) {
            let g = graph_of(&c, parts.clone(), links.clone(), &[]);
            let views = views_of(&c, &g);
            let s = layout(&g, view_of(&views, "v"), &Style::default(), &[]);
            let b = one(&s, region);
            let got = b.collapsed.as_ref().unwrap_or_else(|| {
                panic!("{l:?} {collapse}: {region} is collapsed");
            });
            assert_eq!(got.parts, count, "{l:?} {collapse}");
            let line = if count == 1 {
                String::from("1 part inside")
            } else {
                format!("{count} parts inside")
            };
            assert!(
                b.lines.contains(&line),
                "{l:?} {collapse}: {:?} says {line}",
                b.lines
            );
            assert!(!b.container, "{l:?}: a collapsed box holds no box");
            assert_eq!(
                part_ids(&s),
                drawn.iter().copied().collect(),
                "{l:?} {collapse}"
            );
            let collapsed: Vec<&str> = s
                .boxes
                .iter()
                .filter(|b| b.collapsed.is_some())
                .map(|b| b.id.as_str())
                .collect();
            assert_eq!(collapsed, vec![region], "{l:?} {collapse}");
        }
    }
}

/// A region `r` with two parts, a far part `f`, and five links: two from inside to `f` with one
/// name, one to `f` with another name, one from `f` in, and one inside.
fn rolled_graph(c: &DocumentClass) -> Graph {
    graph_of(
        c,
        json!({
            "r": { "block": "kilo" },
            "a": { "block": "lima", "in": "r" },
            "b": { "block": "lima", "in": "r" },
            "f": { "block": "mike" },
        }),
        json!([
            { "from": "a", "to": "f", "carries": "x" },
            { "from": "b", "to": "f", "carries": "x" },
            { "from": "a", "to": "f", "carries": "y" },
            { "from": "f", "to": "b", "carries": "x" },
            { "from": "a", "to": "b", "carries": "x" },
        ]),
        &[],
    )
}

#[test]
fn region_two_links_from_inside_to_one_far_part_with_one_name_become_one_edge_with_count_2() {
    for (l, c) in layouts("{ id: v, title: V, layout: LAYOUT, collapse: [r] }") {
        let g = rolled_graph(&c);
        let views = views_of(&c, &g);
        let s = layout(&g, view_of(&views, "v"), &Style::default(), &[]);
        let got: BTreeSet<(&str, &str, Option<&str>, Option<usize>)> = s
            .edges
            .iter()
            .map(|e| {
                (
                    e.from.as_str(),
                    e.to.as_str(),
                    e.carries.as_deref(),
                    e.count,
                )
            })
            .collect();
        assert_eq!(
            got,
            [
                ("r", "f", Some("x"), Some(2)),
                ("r", "f", Some("y"), Some(1)),
                ("f", "r", Some("x"), Some(1)),
            ]
            .into(),
            "{l:?}: the link inside is drawn in the region's view only"
        );
        let two = s.edges.iter().find(|e| e.count == Some(2)).unwrap();
        assert_eq!(
            two.links,
            vec!["a->f:x", "b->f:x"],
            "{l:?}: the edge names its links"
        );
        assert!(two.label.ends_with("×2"), "{l:?}: {}", two.label);
        let drawn = svg::svg(&s, "v");
        assert!(
            drawn.contains(">x ×2</text>"),
            "{l:?}: the SVG shows the count"
        );
        let one_link = s
            .edges
            .iter()
            .find(|e| e.carries.as_deref() == Some("y"))
            .unwrap();
        assert_eq!(one_link.links, vec!["a->f:y"], "{l:?}");
        assert!(!one_link.label.contains('×'), "{l:?}: {}", one_link.label);
    }
}

#[test]
fn region_view_holds_the_region_and_its_parts_and_the_box_names_it() {
    for (l, c) in layouts("{ id: v, title: V, layout: LAYOUT, collapse: [r] }") {
        let g = rolled_graph(&c);
        let views = views_of(&c, &g);
        assert_eq!(ids(&views), vec!["v", "v/r"], "{l:?}");
        let drill = &views[1];
        assert_eq!(drill.region.as_deref(), Some("r"));
        assert_eq!(drill.of.as_deref(), Some("v"));
        assert!(drill.collapse.is_none() && !drill.implicit);
        assert_eq!(drill.layout, l);
        let s = layout(&g, view_of(&views, "v"), &Style::default(), &[]);
        assert_eq!(
            one(&s, "r").collapsed.as_ref().map(|c| c.view.as_str()),
            Some("v/r"),
            "{l:?}"
        );
        let inner = layout(&g, drill, &Style::default(), &[]);
        assert_eq!(part_ids(&inner), ["r", "a", "b"].into(), "{l:?}");
        let edges: Vec<&str> = inner.edges.iter().map(|e| e.id.as_str()).collect();
        assert_eq!(edges, vec!["a->b:x"], "{l:?}");
        assert!(inner.boxes.iter().all(|b| b.collapsed.is_none()));
    }

    // With hubs: the view, its edges view, then the view of each collapsed region; the box of
    // the edges view names the same region view.
    let c = class("{ id: v, title: V, layout: rows, hubs: { parts: [f] }, collapse: [r] }");
    let g = rolled_graph(&c);
    let views = views_of(&c, &g);
    assert_eq!(ids(&views), vec!["v", "v/edges", "v/r"]);
    let s = layout(&g, &views[1], &Style::default(), &[]);
    assert_eq!(
        one(&s, "r").collapsed.as_ref().map(|c| c.view.as_str()),
        Some("v/r")
    );
}

fn finding(subject: &str) -> Finding {
    Finding {
        rule: String::from("q"),
        gate: true,
        subject: subject.to_string(),
        message: String::from("m"),
    }
}

#[test]
fn region_finding_inside_is_counted_on_the_box_and_drawn_in_its_view() {
    for (l, c) in layouts("{ id: v, title: V, layout: LAYOUT, collapse: [r] }") {
        let g = rolled_graph(&c);
        let findings = [
            finding("part:a"),
            finding("link:a->b:x"),
            finding("link:a->f:x"),
            finding("part:r"),
        ];
        let views = views_of(&c, &g);
        let s = layout(&g, view_of(&views, "v"), &Style::default(), &findings);
        let r = one(&s, "r");
        assert_eq!(r.findings, vec![4], "{l:?}: the region's own finding");
        assert_eq!(
            r.collapsed.as_ref().map(|c| c.findings.clone()),
            Some(vec![1, 2]),
            "{l:?}: the findings inside"
        );
        assert!(
            r.lines.contains(&String::from("2 findings inside")),
            "{l:?}: {:?}",
            r.lines
        );
        let marks: Vec<usize> = s.marks.iter().map(|m| m.finding).collect();
        assert_eq!(marks, vec![3, 4], "{l:?}");
        let rolled = s.edges.iter().find(|e| e.count == Some(2)).unwrap();
        assert_eq!(rolled.findings, vec![3], "{l:?}: on the rolled edge");
        let inner = layout(&g, view_of(&views, "v/r"), &Style::default(), &findings);
        let marks: Vec<usize> = inner.marks.iter().map(|m| m.finding).collect();
        assert_eq!(marks, vec![1, 2, 4], "{l:?}: drawn in the region's view");
    }
}
