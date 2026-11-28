//! One composed document as a graph, format `metaspec-graph/1`, read from a `parts-links/1`
//! document by the generic reader.
//!
//! A `parts-links/1` document has `parts` (a mapping from a name to a part with a field
//! `block`), `links` (a list of `{from, to, carries, ...}`), `border` (names of parts), and
//! marked places written `{value, vary}`. The reader executes nothing and implements no rule:
//! what it cannot read it leaves out of the graph, and the pack's tool says what is wrong.

use crate::manifest::DocumentClass;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// The format name of a graph.
pub const FORMAT: &str = "metaspec-graph/1";

/// The schema of a graph.
pub const SCHEMA: &str = include_str!("../../../schema/metaspec-graph-1.schema.json");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Graph {
    pub format: String,
    /// The path of the document relative to the pack.
    pub document: String,
    pub class: String,
    /// The document's name: the value of the key the class names, else the file's stem.
    pub name: String,
    pub parts: Vec<Part>,
    pub links: Vec<Link>,
    /// One group per part and word (`in`, `per`) by which it contains other parts.
    pub groups: Vec<Group>,
    /// The names the document's `border` lists, as written.
    pub border: Vec<String>,
    /// Marked places outside the parts and links, by their path in the document.
    pub markers: Vec<Marker>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Part {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,
    /// The part's field `kind`, when it is one text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Every field but `block`, a marked place read as its value.
    pub fields: Map<String, Value>,
    /// The part this part sits in (`contain: { in: <field> }`); a part may also have `per`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inside: Option<String>,
    /// The part this part exists once per instance of (`contain: { per: <field> }`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per: Option<String>,
    pub on_border: bool,
    pub markers: Vec<Marker>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    /// `<from>-><to>:<carries>`, `?` for a missing end or name; the n-th repeat of the same
    /// text gets `#<n>` appended.
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carries: Option<String>,
    /// Every key but `from`, `to` and `carries`, a marked place read as its value.
    pub fields: Map<String, Value>,
    /// The default of the pack that added the link; the generic reader adds none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_by: Option<String>,
    pub markers: Vec<Marker>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    /// The containing part.
    pub id: String,
    /// `in` or `per`.
    pub contain: String,
    /// The parts it contains directly, in the order of the document.
    pub members: Vec<String>,
}

/// A place a search may change: `{value, vary}` in the document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    /// The field's path below its part or link, or below the document for a marker of the
    /// graph; keys joined with `.`, list positions as numbers.
    pub path: String,
    pub value: Value,
    pub vary: Value,
}

impl Graph {
    pub fn part(&self, id: &str) -> Option<&Part> {
        self.parts.iter().find(|p| p.id == id)
    }

    pub fn link(&self, id: &str) -> Option<&Link> {
        self.links.iter().find(|l| l.id == id)
    }
}

/// A marked place as written: a mapping of exactly `value` and `vary`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Marked {
    pub value: Value,
    pub vary: Value,
}

/// The value and the range of a marked place; none for any other value.
pub fn marked(value: &Value) -> Option<Marked> {
    match value {
        Value::Object(map) if map.len() == 2 => serde_json::from_value(value.clone()).ok(),
        _ => None,
    }
}

/// Whether a value is a marked place: a mapping of exactly `value` and `vary`.
pub fn is_marker(value: &Value) -> bool {
    marked(value).is_some()
}

/// Replaces every marked place in `value` by its value and lists it, with `path` as prefix.
fn unmark(value: &Value, path: &str, out: &mut Vec<Marker>) -> Value {
    if let Some(m) = marked(value) {
        out.push(Marker {
            path: path.to_string(),
            value: m.value.clone(),
            vary: m.vary,
        });
        return m.value;
    }
    let join = |key: &str| {
        if path.is_empty() {
            key.to_string()
        } else {
            format!("{path}.{key}")
        }
    };
    match value {
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), unmark(v, &join(k), out)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(
            items
                .iter()
                .enumerate()
                .map(|(i, v)| unmark(v, &join(&i.to_string()), out))
                .collect(),
        ),
        other => other.clone(),
    }
}

/// One scalar as a name: a string as it is, a number or a boolean as its text.
fn name_of(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// The file stem of a path with forward slashes.
fn stem(path: &str) -> String {
    let file = path.rsplit('/').next().unwrap_or(path);
    match file.rsplit_once('.') {
        Some((stem, _)) if !stem.is_empty() => stem.to_string(),
        _ => file.to_string(),
    }
}

/// Reads a `parts-links/1` document of `class` at `path` into a graph. A part's block is the
/// word it writes; no alias is applied (see [`read_with_aliases`]).
pub fn read(class: &DocumentClass, path: &str, doc: &Value) -> Graph {
    read_with_aliases(class, path, doc, &BTreeMap::new())
}

/// Reads a `parts-links/1` document of `class` at `path` into a graph, with each part's block
/// word that `aliases` (the manifest's `blocks.aliases`) names replaced by the block it stands
/// for. The graph keeps the block, not the word written; a word no alias names stays.
pub fn read_with_aliases(
    class: &DocumentClass,
    path: &str,
    doc: &Value,
    aliases: &BTreeMap<String, String>,
) -> Graph {
    let name = name_of(doc.get(&class.name)).unwrap_or_else(|| stem(path));
    let border: Vec<String> = match doc.get("border") {
        Some(Value::Array(items)) => items.iter().filter_map(|v| name_of(Some(v))).collect(),
        Some(one) => name_of(Some(one)).into_iter().collect(),
        None => Vec::new(),
    };

    let mut parts = Vec::new();
    if let Some(Value::Object(map)) = doc.get("parts") {
        for (id, raw) in map {
            let mut markers = Vec::new();
            let fields = match raw {
                Value::Object(fields) => fields
                    .iter()
                    .filter(|(k, _)| k.as_str() != "block")
                    .map(|(k, v)| (k.clone(), unmark(v, k, &mut markers)))
                    .collect(),
                _ => Map::new(),
            };
            let block =
                name_of(raw.get("block")).map(|word| aliases.get(&word).cloned().unwrap_or(word));
            let kind = match fields.get("kind") {
                Some(Value::String(s)) => Some(s.clone()),
                _ => None,
            };
            let by = |field: Option<&String>| field.and_then(|f| name_of(fields.get(f)));
            let contain = class.contain.as_ref();
            let inside = by(contain.and_then(|c| c.inside.as_ref()));
            let per = by(contain.and_then(|c| c.per.as_ref()));
            parts.push(Part {
                on_border: border.iter().any(|b| b == id),
                id: id.clone(),
                block,
                kind,
                fields,
                inside,
                per,
                markers,
            });
        }
    }

    let mut links = Vec::new();
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    if let Some(Value::Array(items)) = doc.get("links") {
        for raw in items {
            let Value::Object(map) = raw else { continue };
            let mut markers = Vec::new();
            let fields: Map<String, Value> = map
                .iter()
                .filter(|(k, _)| !matches!(k.as_str(), "from" | "to" | "carries"))
                .map(|(k, v)| (k.clone(), unmark(v, k, &mut markers)))
                .collect();
            let from = name_of(map.get("from"));
            let to = name_of(map.get("to"));
            let carries = name_of(map.get("carries"));
            let shown = |v: &Option<String>| v.clone().unwrap_or_else(|| String::from("?"));
            let text = format!("{}->{}:{}", shown(&from), shown(&to), shown(&carries));
            let n = seen.entry(text.clone()).or_insert(0);
            *n += 1;
            let id = if *n == 1 { text } else { format!("{text}#{n}") };
            links.push(Link {
                id,
                from,
                to,
                carries,
                fields,
                added_by: None,
                markers,
            });
        }
    }

    // One group per containing part and word: a part that holds parts `in` it and parts `per`
    // it has two groups, the `in` group first.
    let mut groups: Vec<Group> = Vec::new();
    for part in &parts {
        for (word, holder) in [("in", &part.inside), ("per", &part.per)] {
            let Some(holder) = holder else { continue };
            if holder == &part.id || !parts.iter().any(|p| &p.id == holder) {
                continue;
            }
            match groups
                .iter_mut()
                .find(|g| &g.id == holder && g.contain == word)
            {
                Some(group) => group.members.push(part.id.clone()),
                None => groups.push(Group {
                    id: holder.clone(),
                    contain: word.to_string(),
                    members: vec![part.id.clone()],
                }),
            }
        }
    }
    let order = |id: &str| parts.iter().position(|p| p.id == id).unwrap_or(usize::MAX);
    groups.sort_by_key(|g| (order(&g.id), g.contain != "in"));

    let mut markers = Vec::new();
    if let Value::Object(map) = doc {
        for (key, value) in map {
            if matches!(key.as_str(), "parts" | "links") {
                continue;
            }
            unmark(value, key, &mut markers);
        }
    }

    Graph {
        format: FORMAT.to_string(),
        document: path.to_string(),
        class: class.class.clone(),
        name,
        parts,
        links,
        groups,
        border,
        markers,
    }
}
