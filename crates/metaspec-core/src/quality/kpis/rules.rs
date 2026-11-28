//! coverage.rules and gate_fidelity: the rules the tool reports and the broken examples.
//!
//! A stub, filled by `story:kpi-quality-rules`. Until then each of its measures answers the
//! gap reading "not built yet".

use crate::quality::registry::{Input, Module};
use crate::quality::report::Outcome;

/// The measures of this file.
pub const MODULE: Module = Module {
    file: "kpis/rules.rs",
    measure,
    built,
};

fn measure(_id: &str, _input: &Input<'_>) -> Outcome {
    Outcome::not_built()
}

fn built(_id: &str) -> bool {
    false
}
