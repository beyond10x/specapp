//! layering: dependency edges that point to a higher layer or form a cycle inside one.
//!
//! A stub, filled by `story:spec-quality-layering`. Until then each of its measures answers the
//! gap reading "not built yet".

use crate::quality::registry::{Input, Module};
use crate::quality::report::Outcome;

/// The measures of this file.
pub const MODULE: Module = Module {
    file: "spec/layering.rs",
    measure,
    built,
};

fn measure(_id: &str, _input: &Input<'_>) -> Outcome {
    Outcome::not_built()
}

fn built(_id: &str) -> bool {
    false
}
