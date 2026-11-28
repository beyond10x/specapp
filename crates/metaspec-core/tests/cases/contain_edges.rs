//! Edges of `contain: { in, per }` in the reader: fields named `in` and `per` themselves, one
//! holder for both words, a part with one of the two, a missing holder, and the order of groups.

use metaspec_core::graph::{self, Graph};
use metaspec_core::manifest;
use serde_json::json;

fn graph_of(parts: serde_json::Value) -> Graph {
    let text = "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: { path: b }\ndocuments:\n  - { class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, contain: { in: in, per: per } }\n";
    let m = manifest::parse(text).unwrap_or_else(|e| panic!("{e:?}"));
    let doc = json!({ "n": "x", "parts": parts, "links": [] });
    graph::read(&m.documents[0], "d/x.yaml", &doc)
}

fn groups(g: &Graph) -> Vec<(String, String, Vec<String>)> {
    g.groups
        .iter()
        .map(|grp| (grp.id.clone(), grp.contain.clone(), grp.members.clone()))
        .collect()
}

fn s(v: &str) -> String {
    v.to_string()
}

#[test]
fn contain_both_fields_named_in_and_per_read_one_holder_each_and_groups_in_document_order() {
    let g = graph_of(json!({
        "ws": { "block": "scope" },
        "pool": { "block": "scope" },
        "daemon": { "block": "work", "in": "ws" },
        "refresh": { "block": "work", "in": "ws", "per": "pool" },
        "data": { "block": "vol", "per": "daemon" },
        "same": { "block": "work", "in": "pool", "per": "pool" },
        "lost": { "block": "work", "in": "nowhere", "per": "ws" },
    }));
    let pair = |id: &str| {
        let p = g.part(id).unwrap();
        (p.inside.clone(), p.per.clone())
    };
    assert_eq!(pair("refresh"), (Some(s("ws")), Some(s("pool"))));
    assert_eq!(pair("daemon"), (Some(s("ws")), None));
    assert_eq!(pair("data"), (None, Some(s("daemon"))));
    assert_eq!(pair("lost"), (Some(s("nowhere")), Some(s("ws"))));
    assert_eq!(
        groups(&g),
        [
            (s("ws"), s("in"), vec![s("daemon"), s("refresh")]),
            (s("ws"), s("per"), vec![s("lost")]),
            (s("pool"), s("in"), vec![s("same")]),
            (s("pool"), s("per"), vec![s("refresh"), s("same")]),
            (s("daemon"), s("per"), vec![s("data")]),
        ]
    );
}

#[test]
fn contain_empty_object_and_null_values_are_refused() {
    let class = |contain: &str| {
        format!(
            "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: {{ path: b }}\ndocuments:\n  - {{ class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, contain: {contain} }}\n"
        )
    };
    for bad in ["{}", "{ in: ~ }", "{ in: a, per: ~ }", "{ per: [a] }"] {
        assert!(manifest::parse(&class(bad)).is_err(), "{bad} loads");
    }
}
