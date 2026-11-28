//! The measures that read only a pack's KPI catalogue and its block list:
//!
//! - `coverage.blocks`: blocks with at least one KPI, over blocks; behind: the blocks without
//!   one.
//! - `completeness.fields`: KPIs with a unit, a direction, a direction for every mode of the
//!   catalogue and a threshold where the gate column sets a gate, over KPIs; behind: the
//!   incomplete KPIs. The gate column as written: a number is a threshold; `true` is a gate
//!   without a threshold and counts as incomplete; `false` or absent is no gate; any other value
//!   is no threshold metaspec can read and counts as incomplete. The modes of the catalogue are
//!   the modes any row writes; a catalogue that writes none asks for none.
//! - `planned`: KPIs that name a block the pack does not have; behind: those KPIs.
//!
//! The modes field is the manifest's (`kpis.fields.modes`); `id`, `block`, `unit`, `direction`
//! and `gate` are the shared column names. A pack without a `kpis` section has a gap for each.
//!
//! A name behind a value is never blank: a KPI row without an id, or with an empty or blank
//! one, is named `#<index>` by its place; a block with a blank name by its domain.

use crate::pack::Block;
use crate::quality::registry::{Input, Module};
use crate::quality::report::{Behind, Outcome};
use serde_json::Value;
use std::collections::BTreeSet;

/// The measures of this file.
pub const MODULE: Module = Module {
    file: "kpis/catalogue_measures.rs",
    measure,
    built,
};

const COVERAGE_BLOCKS: &str = "coverage.blocks";
const COMPLETENESS_FIELDS: &str = "completeness.fields";
const PLANNED: &str = "planned";

fn built(id: &str) -> bool {
    [COVERAGE_BLOCKS, COMPLETENESS_FIELDS, PLANNED].contains(&id)
}

fn measure(id: &str, input: &Input<'_>) -> Outcome {
    let pack = input.pack;
    let Some(spec) = &pack.manifest.kpis else {
        return Outcome::gap(
            Some("L1"),
            "the manifest names no KPI catalogue (no section `kpis`)",
        );
    };
    let rows = &pack.kpis;
    let blocks: Vec<&str> = pack.blocks.iter().map(|b| b.name.as_str()).collect();
    match id {
        COVERAGE_BLOCKS => {
            let named: BTreeSet<&str> = rows.iter().filter_map(block_of).collect();
            let behind: Vec<Behind> = pack
                .blocks
                .iter()
                .filter(|b| !named.contains(b.name.as_str()))
                .map(|b| Behind::new("block", block_name(b)))
                .collect();
            Outcome::share(
                blocks.len() - behind.len(),
                blocks.len(),
                behind,
                Some("L0"),
                "the pack has no blocks",
            )
        }
        COMPLETENESS_FIELDS => {
            let modes = catalogue_modes(rows, &spec.fields.modes);
            let behind: Vec<Behind> = rows
                .iter()
                .enumerate()
                .filter(|(_, row)| !complete(row, &spec.fields.modes, &modes))
                .map(|(index, row)| Behind::new("kpi", id_of(row, index)))
                .collect();
            Outcome::share(
                rows.len() - behind.len(),
                rows.len(),
                behind,
                Some("L1"),
                "the KPI catalogue has no rows",
            )
        }
        PLANNED => {
            let behind: Vec<Behind> = rows
                .iter()
                .enumerate()
                .filter(|(_, row)| block_of(row).is_some_and(|b| !blocks.contains(&b)))
                .map(|(index, row)| Behind::new("kpi", id_of(row, index)))
                .collect();
            Outcome::count(behind.len(), behind)
        }
        _ => Outcome::not_built(),
    }
}

/// The block a row names, as the loader reads it.
fn block_of(row: &Value) -> Option<&str> {
    row.get("block").and_then(Value::as_str)
}

/// The id of a row as the pack writes it; `#<index>` for a row without one, or with one that is
/// empty or blank: a name behind a value is never blank.
fn id_of(row: &Value, index: usize) -> String {
    row.get("id")
        .and_then(Value::as_str)
        .filter(|id| !id.trim().is_empty())
        .map(String::from)
        .unwrap_or_else(|| format!("#{index}"))
}

/// The name of a block as metaspec shows it; its domain when the name is blank (a
/// `strip_prefix` that takes the whole domain).
fn block_name(block: &Block) -> &str {
    if block.name.trim().is_empty() {
        &block.domain
    } else {
        &block.name
    }
}

/// A text field that is set: a string that is not blank.
fn text_set(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_str)
        .is_some_and(|s| !s.trim().is_empty())
}

/// A row's directions per mode, from a map (as a catalogue writes it) or a list of
/// `{mode, direction}`; a mode with no direction text maps to `None`.
fn directions(value: Option<&Value>) -> Vec<(String, Option<&Value>)> {
    match value {
        Some(Value::Object(map)) => map.iter().map(|(k, v)| (k.clone(), Some(v))).collect(),
        Some(Value::Array(list)) => list
            .iter()
            .filter_map(|m| {
                let mode = m.get("mode").and_then(Value::as_str)?;
                Some((mode.to_string(), m.get("direction")))
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// Every mode any row of the catalogue writes.
fn catalogue_modes(rows: &[Value], field: &str) -> BTreeSet<String> {
    rows.iter()
        .flat_map(|row| directions(row.get(field)))
        .map(|(mode, _)| mode)
        .collect()
}

/// Whether a row has every field `completeness.fields` asks for.
fn complete(row: &Value, field: &str, modes: &BTreeSet<String>) -> bool {
    let directions = directions(row.get(field));
    let every_mode = modes.iter().all(|mode| {
        directions
            .iter()
            .any(|(m, direction)| m == mode && text_set(*direction))
    });
    let gate = match row.get("gate") {
        None | Some(Value::Null) | Some(Value::Bool(false)) => true,
        Some(Value::Number(_)) => true,
        Some(_) => false,
    };
    text_set(row.get("unit")) && text_set(row.get("direction")) && every_mode && gate
}
