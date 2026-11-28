//! Hubs: a view that names `hubs` draws each hub once, apart, and each link to it from a part
//! that is no hub as a badge on its caller; its sibling `<view>/edges` draws the same links as
//! edges. On the fixture view with `hubs: {in_degree: 3}` and on hand-written graphs.

use super::common::drawn;
use metaspec_core::graph::{self, Graph};
use metaspec_core::manifest::{self, Hubs, Layout, RowsBy};
use metaspec_core::tool::Subject;
use metaspec_layout::scene::{Rect, Scene, SceneBox};
use metaspec_layout::{Style, ViewSpec, layout, select, svg, views_of};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

/// The ids of a scene's boxes that are the part `id` (not a ghost).
fn boxes_of<'a>(s: &'a Scene, id: &str) -> Vec<&'a SceneBox> {
    s.boxes.iter().filter(|b| b.id == id && !b.ghost).collect()
}

fn inside(r: &Rect, b: &SceneBox) -> bool {
    r.x >= b.x && r.y >= b.y && r.x + r.w <= b.x + b.w + 0.05 && r.y + r.h <= b.y + b.h + 0.05
}

/// One view with hubs of a fixture document: the hubs the in-degree names and the links to them
/// from a part that is no hub.
struct HubCase {
    doc: usize,
    view: ViewSpec,
    hubs: BTreeSet<String>,
    elided: Vec<usize>,
}

fn fixture_cases(docs: &[super::common::Drawn]) -> Vec<HubCase> {
    let mut out = Vec::new();
    for (k, d) in docs.iter().enumerate() {
        for view in &d.views {
            let Some(n) = view.hubs.as_ref().and_then(|h| h.in_degree) else {
                continue;
            };
            let sel = select(&d.graph, view);
            let mut degree: BTreeMap<&str, usize> = BTreeMap::new();
            for &li in &sel.links {
                if let Some(to) = d.graph.links[li].to.as_deref() {
                    *degree.entry(to).or_default() += 1;
                }
            }
            let hubs: BTreeSet<String> = sel
                .parts
                .iter()
                .map(|&i| d.graph.parts[i].id.clone())
                .filter(|id| degree.get(id.as_str()).copied().unwrap_or(0) >= n)
                .collect();
            let elided = sel
                .links
                .iter()
                .copied()
                .filter(|&li| {
                    let l = &d.graph.links[li];
                    l.to.as_ref().is_some_and(|t| hubs.contains(t))
                        && !l.from.as_ref().is_some_and(|f| hubs.contains(f))
                })
                .collect();
            out.push(HubCase {
                doc: k,
                view: view.clone(),
                hubs,
                elided,
            });
        }
    }
    out
}

#[test]
fn hub_fixture_view_with_in_degree_3_draws_each_hub_once_with_no_incoming_edge() {
    let docs = drawn();
    let cases = fixture_cases(&docs);
    assert!(
        cases
            .iter()
            .any(|c| c.view.hubs.as_ref().unwrap().in_degree == Some(3)),
        "a fixture view says hubs: {{in_degree: 3}}"
    );
    let mut hubs = 0;
    for c in &cases {
        let d = &docs[c.doc];
        let s = layout(&d.graph, &c.view, &d.style, &d.findings);
        for hub in &c.hubs {
            let found = boxes_of(&s, hub);
            assert_eq!(
                found.len(),
                1,
                "{} {}: {hub} drawn once",
                d.graph.document,
                c.view.id
            );
            assert!(found[0].hub, "{hub} is marked a hub");
            assert!(
                found[0].parent.is_none(),
                "{hub} is drawn apart, in no other box"
            );
            let incoming: Vec<&str> = s
                .edges
                .iter()
                .filter(|e| &e.to == hub && !c.hubs.contains(&e.from))
                .map(|e| e.id.as_str())
                .collect();
            assert!(
                incoming.is_empty(),
                "{hub}: no incoming edge, got {incoming:?}"
            );
            let degree = d
                .graph
                .links
                .iter()
                .filter(|l| l.to.as_deref() == Some(hub.as_str()))
                .count();
            assert!(degree >= 3, "{hub}: {degree} incoming links");
            hubs += 1;
        }
        for b in s.boxes.iter().filter(|b| b.hub) {
            assert!(c.hubs.contains(&b.id), "{} is no hub of the view", b.id);
        }
        assert_eq!(
            s.rows.last().map(|r| r.title.as_str()),
            Some("hubs"),
            "rows draws the hubs in a band of their own, the bottom one"
        );
    }
    assert!(hubs >= 1, "the fixture view has a hub");
}

#[test]
fn hub_each_caller_carries_one_badge_per_link_to_the_hub_inside_its_box() {
    let docs = drawn();
    let mut most = 0;
    let mut badges = 0;
    for c in fixture_cases(&docs) {
        let d = &docs[c.doc];
        let s = layout(&d.graph, &c.view, &d.style, &d.findings);
        assert_eq!(s.badges.len(), c.elided.len(), "{}", d.graph.document);
        let mut per_caller: BTreeMap<&str, usize> = BTreeMap::new();
        for &li in &c.elided {
            let link = &d.graph.links[li];
            let from = link.from.as_deref().unwrap();
            let to = link.to.as_deref().unwrap();
            let found: Vec<_> = s.badges.iter().filter(|b| b.id == link.id).collect();
            assert_eq!(found.len(), 1, "{}: one badge", link.id);
            let badge = found[0];
            assert_eq!((badge.from.as_str(), badge.to.as_str()), (from, to));
            assert!(
                badge.label.contains(to),
                "{}: the badge names the hub",
                badge.label
            );
            let word = d.style.word(link.carries.as_deref().unwrap());
            assert!(
                badge.label.contains(&word),
                "{}: the badge names the link",
                badge.label
            );
            let caller = s.boxes.iter().find(|b| b.id == from).unwrap();
            assert!(
                inside(&badge.at, caller),
                "{}: {:?} inside {from} {:?}",
                link.id,
                badge.at,
                (caller.x, caller.y, caller.w, caller.h)
            );
            for b in s.boxes.iter().filter(|b| b.parent.as_deref() == Some(from)) {
                let apart = badge.at.y >= b.y + b.h
                    || b.y >= badge.at.y + badge.at.h
                    || badge.at.x >= b.x + b.w
                    || b.x >= badge.at.x + badge.at.w;
                assert!(apart, "{}: the badge covers the box {}", link.id, b.id);
            }
            *per_caller.entry(from).or_default() += 1;
            badges += 1;
        }
        for (caller, n) in &per_caller {
            let mine: Vec<&Rect> = s
                .badges
                .iter()
                .filter(|b| b.from == *caller)
                .map(|b| &b.at)
                .collect();
            assert_eq!(mine.len(), *n, "{caller}");
            for (i, a) in mine.iter().enumerate() {
                for b in &mine[i + 1..] {
                    let apart = a.x + a.w <= b.x
                        || b.x + b.w <= a.x
                        || a.y + a.h <= b.y
                        || b.y + b.h <= a.y;
                    assert!(apart, "{caller}: two badges overlap");
                }
            }
            most = most.max(*n);
        }
        let drawn = svg::svg(&s, "v");
        for b in &s.badges {
            assert!(
                drawn.contains("class=\"hublink\" id=\"v-h")
                    && drawn.contains(&format!("data-id=\"{}\"", b.id.replace('>', "&gt;"))),
                "the SVG draws the badge {}",
                b.id
            );
        }
    }
    assert!(badges >= 3, "{badges} badges");
    assert!(
        most >= 2,
        "a caller with two links to its hub carries two badges"
    );
}

#[test]
fn hub_edges_view_draws_the_same_links_as_edges() {
    let docs = drawn();
    let cases = fixture_cases(&docs);
    assert!(!cases.is_empty());
    for c in cases {
        let d = &docs[c.doc];
        let id = format!("{}/edges", c.view.id);
        let sibling = d
            .views
            .iter()
            .find(|v| v.id == id)
            .unwrap_or_else(|| panic!("{}: no view {id}", d.graph.document));
        assert_eq!(sibling.hubs, None);
        assert_eq!(
            (
                sibling.blocks.clone(),
                sibling.carries.clone(),
                sibling.layout
            ),
            (c.view.blocks.clone(), c.view.carries.clone(), c.view.layout)
        );
        let at = d.views.iter().position(|v| v.id == c.view.id).unwrap();
        assert_eq!(d.views[at + 1].id, id, "the edges view follows its view");
        let with = layout(&d.graph, &c.view, &d.style, &d.findings);
        let without = layout(&d.graph, sibling, &d.style, &d.findings);
        assert!(without.badges.is_empty() && !without.boxes.iter().any(|b| b.hub));
        let mut drawn: Vec<&str> = with
            .edges
            .iter()
            .map(|e| e.id.as_str())
            .chain(with.badges.iter().map(|b| b.id.as_str()))
            .collect();
        drawn.sort();
        let mut edges: Vec<&str> = without.edges.iter().map(|e| e.id.as_str()).collect();
        edges.sort();
        assert_eq!(drawn, edges, "{}: the same links", d.graph.document);
        for &li in &c.elided {
            let link = &d.graph.links[li];
            let e = without.edges.iter().find(|e| e.id == link.id).unwrap();
            assert_eq!(
                (Some(e.from.as_str()), Some(e.to.as_str())),
                (link.from.as_deref(), link.to.as_deref())
            );
        }
    }
}

#[test]
fn hub_finding_on_an_elided_link_is_drawn_on_its_badge() {
    let docs = drawn();
    let mut seen = 0;
    for c in fixture_cases(&docs) {
        let d = &docs[c.doc];
        let s = layout(&d.graph, &c.view, &d.style, &d.findings);
        for (i, f) in d.findings.iter().enumerate() {
            let Subject::Link(id) = f.subject() else {
                continue;
            };
            if !c.elided.iter().any(|&li| d.graph.links[li].id == id) {
                continue;
            }
            let n = i + 1;
            let badge = s.badges.iter().find(|b| b.id == id).unwrap();
            assert_eq!(badge.findings, vec![n], "{id}");
            let marks: Vec<_> = s.marks.iter().filter(|m| m.finding == n).collect();
            assert_eq!(marks.len(), 1, "{id}: one mark");
            let m = marks[0];
            assert_eq!(m.subject, format!("link:{id}"));
            let r = &badge.at;
            assert!(
                m.x - m.w / 2.0 >= r.x - 0.05
                    && m.x + m.w / 2.0 <= r.x + r.w + 0.05
                    && m.y >= r.y
                    && m.y <= r.y + r.h,
                "{id}: the mark {:?} sits on the badge {r:?}",
                (m.x, m.y, m.w)
            );
            let drawn = svg::svg(&s, "v");
            assert!(drawn.contains(&format!("href=\"#finding-{n}\"")));
            seen += 1;
        }
    }
    assert!(
        seen >= 1,
        "a finding on an elided link of a fixture document"
    );
}

fn graph_of(parts: serde_json::Value, links: serde_json::Value) -> Graph {
    let text = "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: { path: b }\ndocuments:\n  - { class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, contain: { in: in, per: per } }\n";
    let m = manifest::parse(text).unwrap_or_else(|e| panic!("{e:?}"));
    let doc = json!({ "n": "x", "parts": parts, "links": links });
    graph::read(&m.documents[0], "d/x.yaml", &doc)
}

fn view(layout_: Layout, hubs: Option<Hubs>) -> ViewSpec {
    ViewSpec {
        id: String::from("v"),
        title: String::from("V"),
        layout: layout_,
        rows: RowsBy::Layer,
        blocks: Vec::new(),
        carries: Vec::new(),
        hubs,
        collapse: None,
        region: None,
        of: None,
        implicit: false,
    }
}

fn ids<T>(items: &[T], id: impl Fn(&T) -> String) -> BTreeSet<String> {
    items.iter().map(id).collect()
}

#[test]
fn hub_links_between_hubs_stay_edges_and_hubs_are_named_by_part_and_block() {
    let g = graph_of(
        json!({
            "alpha": { "block": "kilo" },
            "bravo": { "block": "mike" },
            "charlie": { "block": "lima" },
            "delta": { "block": "lima" },
        }),
        json!([
            { "from": "charlie", "to": "alpha", "carries": "x" },
            { "from": "delta", "to": "alpha", "carries": "x" },
            { "from": "alpha", "to": "bravo", "carries": "y" },
            { "from": "bravo", "to": "alpha", "carries": "y" },
            { "from": "charlie", "to": "bravo", "carries": "z" },
            { "from": "alpha", "to": "charlie", "carries": "z" },
        ]),
    );
    let hubs = Hubs {
        parts: vec![String::from("alpha")],
        blocks: vec![String::from("mike")],
        ..Default::default()
    };
    for l in [Layout::Rows, Layout::Nested] {
        let s = layout(&g, &view(l, Some(hubs.clone())), &Style::default(), &[]);
        assert_eq!(
            ids(&s.edges, |e| e.id.clone()),
            ["alpha->bravo:y", "bravo->alpha:y", "alpha->charlie:z"]
                .map(String::from)
                .into(),
            "{l:?}: links between hubs and links out of a hub stay edges"
        );
        assert_eq!(
            ids(&s.badges, |b| b.id.clone()),
            ["charlie->alpha:x", "delta->alpha:x", "charlie->bravo:z"]
                .map(String::from)
                .into(),
            "{l:?}"
        );
        assert_eq!(
            ids(
                &s.boxes
                    .iter()
                    .filter(|b| b.hub)
                    .cloned()
                    .collect::<Vec<_>>(),
                |b| b.id.clone()
            ),
            ["alpha", "bravo"].map(String::from).into(),
            "{l:?}"
        );
        for b in &s.badges {
            let caller = boxes_of(&s, &b.from)[0];
            assert!(inside(&b.at, caller), "{l:?} {}", b.id);
        }
        let plain = layout(&g, &view(l, None), &Style::default(), &[]);
        assert_eq!(
            plain.edges.len(),
            6,
            "{l:?}: without hubs every link an edge"
        );
        assert!(plain.badges.is_empty());
    }
}

#[test]
fn hub_in_degree_counts_the_links_the_view_shows() {
    let g = graph_of(
        json!({
            "hub": { "block": "kilo" },
            "one": { "block": "lima" },
            "two": { "block": "lima" },
        }),
        json!([
            { "from": "one", "to": "hub", "carries": "x" },
            { "from": "two", "to": "hub", "carries": "x" },
            { "from": "two", "to": "hub", "carries": "y" },
        ]),
    );
    let three = Hubs {
        in_degree: Some(3),
        ..Default::default()
    };
    let s = layout(
        &g,
        &view(Layout::Rows, Some(three.clone())),
        &Style::default(),
        &[],
    );
    assert_eq!(s.badges.len(), 3);
    assert_eq!(s.edges.len(), 0);
    let mut only_x = view(Layout::Rows, Some(three));
    only_x.carries = vec![String::from("x")];
    let s = layout(&g, &only_x, &Style::default(), &[]);
    assert_eq!(
        (s.badges.len(), s.edges.len()),
        (0, 2),
        "two links shown: no hub at 3"
    );
}

#[test]
fn hub_caller_drawn_inside_its_per_host_carries_its_badge_in_its_own_box() {
    let g = graph_of(
        json!({
            "host": { "block": "lima" },
            "inner": { "block": "lima", "per": "host" },
            "hub": { "block": "kilo" },
        }),
        json!([
            { "from": "inner", "to": "hub", "carries": "x" },
            { "from": "host", "to": "hub", "carries": "x" },
        ]),
    );
    let hubs = Hubs {
        in_degree: Some(2),
        ..Default::default()
    };
    for l in [Layout::Rows, Layout::Nested] {
        let s = layout(&g, &view(l, Some(hubs.clone())), &Style::default(), &[]);
        let inner = boxes_of(&s, "inner")[0];
        assert_eq!(inner.parent.as_deref(), Some("host"), "{l:?}");
        let host = boxes_of(&s, "host")[0];
        let badge = s.badges.iter().find(|b| b.from == "inner").unwrap();
        assert!(
            inside(&badge.at, inner),
            "{l:?}: {:?} in {:?}",
            badge.at,
            (inner.x, inner.y, inner.w, inner.h)
        );
        assert!(
            inner.y + inner.h <= host.y + host.h && inner.x + inner.w <= host.x + host.w,
            "{l:?}: the host holds its grown part"
        );
        let own = s.badges.iter().find(|b| b.from == "host").unwrap();
        assert!(inside(&own.at, host), "{l:?}");
        assert!(
            own.at.y + own.at.h <= inner.y || inner.y + inner.h <= own.at.y,
            "{l:?}: the host's badge and its part do not overlap"
        );
    }
}

#[test]
fn hub_views_of_a_class_add_the_edges_view_after_each_view_with_hubs() {
    let text = "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: { path: b }\ndocuments:\n  - { class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, views: [{ id: one, title: One, layout: rows, hubs: { in_degree: 2 } }, { id: two, title: Two, layout: nested }] }\n";
    let m = manifest::parse(text).unwrap_or_else(|e| panic!("{e:?}"));
    // `a` has two incoming links, so the view `one` elides them.
    let g = graph_of(
        json!({ "a": { "block": "kilo" }, "b": { "block": "lima" }, "c": { "block": "lima" } }),
        json!([
            { "from": "b", "to": "a", "carries": "x" },
            { "from": "c", "to": "a", "carries": "x" },
        ]),
    );
    let views = views_of(&m.documents[0], &g);
    let got: Vec<(&str, bool)> = views
        .iter()
        .map(|v| (v.id.as_str(), v.hubs.is_some()))
        .collect();
    assert_eq!(
        got,
        vec![("one", true), ("one/edges", false), ("two", false)]
    );
    assert_ne!(
        views[0].title, views[1].title,
        "the edges view says what it is"
    );
    assert!(!views[1].implicit);
}

#[test]
fn hub_view_whose_document_has_no_hub_adds_no_second_identical_drawing() {
    let text = "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: { path: b }\ndocuments:\n  - { class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, views: [{ id: one, title: One, layout: rows, hubs: { in_degree: 3 } }] }\n";
    let m = manifest::parse(text).unwrap_or_else(|e| panic!("{e:?}"));
    let g = graph_of(
        json!({
            "alpha": { "block": "kilo" },
            "bravo": { "block": "lima" },
            "charlie": { "block": "lima" },
        }),
        json!([
            { "from": "bravo", "to": "alpha", "carries": "x" },
            { "from": "charlie", "to": "alpha", "carries": "x" },
        ]),
    );
    let views = views_of(&m.documents[0], &g);
    let with = layout(&g, &views[0], &Style::default(), &[]);
    assert!(
        with.badges.is_empty() && !with.boxes.iter().any(|b| b.hub),
        "no part has 3 incoming links: the view elides nothing"
    );
    if let Some(sibling) = views.get(1) {
        let without = layout(&g, sibling, &Style::default(), &[]);
        assert_eq!(
            (&with.boxes, &with.edges, &with.rows),
            (&without.boxes, &without.edges, &without.rows),
            "the sibling draws the same drawing"
        );
    }
    let got: Vec<&str> = views.iter().map(|v| v.id.as_str()).collect();
    assert_eq!(
        got,
        vec!["one"],
        "a view that elides nothing in this document needs no `/edges` twin: the page would draw the same drawing twice"
    );
}
