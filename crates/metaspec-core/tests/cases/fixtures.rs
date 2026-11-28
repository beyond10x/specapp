//! Both fixture packs load with the counts their expectation names, no warning, and a report
//! that follows `metaspec-pack-check/1`.

use super::common::{
    block_names, codes, expected, load_changed, load_pack, load_with_aliases, packs, repo,
    schema_errors,
};
use metaspec_core::report::ToolState;
use metaspec_core::words::{Kind, words};
use std::collections::BTreeMap;

#[test]
fn every_fixture_pack_loads_with_its_expected_counts() {
    for (name, dir) in packs() {
        let (report, pack) = load_pack(&dir);
        assert!(report.valid, "{name}: {:?}", report.errors);
        assert!(pack.is_some(), "{name}");
        assert!(report.warnings.is_empty(), "{name}: {:?}", report.warnings);
        let want = expected(&name);
        let got = serde_json::to_value(report.counts.as_ref().expect("counts")).unwrap();
        assert_eq!(got, want["counts"], "{name}");
        assert_eq!(report.level, want["level"].as_str(), "{name}");
        let state = serde_json::to_value(report.tool.as_ref().expect("a tool row").state).unwrap();
        assert_eq!(state, want["tool_state"], "{name}");
    }
}

#[test]
fn the_fixture_packs_differ_in_containment_and_kpi_fields() {
    let manifests: Vec<_> = packs()
        .into_iter()
        .map(|(_, dir)| load_pack(&dir).1.expect("loads").manifest)
        .collect();
    let contains: Vec<_> = manifests
        .iter()
        .flat_map(|m| m.documents.iter().filter_map(|c| c.contain.clone()))
        .map(|c| (c.inside.is_some(), c.per.is_some()))
        .collect();
    assert!(
        contains.iter().any(|&(inside, _)| inside)
            && contains.iter().any(|&(inside, per)| !inside && per),
        "one pack contains with `in`, another with `per` only: {contains:?}"
    );
    let measures: Vec<_> = manifests
        .iter()
        .filter_map(|m| m.kpis.as_ref().map(|k| k.fields.measure.clone()))
        .collect();
    assert!(measures.windows(2).all(|w| w[0] != w[1]), "{measures:?}");
    let with_families = manifests
        .iter()
        .filter(|m| {
            m.presentation
                .as_ref()
                .is_some_and(|p| !p.families.is_empty())
        })
        .count();
    assert!(with_families >= 1 && with_families < manifests.len());
}

#[test]
fn the_report_of_every_fixture_pack_follows_its_schema() {
    for (name, dir) in packs() {
        let (report, _) = load_pack(&dir);
        let value = serde_json::to_value(&report).unwrap();
        assert_eq!(
            schema_errors("metaspec-pack-check-1.schema.json", &value),
            Vec::<String>::new(),
            "{name}"
        );
    }
}

#[test]
fn a_refused_report_follows_the_schema() {
    let (_, dir) = packs().remove(0);
    let (report, pack) = load_changed(&dir, |m| m.blocks.path = String::from("../elsewhere"));
    assert!(!report.valid);
    assert!(pack.is_none());
    let value = serde_json::to_value(&report).unwrap();
    assert_eq!(
        schema_errors("metaspec-pack-check-1.schema.json", &value),
        Vec::<String>::new()
    );
}

#[test]
fn a_block_left_out_of_every_family_is_named() {
    for (name, dir) in packs() {
        let (_, pack) = load_pack(&dir);
        let pack = pack.expect("loads");
        let Some(first) = pack
            .manifest
            .presentation
            .as_ref()
            .and_then(|p| p.families.keys().next().cloned())
        else {
            continue;
        };
        let (report, _) = load_changed(&dir, |m| {
            let families = &mut m.presentation.as_mut().unwrap().families;
            families.get_mut(&first).unwrap().blocks.remove(0);
        });
        assert!(report.valid, "{name}");
        assert_eq!(
            codes(&report.warnings),
            vec!["block_in_no_family"],
            "{name}"
        );
    }
}

#[test]
fn a_block_in_two_families_and_an_unknown_block_are_named() {
    for (name, dir) in packs() {
        let (_, pack) = load_pack(&dir);
        let pack = pack.expect("loads");
        let Some(presentation) = pack.manifest.presentation.clone() else {
            continue;
        };
        if presentation.families.len() < 2 {
            continue;
        }
        let keys: Vec<String> = presentation.families.keys().cloned().collect();
        let (report, _) = load_changed(&dir, |m| {
            let families = &mut m.presentation.as_mut().unwrap().families;
            let moved = families[&keys[0]].blocks[0].clone();
            families.get_mut(&keys[1]).unwrap().blocks.push(moved);
            families
                .get_mut(&keys[1])
                .unwrap()
                .blocks
                .push(String::from("no_such_block_anywhere"));
        });
        let mut got = codes(&report.warnings);
        got.sort();
        assert_eq!(
            got,
            vec!["block_in_two_families", "family_unknown_block"],
            "{name}"
        );
    }
}

#[test]
fn a_tool_that_cannot_be_started_is_named_and_nothing_is_run() {
    for (name, dir) in packs() {
        let (report, _) = load_changed(&dir, |m| {
            m.tool.as_mut().expect("a fixture tool").run[0] =
                String::from("no-such-program-on-any-path");
        });
        assert!(report.valid, "{name}");
        assert_eq!(
            report.tool.as_ref().unwrap().state,
            ToolState::NotFound,
            "{name}"
        );
        assert_eq!(codes(&report.warnings), vec!["tool_not_started"], "{name}");

        let (report, _) = load_changed(&dir, |m| {
            m.tool.as_mut().unwrap().run = vec![String::from("./blocks/system.yaml")];
        });
        assert_eq!(
            report.tool.as_ref().unwrap().state,
            ToolState::NotFound,
            "{name}: not executable"
        );
    }
}

#[test]
fn an_empty_class_and_a_view_of_an_unknown_block_are_named() {
    for (name, dir) in packs() {
        let mut had_view = false;
        let (report, _) = load_changed(&dir, |m| {
            m.documents[0].paths = vec![String::from("nowhere/*.yaml")];
            if let Some(view) = m.documents[0].views.first_mut() {
                view.blocks.push(String::from("no_such_block_anywhere"));
                had_view = true;
            }
        });
        assert!(report.valid, "{name}");
        let mut want = vec!["class_empty"];
        if had_view {
            want.push("view_unknown_block");
        }
        assert_eq!(codes(&report.warnings), want, "{name}");
    }
}

#[test]
fn hub_block_the_blocks_do_not_have_is_named_like_a_view_block() {
    for (name, dir) in packs() {
        let (report, _) = load_changed(&dir, |m| {
            let view = m.documents[0].views.first_mut().expect("a view");
            view.hubs = Some(metaspec_core::manifest::Hubs {
                blocks: vec![String::from("no_such_block_anywhere")],
                ..Default::default()
            });
        });
        assert!(report.valid, "{name}");
        assert_eq!(
            codes(&report.warnings),
            vec!["view_unknown_block"],
            "{name}"
        );
        assert!(
            report.warnings[0]
                .message
                .contains("no_such_block_anywhere"),
            "{name}: {:?}",
            report.warnings
        );
    }
}

#[test]
fn region_collapse_block_the_blocks_do_not_have_is_named_like_a_view_block() {
    for (name, dir) in packs() {
        let (report, _) = load_changed(&dir, |m| {
            let view = m.documents[0].views.first_mut().expect("a view");
            view.collapse = Some(metaspec_core::manifest::Collapse::Blocks(
                metaspec_core::manifest::CollapseBlocks {
                    blocks: vec![String::from("no_such_block_anywhere")],
                },
            ));
        });
        assert!(report.valid, "{name}");
        assert_eq!(
            codes(&report.warnings),
            vec!["view_unknown_block"],
            "{name}"
        );
        assert!(
            report.warnings[0]
                .message
                .contains("no_such_block_anywhere"),
            "{name}: {:?}",
            report.warnings
        );
    }
}

#[test]
fn region_collapse_of_no_region_and_of_a_region_named_edges_are_named() {
    use metaspec_core::manifest::{Collapse, Every};
    let mut tested = 0;
    for (name, fixture) in packs() {
        let manifest = metaspec_core::pack::read_manifest(&super::common::manifest_of(&fixture))
            .expect("manifest");
        let class = &manifest.documents[0];
        let Some(inside) = class.contain.as_ref().and_then(|c| c.inside.clone()) else {
            continue;
        };
        let per = class.contain.as_ref().and_then(|c| c.per.clone());
        let dir = super::common::scratch(&format!("region-warnings-{name}")).join(&name);
        super::common::copy_dir(&fixture, &dir);
        // One document more: a region `edges` with one part in it, and a part another part is
        // only `per` (when the class has `per`).
        let mut parts = serde_json::json!({
            "edges": { "block": "zz_kilo" },
            "zz_inner": { "block": "zz_kilo", inside.as_str(): "edges" },
            "zz_host": { "block": "zz_kilo" },
        });
        if let Some(per) = &per {
            parts["zz_each"] = serde_json::json!({ "block": "zz_kilo", per.as_str(): "zz_host" });
        }
        let doc = serde_json::json!({ class.name.as_str(): "zz", "parts": parts, "links": [] });
        let path = class.paths[0].replace('*', "zz-region-edges");
        std::fs::write(dir.join(&path), doc.to_string()).unwrap();

        let with = |collapse: Collapse| {
            load_changed(&dir, |m| {
                let view = m.documents[0].views.first_mut().expect("a view");
                view.blocks.clear();
                view.collapse = Some(collapse);
            })
            .0
        };
        let ids = |list: &[&str]| Collapse::Parts(list.iter().map(|s| s.to_string()).collect());
        let report = with(ids(&["edges", "zz_host", "zz_nowhere"]));
        assert!(report.valid, "{name}");
        assert_eq!(
            codes(&report.warnings),
            vec![
                "view_unknown_collapse",
                "view_unknown_collapse",
                "view_region_edges"
            ],
            "{name}: {:?}",
            report.warnings
        );
        assert!(report.warnings[0].message.contains("zz_host"), "{name}");
        assert!(report.warnings[1].message.contains("zz_nowhere"), "{name}");
        assert!(
            report.warnings[2].message.contains("edges")
                && report.warnings[2].message.contains(&path),
            "{name}: {}",
            report.warnings[2].message
        );
        let report = with(Collapse::Every(Every::All));
        assert_eq!(codes(&report.warnings), vec!["view_region_edges"], "{name}");
        let report = with(ids(&["zz_inner"]));
        assert_eq!(
            codes(&report.warnings),
            vec!["view_unknown_collapse"],
            "{name}: a part no part sits in is no region"
        );
        let (report, _) = load_changed(&fixture, |m| {
            let view = m.documents[0].views.first_mut().expect("a view");
            view.blocks.clear();
            view.collapse = Some(Collapse::Every(Every::All));
        });
        assert!(
            report.warnings.is_empty(),
            "{name}: no region named edges in the fixture: {:?}",
            report.warnings
        );
        tested += 1;
    }
    assert!(tested >= 1, "a fixture class contains by `in`");
}

#[test]
fn a_kpi_without_the_measure_field_is_named() {
    for (name, dir) in packs() {
        let (report, _) = load_changed(&dir, |m| {
            m.kpis.as_mut().expect("a catalogue").fields.measure = String::from("not_a_field");
        });
        let kpis = report.counts.as_ref().unwrap().kpis;
        let got = codes(&report.warnings);
        assert_eq!(got.len(), kpis, "{name}");
        assert!(got.iter().all(|c| *c == "kpi_without_measure"), "{name}");
    }
}

#[test]
fn the_words_of_every_fixture_pack_are_counted_by_kind() {
    for (name, dir) in packs() {
        let pack = load_pack(&dir).1.expect("loads");
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for (kind, _) in words(&pack) {
            *counts.entry(kind.as_str()).or_default() += 1;
        }
        let want = expected(&name);
        for kind in [Kind::Block, Kind::Layer, Kind::Link, Kind::Kpi, Kind::Rule] {
            assert_eq!(
                counts.get(kind.as_str()).copied().unwrap_or(0) as u64,
                want["words"][kind.as_str()].as_u64().unwrap(),
                "{name}: {}",
                kind.as_str()
            );
        }
    }
}

#[test]
fn the_fixture_packs_share_no_word() {
    let all: Vec<_> = packs()
        .into_iter()
        .map(|(_, dir)| {
            words(&load_pack(&dir).1.expect("loads"))
                .into_iter()
                .map(|(_, w)| w)
                .collect::<std::collections::BTreeSet<_>>()
        })
        .collect();
    let shared: Vec<_> = all[0].intersection(&all[1]).collect();
    assert!(shared.is_empty(), "{shared:?}");
}

#[test]
fn alias_keys_are_words_of_the_pack() {
    for (name, dir) in packs() {
        let blocks = block_names(&dir);
        let (report, pack) = load_with_aliases(&dir, &[("zz_alias_word", &blocks[0])]);
        assert!(report.valid, "{name}: {:?}", report.errors);
        let words = words(&pack.expect("loads"));
        assert!(
            words.contains(&(Kind::Block, String::from("zz_alias_word"))),
            "{name}: a key of blocks.aliases is a block word of the pack, so the vocabulary check sees it"
        );
    }
}

/// Every subject prefix the loader writes into an issue of `metaspec pack check` (a literal
/// `format!("<prefix>:{…}")` in `pack.rs`) is named by the schema's description of a subject.
#[test]
fn every_subject_prefix_of_the_loader_is_named_in_the_pack_check_schema() {
    let source =
        std::fs::read_to_string(repo().join("crates/metaspec-core/src/pack.rs")).expect("pack.rs");
    let schema: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(repo().join("schema/metaspec-pack-check-1.schema.json")).unwrap(),
    )
    .unwrap();
    let described = schema["$defs"]["issue"]["properties"]["subject"]["description"]
        .as_str()
        .expect("a description of the subject");
    let mut prefixes = std::collections::BTreeSet::new();
    for piece in source.split("format!(\"").skip(1) {
        let head: String = piece
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || *c == '_')
            .collect();
        if !head.is_empty() && piece[head.len()..].starts_with(":{") {
            prefixes.insert(head);
        }
    }
    assert!(prefixes.len() >= 5, "{prefixes:?}");
    let missing: Vec<_> = prefixes
        .iter()
        .filter(|p| !described.contains(&format!("{p}:<")))
        .collect();
    assert!(missing.is_empty(), "not named in the schema: {missing:?}");
}
