//! The blocks of a pack as the docs read them from the compiled ESS model: the dependencies
//! between blocks with their counts, and the members of each block. Lifted from agenticspec's
//! generator; a qualified name belongs to the longest domain of the model it starts with, not to
//! a fixed number of name segments.

use serde_json::Value;
use specapp_core::pack::{Block, Pack};
use std::collections::{BTreeMap, BTreeSet};

/// The domain a qualified name belongs to: the longest domain of the model that the name
/// starts with, followed by a dot. None for a name of no domain.
pub fn domain_of<'a>(domains: &'a BTreeSet<String>, name: &str) -> Option<&'a str> {
    let mut best: Option<&str> = None;
    for d in domains {
        if name.len() > d.len()
            && name.starts_with(d.as_str())
            && name.as_bytes()[d.len()] == b'.'
            && best.is_none_or(|b| d.len() > b.len())
        {
            best = Some(d.as_str());
        }
    }
    best
}

fn declared(v: &Value, out: &mut BTreeSet<String>) {
    match v {
        Value::Object(o) => {
            if o.get("kind").and_then(Value::as_str) == Some("declared")
                && let Some(n) = o.get("name").and_then(Value::as_str)
            {
                out.insert(n.to_string());
            }
            o.values().for_each(|x| declared(x, out));
        }
        Value::Array(xs) => xs.iter().for_each(|x| declared(x, out)),
        _ => {}
    }
}

/// Every dependency between two domains, counted per (from, to, member): a member of `from`
/// names a type of `to` (directly or through the types it uses), the identifier of an entity of
/// `to`, or relates to an entity of `to`.
pub fn edges(ir: &Value) -> BTreeMap<(String, String), usize> {
    let domains: BTreeSet<String> = ir["domains"]
        .as_object()
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default();
    let owner_of = |name: &str| domain_of(&domains, name).map(str::to_string);
    let mut identity: BTreeMap<String, String> = BTreeMap::new();
    for e in ir["entities"]
        .as_object()
        .into_iter()
        .flat_map(|o| o.values())
    {
        if let (Some(t), Some(d)) = (
            e["identity"]["type_ref"]["name"].as_str(),
            e["domain"].as_str(),
        ) {
            identity.insert(t.to_string(), d.to_string());
        }
    }
    let mut direct: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (k, v) in ir["types"].as_object().into_iter().flatten() {
        let mut s = BTreeSet::new();
        declared(v, &mut s);
        direct.insert(k.clone(), s);
    }
    let owner = |t: &str| identity.get(t).cloned().or_else(|| owner_of(t));
    let mut raw: BTreeSet<(String, String, String)> = BTreeSet::new();
    for c in ["entities", "commands", "events", "views", "errors", "types"] {
        for (k, v) in ir[c].as_object().into_iter().flatten() {
            let Some(from) = v["domain"]
                .as_str()
                .map(str::to_string)
                .or_else(|| owner_of(k))
            else {
                continue;
            };
            let mut set = BTreeSet::new();
            declared(v, &mut set);
            loop {
                let mut next = set.clone();
                for t in &set {
                    if let Some(d) = direct.get(t) {
                        next.extend(d.iter().cloned());
                    }
                }
                if next.len() == set.len() {
                    break;
                }
                set = next;
            }
            for t in &set {
                if let Some(to) = owner(t)
                    && to != from
                {
                    raw.insert((from.clone(), to, k.clone()));
                }
            }
        }
    }
    for e in ir["entities"]
        .as_object()
        .into_iter()
        .flat_map(|o| o.values())
    {
        let from = e["domain"].as_str().unwrap_or("").to_string();
        let name = e["name"].as_str().unwrap_or("");
        for r in e["relations"].as_array().into_iter().flatten() {
            if let Some(to) = owner_of(r["target"].as_str().unwrap_or(""))
                && to != from
            {
                raw.insert((
                    from.clone(),
                    to,
                    format!("{name}.{}", r["name"].as_str().unwrap_or("")),
                ));
            }
        }
    }
    let mut out = BTreeMap::new();
    for (from, to, _) in raw {
        *out.entry((from, to)).or_insert(0) += 1;
    }
    out
}

/// The members of one domain, by kind (`entities`, `commands`, ...), as the model lists them,
/// sorted by name.
pub fn members<'a>(ir: &'a Value, domain: &str, kind: &str) -> Vec<&'a Value> {
    let mut out: Vec<&Value> = ir["domains"][domain][kind]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter_map(|n| ir[kind].get(n))
        .collect();
    out.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    out
}

/// The summary of a block: its domain's summary in the model.
pub fn summary(ir: &Value, domain: &str) -> String {
    let d = &ir["domains"][domain];
    d["naming"]["summary"]
        .as_str()
        .or_else(|| d["summary"].as_str())
        .unwrap_or("")
        .trim()
        .to_string()
}

/// The blocks of a pack with what the docs read of them.
pub struct Model<'a> {
    pub pack: &'a Pack,
    pub ir: &'a Value,
    pub edges: BTreeMap<(String, String), usize>,
}

impl<'a> Model<'a> {
    pub fn new(pack: &'a Pack) -> Model<'a> {
        Model {
            pack,
            ir: &pack.model,
            edges: edges(&pack.model),
        }
    }

    pub fn block(&self, domain: &str) -> Option<&'a Block> {
        self.pack.blocks.iter().find(|b| b.domain == domain)
    }

    pub fn by_name(&self, name: &str) -> Option<&'a Block> {
        self.pack.blocks.iter().find(|b| b.name == name)
    }

    /// The block a qualified name of the model belongs to.
    pub fn block_of(&self, name: &str) -> Option<&'a Block> {
        let domains: BTreeSet<String> = self.pack.blocks.iter().map(|b| b.domain.clone()).collect();
        domain_of(&domains, name).and_then(|d| self.block(d))
    }

    pub fn count(&self, b: &Block, kind: &str) -> usize {
        self.ir["domains"][&b.domain][kind]
            .as_array()
            .map_or(0, |a| a.len())
    }

    /// The commands' outcomes of a block that refuse with an error.
    pub fn refusals(&self, b: &Block) -> usize {
        members(self.ir, &b.domain, "commands")
            .iter()
            .flat_map(|c| c["outcomes"].as_array().into_iter().flatten())
            .filter(|o| o["error"].is_string())
            .count()
    }

    fn layer(b: &Block) -> u32 {
        b.layer.as_ref().map_or(u32::MAX, |l| l.number)
    }

    /// The blocks `b` depends on, highest layer first, with the number of references.
    pub fn depends_on(&self, b: &Block) -> Vec<(&'a Block, usize)> {
        let mut out: Vec<(&Block, usize)> = self
            .edges
            .iter()
            .filter(|((f, _), _)| *f == b.domain)
            .filter_map(|((_, t), n)| self.block(t).map(|x| (x, *n)))
            .collect();
        out.sort_by(|a, b| {
            (std::cmp::Reverse(Self::layer(a.0)), &a.0.name)
                .cmp(&(std::cmp::Reverse(Self::layer(b.0)), &b.0.name))
        });
        out
    }

    /// The blocks that depend on `b`, lowest layer first, with the number of references.
    pub fn used_by(&self, b: &Block) -> Vec<(&'a Block, usize)> {
        let mut out: Vec<(&Block, usize)> = self
            .edges
            .iter()
            .filter(|((_, t), _)| *t == b.domain)
            .filter_map(|((f, _), n)| self.block(f).map(|x| (x, *n)))
            .collect();
        out.sort_by(|a, b| (Self::layer(a.0), &a.0.name).cmp(&(Self::layer(b.0), &b.0.name)));
        out
    }
}
