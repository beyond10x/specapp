//! Loading a pack: the manifest, the compiled blocks with their layers, the KPI catalogue and
//! the documents of each class. Nothing of the pack is executed; the blocks are compiled with
//! `ess specify compile`.

use crate::manifest::{self, DocumentClass, Manifest, Shape, Visibility};
use crate::paths::{relative_inside, resolved_inside, shown};
use crate::report::{ClassRow, Counts, FORMAT, Issue, LayerRow, Report, ToolRow, ToolState};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

/// The layer of a block: the number and the name of its directory, `<number>-<name>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer {
    pub number: u32,
    pub name: String,
}

/// One block: a domain of the compiled system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// The domain name without the manifest's `strip_prefix`.
    pub name: String,
    pub domain: String,
    pub layer: Option<Layer>,
}

/// One document of a class.
#[derive(Debug, Clone)]
pub struct Document {
    /// The path relative to the pack, with forward slashes.
    pub path: String,
    /// The document, or `None` when it is no YAML document.
    pub value: Option<Value>,
}

/// A loaded pack.
#[derive(Debug, Clone)]
pub struct Pack {
    /// The pack directory, resolved.
    pub root: PathBuf,
    /// The base pack the manifest inherits sections from, when it does.
    pub base: Option<Base>,
    /// The manifest, with the sections it inherits filled in.
    pub manifest: Manifest,
    /// The compiled system, `ess specify compile --format json`.
    pub model: Value,
    /// The blocks in the order of the system's specification list.
    pub blocks: Vec<Block>,
    /// The rows of the KPI catalogue.
    pub kpis: Vec<Value>,
    /// The documents of each class, in the order of `manifest.documents`.
    pub documents: Vec<Vec<Document>>,
}

impl Pack {
    /// The directory `blocks.path` is read in: the base's when the blocks are inherited.
    pub fn blocks_root(&self) -> &Path {
        self.root_of(|i| i.blocks)
    }

    /// The directory `kpis.path` is read in: the base's when the catalogue is inherited.
    pub fn kpis_root(&self) -> &Path {
        self.root_of(|i| i.kpis)
    }

    /// The directory the tool runs in and its `tool.run` is read in: the base's when the tool is
    /// inherited.
    pub fn tool_root(&self) -> &Path {
        self.root_of(|i| i.tool)
    }

    fn root_of(&self, inherited: fn(&Inherited) -> bool) -> &Path {
        match &self.base {
            Some(base) if inherited(&base.inherited) => &base.root,
            _ => &self.root,
        }
    }
}

/// Compiles an ESS system with the `ess` program on PATH and returns its JSON. `ess` runs in the
/// blocks directory with `--path .`, as ess asks: there a release that dispatches hands the run
/// to the release the blocks' `ess-inputs.yaml` pins, where from a directory above it refuses.
pub fn ess_compile(blocks: &Path) -> Result<Value, String> {
    let output = Command::new("ess")
        .args(["specify", "compile", "--path", ".", "--format", "json"])
        .current_dir(blocks)
        .output()
        .map_err(|error| format!("ess cannot be started: {error}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let first = stderr.lines().take(20).collect::<Vec<_>>().join("\n");
        return Err(format!("ess specify compile refused the blocks:\n{first}"));
    }
    serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("ess specify compile printed no JSON: {error}"))
}

/// Reads and parses the manifest at `path`.
pub fn read_manifest(path: &Path) -> Result<Manifest, Vec<Issue>> {
    let text = std::fs::read_to_string(path).map_err(|error| {
        vec![Issue::new(
            "manifest_unreadable",
            "manifest",
            format!("the manifest cannot be read: {error}"),
        )]
    })?;
    manifest::parse(&text)
}

/// Reads the manifest of a pack metaspec knows (`packs/<name>.yaml`, see [`crate::known`]).
/// `None` when metaspec knows no pack of that name.
pub fn known_manifest(name: &str) -> Option<Result<Manifest, Vec<Issue>>> {
    crate::known::manifest(name).map(manifest::parse)
}

/// Loads the pack in `dir` with the manifest at `manifest`. The manifest is metaspec's; it lies
/// outside the pack.
pub fn load(
    dir: &Path,
    manifest: &Path,
    compile: &dyn Fn(&Path) -> Result<Value, String>,
) -> (Report, Option<Pack>) {
    match read_manifest(manifest) {
        Ok(manifest) => load_with(dir, manifest, compile),
        Err(errors) => (Report::refused(errors, Vec::new()), None),
    }
}

/// Where the pack a manifest `extends` is: its manifest, a file given at run time (`--extends`)
/// or else the pack metaspec knows by that name, and its directory (`--base-pack`).
#[derive(Debug, Clone, Copy, Default)]
pub struct BaseArgs<'a> {
    pub manifest: Option<&'a Path>,
    pub dir: Option<&'a Path>,
}

/// The sections a manifest with `extends` leaves out and takes from its base pack. `documents`
/// and `presentation` are never inherited.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Inherited {
    pub blocks: bool,
    pub kpis: bool,
    pub tool: bool,
}

impl Inherited {
    /// The names of the inherited sections, in the order of the manifest.
    pub fn names(&self) -> Vec<&'static str> {
        [
            (self.blocks, "blocks"),
            (self.kpis, "kpis"),
            (self.tool, "tool"),
        ]
        .into_iter()
        .filter_map(|(on, name)| on.then_some(name))
        .collect()
    }

    pub fn any(&self) -> bool {
        self.blocks || self.kpis || self.tool
    }
}

/// The base pack a loaded pack inherits sections from: its name, its directory (resolved), and
/// which sections.
#[derive(Debug, Clone)]
pub struct Base {
    pub pack: String,
    pub root: PathBuf,
    pub inherited: Inherited,
}

/// A manifest with the sections it inherits filled in from its base.
#[derive(Debug, Clone)]
pub struct Inheritance {
    pub manifest: Manifest,
    /// The base, when the manifest inherits a section.
    pub base: Option<Base>,
    /// What metaspec says about `extends` that does not refuse the pack.
    pub warnings: Vec<Issue>,
}

/// The manifest of the pack a manifest `extends`: the file given at run time, which must be the
/// manifest of that pack, or else the pack metaspec knows by that name. `None` for a manifest
/// without `extends`.
pub fn base_manifest(manifest: &Manifest, given: Option<&Path>) -> Option<Result<Manifest, Issue>> {
    let name = manifest.extends.as_ref()?;
    let subject = format!("extends:{name}");
    let read = match given {
        Some(path) => Some(read_manifest(path)),
        None => known_manifest(name),
    };
    Some(match read {
        None => Err(Issue::new(
            "extends_not_found",
            &subject,
            format!(
                "metaspec knows no pack {name} (packs/) and no manifest of it is given (--extends)"
            ),
        )),
        Some(Ok(other)) if other.pack != *name => Err(Issue::new(
            "extends_other_pack",
            &subject,
            format!(
                "the manifest given for {name} is the manifest of {}",
                other.pack
            ),
        )),
        Some(Ok(found)) => Ok(found),
        Some(Err(errors)) => Err(Issue::new(
            "extends_unreadable",
            &subject,
            format!(
                "the manifest of {name} is refused: {}",
                errors
                    .iter()
                    .map(|e| e.message.as_str())
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        )),
    })
}

/// Fills in the sections a manifest with `extends` leaves out (`blocks`, `kpis`, `tool`) from
/// the manifest of its base pack (see [`base_manifest`]), whose paths are read in the base's
/// directory `base.dir`.
///
/// - A manifest that writes its own `blocks` and is given no base directory inherits nothing
///   and loads as before; with a base directory its omitted `kpis` and `tool` are the base's.
/// - A manifest that leaves out `blocks` needs the base directory (`base_pack_missing`).
/// - A base that cannot be read is a warning when the manifest has its own `blocks`, and
///   refuses it when it leaves them out.
/// - A base that itself extends a pack refuses a manifest that inherits a section
///   (`extends_chain`).
/// - A base directory given to a manifest without `extends` is a warning (`base_pack_unused`).
///
/// An inherited tool carries the canonical base directory ([`crate::manifest::Tool::base_dir`]),
/// so its digest, and the trust of it, covers the directory the tool runs in.
pub fn inherit(manifest: Manifest, base: BaseArgs) -> Result<Inheritance, Vec<Issue>> {
    let alone = |manifest, warnings| {
        Ok(Inheritance {
            manifest,
            base: None,
            warnings,
        })
    };
    let Some(found) = base_manifest(&manifest, base.manifest) else {
        let warnings = match base.dir {
            Some(dir) => vec![Issue::new(
                "base_pack_unused",
                "manifest",
                format!(
                    "--base-pack {} is given, but the manifest extends no pack; nothing is read there",
                    dir.display()
                ),
            )],
            None => Vec::new(),
        };
        return alone(manifest, warnings);
    };
    let name = manifest.extends.clone().unwrap_or_default();
    let subject = format!("extends:{name}");
    let found = match found {
        Ok(found) => found,
        Err(issue) if manifest.blocks.is_some() => return alone(manifest, vec![issue]),
        Err(issue) => return Err(vec![issue]),
    };
    // Own blocks and no base directory: nothing is inherited, as before inheritance.
    if manifest.blocks.is_some() && base.dir.is_none() {
        return alone(manifest, Vec::new());
    }
    let inherited = Inherited {
        blocks: manifest.blocks.is_none(),
        kpis: manifest.kpis.is_none() && found.kpis.is_some(),
        tool: manifest.tool.is_none() && found.tool.is_some(),
    };
    if !inherited.any() {
        return alone(manifest, Vec::new());
    }
    if let Some(other) = &found.extends {
        return Err(vec![Issue::new(
            "extends_chain",
            &subject,
            format!(
                "the pack {name} extends {other} itself; metaspec reads one base pack, not a chain"
            ),
        )]);
    }
    let sections = inherited.names().join(", ");
    let Some(dir) = base.dir else {
        return Err(vec![Issue::new(
            "base_pack_missing",
            &subject,
            format!(
                "the manifest inherits {sections} from {name}; give the directory of {name} with --base-pack <DIR>"
            ),
        )]);
    };
    let root = match dir.canonicalize() {
        Ok(root) if root.is_dir() => root,
        _ => {
            return Err(vec![Issue::new(
                "base_pack_missing",
                &subject,
                format!(
                    "the directory of {name} that --base-pack gives does not exist; the manifest inherits {sections} from it"
                ),
            )]);
        }
    };
    let mut manifest = manifest;
    if inherited.blocks {
        manifest.blocks = found.blocks;
    }
    if inherited.kpis {
        manifest.kpis = found.kpis;
    }
    if inherited.tool {
        manifest.tool = found.tool.map(|mut tool| {
            tool.base_dir = Some(root.to_string_lossy().into_owned());
            tool
        });
    }
    Ok(Inheritance {
        manifest,
        base: Some(Base {
            pack: name,
            root,
            inherited,
        }),
        warnings: Vec::new(),
    })
}

/// Loads the pack in `dir` with a manifest already read. A manifest with `extends` resolves its
/// base as [`load_extending`] does without a base given: by the name of a pack metaspec knows,
/// and with no directory to inherit from.
pub fn load_with(
    dir: &Path,
    manifest: Manifest,
    compile: &dyn Fn(&Path) -> Result<Value, String>,
) -> (Report, Option<Pack>) {
    load_extending(dir, manifest, BaseArgs::default(), compile)
}

/// Loads the pack in `dir` with a manifest already read and, when it `extends` a pack, the
/// sections it inherits from that pack (see [`inherit`]). What metaspec says about `extends` is
/// added to the warnings.
pub fn load_extending(
    dir: &Path,
    manifest: Manifest,
    base: BaseArgs,
    compile: &dyn Fn(&Path) -> Result<Value, String>,
) -> (Report, Option<Pack>) {
    match inherit(manifest, base) {
        Ok(inheritance) => {
            let (mut report, pack) =
                load_inherited(dir, inheritance.manifest, inheritance.base, compile);
            report.warnings.extend(inheritance.warnings);
            (report, pack)
        }
        Err(errors) => (Report::refused(errors, Vec::new()), None),
    }
}

/// Where the paths of one section are read: a pack directory, and in a pack with a base the
/// words a refusal of such a path ends with, which name the pack.
struct Place<'a> {
    root: &'a Path,
    label: &'a Option<String>,
}

impl Place<'_> {
    /// `issue`, its message ending with the pack the path was in when a base is in play.
    fn name(&self, mut issue: Issue) -> Issue {
        if let Some(label) = self.label {
            issue.message = format!("{} ({label})", issue.message);
        }
        issue
    }
}

fn load_inherited(
    dir: &Path,
    manifest: Manifest,
    base: Option<Base>,
    compile: &dyn Fn(&Path) -> Result<Value, String>,
) -> (Report, Option<Pack>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    let root = match dir.canonicalize() {
        Ok(root) if root.is_dir() => root,
        _ => {
            errors.push(Issue::new(
                "pack_missing",
                "manifest",
                "the pack directory does not exist",
            ));
            return (Report::refused(errors, warnings), None);
        }
    };
    let Some(blocks_spec) = manifest.blocks.clone() else {
        errors.push(Issue::new(
            "manifest_invalid",
            "manifest",
            "the manifest names no `blocks` and inherits none",
        ));
        return (Report::refused(errors, warnings), None);
    };

    // Where each section is read: its own pack, or the base it is inherited from.
    let own_label = base
        .as_ref()
        .map(|_| format!("a path of the pack {}", manifest.pack));
    let base_label = base
        .as_ref()
        .map(|b| format!("a path of the base pack {}, read in --base-pack", b.pack));
    let own = Place {
        root: &root,
        label: &own_label,
    };
    let place = |inherited: fn(&Inherited) -> bool| match &base {
        Some(b) if inherited(&b.inherited) => Place {
            root: &b.root,
            label: &base_label,
        },
        _ => Place {
            root: &root,
            label: &own_label,
        },
    };
    let blocks_place = place(|i| i.blocks);
    let kpis_place = place(|i| i.kpis);
    let tool_place = place(|i| i.tool);

    // Every path the manifest names, before anything is read.
    let mut named = vec![(blocks_spec.path.clone(), &blocks_place)];
    named.extend(manifest.kpis.iter().map(|k| (k.path.clone(), &kpis_place)));
    for class in &manifest.documents {
        named.extend(class.paths.iter().map(|p| (p.clone(), &own)));
        named.extend(class.exclude.iter().map(|p| (p.clone(), &own)));
    }
    if let Some(program) = manifest.tool.as_ref().and_then(|t| t.run.first())
        && program.contains('/')
    {
        named.push((program.clone(), &tool_place));
    }
    for (text, place) in &named {
        if let Err(why) = relative_inside(text) {
            errors.push(place.name(Issue::new(
                "path_leaves_pack",
                format!("path:{text}"),
                format!("refused: {why}"),
            )));
        }
    }
    let mut classes = BTreeSet::new();
    for class in &manifest.documents {
        if !classes.insert(class.class.as_str()) {
            errors.push(Issue::new(
                "class_duplicate",
                format!("class:{}", class.class),
                "two document classes have this name",
            ));
        }
    }
    if !errors.is_empty() {
        return (Report::refused(errors, warnings), None);
    }

    // The blocks.
    let blocks_dir = match resolve(blocks_place.root, &blocks_spec.path) {
        Ok(path) if path.is_dir() => path,
        Ok(_) => {
            errors.push(blocks_place.name(Issue::new(
                "blocks_missing",
                format!("path:{}", blocks_spec.path),
                "blocks.path is no directory",
            )));
            return (Report::refused(errors, warnings), None);
        }
        Err(issue) => {
            errors.push(blocks_place.name(issue));
            return (Report::refused(errors, warnings), None);
        }
    };
    let model = match compile(&blocks_dir) {
        Ok(model) => model,
        Err(why) => {
            errors.push(blocks_place.name(Issue::new(
                "blocks_refused",
                format!("path:{}", blocks_spec.path),
                why,
            )));
            return (Report::refused(errors, warnings), None);
        }
    };
    let blocks = match read_blocks(
        blocks_place.root,
        &blocks_dir,
        &blocks_spec,
        &model,
        &mut warnings,
    ) {
        Ok(blocks) => blocks,
        Err(issue) => {
            errors.push(blocks_place.name(issue));
            return (Report::refused(errors, warnings), None);
        }
    };
    let names: BTreeSet<&str> = blocks.iter().map(|b| b.name.as_str()).collect();

    // The aliases of a part's block word: each stands for a block, and none is a block itself.
    for (word, block) in &blocks_spec.aliases {
        let subject = format!("alias:{word}");
        if names.contains(word.as_str()) {
            errors.push(Issue::new(
                "manifest_invalid",
                &subject,
                format!(
                    "blocks.aliases names {word}, which is a block of the pack; an alias is no block"
                ),
            ));
        } else if blocks.iter().any(|b| b.domain == *word) {
            errors.push(Issue::new(
                "manifest_invalid",
                &subject,
                format!(
                    "blocks.aliases names {word}, which is the domain of a block of the pack; an alias is no block"
                ),
            ));
        }
        if !names.contains(block.as_str()) {
            errors.push(Issue::new(
                "manifest_invalid",
                &subject,
                format!("blocks.aliases maps {word} to {block}, which the blocks do not have"),
            ));
        }
    }

    // The KPI catalogue.
    let mut kpis = Vec::new();
    let mut planned = BTreeSet::new();
    if let Some(spec) = &manifest.kpis {
        match read_yaml(kpis_place.root, &spec.path) {
            Ok(Value::Object(catalogue)) => match catalogue.get("kpis") {
                Some(Value::Array(rows)) => kpis = rows.clone(),
                _ => errors.push(kpis_place.name(Issue::new(
                    "kpis_unreadable",
                    format!("path:{}", spec.path),
                    "the catalogue has no list `kpis`",
                ))),
            },
            Ok(_) => errors.push(kpis_place.name(Issue::new(
                "kpis_unreadable",
                format!("path:{}", spec.path),
                "the catalogue is no mapping",
            ))),
            Err(issue) => errors.push(kpis_place.name(issue)),
        }
        let mut seen = BTreeSet::new();
        for (index, row) in kpis.iter().enumerate() {
            let id = row.get("id").and_then(Value::as_str);
            let subject = match id {
                Some(id) => format!("kpi:{id}"),
                None => format!("kpi:#{index}"),
            };
            match id {
                None => warnings.push(Issue::new("kpi_without_id", &subject, "the row has no id")),
                Some(id) if !seen.insert(id.to_string()) => warnings.push(Issue::new(
                    "kpi_duplicate",
                    &subject,
                    "two rows have this id",
                )),
                Some(_) => {}
            }
            match row.get("block").and_then(Value::as_str) {
                None => warnings.push(Issue::new(
                    "kpi_without_block",
                    &subject,
                    "the row names no block",
                )),
                Some(block) if !names.contains(block) => {
                    planned.insert(block.to_string());
                }
                Some(_) => {}
            }
            if row.get(&spec.fields.measure).is_none() {
                warnings.push(Issue::new(
                    "kpi_without_measure",
                    &subject,
                    format!(
                        "the row has no field `{}`, which kpis.fields.measure names",
                        spec.fields.measure
                    ),
                ));
            }
        }
    }

    // The documents.
    let mut documents = Vec::new();
    for class in &manifest.documents {
        match read_class(&root, class, &mut warnings) {
            Ok(found) => documents.push(found),
            Err(issue) => {
                errors.push(own.name(issue));
                documents.push(Vec::new());
            }
        }
        for view in &class.views {
            let hub_blocks = view.hubs.iter().flat_map(|h| h.blocks.iter());
            let collapse_blocks = match &view.collapse {
                Some(crate::manifest::Collapse::Blocks(c)) => c.blocks.as_slice(),
                _ => &[],
            };
            for block in view.blocks.iter().chain(hub_blocks).chain(collapse_blocks) {
                if let Some(target) = manifest.blocks().aliases.get(block) {
                    warnings.push(Issue::new(
                        "view_alias_block",
                        format!("view:{}/{}", class.class, view.id),
                        format!("{block} is an alias of {target}; name {target}"),
                    ));
                } else if !names.contains(block.as_str()) {
                    warnings.push(Issue::new(
                        "view_unknown_block",
                        format!("view:{}/{}", class.class, view.id),
                        format!("the view names the block {block}, which the blocks do not have"),
                    ));
                }
            }
            if let (Some(collapse), Some(found)) = (&view.collapse, documents.last()) {
                collapse_warnings(&manifest, class, view, collapse, found, &mut warnings);
            }
        }
    }

    // Presentation.
    if let Some(presentation) = &manifest.presentation
        && !presentation.families.is_empty()
    {
        let mut family_of: BTreeMap<&str, &str> = BTreeMap::new();
        for (family, spec) in &presentation.families {
            for block in &spec.blocks {
                if let Some(target) = manifest.blocks().aliases.get(block) {
                    warnings.push(Issue::new(
                        "family_alias_block",
                        format!("family:{family}"),
                        format!("{block} is an alias of {target}; name {target}"),
                    ));
                } else if !names.contains(block.as_str()) {
                    warnings.push(Issue::new(
                        "family_unknown_block",
                        format!("family:{family}"),
                        format!("the family names the block {block}, which the blocks do not have"),
                    ));
                }
                if let Some(first) = family_of.insert(block.as_str(), family.as_str()) {
                    warnings.push(Issue::new(
                        "block_in_two_families",
                        format!("block:{block}"),
                        format!("the block is in the families {first} and {family}"),
                    ));
                }
            }
        }
        for block in &blocks {
            if !family_of.contains_key(block.name.as_str()) {
                warnings.push(Issue::new(
                    "block_in_no_family",
                    format!("block:{}", block.name),
                    "the block is in no family; it is drawn in the colour of its layer",
                ));
            }
        }
    }

    // The tool: found or not. Nothing is run.
    let tool = match &manifest.tool {
        None => ToolRow {
            run: None,
            verbs: None,
            state: ToolState::None,
        },
        Some(spec) => {
            let state = match find_program(tool_place.root, &spec.run[0]) {
                Ok(()) => ToolState::Found,
                Err(why) => {
                    warnings.push(tool_place.name(Issue::new(
                        "tool_not_started",
                        format!("tool:{}", spec.run[0]),
                        format!("the tool cannot be started: {why}"),
                    )));
                    ToolState::NotFound
                }
            };
            ToolRow {
                run: Some(spec.run.clone()),
                verbs: Some(spec.verbs.names().into_iter().map(String::from).collect()),
                state,
            }
        }
    };

    if !errors.is_empty() {
        return (Report::refused(errors, warnings), None);
    }

    let mut layers: Vec<LayerRow> = Vec::new();
    for block in &blocks {
        let Some(layer) = &block.layer else { continue };
        match layers.iter_mut().find(|row| row.number == layer.number) {
            Some(row) => row.blocks.push(block.name.clone()),
            None => layers.push(LayerRow {
                number: layer.number,
                name: layer.name.clone(),
                blocks: vec![block.name.clone()],
            }),
        }
    }
    layers.sort_by_key(|row| row.number);
    let class_rows: Vec<ClassRow> = manifest
        .documents
        .iter()
        .zip(&documents)
        .map(|(class, found)| ClassRow {
            class: class.class.clone(),
            shape: class.shape.as_str(),
            documents: found.len(),
        })
        .collect();
    let counts = Counts {
        blocks: blocks.len(),
        layers: layers.len(),
        kpis: kpis.len(),
        planned_blocks: planned.len(),
        document_classes: manifest.documents.len(),
        documents: documents.iter().map(Vec::len).sum(),
    };
    let report = Report {
        format: FORMAT,
        valid: true,
        pack: Some(manifest.pack.clone()),
        title: Some(manifest.title.clone()),
        visibility: Some(match manifest.visibility {
            Visibility::Public => "public",
            Visibility::Private => "private",
        }),
        extends: manifest.extends.clone(),
        level: Some(manifest.level()),
        counts: Some(counts),
        layers: Some(layers),
        documents: Some(class_rows),
        tool: Some(tool),
        errors,
        warnings,
    };
    let pack = Pack {
        root,
        base,
        manifest,
        model,
        blocks,
        kpis,
        documents,
    };
    (report, Some(pack))
}

/// Resolves a relative path of the manifest that must exist inside the pack.
fn resolve(root: &Path, text: &str) -> Result<PathBuf, Issue> {
    resolved_inside(root, &root.join(text)).map_err(|why| {
        let code = if root.join(text).exists() {
            "path_leaves_pack"
        } else {
            "path_missing"
        };
        Issue::new(code, format!("path:{text}"), why)
    })
}

/// Reads a YAML file named by a relative path of the manifest.
fn read_yaml(root: &Path, text: &str) -> Result<Value, Issue> {
    let path = resolve(root, text)?;
    let content = std::fs::read_to_string(&path).map_err(|error| {
        Issue::new(
            "path_unreadable",
            format!("path:{text}"),
            format!("cannot be read: {error}"),
        )
    })?;
    serde_yaml_ng::from_str(&content).map_err(|error| {
        Issue::new(
            "path_unreadable",
            format!("path:{text}"),
            format!("is no YAML document: {error}"),
        )
    })
}

/// The blocks of the compiled model, each with the layer of the directory its file is in.
fn read_blocks(
    root: &Path,
    blocks_dir: &Path,
    spec: &manifest::Blocks,
    model: &Value,
    warnings: &mut Vec<Issue>,
) -> Result<Vec<Block>, Issue> {
    let subject = format!("path:{}", spec.path);
    let Some(domains) = model.get("domains").and_then(Value::as_object) else {
        return Err(Issue::new(
            "blocks_refused",
            subject,
            "the compiled model has no domains",
        ));
    };

    // domain -> layer, from the specification list of ess-inputs.yaml
    let inputs = blocks_dir.join("ess-inputs.yaml");
    let inputs: Value = std::fs::read_to_string(&inputs)
        .ok()
        .and_then(|text| serde_yaml_ng::from_str(&text).ok())
        .unwrap_or(Value::Null);
    let mut order: Vec<(String, Option<Layer>)> = Vec::new();
    for entry in inputs
        .get("specification")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        if relative_inside(entry).is_err() {
            continue;
        }
        let Ok(file) = resolved_inside(root, &blocks_dir.join(entry)) else {
            continue;
        };
        let domain = std::fs::read_to_string(&file)
            .ok()
            .and_then(|text| serde_yaml_ng::from_str::<Value>(&text).ok())
            .and_then(|value| {
                value
                    .get("domain")
                    .and_then(Value::as_str)
                    .map(String::from)
            });
        let Some(domain) = domain else { continue };
        let parts: Vec<&str> = entry.split('/').collect();
        let layer = if parts.len() == 2 {
            layer_of(parts[0])
        } else {
            None
        };
        order.push((domain, layer));
    }
    for domain in domains.keys() {
        if !order.iter().any(|(d, _)| d == domain) {
            order.push((domain.clone(), None));
        }
    }

    let mut blocks = Vec::new();
    for (domain, layer) in order {
        if !domains.contains_key(&domain) {
            continue;
        }
        let name = match &spec.strip_prefix {
            Some(prefix) => match domain.strip_prefix(prefix.as_str()) {
                Some(rest) => rest.to_string(),
                None => {
                    warnings.push(Issue::new(
                        "block_prefix",
                        format!("block:{domain}"),
                        format!("the domain does not start with {prefix}, which blocks.strip_prefix names"),
                    ));
                    domain.clone()
                }
            },
            None => domain.clone(),
        };
        if layer.is_none() {
            warnings.push(Issue::new(
                "block_without_layer",
                format!("block:{name}"),
                "the file of the block is in no numbered layer directory directly under blocks.path",
            ));
        }
        blocks.push(Block {
            name,
            domain,
            layer,
        });
    }
    Ok(blocks)
}

/// The layer a directory name `<number>-<name>` stands for.
fn layer_of(dir: &str) -> Option<Layer> {
    let (number, name) = dir.split_once('-')?;
    if number.is_empty() || !number.bytes().all(|b| b.is_ascii_digit()) || name.is_empty() {
        return None;
    }
    Some(Layer {
        number: number.parse().ok()?,
        name: name.to_string(),
    })
}

/// The documents of one class: every file its globs match and no `exclude` glob matches, inside
/// the pack, read as YAML.
/// The warnings of a view's `collapse` over the documents of its class, read as their pages read
/// them (with the manifest's aliases). A region is a part of the view's blocks that such a part
/// sits `in`.
///
/// - `view_unknown_collapse`: an id `collapse` lists that is a region of no document: no part
///   has it, or parts are only `per` it.
/// - `view_region_edges`: a region named [`manifest::REGION_EDGES`] that the view would collapse;
///   it stays expanded, since its view would take the id of the view `<id>/edges`.
fn collapse_warnings(
    manifest: &Manifest,
    class: &DocumentClass,
    view: &manifest::View,
    collapse: &manifest::Collapse,
    documents: &[Document],
    warnings: &mut Vec<Issue>,
) {
    use manifest::Collapse;
    let subject = format!("view:{}/{}", class.class, view.id);
    let mut regions: BTreeSet<String> = BTreeSet::new();
    let mut edges_in: Vec<&str> = Vec::new();
    for document in documents {
        let Some(value) = &document.value else {
            continue;
        };
        let graph = crate::graph::read_with_aliases(
            class,
            &document.path,
            value,
            &manifest.blocks().aliases,
        );
        let shown: BTreeMap<&str, &crate::graph::Part> = graph
            .parts
            .iter()
            .filter(|p| {
                view.blocks.is_empty() || p.block.as_ref().is_some_and(|b| view.blocks.contains(b))
            })
            .map(|p| (p.id.as_str(), p))
            .collect();
        let held: BTreeSet<&str> = shown
            .values()
            .filter_map(|p| p.inside.as_deref().filter(|h| *h != p.id))
            .filter(|h| shown.contains_key(h))
            .collect();
        regions.extend(held.iter().map(|h| h.to_string()));
        if let Some(part) = shown.get(manifest::REGION_EDGES)
            && held.contains(manifest::REGION_EDGES)
        {
            let collapsed = match collapse {
                Collapse::Every(_) => !part
                    .inside
                    .as_deref()
                    .is_some_and(|h| h != part.id && shown.contains_key(h)),
                Collapse::Parts(ids) => ids.iter().any(|id| id == manifest::REGION_EDGES),
                Collapse::Blocks(c) => part.block.as_ref().is_some_and(|b| c.blocks.contains(b)),
            };
            if collapsed {
                edges_in.push(document.path.as_str());
            }
        }
    }
    if let Collapse::Parts(ids) = collapse {
        for id in ids {
            if !regions.contains(id) {
                warnings.push(Issue::new(
                    "view_unknown_collapse",
                    &subject,
                    format!(
                        "the view collapses {id}, which is a region (a part other parts sit in) of no document of the class"
                    ),
                ));
            }
        }
    }
    if !edges_in.is_empty() {
        warnings.push(Issue::new(
            "view_region_edges",
            &subject,
            format!(
                "the view would collapse the region {} of {}; it stays expanded, since its view would take the id {}/{}",
                manifest::REGION_EDGES,
                edges_in.join(", "),
                view.id,
                manifest::REGION_EDGES
            ),
        ));
    }
}

fn read_class(
    root: &Path,
    class: &DocumentClass,
    warnings: &mut Vec<Issue>,
) -> Result<Vec<Document>, Issue> {
    let base = glob::Pattern::escape(&root.to_string_lossy());
    // An exclude is matched against the path in the pack as `shown` writes it, which has no `.`
    // component and no empty one: `./d/*.yaml` names what `d/*.yaml` names.
    let exclude = class
        .exclude
        .iter()
        .map(|pattern| {
            let whole = pattern
                .split('/')
                .filter(|part| !part.is_empty() && *part != ".")
                .collect::<Vec<_>>()
                .join("/");
            glob::Pattern::new(&whole).map_err(|error| {
                Issue::new(
                    "path_invalid",
                    format!("path:{pattern}"),
                    format!("is no glob: {error}"),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let whole_names = glob::MatchOptions {
        require_literal_separator: true,
        ..glob::MatchOptions::new()
    };
    let mut files = BTreeSet::new();
    let mut left_out = vec![0usize; exclude.len()];
    for pattern in &class.paths {
        let full = format!("{base}/{pattern}");
        let matches = glob::glob(&full).map_err(|error| {
            Issue::new(
                "path_invalid",
                format!("path:{pattern}"),
                format!("is no glob: {error}"),
            )
        })?;
        for path in matches.flatten() {
            if !path.is_file() {
                continue;
            }
            let rel = shown(root, &path);
            let hits: Vec<usize> = (0..exclude.len())
                .filter(|&i| exclude[i].matches_with(&rel, whole_names))
                .collect();
            if !hits.is_empty() {
                for i in hits {
                    left_out[i] += 1;
                }
                continue;
            }
            resolved_inside(root, &path)
                .map_err(|why| Issue::new("path_leaves_pack", format!("path:{pattern}"), why))?;
            files.insert(path);
        }
    }
    for (glob, count) in class.exclude.iter().zip(&left_out) {
        if *count == 0 {
            warnings.push(Issue::new(
                "exclude_matches_nothing",
                format!("path:{glob}"),
                format!(
                    "the exclude of the class {} matches no file its paths match",
                    class.class
                ),
            ));
        }
    }
    if files.is_empty() {
        let message = if left_out.iter().any(|n| *n > 0) {
            "the exclude of the class leaves out every file its paths match"
        } else {
            "the paths of the class match no file"
        };
        warnings.push(Issue::new(
            "class_empty",
            format!("class:{}", class.class),
            message,
        ));
    }
    let mut documents = Vec::new();
    for path in files {
        let rel = shown(root, &path);
        let value = std::fs::read_to_string(&path)
            .ok()
            .and_then(|text| serde_yaml_ng::from_str::<Value>(&text).ok());
        match &value {
            None => warnings.push(Issue::new(
                "document_unreadable",
                format!("document:{rel}"),
                "the document is no YAML document",
            )),
            Some(doc) if doc.get(&class.name).and_then(Value::as_str).is_none() => {
                warnings.push(Issue::new(
                    "document_without_name",
                    format!("document:{rel}"),
                    format!(
                        "the document has no `{}`, which the class names as its name",
                        class.name
                    ),
                ))
            }
            Some(_) => {}
        }
        if class.shape == Shape::PartsLinks
            && let Some(doc) = &value
            && doc.get("parts").is_some_and(|p| !p.is_object())
        {
            warnings.push(Issue::new(
                "document_parts",
                format!("document:{rel}"),
                "`parts` is no mapping from a name to a part",
            ));
        }
        documents.push(Document { path: rel, value });
    }
    Ok(documents)
}

/// Whether the first element of `tool.run` names a program that can be started: a path inside
/// the pack (it has a slash) to an executable file, or a name found on PATH.
fn find_program(root: &Path, program: &str) -> Result<(), String> {
    if program.contains('/') {
        let path = resolved_inside(root, &root.join(program))
            .map_err(|_| format!("{program} is no file inside the pack"))?;
        return if executable(&path) {
            Ok(())
        } else {
            Err(format!("{program} is not executable"))
        };
    }
    let path = std::env::var_os("PATH").unwrap_or_default();
    if std::env::split_paths(&path).any(|dir| executable(&dir.join(program))) {
        Ok(())
    } else {
        Err(format!("{program} is not on PATH"))
    }
}

fn executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}
