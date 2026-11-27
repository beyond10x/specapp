//! The manifest of a pack, format `specapp-pack/1`.
//!
//! A manifest is read in two steps: its YAML is validated against the JSON Schema in
//! `schema/specapp-pack-1.schema.json`, then decoded into [`Manifest`]. The schema is the
//! contract; the types below decode exactly what it allows.

use crate::report::Issue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// The format name a manifest carries.
pub const FORMAT: &str = "specapp-pack/1";

/// The schema of the manifest.
pub const SCHEMA: &str = include_str!("../../../schema/specapp-pack-1.schema.json");

/// The colours of specapp's palette, the only colour names a pack may write.
pub const PALETTE: &[&str] = &[
    "blue", "violet", "green", "teal", "amber", "orange", "red", "pink", "cyan", "lime", "brown",
    "slate",
];

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub format: String,
    pub pack: String,
    pub title: String,
    #[serde(default)]
    pub summary: Option<String>,
    pub visibility: Visibility,
    #[serde(default)]
    pub source_url: Option<String>,
    #[serde(default)]
    pub extends: Option<String>,
    pub blocks: Blocks,
    #[serde(default)]
    pub kpis: Option<Kpis>,
    #[serde(default)]
    pub documents: Vec<DocumentClass>,
    #[serde(default)]
    pub tool: Option<Tool>,
    #[serde(default)]
    pub presentation: Option<Presentation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Blocks {
    pub path: String,
    #[serde(default)]
    pub strip_prefix: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Kpis {
    pub path: String,
    #[serde(default)]
    pub fields: KpiFields,
}

/// The pack's names of the neutral KPI fields.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KpiFields {
    #[serde(default = "default_measure")]
    pub measure: String,
    #[serde(default = "default_modes")]
    pub modes: String,
}

fn default_measure() -> String {
    String::from("measure")
}

fn default_modes() -> String {
    String::from("modes")
}

impl Default for KpiFields {
    fn default() -> Self {
        KpiFields {
            measure: default_measure(),
            modes: default_modes(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentClass {
    pub class: String,
    pub title: String,
    pub paths: Vec<String>,
    pub shape: Shape,
    pub name: String,
    #[serde(default)]
    pub contain: Option<Contain>,
    #[serde(default)]
    pub views: Vec<View>,
    #[serde(default)]
    pub tables: Vec<Table>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Shape {
    #[serde(rename = "parts-links/1")]
    PartsLinks,
    #[serde(rename = "records/1")]
    Records,
}

impl Shape {
    pub fn as_str(self) -> &'static str {
        match self {
            Shape::PartsLinks => "parts-links/1",
            Shape::Records => "records/1",
        }
    }
}

/// How one part of a document contains another. The value is the name of the part's field
/// that names the containing part.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Contain {
    /// The part sits inside the part the field names.
    In(String),
    /// The part exists once per instance of the part the field names.
    Per(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct View {
    pub id: String,
    pub title: String,
    pub layout: Layout,
    #[serde(default)]
    pub rows: RowsBy,
    #[serde(default)]
    pub blocks: Vec<String>,
    #[serde(default)]
    pub carries: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    Rows,
    Nested,
}

impl Layout {
    pub fn as_str(self) -> &'static str {
        match self {
            Layout::Rows => "rows",
            Layout::Nested => "nested",
        }
    }
}

/// What a row of the layout `rows` is.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RowsBy {
    /// The layer of the part's block.
    #[default]
    Layer,
    /// The family of the part's block; a block in no family keeps the row of its layer.
    Family,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    pub title: String,
    pub at: String,
    #[serde(default)]
    pub key: Option<String>,
    pub columns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    pub run: Vec<String>,
    pub verbs: Vec<Verb>,
}

/// A verb of the tool protocol beside `describe`, which every tool answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Verb {
    Graph,
    Check,
    Simulate,
    Search,
    Apply,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Presentation {
    #[serde(default)]
    pub families: BTreeMap<String, Family>,
    #[serde(default)]
    pub vocabulary: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Family {
    pub blocks: Vec<String>,
    pub colour: String,
    pub icon: String,
}

/// Reads a manifest from its YAML text: validates it against the schema, then decodes it.
/// Every schema violation is one error, named by the place in the manifest.
pub fn parse(text: &str) -> Result<Manifest, Vec<Issue>> {
    let value: Value = serde_yaml_ng::from_str(text).map_err(|error| {
        vec![Issue::new(
            "manifest_unreadable",
            "manifest",
            format!("the manifest is no YAML document: {error}"),
        )]
    })?;
    validate(&value)?;
    serde_json::from_value(value).map_err(|error| {
        vec![Issue::new(
            "manifest_invalid",
            "manifest",
            format!("the manifest does not decode: {error}"),
        )]
    })
}

/// Validates a manifest, already read into JSON, against the schema of `specapp-pack/1`.
pub fn validate(value: &Value) -> Result<(), Vec<Issue>> {
    if let Some(format) = value.get("format").and_then(Value::as_str)
        && format != FORMAT
    {
        return Err(vec![Issue::new(
            "manifest_format",
            "manifest",
            format!("the manifest is in format {format}; this specapp reads {FORMAT}"),
        )]);
    }
    let schema: Value = serde_json::from_str(SCHEMA).expect("the manifest schema is JSON");
    let validator = jsonschema::validator_for(&schema).expect("the manifest schema compiles");
    let issues: Vec<Issue> = validator
        .iter_errors(value)
        .map(|error| {
            let at = error.instance_path().to_string();
            let at = if at.is_empty() { String::from("/") } else { at };
            Issue::new("manifest_invalid", "manifest", format!("at {at}: {error}"))
        })
        .collect();
    if issues.is_empty() {
        Ok(())
    } else {
        Err(issues)
    }
}

impl Manifest {
    /// The levels of adoption the manifest declares, cheapest first; the last is the highest.
    pub fn level(&self) -> &'static str {
        let verbs = self
            .tool
            .as_ref()
            .map(|t| t.verbs.as_slice())
            .unwrap_or(&[]);
        if verbs.contains(&Verb::Apply) {
            "L5"
        } else if verbs.contains(&Verb::Simulate) || verbs.contains(&Verb::Search) {
            "L4"
        } else if verbs.contains(&Verb::Check) {
            "L3"
        } else if !self.documents.is_empty() {
            "L2"
        } else if self.kpis.is_some() {
            "L1"
        } else {
            "L0"
        }
    }
}
