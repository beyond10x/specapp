//! refusal power and document health from the check of every document.
//!
//! A stub, filled by `story:spec-quality-refusal`. Until then each of its measures answers the
//! gap reading "not built yet".

use crate::quality::registry::{Input, Module};
use crate::quality::report::Outcome;

/// The measures of this file.
pub const MODULE: Module = Module {
    file: "spec/refusal.rs",
    measure,
    built,
};

fn measure(_id: &str, _input: &Input<'_>) -> Outcome {
    Outcome::not_built()
}

fn built(_id: &str) -> bool {
    false
}
