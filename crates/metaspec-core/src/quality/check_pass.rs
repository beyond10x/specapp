//! The check of every document of a pack, run once per run of a question for every measure that
//! reads it (`Input::check_pass`). A stub: filled by `story:kpi-quality-values`; until then the
//! pass answers "not built yet".

use super::registry::Input;
use super::report::NOT_BUILT;

/// What the check of every document answered. Its fields are added by the story that fills
/// this file.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckPass {}

/// Runs the check of every document of the input's pack.
pub fn run(_input: &Input<'_>) -> Result<CheckPass, String> {
    Err(NOT_BUILT.to_string())
}
