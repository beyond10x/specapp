//! Edges of a class that contains by `in` and by `per` at once, on hand-written graphs: the
//! same holder for both, an `in` container the view leaves out, and a cycle through both words.

use metaspec_core::graph::{self, Graph};
use metaspec_core::manifest::{self, Layout, RowsBy};
use metaspec_layout::scene::Scene;
use metaspec_layout::{Style, ViewSpec, layout};
use serde_json::json;

fn graph_of(parts: serde_json::Value) -> Graph {
    let text = "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: { path: b }\ndocuments:\n  - { class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, contain: { in: in, per: per } }\n";
    let m = manifest::parse(text).unwrap_or_else(|e| panic!("{e:?}"));
    let doc = json!({ "n": "x", "parts": parts, "links": [] });
    graph::read(&m.documents[0], "d/x.yaml", &doc)
}

fn view(layout_: Layout, blocks: &[&str]) -> ViewSpec {
    ViewSpec {
        id: String::from("v"),
        title: String::from("V"),
        layout: layout_,
        rows: RowsBy::Layer,
        blocks: blocks.iter().map(|b| b.to_string()).collect(),
        carries: Vec::new(),
        implicit: false,
    }
}

fn draw(g: &Graph, v: &ViewSpec) -> Scene {
    layout(g, v, &Style::default(), &[])
}

fn one_box<'a>(s: &'a Scene, id: &str) -> &'a metaspec_layout::scene::SceneBox {
    let found: Vec<_> = s.boxes.iter().filter(|b| b.id == id && !b.ghost).collect();
    assert_eq!(
        found.len(),
        1,
        "{id}: {:?}",
        s.boxes.iter().map(|b| &b.id).collect::<Vec<_>>()
    );
    found[0]
}

#[test]
fn contain_both_same_holder_rows_line_does_not_repeat_the_host_it_is_drawn_in() {
    // A part that sits in and exists per the same part: rows draws it inside that host, so its
    // line names no containment the drawing already shows (nested does the same, `nested.rs`).
    let g = graph_of(json!({
        "echo": { "block": "alpha" },
        "delta": { "block": "bravo", "in": "echo", "per": "echo" },
    }));
    let s = draw(&g, &view(Layout::Rows, &[]));
    let delta = one_box(&s, "delta");
    assert_eq!(delta.parent.as_deref(), Some("echo"));
    assert!(
        !delta.label.ends_with(" · in echo"),
        "drawn inside echo, and its line says it again: {:?}",
        delta.label
    );
}

#[test]
fn contain_both_nested_in_container_not_shown_draws_it_in_its_per_and_notes_its_in() {
    // design/metaspec.md: inside its `in` container, "its `per` container when the `in` one is
    // not shown", and the other is written as a line of its box.
    let g = graph_of(json!({
        "host": { "block": "alpha" },
        "owner": { "block": "bravo" },
        "item": { "block": "bravo", "in": "host", "per": "owner" },
    }));
    let s = draw(&g, &view(Layout::Nested, &["bravo"]));
    let item = one_box(&s, "item");
    assert_eq!(item.parent.as_deref(), Some("owner"));
    assert!(
        item.lines.contains(&String::from("in host")),
        "{:?}",
        item.lines
    );
    assert!(
        !item.lines.contains(&String::from("per owner")),
        "{:?}",
        item.lines
    );
}

#[test]
fn contain_both_cycle_through_in_and_per_draws_every_part_once_and_nests_no_box_in_itself() {
    let g = graph_of(json!({
        "a": { "block": "x", "in": "b", "per": "c" },
        "b": { "block": "x", "per": "a" },
        "c": { "block": "x", "in": "a" },
    }));
    for v in [view(Layout::Nested, &[]), view(Layout::Rows, &[])] {
        let s = draw(&g, &v);
        for id in ["a", "b", "c"] {
            let mut seen = vec![id.to_string()];
            let mut cur = one_box(&s, id).parent.clone();
            while let Some(p) = cur {
                assert!(
                    !seen.contains(&p),
                    "{:?}: {id} is drawn inside itself via {seen:?}",
                    v.layout
                );
                seen.push(p.clone());
                cur = one_box(&s, &p).parent.clone();
            }
        }
    }
}
