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

/// The name of the manifest in a pack directory.
pub const MANIFEST: &str = "specapp.yaml";

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

/// Compiles an ESS system with the `ess` program on PATH and returns its JSON.
pub fn ess_compile(blocks: &Path) -> Result<Value, String> {
    let output = Command::new("ess")
        .args(["specify", "compile", "--path"])
        .arg(blocks)
        .args(["--format", "json"])
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

/// The manifest a pack directory is read with: the one named, or `specapp.yaml` in it.
pub fn manifest_path(dir: &Path, manifest: Option<&Path>) -> PathBuf {
    match manifest {
        Some(path) => path.to_path_buf(),
        None => dir.join(MANIFEST),
    }
}

/// Reads and parses the manifest.
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

/// Loads the pack in `dir` with the manifest `manifest` (default: `dir/specapp.yaml`).
pub fn load(
    dir: &Path,
    manifest: Option<&Path>,
    compile: &dyn Fn(&Path) -> Result<Value, String>,
) -> (Report, Option<Pack>) {
    match read_manifest(&manifest_path(dir, manifest)) {
        Ok(manifest) => load_with(dir, manifest, compile),
        Err(errors) => (Report::refused(errors, Vec::new()), None),
    }
}

/// Loads the pack in `dir` with a manifest already read.
pub fn load_with(
    dir: &Path,
    manifest: Manifest,
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

    // Every path the manifest names, before anything is read.
    let mut named = vec![manifest.blocks.path.clone()];
    named.extend(manifest.kpis.iter().map(|k| k.path.clone()));
    for class in &manifest.documents {
        named.extend(class.paths.iter().cloned());
    }
    if let Some(program) = manifest.tool.as_ref().and_then(|t| t.run.first())
        && program.contains('/')
    {
        named.push(program.clone());
    }
    for text in &named {
        if let Err(why) = relative_inside(text) {
            errors.push(Issue::new(
                "path_leaves_pack",
                format!("path:{text}"),
                format!("refused: {why}"),
            ));
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
    let blocks_dir = match resolve(&root, &manifest.blocks.path) {
        Ok(path) if path.is_dir() => path,
        Ok(_) => {
            errors.push(Issue::new(
                "blocks_missing",
                format!("path:{}", manifest.blocks.path),
                "blocks.path is no directory",
            ));
            return (Report::refused(errors, warnings), None);
        }
        Err(issue) => {
            errors.push(issue);
            return (Report::refused(errors, warnings), None);
        }
    };
    let model = match compile(&blocks_dir) {
        Ok(model) => model,
        Err(why) => {
            errors.push(Issue::new(
                "blocks_refused",
                format!("path:{}", manifest.blocks.path),
                why,
            ));
            return (Report::refused(errors, warnings), None);
        }
    };
    let blocks = match read_blocks(&root, &blocks_dir, &manifest, &model, &mut warnings) {
        Ok(blocks) => blocks,
        Err(issue) => {
            errors.push(issue);
            return (Report::refused(errors, warnings), None);
        }
    };
    let names: BTreeSet<&str> = blocks.iter().map(|b| b.name.as_str()).collect();

    // The KPI catalogue.
    let mut kpis = Vec::new();
    let mut planned = BTreeSet::new();
    if let Some(spec) = &manifest.kpis {
        match read_yaml(&root, &spec.path) {
            Ok(Value::Object(catalogue)) => match catalogue.get("kpis") {
                Some(Value::Array(rows)) => kpis = rows.clone(),
                _ => errors.push(Issue::new(
                    "kpis_unreadable",
                    format!("path:{}", spec.path),
                    "the catalogue has no list `kpis`",
                )),
            },
            Ok(_) => errors.push(Issue::new(
                "kpis_unreadable",
                format!("path:{}", spec.path),
                "the catalogue is no mapping",
            )),
            Err(issue) => errors.push(issue),
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
                errors.push(issue);
                documents.push(Vec::new());
            }
        }
        for view in &class.views {
            for block in &view.blocks {
                if !names.contains(block.as_str()) {
                    warnings.push(Issue::new(
                        "view_unknown_block",
                        format!("view:{}/{}", class.class, view.id),
                        format!("the view names the block {block}, which the blocks do not have"),
                    ));
                }
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
                if !names.contains(block.as_str()) {
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
            let state = match find_program(&root, &spec.run[0]) {
                Ok(()) => ToolState::Found,
                Err(why) => {
                    warnings.push(Issue::new(
                        "tool_not_started",
                        format!("tool:{}", spec.run[0]),
                        format!("the tool cannot be started: {why}"),
                    ));
                    ToolState::NotFound
                }
            };
            ToolRow {
                run: Some(spec.run.clone()),
                verbs: Some(
                    spec.verbs
                        .iter()
                        .map(|v| {
                            serde_json::to_value(v)
                                .ok()
                                .and_then(|v| v.as_str().map(String::from))
                                .unwrap_or_default()
                        })
                        .collect(),
                ),
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
    manifest: &Manifest,
    model: &Value,
    warnings: &mut Vec<Issue>,
) -> Result<Vec<Block>, Issue> {
    let subject = format!("path:{}", manifest.blocks.path);
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
        let name = match &manifest.blocks.strip_prefix {
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

/// The documents of one class: every file its globs match, inside the pack, read as YAML.
fn read_class(
    root: &Path,
    class: &DocumentClass,
    warnings: &mut Vec<Issue>,
) -> Result<Vec<Document>, Issue> {
    let base = glob::Pattern::escape(&root.to_string_lossy());
    let mut files = BTreeSet::new();
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
            resolved_inside(root, &path)
                .map_err(|why| Issue::new("path_leaves_pack", format!("path:{pattern}"), why))?;
            files.insert(path);
        }
    }
    if files.is_empty() {
        warnings.push(Issue::new(
            "class_empty",
            format!("class:{}", class.class),
            "the paths of the class match no file",
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
