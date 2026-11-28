//! The pages of documents: per class an index, one page per `parts-links/1` document (its views
//! as drawings, its check, its parts and links as tables, its marked places), one page per
//! `records/1` document with the tables the manifest names, the rules page, and the pack page.

use crate::document::{DocumentPage, check_sentence, file_stem, page_names, state_word};
use crate::md::{cell, code, plural, text};
use crate::site::{Ctx, Item, Page, category, rel};
use metaspec_core::graph::Marker;
use metaspec_core::manifest::{DocumentClass, Shape};
use metaspec_core::pack::Document;
use metaspec_core::tool::CheckState;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// The pages of every document class and their sidebar items.
pub fn pages(ctx: &Ctx) -> (Vec<Page>, Vec<Item>) {
    let mut pages = Vec::new();
    let mut items = Vec::new();
    for ((class, documents), drawn) in ctx
        .pack
        .manifest
        .documents
        .iter()
        .zip(&ctx.pack.documents)
        .zip(&ctx.drawn.classes)
    {
        let dir = ctx.class_dir(&class.class);
        let mut class_pages = Vec::new();
        match class.shape {
            Shape::PartsLinks => {
                for d in &drawn.pages {
                    class_pages.push(document_page(ctx, class, d));
                }
            }
            Shape::Records => {
                let readable: Vec<&Document> =
                    documents.iter().filter(|d| d.value.is_some()).collect();
                let paths: Vec<&str> = readable.iter().map(|d| d.path.as_str()).collect();
                let names = page_names(&dir, &paths);
                for (d, id) in readable.into_iter().zip(names) {
                    class_pages.push(record_page(ctx, class, d, id));
                }
            }
        }
        let index = class_index(ctx, class, documents, &class_pages, &dir);
        items.push(category(
            class.title.clone(),
            Some(index.id.clone()),
            class_pages
                .iter()
                .map(|p| Item::Doc(p.id.clone()))
                .collect(),
        ));
        pages.push(index);
        pages.extend(class_pages);
    }
    (pages, items)
}

fn title_of(p: &Page) -> String {
    p.front
        .iter()
        .find(|(k, _)| *k == "title")
        .and_then(|(_, v)| v.as_str())
        .unwrap_or(&p.id)
        .to_string()
}

fn summary_of(value: Option<&Value>) -> Option<String> {
    value?
        .get("summary")
        .and_then(Value::as_str)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn class_index(
    ctx: &Ctx,
    class: &DocumentClass,
    documents: &[Document],
    pages: &[Page],
    dir: &str,
) -> Page {
    let id = format!("{dir}/index");
    let mut p = Page::new(id.clone(), "class");
    p.set("title", class.title.clone());
    p.set("sidebar_label", "Overview");
    p.set(
        "description",
        format!(
            "{} of {}: {}.",
            class.title,
            ctx.pack.manifest.title,
            plural(documents.len(), "document", "documents")
        ),
    );
    p.set("source", class.paths.join(", "));
    let mut s = String::from(ctx.banner());
    let _ = writeln!(
        s,
        "{} of the class {} ({}), read from {}.\n",
        plural(documents.len(), "document", "documents"),
        code(&class.class),
        code(class.shape.as_str()),
        class
            .paths
            .iter()
            .map(|g| code(g))
            .collect::<Vec<_>>()
            .join(", ")
    );
    match class.shape {
        Shape::PartsLinks => {
            let drawn = ctx
                .drawn
                .classes
                .iter()
                .find(|c| c.class == class.class)
                .map(|c| c.pages.as_slice())
                .unwrap_or(&[]);
            s.push_str("Each page draws the document in the views of its class, with the findings of the pack's tool on the parts and links they name.\n\n");
            s.push_str("| document | parts | links | check | findings |\n|---|---|---|---|---|\n");
            for d in drawn {
                let check = match (d.check.state, &d.check.result) {
                    (CheckState::Ran, Some(r)) if r.valid => String::from("valid"),
                    (CheckState::Ran, _) => String::from("not valid"),
                    (state, _) => format!("not checked ({})", state_word(state)),
                };
                let _ = writeln!(
                    s,
                    "| [{}]({}) | {} | {} | {check} | {} |",
                    text(&d.name),
                    rel(&id, &d.page),
                    d.graph.parts.len(),
                    d.graph.links.len(),
                    d.findings.len()
                );
            }
        }
        Shape::Records => {
            let mut header = String::from("| document |");
            let mut rule = String::from("|---|");
            for t in &class.tables {
                let _ = write!(header, " {} |", text(&t.title));
                rule.push_str("---|");
            }
            let _ = writeln!(s, "{header}\n{rule}");
            for page in pages {
                let mut row = format!("| [{}]({}) |", text(&title_of(page)), rel(&id, &page.id));
                if let Some(Value::Array(tables)) = page.data.get("tables") {
                    for t in tables {
                        let n = t["rows"].as_array().map_or(0, Vec::len);
                        let _ = write!(row, " {n} |");
                    }
                }
                let _ = writeln!(s, "{row}");
            }
        }
    }
    p.markdown = s;
    p
}

fn marked_rows(prefix: &str, markers: &[Marker], out: &mut Vec<(String, String, String)>) {
    for m in markers {
        let place = if prefix.is_empty() {
            m.path.clone()
        } else {
            format!("{prefix}.{}", m.path)
        };
        let vary = match &m.vary {
            Value::Object(r) if r.contains_key("min") || r.contains_key("max") => {
                let mut t = format!(
                    "{} to {}",
                    cell(r.get("min").unwrap_or(&Value::Null)),
                    cell(r.get("max").unwrap_or(&Value::Null))
                );
                if let Some(step) = r.get("step") {
                    let _ = write!(t, " by {}", cell(step));
                }
                t
            }
            other => cell(other),
        };
        out.push((place, cell(&m.value), vary));
    }
}

/// The section of the KPI values the pack's tool measured on a document: one row per KPI, in
/// the order of the KPI catalogue, then the KPIs the catalogue does not hold, by id.
fn kpi_values(ctx: &Ctx, id: &str, kpis: &BTreeMap<String, serde_json::Number>, s: &mut String) {
    s.push_str("## KPIs\n\n");
    if kpis.is_empty() {
        s.push_str("The pack's tool measured no KPI of this document.\n\n");
        return;
    }
    let _ = write!(
        s,
        "{} the pack's tool measured on the document as written",
        plural(kpis.len(), "KPI", "KPIs")
    );
    if ctx.pack.manifest.kpis.is_some() {
        let _ = write!(
            s,
            ", with what the [KPI catalogue]({}) says of each",
            rel(id, "kpis")
        );
    }
    s.push_str(".\n\n");
    s.push_str("| KPI | value | unit | what it measures |\n|---|---|---|---|\n");
    let known = ctx.kpis.iter().filter(|k| kpis.contains_key(&k.id));
    for k in known {
        let name = match ctx.block_target(&k.block) {
            Some((target, anchor)) => format!(
                "[{}]({}#{})",
                code(&k.id),
                rel(id, &target),
                anchor.unwrap_or_else(|| String::from("kpis"))
            ),
            None => code(&k.id),
        };
        let _ = writeln!(
            s,
            "| {name} | {} | {} | {} |",
            kpis[&k.id],
            if k.unit.is_empty() {
                String::from("–")
            } else {
                text(&k.unit)
            },
            if k.summary.is_empty() {
                String::from("–")
            } else {
                text(&k.summary)
            }
        );
    }
    for (kpi, value) in kpis {
        if !ctx.kpis.iter().any(|k| &k.id == kpi) {
            let _ = writeln!(
                s,
                "| {} | {value} | – | not in the KPI catalogue |",
                code(kpi)
            );
        }
    }
    s.push('\n');
}

fn document_page(ctx: &Ctx, class: &DocumentClass, d: &DocumentPage) -> Page {
    let id = d.page.clone();
    let mut p = Page::new(id.clone(), "document");
    p.set("title", d.name.clone());
    p.set("sidebar_label", d.name.clone());
    let value = ctx
        .pack
        .documents
        .iter()
        .flatten()
        .find(|x| x.path == d.document)
        .and_then(|x| x.value.as_ref());
    if let Some(summary) = summary_of(value) {
        p.set("description", summary.clone());
        p.set("lede", summary);
    }
    p.set("kicker", class.title.clone());
    ctx.source(&mut p, &d.document);
    let mut s = String::from(ctx.banner());

    let (sentence, _) = check_sentence(d);
    s.push_str("## Check\n\n");
    let _ = writeln!(s, "{}\n", text(&sentence));
    s.push_str("A finding is a numbered mark on the part or link it names (filled: gated; outlined: not gated) and a row of the list of findings.\n\n");

    for (n, scene) in d.scenes.iter().enumerate() {
        let _ = writeln!(s, "## {}\n", text(&scene.view.title));
        let boxes = scene.boxes.iter().filter(|b| !b.ghost).count();
        let ghosts = scene.boxes.len() - boxes;
        let mut meta = format!(
            "Layout {}: {}, {}, {}.",
            code(&scene.view.layout),
            plural(boxes, "part", "parts"),
            plural(scene.edges.len(), "link", "links"),
            plural(scene.marks.len(), "finding mark", "finding marks")
        );
        if ghosts > 0 {
            let _ = write!(
                meta,
                " {} for names that links give and no part has.",
                plural(ghosts, "dashed box", "dashed boxes")
            );
        }
        if scene.view.implicit {
            meta.push_str(
                " metaspec adds this view because the views of the class leave a part or link out.",
            );
        }
        let _ = writeln!(s, "{meta}\n");
        s.push_str(&p.component(
            "GraphView",
            &format!("scene-{}", n + 1),
            serde_json::to_value(scene).unwrap_or(Value::Null),
        ));
    }

    s.push_str("## Findings\n\n");
    let views: Vec<Value> = d
        .scenes
        .iter()
        .map(|sc| json!({"id": sc.view.id, "title": sc.view.title}))
        .collect();
    if d.findings.is_empty() {
        s.push_str("None.\n\n");
    } else {
        s.push_str(&p.component(
            "FindingsList",
            "findings",
            json!({"findings": d.findings, "views": views}),
        ));
    }

    if let Some(kpis) = d.check.result.as_ref().and_then(|r| r.kpis.as_ref()) {
        kpi_values(ctx, &id, kpis, &mut s);
    }

    s.push_str("## Parts\n\n");
    if d.graph.parts.is_empty() {
        s.push_str("None.\n\n");
    } else {
        s.push_str("| part | block | contained | fields |\n|---|---|---|---|\n");
        for part in &d.graph.parts {
            let block = match &part.block {
                Some(b) => match ctx.block_target(b) {
                    Some((target, anchor)) => format!(
                        "[{}]({}{})",
                        text(&ctx.style.word(b)),
                        rel(&id, &target),
                        anchor.map(|a| format!("#{a}")).unwrap_or_default()
                    ),
                    None => format!("{} (no block of the pack)", code(b)),
                },
                None => String::from("–"),
            };
            let contained = match (&part.inside, &part.per) {
                (Some(i), Some(per)) => format!("in {}, per {}", code(i), code(per)),
                (Some(i), None) => format!("in {}", code(i)),
                (None, Some(per)) => format!("per {}", code(per)),
                (None, None) => String::from("–"),
            };
            let fields: Vec<String> = part
                .fields
                .iter()
                .map(|(k, v)| format!("{} {}", code(k), cell(v)))
                .collect();
            let _ = writeln!(
                s,
                "| {}{} | {block} | {contained} | {} |",
                code(&part.id),
                if part.on_border {
                    " (on the border)"
                } else {
                    ""
                },
                if fields.is_empty() {
                    String::from("–")
                } else {
                    fields.join("; ")
                }
            );
        }
        s.push('\n');
    }

    s.push_str("## Links\n\n");
    if d.graph.links.is_empty() {
        s.push_str("None.\n\n");
    } else {
        s.push_str("| from | carries | to | fields |\n|---|---|---|---|\n");
        for l in &d.graph.links {
            let name = |v: &Option<String>| v.as_deref().map_or(String::from("?"), code);
            let fields: Vec<String> = l
                .fields
                .iter()
                .map(|(k, v)| format!("{} {}", code(k), cell(v)))
                .collect();
            let _ = writeln!(
                s,
                "| {} | {} | {} | {} |",
                name(&l.from),
                l.carries
                    .as_deref()
                    .map_or(String::from("?"), |c| code(&ctx.style.word(c))),
                name(&l.to),
                if fields.is_empty() {
                    String::from("–")
                } else {
                    fields.join("; ")
                }
            );
        }
        s.push('\n');
    }

    if !d.graph.border.is_empty() {
        s.push_str("## Border\n\n");
        let names: Vec<String> = d.graph.border.iter().map(|b| code(b)).collect();
        let _ = writeln!(
            s,
            "The parts on the border of the document: {}.\n",
            names.join(", ")
        );
    }

    s.push_str("## Marked places\n\n");
    let mut marked: Vec<(String, String, String)> = Vec::new();
    for part in &d.graph.parts {
        marked_rows(&format!("parts.{}", part.id), &part.markers, &mut marked);
    }
    for (i, l) in d.graph.links.iter().enumerate() {
        marked_rows(&format!("links.{i}"), &l.markers, &mut marked);
    }
    marked_rows("", &d.graph.markers, &mut marked);
    if marked.is_empty() {
        s.push_str("None: no value of this document is marked for a search.\n\n");
    } else {
        s.push_str("A marked place carries `vary`: the value as written and the values a search may try there.\n\n");
        s.push_str("| place | as written | a search may try |\n|---|---|---|\n");
        for (place, v, vary) in marked {
            let _ = writeln!(s, "| {} | {v} | {vary} |", code(&place));
        }
        s.push('\n');
    }

    let runs = ctx.pack.manifest.tool.as_ref().is_some_and(|t| {
        t.verbs
            .names()
            .iter()
            .any(|v| *v == "simulate" || *v == "search")
    });
    if runs {
        s.push_str("## Run\n\n");
        s.push_str(&p.component("ResultSections", "result", Value::Null));
    }
    p.markdown = s;
    p
}

/// The rows of one table of a record: `at` is a list of mappings, or a mapping from a key to a
/// mapping (the key is the first column).
fn table_rows(value: &Value, at: &str, key: Option<&str>, columns: &[String]) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let cells = |item: &Value| -> Vec<String> {
        columns
            .iter()
            .map(|c| match item.get(c) {
                None | Some(Value::Null) => String::new(),
                Some(Value::String(s)) => s.clone(),
                Some(v) => crate::kpis::scalar(Some(v)),
            })
            .collect()
    };
    match value.get(at) {
        Some(Value::Array(items)) => {
            for item in items {
                let mut row = Vec::new();
                if let Some(k) = key {
                    row.push(crate::kpis::scalar(item.get(k)));
                }
                row.extend(cells(item));
                rows.push(row);
            }
        }
        Some(Value::Object(map)) => {
            for (k, item) in map {
                let mut row = Vec::new();
                if key.is_some() {
                    row.push(k.clone());
                }
                row.extend(cells(item));
                rows.push(row);
            }
        }
        _ => {}
    }
    rows
}

fn record_page(ctx: &Ctx, class: &DocumentClass, d: &Document, id: String) -> Page {
    let mut p = Page::new(id, "record");
    let value = d.value.as_ref().unwrap_or(&Value::Null);
    let name = value
        .get(&class.name)
        .and_then(|v| match v {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            _ => None,
        })
        .unwrap_or_else(|| file_stem(&d.path));
    p.set("title", name.clone());
    p.set("sidebar_label", name);
    if let Some(summary) = summary_of(Some(value)) {
        p.set("description", summary.clone());
        p.set("lede", summary);
    }
    p.set("kicker", class.title.clone());
    ctx.source(&mut p, &d.path);
    let mut s = String::from(ctx.banner());
    let mut tables = Vec::new();
    for (n, t) in class.tables.iter().enumerate() {
        let mut columns: Vec<String> = Vec::new();
        if let Some(k) = &t.key {
            columns.push(k.clone());
        }
        columns.extend(t.columns.iter().cloned());
        let rows = table_rows(value, &t.at, t.key.as_deref(), &t.columns);
        let _ = writeln!(s, "## {}\n", text(&t.title));
        let data = json!({
            "title": t.title,
            "at": t.at,
            "key": t.key,
            "columns": columns,
            "rows": rows,
        });
        tables.push(data.clone());
        if rows.is_empty() {
            let _ = writeln!(s, "The document has no rows at {}.\n", code(&t.at));
            continue;
        }
        let _ = writeln!(
            s,
            "{} at {}.\n",
            plural(rows.len(), "row", "rows"),
            code(&t.at)
        );
        p.components.insert("RecordTable");
        let _ = writeln!(
            s,
            "<RecordTable data={{page.data.tables[{n}]}} presentation={{site.presentation}} />\n"
        );
    }
    p.data.insert(String::from("tables"), Value::Array(tables));
    p.markdown = s;
    p
}

/// The rules page: every rule id the tool reports across the drawn documents, with the documents
/// that break it.
/// A rule on the rules page: whether a finding of it is gated, and each document (name, page,
/// number of findings) that breaks it.
type RuleUse = (bool, Vec<(String, String, usize)>);

pub fn rules_page(ctx: &Ctx) -> Page {
    let id = "rules";
    let mut p = Page::new(id, "rules");
    p.set("title", "Rules");
    p.set("sidebar_label", "Rules");
    p.set(
        "description",
        format!(
            "Every rule the tool of {} reports across its documents, with the documents that break it.",
            ctx.pack.manifest.title
        ),
    );
    let mut s = String::from(ctx.banner());
    let mut rules: BTreeMap<String, RuleUse> = BTreeMap::new();
    let mut ran = 0usize;
    let mut not_ran: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut total = 0usize;
    for class in &ctx.drawn.classes {
        for d in &class.pages {
            total += 1;
            if d.check.state == CheckState::Ran {
                ran += 1;
            } else {
                *not_ran.entry(state_word(d.check.state)).or_insert(0) += 1;
            }
            let mut counts: BTreeMap<&str, (bool, usize)> = BTreeMap::new();
            for f in &d.findings {
                let e = counts.entry(f.rule.as_str()).or_insert((false, 0));
                e.0 |= f.gate;
                e.1 += 1;
            }
            for (rule, (gate, n)) in counts {
                let e = rules.entry(rule.to_string()).or_insert((false, Vec::new()));
                e.0 |= gate;
                e.1.push((d.name.clone(), d.page.clone(), n));
            }
        }
    }
    let _ = write!(
        s,
        "The pack's tool checked {} of {}.",
        ran,
        plural(total, "drawn document", "drawn documents")
    );
    if !not_ran.is_empty() {
        let list: Vec<String> = not_ran
            .iter()
            .map(|(state, n)| format!("{n} {state}"))
            .collect();
        let _ = write!(s, " Not checked: {}.", list.join(", "));
    }
    s.push_str(
        " A rule appears here once a document breaks it; a pack with a broken example per rule shows that each rule can refuse.\n\n",
    );
    if rules.is_empty() {
        s.push_str("No document breaks a rule.\n");
    } else {
        s.push_str("| rule | gate | documents that break it |\n|---|---|---|\n");
        for (rule, (gate, docs)) in &rules {
            let list: Vec<String> = docs
                .iter()
                .map(|(name, page, n)| format!("[{}]({}) ({n})", text(name), rel(id, page)))
                .collect();
            let _ = writeln!(
                s,
                "| {} | {} | {} |",
                code(rule),
                if *gate { "gate" } else { "not gated" },
                list.join(", ")
            );
        }
    }
    p.markdown = s;
    p
}

/// The pack page: what the pack is, its counts, and where each part of the site is.
pub fn pack_page(ctx: &Ctx, pages: &[Page]) -> Page {
    let id = "index";
    let m = &ctx.pack.manifest;
    let mut p = Page::new(id, "pack");
    p.set("title", m.title.clone());
    p.set("sidebar_label", "Overview");
    if let Some(summary) = &m.summary {
        p.set("description", summary.clone());
        p.set("lede", summary.clone());
    }
    p.flag("hide_table_of_contents", true);
    let mut s = String::from(ctx.banner());
    let layers = ctx
        .pack
        .blocks
        .iter()
        .filter_map(|b| b.layer.as_ref().map(|l| l.number))
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    s.push_str("| part of the pack | count |\n|---|---|\n");
    let _ = writeln!(
        s,
        "| [Building blocks]({}) | {} in {} |",
        rel(id, "blocks/index"),
        plural(ctx.pack.blocks.len(), "block", "blocks"),
        plural(layers, "layer", "layers")
    );
    let _ = writeln!(
        s,
        "| [Planned blocks]({}) | {} |",
        rel(id, "blocks/planned"),
        ctx.planned.len()
    );
    if m.kpis.is_some() {
        let _ = writeln!(
            s,
            "| [KPI catalogue]({}) | {} |",
            rel(id, "kpis"),
            plural(ctx.kpis.len(), "KPI", "KPIs")
        );
    }
    for (class, documents) in m.documents.iter().zip(&ctx.pack.documents) {
        let _ = writeln!(
            s,
            "| [{}]({}) | {} |",
            text(&class.title),
            rel(id, &format!("{}/index", ctx.class_dir(&class.class))),
            plural(documents.len(), "document", "documents")
        );
    }
    if pages.iter().any(|x| x.kind == "rules") {
        let rules = ctx
            .drawn
            .classes
            .iter()
            .flat_map(|c| c.pages.iter())
            .flat_map(|d| d.findings.iter().map(|f| f.rule.as_str()))
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        let _ = writeln!(
            s,
            "| [Rules]({}) | {} broken in the documents |",
            rel(id, "rules"),
            plural(rules, "rule", "rules")
        );
    }
    s.push('\n');
    let tool = match &m.tool {
        None => {
            String::from("The pack names no tool: metaspec draws its documents and checks none.")
        }
        Some(t) => {
            let verbs = t.verbs.names();
            format!(
                "The pack's tool answers {}; the findings on these pages are its answers.",
                verbs.iter().map(|v| code(v)).collect::<Vec<_>>().join(", ")
            )
        }
    };
    let _ = writeln!(s, "Level of adoption: {}. {tool}\n", code(m.level()));
    if let Some(url) = &m.source_url {
        let _ = writeln!(s, "Source: [{}]({url}).", text(url));
    }
    p.markdown = s;
    p
}
