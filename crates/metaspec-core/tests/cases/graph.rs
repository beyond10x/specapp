//! The reader of `parts-links/1` documents: every document of both fixture packs reads into a
//! graph that follows `metaspec-graph/1`, with containment, markers and link ids as the format
//! says.

use super::common::{block_names, load_pack, load_with_aliases, packs, schema_errors};
use metaspec_core::graph::{self, Graph};
use metaspec_core::manifest::{self, Contain, Shape};
use serde_json::Value;
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
            schema_errors("metaspec-graph-1.schema.json", &value),
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
        std::fs::read_to_string(super::common::repo().join("schema/metaspec-graph-1.schema.json"))
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
                let Some(Contain { inside, per }) = &contain else {
                    panic!("{name}: a group without containment");
                };
                match group.contain.as_str() {
                    "in" => {
                        assert!(inside.is_some(), "{name}: an `in` group, no `in` field");
                        assert_eq!(part.inside.as_deref(), Some(group.id.as_str()), "{name}");
                    }
                    "per" => {
                        assert!(per.is_some(), "{name}: a `per` group, no `per` field");
                        assert_eq!(part.per.as_deref(), Some(group.id.as_str()), "{name}");
                    }
                    other => panic!("{name}: a group of {other}"),
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

#[test]
fn contain_both_keeps_inside_and_per_of_a_part_and_a_group_per_holder_and_word() {
    let text = "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: { path: b }\ndocuments:\n  - { class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, contain: { in: holder, per: owner } }\n";
    let m = manifest::parse(text).unwrap_or_else(|e| panic!("{e:?}"));
    let doc: Value = serde_yaml_ng::from_str(
        "n: x\nparts:\n  host: { block: h }\n  log: { block: l, holder: host }\n  site: { block: s }\n  spare: { block: w, holder: site }\n  worker: { block: w, holder: site, owner: host }\nlinks: []\n",
    )
    .unwrap();
    let g = graph::read(&m.documents[0], "d/x.yaml", &doc);
    assert_eq!(
        schema_errors(
            "metaspec-graph-1.schema.json",
            &serde_json::to_value(&g).unwrap()
        ),
        Vec::<String>::new()
    );
    let worker = g.part("worker").unwrap();
    assert_eq!(
        (worker.inside.as_deref(), worker.per.as_deref()),
        (Some("site"), Some("host"))
    );
    let spare = g.part("spare").unwrap();
    assert_eq!(
        (spare.inside.as_deref(), spare.per.as_deref()),
        (Some("site"), None)
    );
    let groups: Vec<(&str, &str, Vec<&str>)> = g
        .groups
        .iter()
        .map(|grp| {
            (
                grp.id.as_str(),
                grp.contain.as_str(),
                grp.members.iter().map(String::as_str).collect(),
            )
        })
        .collect();
    assert_eq!(
        groups,
        [
            ("host", "in", vec!["log"]),
            ("host", "per", vec!["worker"]),
            ("site", "in", vec!["spare", "worker"]),
        ]
    );
}

#[test]
fn contain_both_in_a_fixture_document_keeps_both_and_a_group_of_each() {
    let mut both = 0;
    for (name, contain, g, _) in graphs() {
        for part in &g.parts {
            let (Some(inside), Some(per)) = (&part.inside, &part.per) else {
                continue;
            };
            let class = serde_json::to_value(&contain).unwrap();
            assert!(
                class.get("in").is_some() && class.get("per").is_some(),
                "{name}"
            );
            let member_of = |id: &str, word: &str| {
                g.groups.iter().any(|grp| {
                    grp.id == id && grp.contain == word && grp.members.contains(&part.id)
                })
            };
            assert!(member_of(inside, "in"), "{name} {}", part.id);
            assert!(member_of(per, "per"), "{name} {}", part.id);
            both += 1;
        }
    }
    assert!(
        both >= 1,
        "a fixture document has a part both in and per another"
    );
}

/// A word no fixture pack has as a block, given an alias in the tests below.
const ALIAS: &str = "zz_alias_word";
/// A word no fixture pack has as a block and no alias names.
const NO_BLOCK: &str = "zz_no_block_word";

/// The first `parts-links/1` class of a pack.
fn graph_class(pack: &metaspec_core::Pack) -> &manifest::DocumentClass {
    pack.manifest
        .documents
        .iter()
        .find(|c| c.shape == Shape::PartsLinks)
        .expect("a parts-links/1 class")
}

#[test]
fn alias_names_the_target_block() {
    for (name, dir) in packs() {
        let blocks = block_names(&dir);
        let target = blocks[0].as_str();
        let (report, pack) = load_with_aliases(&dir, &[(ALIAS, target)]);
        assert!(report.valid, "{name}: {:?}", report.errors);
        let pack = pack.expect("loads");
        let class = graph_class(&pack);
        let doc: Value = serde_yaml_ng::from_str(&format!(
            "n: x\nparts:\n  a: {{ block: {ALIAS} }}\n  b: {{ block: {target} }}\nlinks:\n  - {{ from: a, to: b, carries: c }}\n"
        ))
        .unwrap();
        let g = graph::read_with_aliases(class, "d/x.yaml", &doc, &pack.manifest.blocks.aliases);
        assert_eq!(
            schema_errors(
                "metaspec-graph-1.schema.json",
                &serde_json::to_value(&g).unwrap()
            ),
            Vec::<String>::new(),
            "{name}"
        );
        let block_of = |id: &str| g.part(id).unwrap().block.clone();
        assert_eq!(
            block_of("a").as_deref(),
            Some(target),
            "{name}: the aliased word stands for its block"
        );
        assert_eq!(block_of("b").as_deref(), Some(target), "{name}");
        assert!(!g.part("a").unwrap().fields.contains_key("block"));
        let plain = graph::read(class, "d/x.yaml", &doc);
        assert_eq!(
            plain.part("a").unwrap().block.as_deref(),
            Some(ALIAS),
            "{name}: graph::read applies no alias"
        );
    }
}

#[test]
fn alias_unknown_word_keeps_its_word() {
    for (name, dir) in packs() {
        let blocks = block_names(&dir);
        assert!(!blocks.iter().any(|b| b == NO_BLOCK), "{name}");
        let (report, pack) = load_with_aliases(&dir, &[(ALIAS, &blocks[0])]);
        assert!(report.valid, "{name}: {:?}", report.errors);
        let pack = pack.expect("loads");
        let class = graph_class(&pack);
        let doc: Value = serde_yaml_ng::from_str(&format!(
            "n: x\nparts:\n  a: {{ block: {NO_BLOCK} }}\n  b: {{ k: 1 }}\nlinks: []\n"
        ))
        .unwrap();
        let g = graph::read_with_aliases(class, "d/x.yaml", &doc, &pack.manifest.blocks.aliases);
        assert_eq!(
            g.part("a").unwrap().block.as_deref(),
            Some(NO_BLOCK),
            "{name}: a word with neither alias nor block keeps its word"
        );
        assert_eq!(g.part("b").unwrap().block, None, "{name}: no block, none");
    }
}
