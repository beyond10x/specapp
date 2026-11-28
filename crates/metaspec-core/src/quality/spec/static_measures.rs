//! validity, ESS completeness, honesty and adoption level of the specification.
//!
//! A stub, filled by `story:spec-quality-static`. Until then each of its measures answers the
//! gap reading "not built yet".

use crate::quality::registry::{Input, Module};
use crate::quality::report::Outcome;

/// The measures of this file.
pub const MODULE: Module = Module {
    file: "spec/static_measures.rs",
    measure,
    built,
};

fn measure(_id: &str, _input: &Input<'_>) -> Outcome {
    Outcome::not_built()
}

fn built(_id: &str) -> bool {
    false
}
