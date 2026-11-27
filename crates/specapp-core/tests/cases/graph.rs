//! The reader of `parts-links/1` documents: every document of both fixture packs reads into a
//! graph that follows `specapp-graph/1`, with containment, markers and link ids as the format
//! says.

use super::common::{load_pack, packs, schema_errors};
use serde_json::Value;
use specapp_core::graph::{self, Graph};
use specapp_core::manifest::{Contain, Shape};
use std::collections::BTreeSet;

fn graphs() -> Vec<(String, Option<Contain>, Graph, Value)> {
    let mut out = Vec::new();
    for (name, dir) in packs() {
        let pack = load_pack(&dir).1.expect("loads");
        for (class, documents) in pack.manifest.documents.iter().zip(&pack.documents) {
            if class.shape != Shape::PartsLinks {
                continue;
            }
            for document in documents {
                let value = document.value.clone().expect("YAML");
                out.push((
                    name.clone(),
                    class.contain.clone(),
                    graph::read(class, &document.path, &value),
                    value,
                ));
            }
        }
    }
    assert!(out.len() >= 3);
    out
}

#[test]
fn every_document_reads_into_a_graph_that_follows_the_schema() {
    for (name, _, g, doc) in graphs() {
        let value = serde_json::to_value(&g).unwrap();
        assert_eq!(
            schema_errors("specapp-graph-1.schema.json", &value),
            Vec::<String>::new(),
            "{name} {}",
            g.document
        );
        assert_eq!(g.parts.len(), doc["parts"].as_object().unwrap().len());
        assert_eq!(g.links.len(), doc["links"].as_array().unwrap().len());
        let ids: BTreeSet<&str> = g.links.iter().map(|l| l.id.as_str()).collect();
        assert_eq!(ids.len(), g.links.len(), "link ids are unique");
        for part in &g.parts {
            assert!(!part.fields.contains_key("block"));
            assert!(part.fields.values().all(|v| !graph::is_marker(v)));
            assert_eq!(part.on_border, g.border.contains(&part.id));
        }
    }
}

#[test]
fn the_embedded_schema_is_the_file_in_schema() {
    let file =
        std::fs::read_to_string(super::common::repo().join("schema/specapp-graph-1.schema.json"))
            .unwrap();
    assert_eq!(file, graph::SCHEMA);
}

#[test]
fn containment_follows_the_class_in_one_pack_and_per_in_the_other() {
    let mut seen = BTreeSet::new();
    for (name, contain, g, _) in graphs() {
        for group in &g.groups {
            for member in &group.members {
                let part = g.part(member).unwrap();
                match &contain {
                    Some(Contain::In(_)) => {
                        assert_eq!(part.inside.as_deref(), Some(group.id.as_str()), "{name}");
                        assert_eq!(group.contain, "in");
                    }
                    Some(Contain::Per(_)) => {
                        assert_eq!(part.per.as_deref(), Some(group.id.as_str()), "{name}");
                        assert_eq!(group.contain, "per");
                    }
                    None => panic!("{name}: a group without containment"),
                }
                seen.insert(group.contain.clone());
            }
        }
    }
    assert_eq!(
        seen,
        BTreeSet::from([String::from("in"), String::from("per")])
    );
}

#[test]
fn a_marked_place_is_read_as_its_value_and_listed() {
    let mut markers = 0;
    for (_, _, g, doc) in graphs() {
        for part in &g.parts {
            for m in &part.markers {
                let raw = &doc["parts"][&part.id][&m.path];
                let written = graph::marked(raw).expect("written as a marked place");
                assert_eq!(written.value, m.value);
                assert_eq!(written.vary, m.vary);
                assert_eq!(part.fields[&m.path], m.value);
                markers += 1;
            }
        }
    }
    assert!(markers >= 2, "both packs mark a place");
}

#[test]
fn a_repeated_link_gets_a_number_and_a_missing_end_a_question_mark() {
    let (_, dir) = packs().remove(0);
    let pack = load_pack(&dir).1.unwrap();
    let class = pack
        .manifest
        .documents
        .iter()
        .find(|c| c.shape == Shape::PartsLinks)
        .unwrap();
    let doc: Value = serde_yaml_ng::from_str(
        "n: x\nparts: { a: { block: b, k: { value: 1, vary: [1, 2] } } }\nlinks:\n  - { from: a, to: a, carries: c }\n  - { from: a, to: a, carries: c, w: { value: 2, vary: [2] } }\n  - { to: a }\n  - 7\nborder: a\nextra: { deep: { value: 3, vary: { min: 1, max: 4 } } }\n",
    )
    .unwrap();
    let g = graph::read(class, "d/x.yaml", &doc);
    let ids: Vec<&str> = g.links.iter().map(|l| l.id.as_str()).collect();
    assert_eq!(ids, ["a->a:c", "a->a:c#2", "?->a:?"]);
    assert_eq!(g.links[1].markers.len(), 1);
    assert_eq!(g.links[1].fields["w"], 2);
    assert_eq!(g.parts[0].markers[0].path, "k");
    assert!(g.parts[0].on_border);
    assert_eq!(g.markers.len(), 1);
    assert_eq!(g.markers[0].path, "extra.deep");
    assert_eq!(
        g.name, "x",
        "the file stem when the class's name key is missing"
    );
}
