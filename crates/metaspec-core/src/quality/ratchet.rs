//! The ratchet: each value of a report held against a baseline per pack and measure; the first
//! run sets the baseline, a better or equal value is taken into it, a worse one fails. A stub:
//! filled by `story:quality-ratchet`; until then it answers "not built yet".

use super::report::{NOT_BUILT, QualityReport};

/// Holds the values of a report against their baselines.
pub fn hold(_report: &QualityReport) -> Result<(), String> {
    Err(NOT_BUILT.to_string())
}
