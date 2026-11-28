//! The KPI catalogue page: every KPI in one table that filters by block, measure, status and
//! gate, and a list by block.

use crate::kpis::FORMAT;
use crate::md::{code, plural, text};
use crate::site::{Ctx, Page, anchor, rel};
use serde_json::{Value, json};
use std::fmt::Write as _;

pub fn page(ctx: &Ctx) -> Page {
    let id = "kpis";
    let mut p = Page::new(id, "kpis");
    let fields = ctx.pack.manifest.kpis.as_ref().map(|k| k.fields.clone());
    let (measure, modes) = fields
        .map(|f| (f.measure, f.modes))
        .unwrap_or_else(|| (String::from("measure"), String::from("modes")));
    p.set("title", "KPI catalogue");
    p.set("sidebar_label", "KPI catalogue");
    p.set(
        "description",
        format!(
            "Every KPI of every building block of {}: what it measures, which way is better, whether it is a gate, and its status.",
            ctx.pack.manifest.title
        ),
    );
    if let Some(k) = &ctx.pack.manifest.kpis {
        ctx.source_in(&mut p, ctx.pack.kpis_root(), &k.path);
    }
    p.flag("hide_table_of_contents", true);
    let mut s = String::from(ctx.banner());
    let n = ctx.kpis.len();
    let gates = ctx.kpis.iter().filter(|k| k.gate).count();
    let blocks = ctx
        .pack
        .blocks
        .iter()
        .filter(|b| ctx.kpis_of(&b.name).next().is_some())
        .count();
    let _ = writeln!(
        s,
        "{} over {}, {} of them planned; {gates} of the KPIs are gates. \
The column {} is the pack's measure of a KPI, and {} its direction per mode. \
Filter by block, {}, status or gate, search by name, and sort by any column; the address keeps the filters.\n",
        plural(n, "KPI", "KPIs"),
        plural(blocks + ctx.planned.len(), "block", "blocks"),
        ctx.planned.len(),
        code(&measure),
        code(&modes),
        text(&measure)
    );
    let rows: Vec<Value> = ctx
        .kpis
        .iter()
        .map(|k| {
            let b = ctx.model.by_name(&k.block);
            let mut row = serde_json::to_value(k).unwrap_or(Value::Null);
            if let Value::Object(o) = &mut row {
                o.insert(
                    String::from("layer"),
                    json!(b.and_then(|b| b.layer.as_ref().map(|l| l.number))),
                );
                o.insert(String::from("planned"), json!(b.is_none()));
                o.insert(
                    String::from("route"),
                    json!(ctx.block_route(&k.block).map(|r| if b.is_some() {
                        format!("{r}#kpis")
                    } else {
                        r
                    })),
                );
            }
            row
        })
        .collect();
    s.push_str(&p.component(
        "KpiCatalogue",
        "catalogue",
        json!({
            "format": FORMAT,
            "measure": measure,
            "modes": modes,
            "kpis": rows,
        }),
    ));
    s.push_str("## By block\n\n");
    if n == 0 {
        s.push_str("The catalogue holds no KPI.\n");
    }
    for b in &ctx.pack.blocks {
        let ids: Vec<String> = ctx.kpis_of(&b.name).map(|k| code(&k.id)).collect();
        if ids.is_empty() {
            continue;
        }
        let _ = writeln!(
            s,
            "- [{}]({}#kpis): {}",
            text(&b.name),
            rel(id, &ctx.block_id(b)),
            ids.join(", ")
        );
    }
    for name in &ctx.planned {
        let ids: Vec<String> = ctx.kpis_of(name).map(|k| code(&k.id)).collect();
        let _ = writeln!(
            s,
            "- [{}]({}#{}) (planned): {}",
            text(name),
            rel(id, "blocks/planned"),
            anchor(name),
            ids.join(", ")
        );
    }
    p.markdown = s;
    p
}
