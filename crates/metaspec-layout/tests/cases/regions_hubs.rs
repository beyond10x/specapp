//! Collapsed regions together with hubs: a rolled-up link that goes to a hub, a hub found by its
//! number of incoming links, and a region whose id is the word `edges`.

use metaspec_core::graph::{self, Graph};
use metaspec_core::manifest::{self, DocumentClass, Layout};
use metaspec_core::tool::Finding;
use metaspec_layout::{Style, layout, views_of};
use serde_json::json;
use std::collections::BTreeMap;

/// A class of an invented pack with `contain: { in: in, per: per }` and the views `views`.
fn class(views: &str) -> DocumentClass {
    let text = format!(
        "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: {{ path: b }}\ndocuments:\n  - {{ class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, contain: {{ in: in, per: per }}, views: [{views}] }}\n"
    );
    let m = manifest::parse(&text).unwrap_or_else(|e| panic!("{e:?}"));
    m.documents[0].clone()
}

fn graph_of(class: &DocumentClass, parts: serde_json::Value, links: serde_json::Value) -> Graph {
    let doc = json!({ "n": "x", "parts": parts, "links": links });
    graph::read_with_aliases(class, "d/x.yaml", &doc, &BTreeMap::new())
}

fn layouts(views: &str) -> Vec<(Layout, DocumentClass)> {
    [Layout::Rows, Layout::Nested]
        .into_iter()
        .map(|l| (l, class(&views.replace("LAYOUT", l.as_str()))))
        .collect()
}

/// A region `region` with two parts `a` and `b`, a far part `f`, and two links from inside to
/// `f` with one name.
fn two_to_far(c: &DocumentClass, region: &str) -> Graph {
    graph_of(
        c,
        json!({
            region: { "block": "kilo" },
            "a": { "block": "lima", "in": region },
            "b": { "block": "lima", "in": region },
            "f": { "block": "mike" },
        }),
        json!([
            { "from": "a", "to": "f", "carries": "x" },
            { "from": "b", "to": "f", "carries": "x" },
        ]),
    )
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
fn region_rolled_link_drawn_as_a_hub_badge_keeps_its_count() {
    // Two links from inside `r` to the hub `f` with one name roll up to one link `r->f:x`
    // standing for two; drawn as a badge, it has to say so, as the edge does with `x ×2`.
    for (l, c) in
        layouts("{ id: v, title: V, layout: LAYOUT, hubs: { parts: [f] }, collapse: [r] }")
    {
        let g = two_to_far(&c, "r");
        let views = views_of(&c, &g);
        let s = layout(&g, &views[0], &Style::default(), &[]);
        assert_eq!(s.badges.len(), 1, "{l:?}: one badge on the box of r");
        let badge = &s.badges[0];
        assert_eq!(
            (badge.from.as_str(), badge.to.as_str()),
            ("r", "f"),
            "{l:?}"
        );
        assert!(
            badge.label.contains("×2"),
            "{l:?}: the badge of a link standing for two says its count: {:?}",
            badge.label
        );
    }
}

#[test]
fn region_rolled_link_drawn_as_a_hub_badge_carries_the_findings_of_its_links() {
    // A finding on `a->f:x` is a mark on the rolled-up edge when `f` is no hub; when `f` is a
    // hub the same rolled-up link is a badge and the finding must be marked there.
    for (l, c) in
        layouts("{ id: v, title: V, layout: LAYOUT, hubs: { parts: [f] }, collapse: [r] }")
    {
        let g = two_to_far(&c, "r");
        let views = views_of(&c, &g);
        let findings = [finding("link:a->f:x")];
        let s = layout(&g, &views[0], &Style::default(), &findings);
        assert_eq!(s.badges.len(), 1, "{l:?}");
        assert_eq!(
            s.badges[0].findings,
            vec![1],
            "{l:?}: the badge stands for a->f:x and carries its finding"
        );
        let marks: Vec<usize> = s.marks.iter().map(|m| m.finding).collect();
        assert_eq!(marks, vec![1], "{l:?}: the finding is marked in the view");
    }
}

#[test]
fn region_hub_by_in_degree_is_the_same_hub_where_the_edges_view_is_made_and_where_it_is_drawn() {
    // `views_of` adds `v/edges` when the view draws a link as a badge. It decides on the links
    // before the roll-up (two links to `f`), the drawing on the links after it (one link to
    // `f`): with `in_degree: 2` the two disagree.
    for (l, c) in
        layouts("{ id: v, title: V, layout: LAYOUT, hubs: { in_degree: 2 }, collapse: [r] }")
    {
        let g = two_to_far(&c, "r");
        let views = views_of(&c, &g);
        let has_edges_view = views.iter().any(|v| v.id == "v/edges");
        let s = layout(&g, &views[0], &Style::default(), &[]);
        assert_eq!(
            has_edges_view,
            !s.badges.is_empty(),
            "{l:?}: views {:?}; the view draws {} badges, so `v/edges` follows it only when it draws one",
            views.iter().map(|v| v.id.as_str()).collect::<Vec<_>>(),
            s.badges.len()
        );
    }
}

#[test]
fn region_named_edges_stays_expanded_beside_the_edges_view() {
    // A region whose part id is `edges`, in a view with hubs: its view would take the id of the
    // view `v/edges` (every link an edge). Decided (coordinator, 2026-11-28): such a part is not
    // collapsed, and `metaspec pack check` names it (`view_region_edges`).
    for (l, c) in
        layouts("{ id: v, title: V, layout: LAYOUT, hubs: { parts: [f] }, collapse: [edges] }")
    {
        let g = two_to_far(&c, "edges");
        let views = views_of(&c, &g);
        let ids: Vec<&str> = views.iter().map(|v| v.id.as_str()).collect();
        let mut unique = ids.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(
            unique.len(),
            ids.len(),
            "{l:?}: view ids are unique: {ids:?}"
        );
        assert!(
            views.iter().all(|v| v.region.is_none()),
            "{l:?}: no view of the region: {ids:?}"
        );
        let s = layout(&g, &views[0], &Style::default(), &[]);
        assert!(
            s.boxes.iter().all(|b| b.collapsed.is_none()),
            "{l:?}: the region named edges stays expanded"
        );
    }
}
