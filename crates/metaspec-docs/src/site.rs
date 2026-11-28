//! The site: one for every pack it is given. Per pack its pages, their files and `site.json`
//! with the sidebar and the presentation table; the site's `index.json` and page `/`; and the
//! comparison with an output directory (`--check`).

use crate::document::{self, Drawn, visibility};
use crate::kpis::{self, Kpi};
use crate::md::{plural, text, yaml};
use crate::model::Model;
use crate::{blocks, catalogue, documents};
use metaspec_core::pack::{Block, Pack};
use metaspec_core::tool::CheckState;
use metaspec_layout::{Style, icons, palette};
use serde::Serialize;
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::Path;
use std::time::Duration;

/// The format of one page as data.
pub const PAGE_FORMAT: &str = "metaspec-page/1";

/// The format of `site.json`.
pub const SITE_FORMAT: &str = "metaspec-site/1";

/// The schema of a page.
pub const PAGE_SCHEMA: &str = include_str!("../../../schema/metaspec-page-1.schema.json");

/// The schema of `site.json`.
pub const SITE_SCHEMA: &str = include_str!("../../../schema/metaspec-site-1.schema.json");

/// The format of `index.json`, the site's list of its packs.
pub const INDEX_FORMAT: &str = "metaspec-index/1";

/// The schema of `index.json`.
pub const INDEX_SCHEMA: &str = include_str!("../../../schema/metaspec-index-1.schema.json");

/// The title of the site, on every page.
pub const SITE_TITLE: &str = "metaspec";

/// The module the generated pages import their components from.
pub const UI_MODULE: &str = "@beyond10x/metaspec-ui";

/// Names of the first directory of a page that metaspec uses itself; a document class with
/// such a name gets its pages under `documents/<class>`.
const RESERVED: &[&str] = &["blocks", "kpis", "rules", "index", "documents"];

/// One generated page.
#[derive(Debug, Clone)]
pub struct Page {
    /// The path below `docs/` without `.mdx`, also the page's id in the sidebar.
    pub id: String,
    /// What the page shows: `pack`, `layer-map`, `block`, `planned`, `kpis`, `class`,
    /// `document`, `record`, `rules`.
    pub kind: &'static str,
    /// Front matter in order; a value is a string or a boolean.
    pub front: Vec<(&'static str, Value)>,
    pub markdown: String,
    /// The components the Markdown draws.
    pub components: BTreeSet<&'static str>,
    /// The data of its components, read as `page.data.<key>`.
    pub data: Map<String, Value>,
}

impl Page {
    pub fn new(id: impl Into<String>, kind: &'static str) -> Page {
        Page {
            id: id.into(),
            kind,
            front: Vec::new(),
            markdown: String::new(),
            components: BTreeSet::new(),
            data: Map::new(),
        }
    }

    /// Adds a front matter field with a text value.
    pub fn set(&mut self, key: &'static str, value: impl Into<String>) {
        self.front.push((key, Value::String(value.into())));
    }

    /// Adds a front matter field with a boolean value.
    pub fn flag(&mut self, key: &'static str, value: bool) {
        self.front.push((key, Value::Bool(value)));
    }

    /// Draws a component: `<Name data={page.data.<key>} presentation={site.presentation} />`
    /// with `data` stored under `key`. Returns the tag.
    pub fn component(&mut self, name: &'static str, key: &str, data: Value) -> String {
        self.components.insert(name);
        self.data.insert(key.to_string(), data);
        format!(
            "<{name} data={{page.data[{}]}} presentation={{site.presentation}} />\n\n",
            serde_json::to_string(key).unwrap_or_default()
        )
    }

    /// The slug of the page below the pack's route: `/` for the pack page, else `/<id>` without
    /// a trailing `index`.
    pub fn slug(&self) -> String {
        slug_of(&self.id)
    }

    /// The title of the page, from its front matter.
    pub fn title(&self) -> Value {
        self.front
            .iter()
            .find(|(k, _)| *k == "title")
            .map(|(_, v)| v.clone())
            .unwrap_or(Value::Null)
    }

    /// The page as data, `metaspec-page/1`.
    pub fn json(&self) -> Value {
        let front: Map<String, Value> = self
            .front
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        json!({
            "format": PAGE_FORMAT,
            "id": self.id,
            "kind": self.kind,
            "front_matter": front,
            "markdown": self.markdown,
            "components": self.components.iter().collect::<Vec<_>>(),
            "data": self.data,
        })
    }

    /// The page as MDX: front matter, the imports of its data and components, the Markdown.
    pub fn mdx(&self) -> String {
        let up = "../".repeat(self.id.matches('/').count() + 1);
        let mut s = String::from("---\n");
        for (k, v) in &self.front {
            match v {
                Value::String(text) => s.push_str(&format!("{k}: {}\n", yaml(text))),
                other => s.push_str(&format!("{k}: {other}\n")),
            }
        }
        s.push_str("---\n\n");
        if !self.components.is_empty() {
            s.push_str(&format!(
                "import page from '{up}pages/{}.json';\nimport site from '{up}site.json';\nimport {{{}}} from '{UI_MODULE}';\n\n",
                self.id,
                self.components.iter().copied().collect::<Vec<_>>().join(", ")
            ));
        }
        s.push_str("{/* Generated by metaspec docs build. Do not edit. */}\n\n");
        s.push_str(&self.markdown);
        if !s.ends_with('\n') {
            s.push('\n');
        }
        s
    }
}

/// The slug of a page id.
pub fn slug_of(id: &str) -> String {
    if id == "index" {
        return String::from("/");
    }
    let id = id.strip_suffix("/index").unwrap_or(id);
    format!("/{id}")
}

/// The route of a page of a pack in the site: the pack's route (`/<pack>`) and the page's slug.
pub fn route_of(base: &str, id: &str) -> String {
    let slug = slug_of(id);
    match (base.trim_end_matches('/'), slug.as_str()) {
        ("", slug) => slug.to_string(),
        (base, "/") => base.to_string(),
        (base, slug) => format!("{base}{slug}"),
    }
}

/// A Markdown link target from the page `from` to the page `to`: the relative path of its file.
pub fn rel(from: &str, to: &str) -> String {
    let from_dir: Vec<&str> = {
        let mut parts: Vec<&str> = from.split('/').collect();
        parts.pop();
        parts
    };
    let to_parts: Vec<&str> = to.split('/').collect();
    let common = from_dir
        .iter()
        .zip(&to_parts)
        .take_while(|(a, b)| a == b)
        .count();
    let mut out = String::new();
    for _ in common..from_dir.len() {
        out.push_str("../");
    }
    if out.is_empty() {
        out.push_str("./");
    }
    out.push_str(&to_parts[common..].join("/"));
    out.push_str(".mdx");
    out
}

/// Everything the page writers read.
pub struct Ctx<'a> {
    pub pack: &'a Pack,
    pub model: Model<'a>,
    pub kpis: Vec<Kpi>,
    pub style: Style,
    /// Blocks the KPI catalogue names that the blocks do not have, in catalogue order.
    pub planned: Vec<String>,
    pub drawn: Drawn,
    /// The directory of each document class's pages.
    pub class_dirs: BTreeMap<String, String>,
    /// The route of the pack's pages in the site, `/<pack>`.
    pub base: String,
}

impl Ctx<'_> {
    /// The compiled model of the blocks.
    pub fn ir(&self) -> &Value {
        self.model.ir
    }

    /// The page id of a block.
    pub fn block_id(&self, b: &Block) -> String {
        match &b.layer {
            Some(l) => format!("blocks/{}-{}/{}", l.number, l.name, b.name),
            None => format!("blocks/no-layer/{}", b.name),
        }
    }

    /// The page id of a block by name, or the planned page's anchor for a planned block.
    pub fn block_target(&self, name: &str) -> Option<(String, Option<String>)> {
        if let Some(b) = self.model.by_name(name) {
            return Some((self.block_id(b), None));
        }
        if self.planned.iter().any(|p| p == name) {
            return Some((String::from("blocks/planned"), Some(anchor(name))));
        }
        None
    }

    /// The route of a block for a component: its page's route, or the planned page's with an
    /// anchor.
    pub fn block_route(&self, name: &str) -> Option<String> {
        self.block_target(name).map(|(id, a)| match a {
            Some(a) => format!("{}#{a}", route_of(&self.base, &id)),
            None => route_of(&self.base, &id),
        })
    }

    pub fn class_dir(&self, class: &str) -> String {
        self.class_dirs
            .get(class)
            .cloned()
            .unwrap_or_else(|| class.to_string())
    }

    /// The KPIs of a block, in catalogue order.
    pub fn kpis_of<'b>(&'b self, block: &'b str) -> impl Iterator<Item = &'b Kpi> + 'b {
        self.kpis.iter().filter(move |k| k.block == block)
    }

    /// The front matter fields of a file of the pack: `source`, and `source_url` when the
    /// manifest says where a file of the pack is read (`source_file_url`).
    pub fn source(&self, page: &mut Page, path: &str) {
        page.set("source", path);
        if let Some(url) = self.source_url(path) {
            page.set("source_url", url);
        }
    }

    /// `source` for a file of a section read in `root`: none when the section is inherited
    /// from a base pack, whose files the pack's `source_file_url` does not reach.
    pub fn source_in(&self, page: &mut Page, root: &Path, path: &str) {
        if root == self.pack.root {
            self.source(page, path);
        }
    }

    pub fn source_url(&self, path: &str) -> Option<String> {
        self.pack
            .manifest
            .source_file_url
            .as_ref()
            .map(|t| t.replace("{path}", path))
    }

    /// The text every page of a private pack starts with.
    pub fn banner(&self) -> &'static str {
        if visibility(self.pack) == "private" {
            ":::warning[Private pack]\n\nThis page is of a private pack. It is not for publication.\n\n:::\n\n"
        } else {
            ""
        }
    }
}

/// The anchor Docusaurus gives a heading of plain text.
pub fn anchor(text: &str) -> String {
    let mut out = String::new();
    for c in text.trim().chars() {
        if c.is_alphanumeric() || c == '_' || c == '-' {
            out.extend(c.to_lowercase());
        } else if c.is_whitespace() {
            out.push('-');
        }
    }
    out
}

/// Everything one docs build writes: path relative to the output directory, and its text.
#[derive(Debug, Clone, Default)]
pub struct Built {
    pub files: BTreeMap<String, String>,
    /// The number of pages of each kind, over every pack.
    pub kinds: BTreeMap<&'static str, usize>,
    /// Per pack, in the order of the site: its name and the number of its pages of each kind.
    pub packs: Vec<(String, BTreeMap<&'static str, usize>)>,
}

impl Built {
    /// The number of pages of the packs; the site's own page `/` is not counted.
    pub fn pages(&self) -> usize {
        self.kinds.values().sum()
    }
}

/// The pages of one pack: files relative to the pack's directory of the site, and its entry in
/// `index.json`.
struct PackBuilt {
    files: BTreeMap<String, String>,
    kinds: BTreeMap<&'static str, usize>,
    entry: Value,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Item {
    Doc(String),
    Category(Category),
}

#[derive(Serialize)]
pub struct Category {
    r#type: &'static str,
    label: String,
    collapsed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<DocLink>,
    items: Vec<Item>,
}

#[derive(Serialize)]
pub struct DocLink {
    r#type: &'static str,
    id: String,
}

/// A sidebar category of pages, linked to a page when `link` names one.
pub fn category(label: impl Into<String>, link: Option<String>, items: Vec<Item>) -> Item {
    Item::Category(Category {
        r#type: "category",
        label: label.into(),
        collapsed: true,
        link: link.map(|id| DocLink { r#type: "doc", id }),
        items,
    })
}

/// The presentation table of a pack: per block its colour, icon, group, layer and route, the
/// groups in legend order, and the paths of the icons in use.
fn presentation(ctx: &Ctx) -> Value {
    let mut blocks = Map::new();
    let mut groups: Vec<Value> = Vec::new();
    let mut seen_groups: BTreeSet<String> = BTreeSet::new();
    let mut used_icons: BTreeSet<String> = BTreeSet::new();
    for b in &ctx.pack.blocks {
        let s = ctx.style.block(Some(&b.name));
        if let Some(icon) = &s.icon {
            used_icons.insert(icon.clone());
        }
        if seen_groups.insert(s.group.clone()) {
            groups.push(json!({"label": s.group, "colour": s.colour, "icon": s.icon}));
        }
        blocks.insert(
            b.name.clone(),
            json!({
                "label": ctx.style.word(&b.name),
                "colour": s.colour,
                "icon": s.icon,
                "group": s.group,
                "layer": b.layer.as_ref().map(|l| l.number),
                "layer_name": b.layer.as_ref().map(|l| l.name.clone()),
                "route": ctx.block_route(&b.name),
            }),
        );
    }
    for p in &ctx.planned {
        let s = ctx.style.block(Some(p));
        blocks.insert(
            p.clone(),
            json!({
                "label": ctx.style.word(p),
                "colour": s.colour,
                "icon": null,
                "group": "planned",
                "layer": null,
                "layer_name": null,
                "route": ctx.block_route(p),
            }),
        );
    }
    for class in &ctx.drawn.classes {
        for page in &class.pages {
            for scene in &page.scenes {
                for entry in &scene.legend {
                    if let Some(icon) = &entry.icon {
                        used_icons.insert(icon.clone());
                    }
                }
            }
        }
    }
    let icons: Map<String, Value> = used_icons
        .iter()
        .filter_map(|n| icons::path(n).map(|p| (n.clone(), Value::String(p.to_string()))))
        .collect();
    json!({
        "blocks": blocks,
        "groups": groups,
        "icons": icons,
        "vocabulary": ctx.style.vocabulary,
    })
}

/// The palette as CSS for a site whose dark theme is `html[data-theme='dark']`: per colour a
/// class `sa-c-<name>` that sets `--sa-fill` and `--sa-stroke`, and the page colours of the
/// drawings.
pub fn palette_css() -> String {
    let page = |p: &palette::Page| {
        format!(
            "--sa-bg:{};--sa-fg:{};--sa-muted:{};--sa-line:{};--sa-band:{};--sa-finding:{};--sa-finding-fg:{};--sa-link:{};",
            p.background, p.text, p.muted, p.line, p.band, p.finding, p.finding_text, p.link
        )
    };
    let mut out = String::from(
        "/* Generated by metaspec docs build from metaspec's palette. Do not edit. */\n",
    );
    out.push_str(&format!(":root{{{}}}\n", page(&palette::LIGHT)));
    for c in palette::COLOURS {
        out.push_str(&format!(
            ".sa-c-{}{{--sa-fill:{};--sa-stroke:{}}}\n",
            c.name, c.light_fill, c.light_stroke
        ));
    }
    out.push_str(&format!(
        "html[data-theme='dark']{{{}}}\n",
        page(&palette::DARK)
    ));
    for c in palette::COLOURS {
        out.push_str(&format!(
            "html[data-theme='dark'] .sa-c-{}{{--sa-fill:{};--sa-stroke:{}}}\n",
            c.name, c.dark_fill, c.dark_stroke
        ));
    }
    out
}

fn json_text(v: &Value) -> String {
    let mut s = serde_json::to_string_pretty(v).unwrap_or_default();
    s.push('\n');
    s
}

/// The check of a pack's documents in the site's list of packs: how many the pack's tool found
/// valid, refused, or did not check, and one word for the pack. `refused` when the tool refused
/// one document; `valid` when it checked every drawn document and refused none; else
/// `not checked`.
fn check_summary(drawn: &Drawn) -> Value {
    let (mut valid, mut refused, mut unchecked) = (0usize, 0usize, 0usize);
    for page in drawn.classes.iter().flat_map(|c| c.pages.iter()) {
        match (page.check.state, &page.check.result) {
            (CheckState::Ran, Some(r)) if r.valid => valid += 1,
            (CheckState::Ran, Some(_)) | (CheckState::Refused, _) => refused += 1,
            _ => unchecked += 1,
        }
    }
    let status = if refused > 0 {
        "refused"
    } else if valid > 0 && unchecked == 0 {
        "valid"
    } else {
        "not checked"
    };
    json!({"status": status, "valid": valid, "refused": refused, "not_checked": unchecked})
}

/// Builds the pages of one pack, routed below `base` (`/<pack>`). The tool's `check` runs for a
/// trusted pack only (`state` is the state directory with the trust file).
fn build_pack(pack: &Pack, state: Option<&Path>, deadline: Duration, base: &str) -> PackBuilt {
    let model = Model::new(pack);
    let kpis = kpis::read(pack);
    let mut planned: Vec<String> = Vec::new();
    for k in &kpis {
        if !k.block.is_empty() && model.by_name(&k.block).is_none() && !planned.contains(&k.block) {
            planned.push(k.block.clone());
        }
    }
    let class_dirs: BTreeMap<String, String> = pack
        .manifest
        .documents
        .iter()
        .map(|c| {
            let dir = if RESERVED.contains(&c.class.as_str()) {
                format!("documents/{}", c.class)
            } else {
                c.class.clone()
            };
            (c.class.clone(), dir)
        })
        .collect();
    let dirs = class_dirs.clone();
    let drawn = document::build(pack, state, deadline, &|class: &str| {
        dirs.get(class)
            .cloned()
            .unwrap_or_else(|| class.to_string())
    });
    let ctx = Ctx {
        pack,
        model,
        kpis,
        style: Style::of_pack(pack),
        planned,
        drawn,
        class_dirs,
        base: base.to_string(),
    };

    let mut pages: Vec<Page> = Vec::new();
    let mut sidebar: Vec<Item> = vec![Item::Doc(String::from("index"))];

    let (block_pages, block_items) = blocks::pages(&ctx);
    pages.extend(block_pages);
    sidebar.push(category(
        "Building blocks",
        Some(String::from("blocks/index")),
        block_items,
    ));

    if pack.manifest.kpis.is_some() {
        pages.push(catalogue::page(&ctx));
        sidebar.push(Item::Doc(String::from("kpis")));
    }

    let (doc_pages, doc_items) = documents::pages(&ctx);
    pages.extend(doc_pages);
    sidebar.extend(doc_items);

    if pack.manifest.tool.is_some() {
        pages.push(documents::rules_page(&ctx));
        sidebar.push(Item::Doc(String::from("rules")));
    }

    let index = crate::documents::pack_page(&ctx, &pages);
    pages.insert(0, index);

    let mut files = BTreeMap::new();
    let mut kinds = BTreeMap::new();
    for p in &pages {
        *kinds.entry(p.kind).or_insert(0) += 1;
        files.insert(format!("docs/{}.mdx", p.id), p.mdx());
        files.insert(format!("pages/{}.json", p.id), json_text(&p.json()));
    }
    let counts = json!({
        "blocks": pack.blocks.len(),
        "layers": pack.blocks.iter().filter_map(|b| b.layer.as_ref().map(|l| l.number)).collect::<BTreeSet<_>>().len(),
        "kpis": ctx.kpis.len(),
        "planned_blocks": ctx.planned.len(),
        "documents": pack.documents.iter().map(Vec::len).sum::<usize>(),
        "pages": pages.len(),
    });
    let site = json!({
        "format": SITE_FORMAT,
        "pack": pack.manifest.pack,
        "title": pack.manifest.title,
        "summary": pack.manifest.summary,
        "visibility": visibility(pack),
        "source_url": pack.manifest.source_url,
        "level": pack.manifest.level(),
        "counts": counts,
        "sidebar": serde_json::to_value(&sidebar).unwrap_or(Value::Null),
        "pages": pages.iter().map(|p| json!({
            "id": p.id,
            "kind": p.kind,
            "slug": route_of(base, &p.id),
            "title": p.title(),
        })).collect::<Vec<_>>(),
        "presentation": presentation(&ctx),
    });
    files.insert(String::from("site.json"), json_text(&site));
    let entry = json!({
        "pack": pack.manifest.pack,
        "title": pack.manifest.title,
        "summary": pack.manifest.summary,
        "visibility": visibility(pack),
        "source_url": pack.manifest.source_url,
        "level": pack.manifest.level(),
        "route": base,
        "site": format!("{}/site.json", pack.manifest.pack),
        "counts": counts,
        "check": check_summary(&ctx.drawn),
    });
    PackBuilt {
        files,
        kinds,
        entry,
    }
}

/// The site's own page `/`: every pack with its summary, its counts and the check of its
/// documents, a link to the pack's pages.
fn index_page(entries: &[Value]) -> String {
    let mut s = String::from("---\n");
    s.push_str(&format!("title: {SITE_TITLE}\n"));
    s.push_str(&format!(
        "description: {}\n",
        yaml("The docs of every specification pack of this site.")
    ));
    s.push_str("hide_table_of_contents: true\n---\n\n");
    s.push_str("{/* Generated by metaspec docs build. Do not edit. */}\n\n");
    s.push_str(&format!("# {SITE_TITLE}\n\n"));
    s.push_str(&format!(
        "{} on this site. Each has its building blocks, its documents and, where the pack has \
         them, its KPIs and the findings of its tool.\n\n",
        plural(entries.len(), "pack", "packs")
    ));
    s.push_str("| pack | what it holds | blocks | documents | KPIs | check |\n");
    s.push_str("|---|---|---|---|---|---|\n");
    for e in entries {
        let n = |k: &str| e["counts"][k].as_u64().unwrap_or(0);
        let check = &e["check"];
        let count = |k: &str| check[k].as_u64().unwrap_or(0) as usize;
        let status = check["status"].as_str().unwrap_or("not checked");
        let parts: Vec<(usize, &str)> = [
            ("refused", "refused"),
            ("valid", "valid"),
            ("not_checked", "not checked"),
        ]
        .iter()
        .filter(|(k, _)| count(k) > 0)
        .map(|(k, word)| (count(k), *word))
        .collect();
        let detail = match parts.as_slice() {
            [] => format!("{status} (no document to check)"),
            [(n, _)] => format!("{status} ({})", plural(*n, "document", "documents")),
            _ => format!(
                "{status} ({})",
                parts
                    .iter()
                    .map(|(n, word)| format!("{n} {word}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        };
        let _ = writeln!(
            s,
            "| [{}]({}) | {} | {} | {} | {} | {} |",
            text(e["title"].as_str().unwrap_or("")),
            e["route"].as_str().unwrap_or("/"),
            text(e["summary"].as_str().unwrap_or("")),
            n("blocks"),
            n("documents"),
            n("kpis"),
            detail
        );
    }
    s
}

/// Builds every file of one docs site over the loaded packs: per pack its pages under
/// `<pack>/` (routed below `/<pack>`), the site's list of packs `index.json`, its page
/// `_index/index.mdx` and `palette.css`. The packs are ordered by name. Refuses two packs of one name.
pub fn build(packs: &[&Pack], state: Option<&Path>, deadline: Duration) -> Result<Built, String> {
    let mut sorted: Vec<&Pack> = packs.to_vec();
    sorted.sort_by(|a, b| a.manifest.pack.cmp(&b.manifest.pack));
    if let Some(w) = sorted
        .windows(2)
        .find(|w| w[0].manifest.pack == w[1].manifest.pack)
    {
        return Err(format!(
            "two packs are named {}; a site holds each pack once",
            w[0].manifest.pack
        ));
    }
    let mut built = Built::default();
    let mut entries = Vec::new();
    for pack in sorted {
        let name = pack.manifest.pack.clone();
        let one = build_pack(pack, state, deadline, &format!("/{name}"));
        for (path, text) in one.files {
            built.files.insert(format!("{name}/{path}"), text);
        }
        for (kind, n) in &one.kinds {
            *built.kinds.entry(kind).or_insert(0) += n;
        }
        built.packs.push((name, one.kinds));
        entries.push(one.entry);
    }
    let index = json!({
        "format": INDEX_FORMAT,
        "title": SITE_TITLE,
        "packs": entries,
    });
    built
        .files
        .insert(String::from("index.json"), json_text(&index));
    built
        .files
        .insert(String::from("_index/index.mdx"), index_page(&entries));
    built
        .files
        .insert(String::from("palette.css"), palette_css());
    Ok(built)
}

fn walk(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = entry.path();
        if p.is_dir() {
            walk(&p, out);
        } else {
            out.push(p);
        }
    }
}

fn on_disk(out: &Path) -> BTreeSet<String> {
    let mut found = Vec::new();
    walk(out, &mut found);
    found
        .into_iter()
        .filter_map(|p| {
            p.strip_prefix(out)
                .ok()
                .map(|r| r.to_string_lossy().replace('\\', "/"))
        })
        .collect()
}

/// One line per file that differs from the generation: `changed`, `missing`, or `not owned`
/// (a file in the output directory the generation does not write).
pub fn check(out: &Path, built: &Built) -> Vec<String> {
    let mut lines = Vec::new();
    for (path, text) in &built.files {
        match std::fs::read(out.join(path)) {
            Ok(bytes) if bytes == text.as_bytes() => {}
            Ok(_) => lines.push(format!("changed: {path}")),
            Err(_) => lines.push(format!("missing: {path}")),
        }
    }
    for path in on_disk(out) {
        if !built.files.contains_key(&path) {
            lines.push(format!("not owned: {path}"));
        }
    }
    lines
}

/// Whether a directory is the output of an earlier docs build: it has an `index.json` of
/// format `metaspec-index/1`, or, written before one site held every pack, a `site.json` of
/// format `metaspec-site/1`.
fn is_output(out: &Path) -> bool {
    let format = |file: &str| {
        std::fs::read_to_string(out.join(file))
            .ok()
            .and_then(|t| serde_json::from_str::<Value>(&t).ok())
            .and_then(|v| v["format"].as_str().map(String::from))
    };
    format("index.json").as_deref() == Some(INDEX_FORMAT)
        || format("site.json").as_deref() == Some(SITE_FORMAT)
}

/// Writes every file that differs and removes the files it does not own. Refuses a directory
/// that holds files and is not the output of an earlier docs build. Returns one line per file
/// written or removed.
pub fn write(out: &Path, built: &Built) -> Result<Vec<String>, String> {
    if out.exists() {
        let empty = std::fs::read_dir(out)
            .map_err(|e| format!("{} cannot be read: {e}", out.display()))?
            .next()
            .is_none();
        if !empty && !is_output(out) {
            return Err(format!(
                "{} holds files and no index.json of a docs build; metaspec writes only into an empty directory or its own output",
                out.display()
            ));
        }
    }
    let mut changed = Vec::new();
    for (path, text) in &built.files {
        let target = out.join(path);
        if std::fs::read(&target).is_ok_and(|b| b == text.as_bytes()) {
            continue;
        }
        if let Some(dir) = target.parent() {
            std::fs::create_dir_all(dir).map_err(|e| format!("{path}: {e}"))?;
        }
        std::fs::write(&target, text).map_err(|e| format!("{path}: {e}"))?;
        changed.push(format!("written: {path}"));
    }
    for path in on_disk(out) {
        if !built.files.contains_key(&path) {
            std::fs::remove_file(out.join(&path)).map_err(|e| format!("{path}: {e}"))?;
            changed.push(format!("removed: {path}"));
        }
    }
    remove_empty(out);
    Ok(changed)
}

fn remove_empty(dir: &Path) {
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = entry.path();
        if p.is_dir() {
            remove_empty(&p);
            let _ = std::fs::remove_dir(&p);
        }
    }
}
