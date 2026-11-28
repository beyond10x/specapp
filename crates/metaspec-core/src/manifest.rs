//! The manifest of a pack, format `metaspec-pack/1`.
//!
//! A manifest is metaspec's description of a pack; the pack knows nothing of it. The manifests
//! of public packs are metaspec's `packs/<name>.yaml` (see [`crate::known`]); the manifest of a
//! private pack is given by its path at run time.
//!
//! A manifest is read in two steps: its YAML is validated against the JSON Schema in
//! `schema/metaspec-pack-1.schema.json`, then decoded into [`Manifest`]. The schema is the
//! contract; the types below decode exactly what it allows.

use crate::report::Issue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// The format name a manifest carries.
pub const FORMAT: &str = "metaspec-pack/1";

/// The schema of the manifest.
pub const SCHEMA: &str = include_str!("../../../schema/metaspec-pack-1.schema.json");

/// The colours of metaspec's palette, the only colour names a pack may write.
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
    /// Where one file of the pack is read on the web, with the placeholder `{path}`.
    #[serde(default)]
    pub source_file_url: Option<String>,
    #[serde(default)]
    pub extends: Option<String>,
    /// The blocks. Only a manifest with `extends` may leave them out: it inherits the base's
    /// (see [`crate::pack::inherit`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Blocks>,
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
    /// A part's block word mapped to the block of the pack it stands for. Every target is a
    /// block of the pack and no word is one; the pack's loader refuses the manifest otherwise.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub aliases: BTreeMap<String, String>,
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
    /// Globs of files that `paths` match and that are no documents of the class: such a file
    /// is not read. Each glob is matched against the whole path relative to the pack.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<String>,
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

/// How one part of a document contains another: `{ in: <field> }`, `{ per: <field> }` or both.
/// Each value is the name of the part's field that names the containing part; the schema holds
/// that at least one is given.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Contain {
    /// The part sits inside the part the field names.
    #[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
    pub inside: Option<String>,
    /// The part exists once per instance of the part the field names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per: Option<String>,
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
    /// The parts the view draws apart, each link to them a badge on its caller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hubs: Option<Hubs>,
    /// The regions the view draws collapsed, each one box with a view of its own parts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collapse: Option<Collapse>,
}

/// The regions a view draws collapsed. A region is a shown part that a shown part sits `in`.
/// `[<part id>]` names regions, `all` collapses every region that sits in no shown region, and
/// `{ blocks: [<block>] }` every region whose block (after `blocks.aliases`) is listed. A
/// region inside a collapsed region is inside its box.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Collapse {
    /// `all`.
    Every(Every),
    /// The ids of the regions, one or more.
    Parts(Vec<String>),
    /// The blocks whose regions collapse.
    Blocks(CollapseBlocks),
}

/// The id of a part that `collapse` never collapses: the view of the region would take the id
/// of the view `<id>/edges` of a view with hubs. `metaspec pack check` names such a region
/// (`view_region_edges`).
pub const REGION_EDGES: &str = "edges";

/// The word `all` of `collapse`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Every {
    All,
}

/// `collapse: { blocks: [<block>] }`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CollapseBlocks {
    pub blocks: Vec<String>,
}

/// The hubs of a view: the parts with at least `in_degree` incoming links shown in the view,
/// the parts `parts` names and the parts of the blocks `blocks` names. The schema holds that at
/// least one is given.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Hubs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_degree: Option<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<String>,
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

/// The pack's own tool and, per verb metaspec offers, the adapter that runs it and maps its
/// answer. The tool knows nothing of metaspec; the adapter is metaspec's.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    /// The argv prefix of the tool. A first element with a slash is a path inside the pack.
    pub run: Vec<String>,
    #[serde(default)]
    pub verbs: Verbs,
}

/// The verbs a pack's tool is used for, each with its adapter.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Verbs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<CheckAdapter>,
}

impl Verbs {
    /// The names of the declared verbs.
    pub fn names(&self) -> Vec<&'static str> {
        let mut names = Vec::new();
        if self.check.is_some() {
            names.push("check");
        }
        names
    }
}

/// How metaspec runs the tool to check one document and reads its answer.
///
/// metaspec runs `<tool.run> <args> <document>` in the pack directory and reads stdout as JSON.
/// `at` points at the document's result in that answer; every other pointer is relative to it.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckAdapter {
    /// The arguments after `tool.run`; the document's path is appended.
    #[serde(default)]
    pub args: Vec<String>,
    /// The exit codes that come with an answer. Default: 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<Vec<i32>>,
    /// A JSON pointer to the document's result in the answer. Default: the whole answer.
    #[serde(default)]
    pub at: String,
    /// A JSON pointer to a text that, when the result has it, refuses the run with that text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Whether the document is valid. Default: valid when no finding is gated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<Test>,
    /// A JSON pointer to the findings per rule, a mapping from a rule to a count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counts: Option<String>,
    /// A JSON pointer to KPI values of the document, a mapping from a KPI id to a number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kpis: Option<String>,
    /// The lists of findings in the result, in the order the findings are shown.
    #[serde(default)]
    pub findings: Vec<FindingList>,
}

impl CheckAdapter {
    /// The exit codes that come with an answer.
    pub fn exit_codes(&self) -> Vec<i32> {
        self.exit.clone().unwrap_or_else(|| vec![0])
    }
}

/// A truth read from the answer: the boolean at `at`, or whether the value at `at` equals
/// `equals`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Test {
    pub at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equals: Option<Value>,
}

/// Whether a finding is gated: always, never, or as a test of the finding says.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Gate {
    Always(bool),
    When(Test),
}

/// One list of findings in the answer and how one entry maps to a finding. `rule` and
/// `message` are templates: `{<JSON pointer>}` stands for the text at that pointer in the entry.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FindingList {
    /// A JSON pointer to the list; a list the answer leaves out is empty.
    pub at: String,
    pub rule: String,
    pub gate: Gate,
    pub message: String,
    pub subject: SubjectMap,
}

/// What a finding is about: always the document, or a rule over the entry.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SubjectMap {
    /// Always `document` (the schema allows no other text).
    Document(String),
    Rule(SubjectRule),
}

/// The subject of a finding: an id, and whether it names a part or a link.
///
/// With `link_when_contains`, an id that contains that text is a link and any other a part.
/// With `kind`, the template's value is looked up in `kinds`; a value not listed means the
/// document. With neither, the id is a part.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubjectRule {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_when_contains: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub kinds: BTreeMap<String, SubjectKind>,
    /// What an id the drawn graph does not have is about. Default: `keep`.
    #[serde(default)]
    pub unknown: Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SubjectKind {
    Part,
    Link,
    Document,
}

/// What a subject the graph does not have becomes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Unknown {
    /// It stays a part or a link; the page lists it and draws no mark.
    #[default]
    Keep,
    /// It is about the document; its id is put before the message unless it is the
    /// document's name.
    Document,
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
    let manifest: Manifest = serde_json::from_value(value).map_err(|error| {
        vec![Issue::new(
            "manifest_invalid",
            "manifest",
            format!("the manifest does not decode: {error}"),
        )]
    })?;
    if manifest.blocks.is_none() && manifest.extends.is_none() {
        return Err(vec![Issue::new(
            "manifest_invalid",
            "manifest",
            "at /: the manifest names no `blocks`; only a manifest that `extends` a pack may leave them out, to inherit the base's",
        )]);
    }
    Ok(manifest)
}

/// The `blocks` of a manifest that names none, for [`Manifest::blocks`].
static NO_BLOCKS: Blocks = Blocks {
    path: String::new(),
    strip_prefix: None,
    aliases: BTreeMap::new(),
};

/// Validates a manifest, already read into JSON, against the schema of `metaspec-pack/1`.
pub fn validate(value: &Value) -> Result<(), Vec<Issue>> {
    if let Some(format) = value.get("format").and_then(Value::as_str)
        && format != FORMAT
    {
        return Err(vec![Issue::new(
            "manifest_format",
            "manifest",
            format!("the manifest is in format {format}; this metaspec reads {FORMAT}"),
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
    /// The `blocks` section. A loaded pack's manifest always has one, its own or the one it
    /// inherits; a manifest read alone that leaves it out (it `extends` a pack) has an empty one.
    pub fn blocks(&self) -> &Blocks {
        self.blocks.as_ref().unwrap_or(&NO_BLOCKS)
    }

    /// The levels of adoption the manifest declares, cheapest first; the last is the highest.
    pub fn level(&self) -> &'static str {
        if self.tool.as_ref().is_some_and(|t| t.verbs.check.is_some()) {
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
