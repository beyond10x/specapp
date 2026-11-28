//! The words of a pack: what it calls its blocks, layers, links, KPIs and rules. metaspec's own
//! code must contain none of them; `scripts/check-vocabulary.sh` holds that with this list.

use crate::manifest::Shape;
use crate::pack::Pack;
use serde_json::Value;
use std::collections::BTreeSet;

/// What a word names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Block,
    Layer,
    Link,
    Kpi,
    Rule,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Block => "block",
            Kind::Layer => "layer",
            Kind::Link => "link",
            Kind::Kpi => "kpi_id",
            Kind::Rule => "rule",
        }
    }
}

/// The words of a loaded pack, sorted and without repetition:
///
/// - block: the block names and the keys of the manifest's `blocks.aliases`;
/// - layer: the layer names;
/// - link: every `carries` of a link of a `parts-links/1` document, every `carries` of a view,
///   and every key of a `links` mapping in any document;
/// - kpi: the ids of the KPI catalogue, and every `id` of an entry of a `kpis` list anywhere in
///   any document;
/// - rule: every `id` of an entry of a `rules` list anywhere in any document.
pub fn words(pack: &Pack) -> BTreeSet<(Kind, String)> {
    let mut out = BTreeSet::new();
    for block in &pack.blocks {
        out.insert((Kind::Block, block.name.clone()));
        if let Some(layer) = &block.layer {
            out.insert((Kind::Layer, layer.name.clone()));
        }
    }
    for word in pack.manifest.blocks.aliases.keys() {
        out.insert((Kind::Block, word.clone()));
    }
    for row in &pack.kpis {
        if let Some(id) = row.get("id").and_then(Value::as_str) {
            out.insert((Kind::Kpi, id.to_string()));
        }
    }
    for (class, documents) in pack.manifest.documents.iter().zip(&pack.documents) {
        for view in &class.views {
            for carries in &view.carries {
                out.insert((Kind::Link, carries.clone()));
            }
        }
        for document in documents {
            let Some(value) = &document.value else {
                continue;
            };
            if class.shape == Shape::PartsLinks
                && let Some(links) = value.get("links").and_then(Value::as_array)
            {
                for link in links {
                    if let Some(carries) = link.get("carries").and_then(Value::as_str) {
                        out.insert((Kind::Link, carries.to_string()));
                    }
                }
            }
            walk(value, &mut out);
        }
    }
    out
}

/// Collects rule ids and link names declared anywhere in a document.
fn walk(value: &Value, out: &mut BTreeSet<(Kind, String)>) {
    match value {
        Value::Object(map) => {
            for (key, inner) in map {
                match (key.as_str(), inner) {
                    ("rules", Value::Array(rows)) | ("kpis", Value::Array(rows)) => {
                        let kind = if key == "rules" {
                            Kind::Rule
                        } else {
                            Kind::Kpi
                        };
                        for row in rows {
                            if let Some(id) = row.get("id").and_then(Value::as_str) {
                                out.insert((kind, id.to_string()));
                            }
                        }
                    }
                    ("links", Value::Object(links)) => {
                        for name in links.keys() {
                            out.insert((Kind::Link, name.clone()));
                        }
                    }
                    _ => {}
                }
                walk(inner, out);
            }
        }
        Value::Array(items) => {
            for item in items {
                walk(item, out);
            }
        }
        _ => {}
    }
}
