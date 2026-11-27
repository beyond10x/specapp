//! specapp-layout: a graph (`specapp-graph/1`) and a view of its document class to a scene
//! (`specapp-scene/1`), and a scene as SVG.
//!
//! Two layouts:
//!
//! - `rows`: a part's row is the layer of its block, or its family's row. Rows are ordered by
//!   barycentre sweeps and placed by isotonic regression; an edge label sits in the gap between
//!   two rows with the room its text needs. A part that exists once per instance of another
//!   part (`per`) is drawn inside that part.
//! - `nested`: a part is drawn inside the part that contains it (`in` or `per`).
//!
//! The same input gives the same bytes: no hash order, no clock, coordinates rounded to one
//! decimal. This crate knows no pack: names, colours and icons come from the manifest.

pub mod icons;
pub mod nested;
pub mod palette;
pub mod rows;
pub mod scene;
pub mod svg;
pub mod text;

use scene::{LegendEntry, Mark, Scene, SceneBox, SceneView};
use specapp_core::graph::{Graph, Part};
use specapp_core::manifest::{DocumentClass, Layout, RowsBy, View};
use specapp_core::pack::Pack;
use specapp_core::tool::{Finding, Subject};
use std::collections::{BTreeMap, BTreeSet};

/// How one block is drawn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockStyle {
    /// A colour of specapp's palette.
    pub colour: String,
    pub icon: Option<String>,
    /// What the colour stands for: the family, or the layer when the block is in no family.
    pub group: String,
    pub layer: Option<(u32, String)>,
    pub family: Option<String>,
    /// The lowest layer of the block's family, or the block's own layer.
    pub family_rank: Option<u32>,
}

/// The presentation of a pack: per block its colour, icon, group and layer, and the pack's
/// display words.
#[derive(Debug, Clone, Default)]
pub struct Style {
    pub blocks: BTreeMap<String, BlockStyle>,
    pub vocabulary: BTreeMap<String, String>,
}

impl Style {
    /// The presentation of a loaded pack. A block in no family takes the colour of its layer,
    /// from specapp's palette in the order of the layers.
    pub fn of_pack(pack: &Pack) -> Style {
        let mut layers: Vec<u32> = pack
            .blocks
            .iter()
            .filter_map(|b| b.layer.as_ref().map(|l| l.number))
            .collect();
        layers.sort_unstable();
        layers.dedup();
        let presentation = pack.manifest.presentation.clone().unwrap_or_default();
        let mut family_of: BTreeMap<&str, &str> = BTreeMap::new();
        for (name, family) in &presentation.families {
            for block in &family.blocks {
                family_of.entry(block.as_str()).or_insert(name.as_str());
            }
        }
        let layer_of: BTreeMap<&str, u32> = pack
            .blocks
            .iter()
            .filter_map(|b| b.layer.as_ref().map(|l| (b.name.as_str(), l.number)))
            .collect();
        let mut blocks = BTreeMap::new();
        for block in &pack.blocks {
            let layer = block.layer.as_ref().map(|l| (l.number, l.name.clone()));
            let family = family_of.get(block.name.as_str()).map(|f| f.to_string());
            let style = match &family {
                Some(name) => {
                    let spec = &presentation.families[name];
                    let rank = spec
                        .blocks
                        .iter()
                        .filter_map(|b| layer_of.get(b.as_str()).copied())
                        .min();
                    BlockStyle {
                        colour: spec.colour.clone(),
                        icon: Some(spec.icon.clone()),
                        group: name.clone(),
                        layer: layer.clone(),
                        family: family.clone(),
                        family_rank: rank,
                    }
                }
                None => {
                    let (colour, group) = match &layer {
                        Some((number, name)) => {
                            let index = layers.iter().position(|n| n == number).unwrap_or(0);
                            (
                                palette::NAMES[index % palette::NAMES.len()].to_string(),
                                layer_title(*number, name),
                            )
                        }
                        None => (String::from("slate"), String::from("no layer")),
                    };
                    BlockStyle {
                        colour,
                        icon: None,
                        group,
                        layer: layer.clone(),
                        family: None,
                        family_rank: layer.as_ref().map(|l| l.0),
                    }
                }
            };
            blocks.insert(block.name.clone(), style);
        }
        Style {
            blocks,
            vocabulary: presentation.vocabulary,
        }
    }

    /// How a block is drawn; a block the pack does not have is drawn in slate.
    pub fn block(&self, name: Option<&str>) -> BlockStyle {
        name.and_then(|n| self.blocks.get(n))
            .cloned()
            .unwrap_or_else(|| BlockStyle {
                colour: String::from("slate"),
                icon: None,
                group: String::from(if name.is_some() {
                    "block the pack does not have"
                } else {
                    "no block"
                }),
                layer: None,
                family: None,
                family_rank: None,
            })
    }

    /// A block or link name in the pack's words for display.
    pub fn word(&self, name: &str) -> String {
        self.vocabulary
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_string())
    }
}

/// The title of a layer row: its number and name.
pub fn layer_title(number: u32, name: &str) -> String {
    format!("{number} {name}")
}

/// One view of a document class as the layout reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewSpec {
    pub id: String,
    pub title: String,
    pub layout: Layout,
    pub rows: RowsBy,
    /// The blocks the view shows; empty shows every block.
    pub blocks: Vec<String>,
    /// The link names the view shows; empty shows every link.
    pub carries: Vec<String>,
    pub implicit: bool,
}

impl From<&View> for ViewSpec {
    fn from(view: &View) -> ViewSpec {
        ViewSpec {
            id: view.id.clone(),
            title: view.title.clone(),
            layout: view.layout,
            rows: view.rows,
            blocks: view.blocks.clone(),
            carries: view.carries.clone(),
            implicit: false,
        }
    }
}

/// The views a document of `class` is drawn in: the declared views, then a view of every part
/// and link when the declared views leave one out (or declare none). Nothing of a document is
/// hidden.
pub fn views_of(class: &DocumentClass, graph: &Graph) -> Vec<ViewSpec> {
    let mut views: Vec<ViewSpec> = class.views.iter().map(ViewSpec::from).collect();
    let mut parts = BTreeSet::new();
    let mut links = BTreeSet::new();
    for view in &views {
        let shown = select(graph, view);
        parts.extend(shown.parts.iter().copied());
        links.extend(shown.links.iter().copied());
    }
    if views.is_empty() || parts.len() < graph.parts.len() || links.len() < graph.links.len() {
        let mut id = String::from("all");
        let mut n = 1;
        while views.iter().any(|v| v.id == id) {
            n += 1;
            id = format!("all-{n}");
        }
        views.push(ViewSpec {
            id,
            title: String::from("All parts and links"),
            layout: Layout::Rows,
            rows: RowsBy::Layer,
            blocks: Vec::new(),
            carries: Vec::new(),
            implicit: true,
        });
    }
    views
}

/// What a view shows of a graph: indices of parts and links, and the names links give that no
/// part has.
#[derive(Debug, Clone, Default)]
pub struct Selection {
    pub parts: Vec<usize>,
    pub links: Vec<usize>,
    pub ghosts: Vec<String>,
}

/// The parts of the view's blocks (every part when it names none) and the links of the view's
/// names (every link when it names none) whose ends are both shown or are no part at all.
pub fn select(graph: &Graph, view: &ViewSpec) -> Selection {
    let shows_block = |part: &Part| {
        view.blocks.is_empty()
            || part
                .block
                .as_ref()
                .is_some_and(|b| view.blocks.iter().any(|v| v == b))
    };
    let parts: Vec<usize> = (0..graph.parts.len())
        .filter(|&i| shows_block(&graph.parts[i]))
        .collect();
    let shown = |name: &str| parts.iter().any(|&i| graph.parts[i].id == name);
    let exists = |name: &str| graph.parts.iter().any(|p| p.id == name);
    let mut links = Vec::new();
    let mut ghosts: Vec<String> = Vec::new();
    for (i, link) in graph.links.iter().enumerate() {
        let carried = view.carries.is_empty()
            || link
                .carries
                .as_ref()
                .is_some_and(|c| view.carries.iter().any(|v| v == c));
        if !carried {
            continue;
        }
        let ends = [link.from.as_deref(), link.to.as_deref()];
        let fine = ends.iter().all(|end| match end {
            Some(name) => shown(name) || !exists(name),
            None => true,
        });
        if !fine {
            continue;
        }
        links.push(i);
        for end in ends {
            let name = end.unwrap_or("?");
            if !exists(name) && !ghosts.iter().any(|g| g == name) {
                ghosts.push(name.to_string());
            }
        }
    }
    Selection {
        parts,
        links,
        ghosts,
    }
}

/// Lays out one view of a graph. `findings` is the check result of the document; a finding's
/// number is its position in that list, from 1.
pub fn layout(graph: &Graph, view: &ViewSpec, style: &Style, findings: &[Finding]) -> Scene {
    let selection = select(graph, view);
    let (_, marked) = numbers(findings);
    let mut scene = match view.layout {
        Layout::Rows => rows::layout(graph, view, style, &selection, &marked),
        Layout::Nested => nested::layout(graph, style, &selection, &marked),
    };
    scene.format = scene::FORMAT.to_string();
    scene.document = graph.document.clone();
    scene.view = SceneView {
        id: view.id.clone(),
        title: view.title.clone(),
        layout: view.layout.as_str().to_string(),
        implicit: view.implicit,
    };
    attach(&mut scene, findings);
    scene.legend = legend(&scene, style);
    scene
}

/// Width of the mark of finding `n`.
pub fn mark_width(n: usize) -> f64 {
    text::width(&format!("!{n}"), 10.0, true) + 10.0
}

/// Numbers of findings by the id of their subject.
pub type BySubject<'a> = BTreeMap<&'a str, Vec<usize>>;

/// The findings of a part, a link and a ghost, by subject.
pub fn numbers(findings: &[Finding]) -> (BySubject<'_>, BySubject<'_>) {
    let mut parts: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    let mut links: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, finding) in findings.iter().enumerate() {
        match finding.subject() {
            Subject::Part(id) => parts.entry(id).or_default().push(i + 1),
            Subject::Link(id) => links.entry(id).or_default().push(i + 1),
            Subject::Document => {}
        }
    }
    (parts, links)
}

/// Puts the numbers of the findings on the boxes and edges they are about, and one mark per
/// finding on each: at the top right corner of a box, at the left of an edge's label. An edge
/// label has room for its marks (the layouts measure it with [`label_width`]).
fn attach(scene: &mut Scene, findings: &[Finding]) {
    let (parts, links) = numbers(findings);
    let mut marks = Vec::new();
    for b in &mut scene.boxes {
        let Some(ns) = parts.get(b.id.as_str()) else {
            continue;
        };
        b.findings = ns.clone();
        let mut right = b.x + b.w - 6.0;
        for &n in ns {
            let w = mark_width(n);
            marks.push(Mark {
                finding: n,
                rule: findings[n - 1].rule.clone(),
                gate: findings[n - 1].gate,
                subject: findings[n - 1].subject.clone(),
                x: scene::round(right - w / 2.0),
                y: scene::round(b.y),
                w: scene::round(w),
            });
            right -= w + 3.0;
        }
    }
    for e in &mut scene.edges {
        let Some(ns) = links.get(e.id.as_str()) else {
            continue;
        };
        e.findings = ns.clone();
        let mut left = e.label_at.x + 3.0;
        for &n in ns {
            let w = mark_width(n);
            marks.push(Mark {
                finding: n,
                rule: findings[n - 1].rule.clone(),
                gate: findings[n - 1].gate,
                subject: findings[n - 1].subject.clone(),
                x: scene::round(left + w / 2.0),
                y: scene::round(e.label_at.y + e.label_at.h / 2.0),
                w: scene::round(w),
            });
            left += w + 3.0;
        }
    }
    marks.sort_by_key(|m| m.finding);
    scene.marks = marks;
}

/// The width of an edge label with room for the marks of its findings.
pub fn label_width(label: &str, marks: &[usize]) -> f64 {
    text::width(label, 11.0, false) + 14.0 + marks.iter().map(|&n| mark_width(n) + 3.0).sum::<f64>()
}

/// What each colour of the scene stands for, in the order the boxes first show it.
fn legend(scene: &Scene, style: &Style) -> Vec<LegendEntry> {
    let mut out: Vec<LegendEntry> = Vec::new();
    for b in &scene.boxes {
        let block = match (&b.block, b.ghost) {
            (_, true) => None,
            (Some(block), _) => Some(style.word(block)),
            (None, _) => Some(String::from("no block")),
        };
        match out.iter_mut().find(|e| e.label == b.group) {
            Some(entry) => {
                if let Some(block) = block
                    && !entry.blocks.contains(&block)
                {
                    entry.blocks.push(block);
                }
            }
            None => out.push(LegendEntry {
                label: b.group.clone(),
                colour: b.colour.clone(),
                icon: b.icon.clone(),
                blocks: block.into_iter().collect(),
            }),
        }
    }
    out
}

/// The text of a part's box: its label, lines and hover text, and its width and height.
pub struct Card {
    pub label: String,
    pub lines: Vec<String>,
    pub title: String,
    pub w: f64,
    pub h: f64,
}

pub const PAD: f64 = 10.0;
pub const LABEL_H: f64 = 18.0;
pub const LINE_H: f64 = 15.0;
pub const ICON: f64 = 16.0;
pub const MIN_W: f64 = 112.0;

/// The box text of a part. `noted` adds a line per containment the layout does not draw.
pub fn card(part: &Part, style: &Style, noted: &[String]) -> Card {
    let block = style.block(part.block.as_deref());
    let mut first = match &part.block {
        Some(name) => style.word(name),
        None => String::from("no block"),
    };
    if let Some(kind) = &part.kind {
        first.push_str(" · ");
        first.push_str(kind);
    }
    let mut lines = vec![first];
    lines.extend(noted.iter().cloned());
    if !part.markers.is_empty() {
        let names: Vec<&str> = part.markers.iter().map(|m| m.path.as_str()).collect();
        lines.push(format!("varies: {}", names.join(", ")));
    }
    if part.on_border {
        lines.push(String::from("on the border"));
    }
    let mut title = format!(
        "{} (block {})",
        part.id,
        part.block.as_deref().unwrap_or("none")
    );
    title.push_str(&format!(", {}", block.group));
    for (key, value) in &part.fields {
        title.push_str(&format!("\n{key}: {}", text::value(value)));
    }
    for marker in &part.markers {
        title.push_str(&format!(
            "\nvaries {}: {} over {}",
            marker.path,
            text::value(&marker.value),
            text::value(&marker.vary)
        ));
    }
    let icon = if block.icon.is_some() {
        ICON + 6.0
    } else {
        0.0
    };
    let w = lines
        .iter()
        .map(|l| text::width(l, 11.0, false))
        .fold(text::width(&part.id, 13.0, true) + icon, f64::max)
        + 2.0 * PAD;
    let h = PAD + LABEL_H + LINE_H * lines.len() as f64 + PAD - 4.0;
    Card {
        label: part.id.clone(),
        lines,
        title,
        w: w.max(MIN_W).ceil(),
        h,
    }
}

/// The box text of a ghost: a name a link gives that no part has.
pub fn ghost_card(name: &str) -> Card {
    let lines = vec![String::from("no part has this name")];
    let w = (text::width(name, 13.0, true).max(text::width(&lines[0], 11.0, false)) + 2.0 * PAD)
        .max(MIN_W)
        .ceil();
    Card {
        label: name.to_string(),
        title: format!("{name}: a link names it, and no part of the document has this name"),
        lines,
        w,
        h: PAD + LABEL_H + LINE_H + PAD - 4.0,
    }
}

/// A box of a scene at `(x, y)` with the text of `card`.
pub fn scene_box(
    part: Option<&Part>,
    id: &str,
    card: &Card,
    style: &Style,
    at: (f64, f64, f64, f64),
) -> SceneBox {
    let block = part.and_then(|p| p.block.clone());
    let (colour, icon, group, ghost) = match part {
        Some(p) => {
            let s = style.block(p.block.as_deref());
            (s.colour, s.icon, s.group, false)
        }
        None => (
            String::from("slate"),
            None,
            String::from("a name no part has"),
            true,
        ),
    };
    SceneBox {
        id: id.to_string(),
        block,
        label: card.label.clone(),
        lines: card.lines.clone(),
        colour,
        icon,
        group,
        x: scene::round(at.0),
        y: scene::round(at.1),
        w: scene::round(at.2),
        h: scene::round(at.3),
        parent: None,
        container: false,
        on_border: part.is_some_and(|p| p.on_border),
        ghost,
        findings: Vec::new(),
        title: card.title.clone(),
    }
}

/// The hover text of a link.
pub fn link_title(link: &specapp_core::graph::Link) -> String {
    let mut title = link.id.clone();
    if let Some(by) = &link.added_by {
        title.push_str(&format!("\nadded by {by}"));
    }
    for (key, value) in &link.fields {
        title.push_str(&format!("\n{key}: {}", text::value(value)));
    }
    for marker in &link.markers {
        title.push_str(&format!(
            "\nvaries {}: {} over {}",
            marker.path,
            text::value(&marker.value),
            text::value(&marker.vary)
        ));
    }
    title
}

/// The label of a link: its name in the pack's words, `?` when it names none.
pub fn link_label(link: &specapp_core::graph::Link, style: &Style) -> String {
    match &link.carries {
        Some(c) => style.word(c),
        None => String::from("?"),
    }
}
