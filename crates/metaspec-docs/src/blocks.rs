//! The layer map, one page per block, and the page of planned blocks. Lifted from agenticspec's
//! generator: the sections of a block page are its dependencies, its users, its entities with
//! their lifecycles, its commands with their refusals, its events, views, types and KPIs, and
//! the documents that use it.

use crate::kpis::Kpi;
use crate::layermap::{self, Node};
use crate::md::{code, condition, first_sentence, plural, predicate, short, text, type_ref};
use crate::model::members;
use crate::site::{Ctx, Item, Page, anchor, category, rel};
use metaspec_core::pack::Block;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// Every page of the blocks and their sidebar items.
pub fn pages(ctx: &Ctx) -> (Vec<Page>, Vec<Item>) {
    let mut pages = vec![index(ctx)];
    for b in &ctx.pack.blocks {
        pages.push(page(ctx, b));
    }
    pages.push(planned_page(ctx));

    let mut items = Vec::new();
    let mut by_layer: BTreeMap<Option<u32>, Vec<&Block>> = BTreeMap::new();
    for b in &ctx.pack.blocks {
        by_layer
            .entry(b.layer.as_ref().map(|l| l.number))
            .or_default()
            .push(b);
    }
    for (layer, bs) in by_layer {
        let label = match (layer, &bs[0].layer) {
            (Some(n), Some(l)) => format!("{n} · {}", l.name),
            _ => String::from("no layer"),
        };
        items.push(category(
            label,
            None,
            bs.iter().map(|b| Item::Doc(ctx.block_id(b))).collect(),
        ));
    }
    items.push(Item::Doc(String::from("blocks/planned")));
    (pages, items)
}

fn layer_text(b: &Block) -> String {
    match &b.layer {
        Some(l) => format!("Layer {} · {}", l.number, l.name),
        None => String::from("No layer"),
    }
}

/// The layer map page.
pub fn index(ctx: &Ctx) -> Page {
    let id = "blocks/index";
    let mut p = Page::new(id, "layer-map");
    let blocks = &ctx.pack.blocks;
    let layers: Vec<u32> = {
        let mut l: Vec<u32> = blocks
            .iter()
            .filter_map(|b| b.layer.as_ref().map(|l| l.number))
            .collect();
        l.sort_unstable();
        l.dedup();
        l
    };
    p.set("title", "Building blocks");
    p.set("sidebar_label", "Layer map");
    p.set(
        "description",
        format!(
            "Every building block of {}, by layer, with the blocks it depends on.",
            ctx.pack.manifest.title
        ),
    );
    p.set(
        "lede",
        format!(
            "{} in {}. Each block is a contract of commands, events and views.",
            plural(blocks.len(), "block", "blocks"),
            plural(layers.len(), "layer", "layers")
        ),
    );
    ctx.source(&mut p, &format!("{}/", ctx.pack.manifest.blocks.path));
    p.flag("hide_table_of_contents", true);
    let mut s = String::from(ctx.banner());

    let entities: usize = blocks.iter().map(|b| ctx.model.count(b, "entities")).sum();
    let commands: usize = blocks.iter().map(|b| ctx.model.count(b, "commands")).sum();
    let events: usize = blocks.iter().map(|b| ctx.model.count(b, "events")).sum();

    // The map.
    let max_layer = layers.last().copied();
    let nodes: Vec<Node> = blocks
        .iter()
        .map(|b| match &b.layer {
            Some(l) => Node {
                rank: layers.iter().position(|n| *n == l.number).unwrap_or(0),
                row_name: l.name.clone(),
                row_number: Some(l.number),
            },
            None => Node {
                rank: layers.len(),
                row_name: String::from("no layer"),
                row_number: None,
            },
        })
        .collect();
    let _ = max_layer;
    let index_of = |d: &str| blocks.iter().position(|b| b.domain == d);
    let pairs: Vec<(usize, usize, usize)> = ctx
        .model
        .edges
        .iter()
        .filter_map(|((f, t), n)| Some((index_of(f)?, index_of(t)?, *n)))
        .collect();
    let map = layermap::layout(&nodes, &pairs);

    let _ = writeln!(
        s,
        "The blocks declare {entities} entities, {commands} commands and {events} events. \
An arrow points from a block to a block it depends on; point at a block to see what it uses and what uses it.\n"
    );
    if map.band
        && let Some(card) = map.cards.iter().find(|c| c.band)
    {
        let _ = writeln!(
            s,
            "Every other block uses {}, the block of the lowest layer; its arrows are left out.\n",
            code(&blocks[card.index].name)
        );
    }
    let cards: Vec<Value> = map
        .cards
        .iter()
        .map(|c| {
            let b = &blocks[c.index];
            json!({
                "name": b.name,
                "label": ctx.style.word(&b.name),
                "layer": b.layer.as_ref().map(|l| l.number),
                "summary": first_sentence(&crate::model::summary(ctx.ir(), &b.domain)),
                "route": ctx.block_route(&b.name),
                "kpis": ctx.kpis_of(&b.name).count(),
                "entities": ctx.model.count(b, "entities"),
                "commands": ctx.model.count(b, "commands"),
                "events": ctx.model.count(b, "events"),
                "views": ctx.model.count(b, "views"),
                "uses": ctx.model.depends_on(b).iter().map(|(d, _)| d.name.clone()).collect::<Vec<_>>(),
                "used_by": ctx.model.used_by(b).iter().map(|(d, _)| d.name.clone()).collect::<Vec<_>>(),
                "band": c.band,
                "x": c.x, "y": c.y, "w": c.w, "h": c.h,
            })
        })
        .collect();
    let edges: Vec<Value> = map
        .edges
        .iter()
        .map(|e| {
            json!({
                "from": blocks[e.from].name,
                "to": blocks[e.to].name,
                "count": e.count,
                "d": e.path,
            })
        })
        .collect();
    let rows: Vec<Value> = map
        .rows
        .iter()
        .map(|r| json!({"layer": r.number, "name": r.name, "y": r.y, "h": r.h}))
        .collect();
    s.push_str(&p.component(
        "LayerMap",
        "map",
        json!({
            "width": map.width,
            "height": map.height,
            "rows": rows,
            "blocks": cards,
            "edges": edges,
        }),
    ));

    s.push_str("## Every block\n\n");
    s.push_str("| layer | block | what it is | depends on | KPIs |\n|---|---|---|---|---|\n");
    let mut last: Option<Option<u32>> = None;
    for b in blocks.iter().rev() {
        let layer = b.layer.as_ref().map(|l| l.number);
        let layer_cell = if last != Some(layer) {
            last = Some(layer);
            match &b.layer {
                Some(l) => format!("**{}** {}", l.number, text(&l.name)),
                None => String::from("no layer"),
            }
        } else {
            String::new()
        };
        let deps: Vec<String> = ctx
            .model
            .depends_on(b)
            .iter()
            .map(|(d, _)| format!("[{}]({})", text(&d.name), rel(id, &ctx.block_id(d))))
            .collect();
        let _ = writeln!(
            s,
            "| {layer_cell} | [{}]({}) | {} | {} | {} |",
            text(&b.name),
            rel(id, &ctx.block_id(b)),
            text(&first_sentence(&crate::model::summary(ctx.ir(), &b.domain))),
            if deps.is_empty() {
                "–".into()
            } else {
                deps.join(", ")
            },
            ctx.kpis_of(&b.name).count()
        );
    }
    s.push_str("\n## Planned blocks\n\n");
    if ctx.planned.is_empty() {
        let _ = writeln!(
            s,
            "The KPI catalogue names no block beyond these. [Planned blocks]({}) lists a block once it does.",
            rel(id, "blocks/planned")
        );
    } else {
        let names: Vec<String> = ctx
            .planned
            .iter()
            .map(|p| format!("[{}]({}#{})", text(p), rel(id, "blocks/planned"), anchor(p)))
            .collect();
        let _ = writeln!(
            s,
            "The KPI catalogue already measures blocks the blocks do not declare yet: {}. See [planned blocks]({}).",
            names.join(", "),
            rel(id, "blocks/planned")
        );
    }
    p.markdown = s;
    p
}

fn arr(v: &Value) -> Vec<Value> {
    v.as_array().cloned().unwrap_or_default()
}

fn field_rows(s: &mut String, fields: &[Value]) {
    let meanings = fields.iter().any(|f| f["naming"]["summary"].is_string());
    if meanings {
        s.push_str("| field | type | meaning |\n|---|---|---|\n");
    } else {
        s.push_str("| field | type |\n|---|---|\n");
    }
    for f in fields {
        let name = code(f["name"].as_str().unwrap_or("?"));
        let ty = code(&type_ref(&f["type_ref"]));
        if meanings {
            let _ = writeln!(
                s,
                "| {name} | {ty} | {} |",
                text(f["naming"]["summary"].as_str().unwrap_or(""))
            );
        } else {
            let _ = writeln!(s, "| {name} | {ty} |");
        }
    }
    s.push('\n');
}

fn inline_fields(fields: &[Value]) -> String {
    if fields.is_empty() {
        return "–".into();
    }
    fields
        .iter()
        .map(|f| {
            format!(
                "{} {}",
                code(f["name"].as_str().unwrap_or("?")),
                text(&type_ref(&f["type_ref"]))
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn lifecycle(p: &mut Page, s: &mut String, entity: &Value, n: usize) {
    let lc = &entity["lifecycle"];
    if lc.is_null() {
        return;
    }
    let states: Vec<&str> = lc["states"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    let transitions: Vec<Value> = arr(&lc["transitions"])
        .iter()
        .map(|t| json!({"name": t["name"], "from": t["from"], "to": t["to"]}))
        .collect();
    let name = short(entity["name"].as_str().unwrap_or("?"));
    if transitions.is_empty() {
        if let Some(st) = lc["initial"].as_str() {
            let _ = writeln!(
                s,
                "Lifecycle: one state, {}; a recorded {} does not change.\n",
                code(st),
                text(name)
            );
        }
        return;
    }
    s.push_str(&p.component(
        "Lifecycle",
        &format!("lifecycle-{n}"),
        json!({
            "entity": name,
            "initial": lc["initial"].as_str().unwrap_or(""),
            "states": states,
            "terminal": lc["terminal"].as_array().cloned().unwrap_or_default(),
            "transitions": transitions,
        }),
    ));
}

fn result(o: &Value, ir: &Value) -> String {
    let mut parts = Vec::new();
    let subject = &o["subject"];
    if let Some(entity) = subject["entity"].as_str() {
        let e = text(short(entity));
        match subject["effect"].as_str() {
            Some("creates") => {
                let initial = ir["entities"][entity]["lifecycle"]["initial"].as_str();
                parts.push(match initial {
                    Some(st) => format!("creates a **{e}** in {}", code(st)),
                    None => format!("creates a **{e}**"),
                });
            }
            Some("moves") => {
                let t = &subject["transition"];
                let from: Vec<String> = t["from"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                    .map(code)
                    .collect();
                parts.push(format!(
                    "moves the **{e}** {} → {}",
                    from.join(", "),
                    code(t["to"].as_str().unwrap_or("?"))
                ));
            }
            Some("updates") => parts.push(format!("updates the **{e}**")),
            Some("preserves") => parts.push(format!("leaves the **{e}** as it is")),
            _ => {}
        }
    }
    for a in arr(&o["affects"]) {
        let e = text(short(a["entity"].as_str().unwrap_or("?")));
        if let Some(to) = a["moves"]["to"].as_str() {
            parts.push(format!(
                "moves each **{e}** with {} to {}",
                code(&predicate(&a["filter"])),
                code(to)
            ));
        } else {
            parts.push(format!(
                "updates each **{e}** with {}",
                code(&predicate(&a["filter"]))
            ));
        }
    }
    let emits: Vec<String> = arr(&o["emits"])
        .iter()
        .filter_map(Value::as_str)
        .map(|e| code(short(e)))
        .collect();
    if !emits.is_empty() {
        parts.push(format!("emits {}", emits.join(", ")));
    }
    if parts.is_empty() {
        "no change".into()
    } else {
        parts.join("; ")
    }
}

/// The KPI table of a list of KPIs, grouped by the pack's measure value.
pub fn kpi_sections(ctx: &Ctx, s: &mut String, kpis: &[&Kpi], level: usize) {
    let mut measures: Vec<&str> = Vec::new();
    for k in kpis {
        if !measures.contains(&k.measure.as_str()) {
            measures.push(&k.measure);
        }
    }
    let modes = ctx
        .pack
        .manifest
        .kpis
        .as_ref()
        .map_or(String::from("modes"), |k| k.fields.modes.clone());
    for m in measures {
        let of: Vec<&Kpi> = kpis.iter().copied().filter(|k| k.measure == m).collect();
        let title = if m.is_empty() {
            String::from("Without a measure")
        } else {
            let mut t = m.to_string();
            if let Some(first) = t.get_mut(..1) {
                first.make_ascii_uppercase();
            }
            t
        };
        let _ = writeln!(s, "{} {}\n", "#".repeat(level), text(&title));
        s.push_str("| KPI | unit | better | gate | status |\n|---|---|---|---|---|\n");
        for k in of {
            let mut what = format!("{}<br/>{}", code(&k.id), text(&k.summary));
            if !k.how.is_empty() {
                let _ = write!(what, "<br/><small>How: {}</small>", text(&k.how));
            }
            if !k.modes.is_empty() {
                let list: Vec<String> = k
                    .modes
                    .iter()
                    .map(|m| format!("{}: {}", m.mode, m.direction))
                    .collect();
                let _ = write!(
                    what,
                    "<br/><small>By {}: {}</small>",
                    text(&modes),
                    text(&list.join(", "))
                );
            }
            let _ = writeln!(
                s,
                "| {what} | {} | {} | {} | {} |",
                if k.unit.is_empty() {
                    "–".into()
                } else {
                    text(&k.unit)
                },
                if k.direction.is_empty() {
                    "–".into()
                } else {
                    text(&k.direction)
                },
                if k.gate { "gate" } else { "–" },
                if k.status.is_empty() {
                    "–".into()
                } else {
                    text(&k.status)
                },
            );
        }
        s.push('\n');
    }
}

/// The header data of a block or a planned block.
fn header(ctx: &Ctx, name: &str, b: Option<&Block>, counts: Value) -> Value {
    json!({
        "name": name,
        "label": ctx.style.word(name),
        "layer": b.and_then(|b| b.layer.as_ref().map(|l| l.number)),
        "layer_name": b.and_then(|b| b.layer.as_ref().map(|l| l.name.clone())),
        "planned": b.is_none(),
        "counts": counts,
    })
}

/// The page of one block.
pub fn page(ctx: &Ctx, b: &Block) -> Page {
    let ir = ctx.ir();
    let id = ctx.block_id(b);
    let mut p = Page::new(id.clone(), "block");
    let summary = crate::model::summary(ir, &b.domain);
    p.set("title", ctx.style.word(&b.name));
    p.set("sidebar_label", ctx.style.word(&b.name));
    let first = first_sentence(&summary);
    if !first.is_empty() {
        p.set("description", first);
    }
    p.set("kicker", layer_text(b));
    if let Some(file) = block_file(ctx, b) {
        ctx.source(&mut p, &file);
    }
    let mut s = String::from(ctx.banner());
    let entities = members(ir, &b.domain, "entities");
    let commands = members(ir, &b.domain, "commands");
    let events = members(ir, &b.domain, "events");
    let views = members(ir, &b.domain, "views");
    let actors = members(ir, &b.domain, "actors");
    let kpis: Vec<&Kpi> = ctx.kpis_of(&b.name).collect();
    let counts = json!({
        "entities": entities.len(),
        "commands": commands.len(),
        "events": events.len(),
        "views": views.len(),
        "refusals": ctx.model.refusals(b),
        "kpis": kpis.len(),
    });
    s.push_str(&p.component(
        "BlockHeader",
        "header",
        header(ctx, &b.name, Some(b), counts),
    ));
    if !summary.is_empty() {
        let _ = writeln!(s, "{}\n", text(&summary));
    }

    let link = |d: &Block, n: usize| {
        format!(
            "- [{}]({}) · {} · {}",
            text(&d.name),
            rel(&id, &ctx.block_id(d)),
            layer_text(d).to_lowercase(),
            plural(n, "reference", "references")
        )
    };
    s.push_str("## Depends on\n\n");
    let deps = ctx.model.depends_on(b);
    if deps.is_empty() {
        s.push_str("Nothing: this block depends on no other block.\n\n");
    } else {
        for (d, n) in deps {
            let _ = writeln!(s, "{}", link(d, n));
        }
        s.push('\n');
    }
    s.push_str("## Used by\n\n");
    let users = ctx.model.used_by(b);
    if users.is_empty() {
        s.push_str("No block depends on this one.\n\n");
    } else {
        for (d, n) in users {
            let _ = writeln!(s, "{}", link(d, n));
        }
        s.push('\n');
    }

    if !entities.is_empty() {
        s.push_str("## Entities\n\n");
        for (n, e) in entities.iter().enumerate() {
            let name = short(e["name"].as_str().unwrap_or("?"));
            let _ = writeln!(s, "### {}\n", text(name));
            if let Some(sum) = e["naming"]["summary"].as_str() {
                let _ = writeln!(s, "{}\n", text(sum));
            }
            if let Some(idn) = e["identity"]["name"].as_str() {
                let _ = writeln!(
                    s,
                    "Identified by {} ({}).\n",
                    code(idn),
                    code(&type_ref(&e["identity"]["type_ref"]))
                );
            }
            let fields = arr(&e["fields"]);
            if !fields.is_empty() {
                field_rows(&mut s, &fields);
            }
            let relations = arr(&e["relations"]);
            if !relations.is_empty() {
                s.push_str("Relations:\n\n");
                for r in relations {
                    let target = r["target"].as_str().unwrap_or("?");
                    let target_text = match ctx.model.block_of(target) {
                        Some(tb) if tb.domain != b.domain => format!(
                            "**{}** of [{}]({})",
                            text(short(target)),
                            text(&tb.name),
                            rel(&id, &ctx.block_id(tb))
                        ),
                        _ => format!("**{}**", text(short(target))),
                    };
                    let _ = writeln!(
                        s,
                        "- {} {} {target_text} ({}, by {})",
                        code(r["name"].as_str().unwrap_or("?")),
                        text(r["kind"].as_str().unwrap_or("references")),
                        text(r["cardinality"].as_str().unwrap_or("one")),
                        code(r["via"].as_str().unwrap_or("?"))
                    );
                }
                s.push('\n');
            }
            let invariants: Vec<String> = arr(&e["invariants"])
                .iter()
                .map(|i| code(&predicate(i)))
                .collect();
            if !invariants.is_empty() {
                let _ = writeln!(s, "Always holds: {}.\n", invariants.join(", "));
            }
            lifecycle(&mut p, &mut s, e, n + 1);
        }
    }

    if !commands.is_empty() {
        s.push_str("## Commands\n\n");
        for c in &commands {
            let qname = c["name"].as_str().unwrap_or("?");
            let _ = writeln!(s, "### {}\n", text(short(qname)));
            let display = c["naming"]["display"].as_str();
            let sum = c["naming"]["summary"].as_str();
            match (display, sum) {
                (Some(d), Some(sum)) => {
                    let _ = writeln!(s, "**{}.** {}\n", text(d), text(sum));
                }
                (Some(d), None) => {
                    let _ = writeln!(s, "**{}.**\n", text(d));
                }
                (None, Some(sum)) => {
                    let _ = writeln!(s, "{}\n", text(sum));
                }
                _ => {}
            }
            let callers: Vec<String> = actors
                .iter()
                .filter(|a| {
                    a["may"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .any(|m| m.as_str() == Some(qname))
                })
                .map(|a| {
                    text(
                        a["naming"]["display"]
                            .as_str()
                            .unwrap_or(short(a["name"].as_str().unwrap_or("?"))),
                    )
                })
                .collect();
            if !callers.is_empty() {
                let _ = writeln!(s, "Called by: {}.\n", callers.join(", "));
            }
            let input = arr(&c["input"]);
            if input.is_empty() {
                s.push_str("No input.\n\n");
            } else {
                field_rows(&mut s, &input);
            }
            let outcomes = arr(&c["outcomes"]);
            let subject = outcomes
                .iter()
                .find_map(|o| o["subject"]["entity"].as_str())
                .or_else(|| entities.first().and_then(|e| e["name"].as_str()));
            s.push_str("| outcome | when | result |\n|---|---|---|\n");
            for o in &outcomes {
                let oname = o["name"].as_str().unwrap_or("?");
                let when = condition(&o["condition"], subject);
                if let Some(err) = o["error"].as_str() {
                    let why = ir["errors"][err]["summary"].as_str().unwrap_or("");
                    let _ = writeln!(
                        s,
                        "| {} (refused) | {when} | {}{} |",
                        code(oname),
                        code(short(err)),
                        if why.is_empty() {
                            String::new()
                        } else {
                            format!(": {}", text(why))
                        }
                    );
                } else {
                    let _ = writeln!(s, "| {} | {when} | {} |", code(oname), result(o, ir));
                }
            }
            s.push('\n');
        }
    }

    if !events.is_empty() {
        s.push_str("## Events\n\n| event | fields | emitted by |\n|---|---|---|\n");
        for e in &events {
            let qname = e["name"].as_str().unwrap_or("?");
            let by: Vec<String> = commands
                .iter()
                .filter(|c| {
                    c["outcomes"].as_array().into_iter().flatten().any(|o| {
                        o["emits"]
                            .as_array()
                            .into_iter()
                            .flatten()
                            .any(|x| x.as_str() == Some(qname))
                    })
                })
                .map(|c| code(short(c["name"].as_str().unwrap_or("?"))))
                .collect();
            let mut what = code(short(qname));
            if let Some(sum) = e["naming"]["summary"].as_str() {
                let _ = write!(what, "<br/>{}", text(sum));
            }
            let _ = writeln!(
                s,
                "| {what} | {} | {} |",
                inline_fields(&arr(&e["fields"])),
                if by.is_empty() {
                    "–".into()
                } else {
                    by.join(", ")
                }
            );
        }
        s.push('\n');
    }

    if !views.is_empty() {
        s.push_str("## Views\n\n| view | rows | fields |\n|---|---|---|\n");
        for v in &views {
            let mut what = code(short(v["name"].as_str().unwrap_or("?")));
            if let Some(d) = v["naming"]["display"].as_str() {
                let _ = write!(what, "<br/>{}", text(d));
            }
            let mut rows = match v["source"].as_str() {
                Some(src) => format!("each **{}**", text(short(src))),
                None => "–".into(),
            };
            if !v["filter"].is_null() {
                let _ = write!(rows, " with {}", code(&predicate(&v["filter"])));
            }
            let params = arr(&v["params"]);
            if !params.is_empty() {
                let _ = write!(rows, "<br/>asked with {}", inline_fields(&params));
            }
            let _ = writeln!(
                s,
                "| {what} | {rows} | {} |",
                inline_fields(&arr(&v["fields"]))
            );
        }
        s.push('\n');
    }

    let types: Vec<(&String, &Value)> = ir["domains"][&b.domain]["types"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter(|t| !t.ends_with(".State"))
        .filter_map(|t| ir["types"].as_object().and_then(|o| o.get_key_value(t)))
        .collect();
    if !types.is_empty() {
        s.push_str("## Types\n\n| type | is | values or fields |\n|---|---|---|\n");
        for (name, t) in types {
            let body = &t["body"];
            let (is, detail) = match body["kind"].as_str() {
                Some("enum") => (
                    "one of".to_string(),
                    arr(&body["variants"])
                        .iter()
                        .filter_map(Value::as_str)
                        .map(code)
                        .collect::<Vec<_>>()
                        .join(", "),
                ),
                Some("newtype") => (
                    "an identifier or value".to_string(),
                    code(&type_ref(&body["of"])),
                ),
                Some("struct") => ("a record".to_string(), inline_fields(&arr(&body["fields"]))),
                Some("union") => (
                    "exactly one of".to_string(),
                    body["variants"]
                        .as_object()
                        .map(|m| {
                            m.iter()
                                .map(|(k, v)| format!("{} {}", code(k), text(&type_ref(v))))
                                .collect::<Vec<_>>()
                                .join(", ")
                        })
                        .unwrap_or_default(),
                ),
                Some(other) => (other.to_string(), String::new()),
                None => ("?".into(), String::new()),
            };
            let mut what = code(short(name));
            if let Some(sum) = t["naming"]["summary"].as_str() {
                let _ = write!(what, "<br/>{}", text(sum));
            }
            let _ = writeln!(s, "| {what} | {is} | {detail} |");
        }
        s.push('\n');
    }

    s.push_str("## KPIs\n\n");
    if kpis.is_empty() {
        s.push_str("No KPI of this block is in the catalogue.\n\n");
    } else {
        let _ = writeln!(
            s,
            "{} from the [KPI catalogue]({}).\n",
            plural(kpis.len(), "KPI", "KPIs"),
            rel(&id, "kpis")
        );
        kpi_sections(ctx, &mut s, &kpis, 3);
    }

    s.push_str("## Documents using it\n\n");
    let mut using: Vec<String> = Vec::new();
    for class in &ctx.drawn.classes {
        for page in &class.pages {
            let n = page
                .graph
                .parts
                .iter()
                .filter(|part| part.block.as_deref() == Some(b.name.as_str()))
                .count();
            if n > 0 {
                using.push(format!(
                    "- [{}]({}) ({}) · {}",
                    text(&page.name),
                    rel(&id, &page.page),
                    text(&class.title),
                    plural(n, "part", "parts")
                ));
            }
        }
    }
    if using.is_empty() {
        s.push_str("No document of the pack has a part of this block.\n");
    } else {
        for line in using {
            let _ = writeln!(s, "{line}");
        }
    }
    p.markdown = s;
    p
}

/// The file of a block: the entry of the system's specification list whose domain it is.
fn block_file(ctx: &Ctx, b: &Block) -> Option<String> {
    let blocks_path = ctx.pack.manifest.blocks.path.trim_end_matches('/');
    let inputs = ctx.pack.root.join(blocks_path).join("ess-inputs.yaml");
    let text = std::fs::read_to_string(inputs).ok()?;
    let inputs: Value = serde_yaml_ng::from_str(&text).ok()?;
    for entry in inputs["specification"]
        .as_array()?
        .iter()
        .filter_map(Value::as_str)
    {
        if metaspec_core::paths::relative_inside(entry).is_err() {
            continue;
        }
        let path = ctx.pack.root.join(blocks_path).join(entry);
        let Ok(content) = std::fs::read_to_string(&path) else {
            continue;
        };
        let domain = serde_yaml_ng::from_str::<Value>(&content)
            .ok()
            .and_then(|v| v["domain"].as_str().map(String::from));
        if domain.as_deref() == Some(b.domain.as_str()) {
            return Some(format!("{blocks_path}/{entry}"));
        }
    }
    None
}

/// The page of planned blocks.
pub fn planned_page(ctx: &Ctx) -> Page {
    let id = "blocks/planned";
    let mut p = Page::new(id, "planned");
    p.set("title", "Planned blocks");
    p.set("sidebar_label", "Planned blocks");
    p.set(
        "description",
        "Blocks the KPI catalogue measures before the blocks declare them.",
    );
    if let Some(k) = &ctx.pack.manifest.kpis {
        ctx.source(&mut p, &k.path);
    }
    let mut s = String::from(ctx.banner());
    if ctx.planned.is_empty() {
        s.push_str("None: every block the KPI catalogue names is declared in the blocks.\n");
        p.markdown = s;
        return p;
    }
    let _ = writeln!(
        s,
        "The [KPI catalogue]({}) measures {} the blocks do not declare yet. \
Each gets its own page, with its entities, commands, events and views, once it is declared.\n",
        rel(id, "kpis"),
        plural(ctx.planned.len(), "block", "blocks")
    );
    for (n, name) in ctx.planned.iter().enumerate() {
        let kpis: Vec<&Kpi> = ctx.kpis_of(name).collect();
        let _ = writeln!(s, "## {}\n", text(name));
        s.push_str(&p.component(
            "BlockHeader",
            &format!("header-{}", n + 1),
            header(ctx, name, None, json!({"kpis": kpis.len()})),
        ));
        kpi_sections(ctx, &mut s, &kpis, 3);
    }
    p.markdown = s;
    p
}
