//! A laid-out view of a graph, format `metaspec-scene/1`: boxes, groups, edge paths, labels and
//! the marks of findings, in coordinates. The docs site and the studio draw the same scene.

use serde::{Deserialize, Serialize};

/// The format name of a scene.
pub const FORMAT: &str = "metaspec-scene/1";

/// The schema of a scene.
pub const SCHEMA: &str = include_str!("../../../schema/metaspec-scene-1.schema.json");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    pub format: String,
    /// The path of the document relative to the pack.
    pub document: String,
    pub view: SceneView,
    pub width: f64,
    pub height: f64,
    /// The rows of the layout `rows`, top to bottom; none for `nested`.
    pub rows: Vec<Band>,
    /// Boxes in drawing order: a container before what it contains.
    pub boxes: Vec<SceneBox>,
    pub edges: Vec<Edge>,
    /// The links drawn as a badge on their caller instead of an edge: each link to a hub of the
    /// view (its `hubs`) from a part that is no hub.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub badges: Vec<Badge>,
    /// One mark per finding attached to a box or an edge of this scene.
    pub marks: Vec<Mark>,
    /// What each colour and icon of the scene stands for.
    pub legend: Vec<LegendEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneView {
    pub id: String,
    pub title: String,
    /// `rows` or `nested`.
    pub layout: String,
    /// Whether metaspec added the view because the declared views leave a part or link out.
    pub implicit: bool,
}

/// One row of the layout `rows`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Band {
    pub title: String,
    pub y: f64,
    pub h: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneBox {
    /// The part's id; for a ghost, the name a link gives that no part has.
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,
    /// The first line, the part's id.
    pub label: String,
    /// The lines below it: the block (in the pack's words) and kind, marked places, border.
    pub lines: Vec<String>,
    /// A colour of metaspec's palette.
    pub colour: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// What the colour stands for in words: the family or the layer.
    pub group: String,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    /// The box this box is drawn inside.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Whether boxes are drawn inside it.
    pub container: bool,
    pub on_border: bool,
    /// A name a link gives that no part of the document has.
    pub ghost: bool,
    /// A hub of the view: drawn once, apart, and each link to it is a badge on its caller.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hub: bool,
    /// A collapsed region: one box for the region and the parts inside it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collapsed: Option<Collapsed>,
    /// The numbers of the findings about this part.
    pub findings: Vec<usize>,
    /// The text shown on hover: the identifiers and the fields.
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// The link's id.
    pub id: String,
    /// The id of the box the edge starts at.
    pub from: String,
    /// The id of the box the edge ends at.
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carries: Option<String>,
    /// What the label shows: the link's name in the pack's words.
    pub label: String,
    /// The path from `from` to `to`; the layout `rows` draws it as a curve through the points,
    /// `nested` as straight segments.
    pub points: Vec<[f64; 2]>,
    /// Where the label is drawn, its marks included.
    pub label_at: Rect,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_by: Option<String>,
    /// The numbers of the findings about this link.
    pub findings: Vec<usize>,
    /// The text shown on hover.
    pub title: String,
    /// For an edge across the border of a collapsed region: the number of links it stands for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    /// For an edge across the border of a collapsed region: the ids of the links it stands for,
    /// each from or to a part inside the region, with one far end and one name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<String>,
}

/// What a collapsed region's box holds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collapsed {
    /// The number of parts inside the region that its box stands for.
    pub parts: usize,
    /// The id of the view that draws the region and its parts, `<view>/<region>`.
    pub view: String,
    /// The numbers of the findings about a part inside the region or a link between two of its
    /// parts; they are drawn in the region's view.
    pub findings: Vec<usize>,
}

/// A link to a hub drawn on its caller's box: the hub's name and the link's name in a pill.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Badge {
    /// The link's id.
    pub id: String,
    /// The id of the box the badge is drawn on, the link's caller.
    pub from: String,
    /// The id of the hub's box.
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carries: Option<String>,
    /// What the badge shows: the hub and the link's name in the pack's words.
    pub label: String,
    /// Where the badge is drawn, its marks included; inside the caller's box.
    pub at: Rect,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_by: Option<String>,
    /// The numbers of the findings about this link.
    pub findings: Vec<usize>,
    /// The text shown on hover.
    pub title: String,
    /// For a badge of a link across the border of a collapsed region: the number of links it
    /// stands for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    /// For a badge of a link across the border of a collapsed region: the ids of the links it
    /// stands for.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

/// The mark of one finding on a box or an edge: its number, readable without colour.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mark {
    /// The number of the finding in the document's list, from 1.
    pub finding: usize,
    pub rule: String,
    pub gate: bool,
    /// `part:<id>` or `link:<id>`.
    pub subject: String,
    /// The centre of the mark.
    pub x: f64,
    pub y: f64,
    pub w: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegendEntry {
    pub label: String,
    pub colour: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The blocks of the scene with this colour, in the pack's words.
    pub blocks: Vec<String>,
}

/// A coordinate with at most one decimal, so that the same layout prints the same bytes.
pub fn round(v: f64) -> f64 {
    let r = (v * 10.0).round() / 10.0;
    if r == 0.0 { 0.0 } else { r }
}
