//! mutation score, overall and per mutant class, once per digest of the blocks.
//!
//! A stub, filled by `story:spec-quality-mutation`. Until then each of its measures answers the
//! gap reading "not built yet".

use crate::quality::registry::{Input, Module};
use crate::quality::report::Outcome;

/// The measures of this file.
pub const MODULE: Module = Module {
    file: "spec/mutation.rs",
    measure,
    built,
};

fn measure(_id: &str, _input: &Input<'_>) -> Outcome {
    Outcome::not_built()
}

fn built(_id: &str) -> bool {
    false
}
