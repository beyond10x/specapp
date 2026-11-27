//! The layouts on both fixture packs: every view of every drawn document is a scene that
//! follows `specapp-scene/1`, the same bytes every time and equal to the committed scene; every
//! part a view shows has one box; boxes of one row do not overlap; every finding about a part
//! or a link of the document is marked in a scene.

use super::common::{drawn, golden, schema_errors};
use specapp_core::tool::Subject;
use specapp_layout::{layout, scene, select, svg};
use std::collections::BTreeSet;

#[test]
fn every_scene_follows_the_schema_and_equals_the_committed_scene() {
    let mut n = 0;
    for d in drawn() {
        for view in &d.views {
            let one = layout(&d.graph, view, &d.style, &d.findings);
            let two = layout(&d.graph, view, &d.style, &d.findings);
            let text = serde_json::to_string_pretty(&one).unwrap() + "\n";
            assert_eq!(text, serde_json::to_string_pretty(&two).unwrap() + "\n");
            let value = serde_json::to_value(&one).unwrap();
            assert_eq!(
                schema_errors("specapp-scene-1.schema.json", &value),
                Vec::<String>::new(),
                "{} {}",
                d.graph.document,
                view.id
            );
            let stem = d
                .graph
                .document
                .rsplit('/')
                .next()
                .unwrap()
                .trim_end_matches(".yaml");
            golden(
                &format!("{}/scenes/{}/{stem}.{}.json", d.pack, d.class, view.id),
                &text,
            );
            n += 1;
        }
    }
    assert!(n >= 6, "{n} scenes");
}

#[test]
fn the_embedded_schema_is_the_file_in_schema() {
    let file =
        std::fs::read_to_string(super::common::repo().join("schema/specapp-scene-1.schema.json"))
            .unwrap();
    assert_eq!(file, scene::SCHEMA);
}

#[test]
fn every_shown_part_has_one_box_and_every_shown_link_one_edge() {
    for d in drawn() {
        for view in &d.views {
            let s = layout(&d.graph, view, &d.style, &d.findings);
            let shown = select(&d.graph, view);
            let parts: Vec<&str> = s
                .boxes
                .iter()
                .filter(|b| !b.ghost)
                .map(|b| b.id.as_str())
                .collect();
            let want: Vec<&str> = shown
                .parts
                .iter()
                .map(|&i| d.graph.parts[i].id.as_str())
                .collect();
            assert_eq!(
                parts.iter().collect::<BTreeSet<_>>(),
                want.iter().collect::<BTreeSet<_>>(),
                "{} {}",
                d.graph.document,
                view.id
            );
            assert_eq!(parts.len(), want.len(), "one box per part");
            assert_eq!(s.edges.len(), shown.links.len());
            let ghosts: Vec<&str> = s
                .boxes
                .iter()
                .filter(|b| b.ghost)
                .map(|b| b.id.as_str())
                .collect();
            assert_eq!(
                ghosts,
                shown.ghosts.iter().map(String::as_str).collect::<Vec<_>>()
            );
            for e in &s.edges {
                assert!(e.points.len() >= 2);
                assert!(
                    s.boxes.iter().any(|b| b.id == e.from) && s.boxes.iter().any(|b| b.id == e.to)
                );
            }
        }
    }
}

#[test]
fn boxes_and_labels_stay_inside_the_scene_and_top_boxes_do_not_overlap() {
    for d in drawn() {
        for view in &d.views {
            let s = layout(&d.graph, view, &d.style, &d.findings);
            for b in &s.boxes {
                assert!(b.x >= 0.0 && b.y >= 0.0, "{}", b.id);
                assert!(b.x + b.w <= s.width && b.y + b.h <= s.height, "{}", b.id);
            }
            for e in &s.edges {
                let r = &e.label_at;
                assert!(r.x >= 0.0 && r.x + r.w <= s.width + 0.5, "{}", e.id);
            }
            let top: Vec<_> = s.boxes.iter().filter(|b| b.parent.is_none()).collect();
            for (i, a) in top.iter().enumerate() {
                for b in &top[i + 1..] {
                    let apart = a.x + a.w <= b.x
                        || b.x + b.w <= a.x
                        || a.y + a.h <= b.y
                        || b.y + b.h <= a.y;
                    assert!(
                        apart,
                        "{} {}: {} and {}",
                        d.graph.document, view.id, a.id, b.id
                    );
                }
            }
        }
    }
}

#[test]
fn every_finding_about_a_part_or_link_of_the_document_is_marked_and_numbered() {
    let mut marked = 0;
    let mut links = 0;
    for d in drawn() {
        let scenes: Vec<_> = d
            .views
            .iter()
            .map(|v| layout(&d.graph, v, &d.style, &d.findings))
            .collect();
        for (i, f) in d.findings.iter().enumerate() {
            let exists = match f.subject() {
                Subject::Part(id) => d.graph.part(id).is_some(),
                Subject::Link(id) => {
                    links += 1;
                    d.graph.link(id).is_some()
                }
                Subject::Document => false,
            };
            let marks = scenes
                .iter()
                .flat_map(|s| s.marks.iter())
                .filter(|m| m.finding == i + 1)
                .count();
            if exists {
                assert!(
                    marks > 0,
                    "{}: finding {} is marked",
                    d.graph.document,
                    i + 1
                );
                marked += 1;
            } else {
                assert_eq!(marks, 0);
            }
        }
        for s in &scenes {
            let drawn = svg::svg(s, "v");
            for m in &s.marks {
                assert!(
                    drawn.contains(&format!(">!{}</text>", m.finding)),
                    "readable without colour"
                );
                assert!(drawn.contains(&format!("href=\"#finding-{}\"", m.finding)));
            }
        }
    }
    assert!(marked >= 4, "{marked}");
    assert!(links >= 2, "a link finding in each fixture pack");
}

#[test]
fn an_implicit_view_is_added_only_when_the_declared_views_leave_something_out() {
    for d in drawn() {
        let declared: Vec<_> = d.views.iter().filter(|v| !v.implicit).collect();
        let implicit = d.views.iter().filter(|v| v.implicit).count();
        let mut parts = BTreeSet::new();
        let mut links = BTreeSet::new();
        for v in &declared {
            let s = select(&d.graph, v);
            parts.extend(s.parts);
            links.extend(s.links);
        }
        let complete = !declared.is_empty()
            && parts.len() == d.graph.parts.len()
            && links.len() == d.graph.links.len();
        assert_eq!(implicit, usize::from(!complete), "{}", d.graph.document);
    }
}
