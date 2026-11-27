//! The manifest: the schema and the decoder agree, and a manifest the schema refuses is
//! refused with the place named.

use super::common::{packs, repo, schema_errors};
use serde_json::Value;
use specapp_core::manifest::{self, Contain, PALETTE, parse};

fn minimal() -> String {
    String::from(
        "format: specapp-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: { path: b }\n",
    )
}

#[test]
fn every_fixture_manifest_follows_the_schema_and_decodes() {
    for (name, dir) in packs() {
        let text = std::fs::read_to_string(dir.join("specapp.yaml")).unwrap();
        let value: Value = serde_yaml_ng::from_str(&text).unwrap();
        assert_eq!(
            schema_errors("specapp-pack-1.schema.json", &value),
            Vec::<String>::new(),
            "{name}"
        );
        assert!(parse(&text).is_ok(), "{name}");
    }
}

#[test]
fn the_embedded_schema_is_the_file_in_schema() {
    let file = std::fs::read_to_string(repo().join("schema/specapp-pack-1.schema.json")).unwrap();
    assert_eq!(file, manifest::SCHEMA);
}

#[test]
fn the_palette_is_the_colour_list_of_the_schema() {
    let schema: Value = serde_json::from_str(manifest::SCHEMA).unwrap();
    let colours: Vec<&str> = schema["$defs"]["colour"]["enum"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(colours, PALETTE);
}

#[test]
fn a_minimal_manifest_is_level_zero_with_default_kpi_fields() {
    let m = parse(&minimal()).expect("minimal manifest");
    assert_eq!(m.level(), "L0");
    assert!(m.kpis.is_none());
    let m = parse(&format!("{}kpis: {{ path: k.yaml }}\n", minimal())).unwrap();
    assert_eq!(m.level(), "L1");
    let fields = &m.kpis.unwrap().fields;
    assert_eq!(
        (fields.measure.as_str(), fields.modes.as_str()),
        ("measure", "modes")
    );
}

#[test]
fn extends_names_another_pack() {
    let m = parse(&format!("{}extends: base-pack\n", minimal())).unwrap();
    assert_eq!(m.extends.as_deref(), Some("base-pack"));
    assert!(parse(&format!("{}extends: ../base\n", minimal())).is_err());
}

#[test]
fn contain_takes_exactly_one_of_in_and_per() {
    let class = |contain: &str| {
        format!(
            "{}documents:\n  - {{ class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, contain: {contain} }}\n",
            minimal()
        )
    };
    let m = parse(&class("{ in: holder }")).unwrap();
    assert_eq!(
        m.documents[0].contain,
        Some(Contain::In(String::from("holder")))
    );
    let m = parse(&class("{ per: owner }")).unwrap();
    assert_eq!(
        m.documents[0].contain,
        Some(Contain::Per(String::from("owner")))
    );
    assert!(parse(&class("{ in: a, per: b }")).is_err());
    assert!(parse(&class("{}")).is_err());
    assert!(parse(&class("{ field: in }")).is_err());
}

#[test]
fn a_records_class_takes_no_views_and_a_graph_class_no_tables() {
    let records = format!(
        "{}documents:\n  - {{ class: c, title: C, paths: [d/*.yaml], shape: records/1, name: n, views: [{{ id: v, title: V, layout: rows }}] }}\n",
        minimal()
    );
    assert!(parse(&records).is_err());
    let graph = format!(
        "{}documents:\n  - {{ class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, tables: [{{ title: T, at: x, columns: [a] }}] }}\n",
        minimal()
    );
    assert!(parse(&graph).is_err());
}

#[test]
fn a_refusal_names_the_place_and_every_violation() {
    let text = format!(
        "{}presentation:\n  families:\n    f: {{ blocks: [b], colour: '#ff0000', icon: i }}\nunknown_key: 1\n",
        minimal()
    );
    let issues = parse(&text).unwrap_err();
    assert!(issues.len() >= 2, "{issues:?}");
    assert!(
        issues
            .iter()
            .any(|i| i.message.contains("/presentation/families/f/colour"))
    );
    assert!(issues.iter().all(|i| i.code == "manifest_invalid"));
}

#[test]
fn another_format_version_is_refused_with_both_versions_named() {
    let text = minimal().replace("specapp-pack/1", "specapp-pack/2");
    let issues = parse(&text).unwrap_err();
    assert_eq!(issues.len(), 1);
    assert!(issues[0].message.contains("specapp-pack/2"));
    assert!(issues[0].message.contains("specapp-pack/1"));
}

#[test]
fn a_manifest_path_with_dot_dot_is_refused_by_the_schema() {
    let text = minimal().replace("path: b", "path: ../b");
    assert!(parse(&text).is_err());
    let text = minimal().replace("path: b", "path: /b");
    assert!(parse(&text).is_err());
    let text = minimal().replace("path: b", "path: a..b/c");
    assert!(parse(&text).is_ok());
}
