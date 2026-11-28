//! The KPI catalogue with neutral field names, format `specapp-kpis/1`: the pack's names of the
//! measure and the modes come from the manifest (`kpis.fields`), the pack's words for their
//! values stay data, and the pack's own columns are kept as extra columns.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use specapp_core::pack::Pack;

/// The format of the catalogue.
pub const FORMAT: &str = "specapp-kpis/1";

/// One row as a pack writes it: the neutral fields, and everything else.
#[derive(Debug, Clone, Default, Deserialize)]
struct Written {
    #[serde(default)]
    id: Option<Value>,
    #[serde(default)]
    block: Option<Value>,
    #[serde(default)]
    unit: Option<Value>,
    #[serde(default)]
    summary: Option<Value>,
    #[serde(default)]
    how: Option<Value>,
    #[serde(default)]
    direction: Option<Value>,
    #[serde(default)]
    gate: Option<Value>,
    #[serde(default)]
    reads: Option<Value>,
    #[serde(default)]
    status: Option<Value>,
    #[serde(flatten)]
    rest: Map<String, Value>,
}

/// A mode of a KPI and its direction there.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mode {
    pub mode: String,
    pub direction: String,
}

/// One KPI with neutral field names.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kpi {
    pub id: String,
    pub block: String,
    /// The value of the pack's measure field (`kpis.fields.measure`).
    pub measure: String,
    pub unit: String,
    pub summary: String,
    pub how: String,
    pub direction: String,
    /// The pack's modes field (`kpis.fields.modes`), in the order written.
    pub modes: Vec<Mode>,
    pub gate: bool,
    pub reads: Vec<String>,
    pub status: String,
    /// The pack's own columns, as text.
    pub extra: Map<String, Value>,
}

/// One scalar or list as text.
pub fn scalar(v: Option<&Value>) -> String {
    match v {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(s)) => s.trim().to_string(),
        Some(Value::Array(xs)) => xs
            .iter()
            .map(|x| scalar(Some(x)))
            .collect::<Vec<_>>()
            .join(", "),
        Some(other) => other.to_string(),
    }
}

/// The rows of a loaded pack's catalogue with neutral field names. A row without an id is left
/// out; `specapp pack check` warns about it.
pub fn read(pack: &Pack) -> Vec<Kpi> {
    let Some(spec) = &pack.manifest.kpis else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for row in &pack.kpis {
        let Ok(mut w) = serde_json::from_value::<Written>(row.clone()) else {
            continue;
        };
        let id = scalar(w.id.as_ref());
        if id.is_empty() {
            continue;
        }
        let measure = scalar(w.rest.remove(&spec.fields.measure).as_ref());
        let modes = match w.rest.remove(&spec.fields.modes) {
            Some(Value::Object(m)) => m
                .iter()
                .map(|(k, v)| Mode {
                    mode: k.clone(),
                    direction: scalar(Some(v)),
                })
                .collect(),
            _ => Vec::new(),
        };
        let reads = match &w.reads {
            Some(Value::Array(xs)) => xs.iter().map(|x| scalar(Some(x))).collect(),
            Some(Value::String(s)) => vec![s.clone()],
            _ => Vec::new(),
        };
        let extra: Map<String, Value> = w
            .rest
            .iter()
            .map(|(k, v)| (k.clone(), Value::String(scalar(Some(v)))))
            .collect();
        out.push(Kpi {
            block: scalar(w.block.as_ref()),
            unit: scalar(w.unit.as_ref()),
            summary: scalar(w.summary.as_ref()),
            how: scalar(w.how.as_ref()),
            direction: scalar(w.direction.as_ref()),
            gate: matches!(w.gate, Some(Value::Bool(true))),
            status: scalar(w.status.as_ref()),
            id,
            measure,
            modes,
            reads,
            extra,
        });
    }
    out
}
