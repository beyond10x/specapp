//! The manifest: the schema and the decoder agree, and a manifest the schema refuses is
//! refused with the place named.

use super::common::{manifest_of, packs, repo, schema_errors};
use metaspec_core::known;
use metaspec_core::manifest::{self, PALETTE, Visibility, parse};
use serde_json::Value;

fn minimal() -> String {
    String::from(
        "format: metaspec-pack/1\npack: sample\ntitle: Sample\nvisibility: public\nblocks: { path: b }\n",
    )
}

#[test]
fn every_fixture_manifest_follows_the_schema_and_decodes() {
    for (name, dir) in packs() {
        let text = std::fs::read_to_string(manifest_of(&dir)).unwrap();
        let value: Value = serde_yaml_ng::from_str(&text).unwrap();
        assert_eq!(
            schema_errors("metaspec-pack-1.schema.json", &value),
            Vec::<String>::new(),
            "{name}"
        );
        let m = parse(&text).unwrap_or_else(|e| panic!("{name}: {e:?}"));
        assert_eq!(m.pack, name);
    }
}

#[test]
fn every_manifest_under_packs_is_known_public_and_named_after_its_pack() {
    let mut files: Vec<String> = std::fs::read_dir(repo().join("packs"))
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    files.sort();
    let names: Vec<String> = files
        .iter()
        .map(|f| {
            f.strip_suffix(".yaml")
                .unwrap_or_else(|| panic!("packs/ holds manifests only: {f}"))
                .to_string()
        })
        .collect();
    assert!(!names.is_empty());
    assert_eq!(names, known::names());
    for name in &names {
        let text = std::fs::read_to_string(repo().join(format!("packs/{name}.yaml"))).unwrap();
        assert_eq!(known::manifest(name), Some(text.as_str()), "{name}");
        let m = parse(&text).unwrap_or_else(|e| panic!("{name}: {e:?}"));
        assert_eq!(&m.pack, name);
        assert_eq!(
            m.visibility,
            Visibility::Public,
            "{name}: packs/ holds public packs only"
        );
    }
    assert_eq!(known::manifest("no-such-pack"), None);
}

#[test]
fn a_check_adapter_takes_each_subject_rule_alone() {
    let with = |subject: &str| {
        format!(
            "{}tool:\n  run: [t]\n  verbs:\n    check:\n      findings:\n        - {{ at: /f, rule: \"{{/r}}\", gate: true, message: m, subject: {subject} }}\n",
            minimal()
        )
    };
    assert!(parse(&with("document")).is_ok());
    assert!(parse(&with("{ id: \"{/s}\" }")).is_ok());
    assert!(
        parse(&with(
            "{ id: \"{/s}\", link_when_contains: \"->\", unknown: document }"
        ))
        .is_ok()
    );
    assert!(
        parse(&with(
            "{ id: \"{/s}\", kind: \"{/k}\", kinds: { a: part, b: link } }"
        ))
        .is_ok()
    );
    assert!(
        parse(&with("part")).is_err(),
        "a fixed subject is the document"
    );
    assert!(
        parse(&with(
            "{ id: \"{/s}\", link_when_contains: \"->\", kind: \"{/k}\", kinds: { a: part } }"
        ))
        .is_err(),
        "two rules at once"
    );
    assert!(
        parse(&with("{ id: \"{/s}\", kind: \"{/k}\" }")).is_err(),
        "kind without kinds"
    );
    assert!(parse(&with("{ id: \"{/s}\", kinds: { a: elsewhere } }")).is_err());
    assert!(parse(&with("{ id: \"{/s}\", unknown: drop }")).is_err());
    let m = parse(&format!(
        "{}tool:\n  run: [t]\n  verbs:\n    check: {{}}\n",
        minimal()
    ))
    .unwrap();
    assert_eq!(m.level(), "L3");
    assert_eq!(m.tool.unwrap().verbs.check.unwrap().exit_codes(), vec![0]);
    let m = parse(&format!("{}tool:\n  run: [t]\n", minimal())).unwrap();
    assert_eq!(m.level(), "L0", "a tool without a verb adds no level");
    assert!(
        parse(&format!(
            "{}tool:\n  run: [t]\n  verbs:\n    describe: {{}}\n",
            minimal()
        ))
        .is_err(),
        "a verb metaspec does not offer"
    );
    assert!(
        parse(&format!(
            "{}tool:\n  run: [t]\n  verbs:\n    check: {{ at: \"0\" }}\n",
            minimal()
        ))
        .is_err(),
        "a pointer starts with a slash"
    );
}

#[test]
fn the_embedded_schema_is_the_file_in_schema() {
    let file = std::fs::read_to_string(repo().join("schema/metaspec-pack-1.schema.json")).unwrap();
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
fn contain_takes_in_per_or_both() {
    let class = |contain: &str| {
        format!(
            "{}documents:\n  - {{ class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, contain: {contain} }}\n",
            minimal()
        )
    };
    let written = |contain: &str| {
        let m = parse(&class(contain)).unwrap_or_else(|e| panic!("{contain}: {e:?}"));
        serde_json::to_value(&m.documents[0].contain).unwrap()
    };
    assert_eq!(
        written("{ in: holder }"),
        serde_json::json!({ "in": "holder" })
    );
    assert_eq!(
        written("{ per: owner }"),
        serde_json::json!({ "per": "owner" })
    );
    assert_eq!(
        written("{ in: holder, per: owner }"),
        serde_json::json!({ "in": "holder", "per": "owner" })
    );
    assert!(parse(&class("{}")).is_err());
    assert!(parse(&class("{ field: in }")).is_err());
    assert!(parse(&class("{ in: a, per: b, each: c }")).is_err());
    assert!(parse(&class("{ in: '', per: b }")).is_err());
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
    let text = minimal().replace("metaspec-pack/1", "metaspec-pack/2");
    let issues = parse(&text).unwrap_err();
    assert_eq!(issues.len(), 1);
    assert!(issues[0].message.contains("metaspec-pack/1"));
    assert!(issues[0].message.contains("metaspec-pack/2"));
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

#[test]
fn exclude_takes_globs_inside_the_pack() {
    let class = |extra: &str| {
        format!(
            "{}documents:\n  - {{ class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n{extra} }}\n",
            minimal()
        )
    };
    let m = parse(&class("")).unwrap();
    assert!(m.documents[0].exclude.is_empty(), "no exclude by default");
    let m = parse(&class(", exclude: [d/*.side.yaml, d/other.yaml]")).unwrap();
    assert_eq!(
        m.documents[0].exclude,
        vec!["d/*.side.yaml", "d/other.yaml"]
    );
    assert!(parse(&class(", exclude: [../d/*.yaml]")).is_err());
    assert!(parse(&class(", exclude: [/d/*.yaml]")).is_err());
    assert!(parse(&class(", exclude: d/*.yaml")).is_err());
    assert!(parse(&class(", exclude: ['']")).is_err());
}

#[test]
fn hub_view_takes_an_in_degree_named_parts_or_blocks() {
    let view = |hubs: &str| {
        format!(
            "{}documents:\n  - {{ class: c, title: C, paths: [d/*.yaml], shape: parts-links/1, name: n, views: [{{ id: v, title: V, layout: rows{hubs} }}] }}\n",
            minimal()
        )
    };
    let hubs_of = |hubs: &str| {
        let m = parse(&view(hubs)).unwrap_or_else(|e| panic!("{hubs}: {e:?}"));
        m.documents[0].views[0].hubs.clone()
    };
    assert_eq!(hubs_of(""), None, "no hubs by default");
    let h = hubs_of(", hubs: { in_degree: 3 }").unwrap();
    assert_eq!(h.in_degree, Some(3));
    assert!(h.parts.is_empty() && h.blocks.is_empty());
    let h = hubs_of(", hubs: { parts: [p1, 'p-2'], blocks: [b1] }").unwrap();
    assert_eq!(h.in_degree, None);
    assert_eq!(h.parts, vec!["p1", "p-2"]);
    assert_eq!(h.blocks, vec!["b1"]);
    let h = hubs_of(", hubs: { in_degree: 2, parts: [p1] }").unwrap();
    assert_eq!((h.in_degree, h.parts.len()), (Some(2), 1));
    for bad in [
        ", hubs: {}",
        ", hubs: { in_degree: 0 }",
        ", hubs: { in_degree: -1 }",
        ", hubs: { in_degree: two }",
        ", hubs: { parts: [''] }",
        ", hubs: { blocks: ['../b'] }",
        ", hubs: { degree: 3 }",
        ", hubs: 3",
    ] {
        assert!(parse(&view(bad)).is_err(), "{bad} is refused");
    }
    let m = parse(&view("")).unwrap();
    assert!(
        !serde_json::to_string(&m).unwrap().contains("hubs"),
        "a view without hubs writes none, so a manifest's digest stays"
    );
}
