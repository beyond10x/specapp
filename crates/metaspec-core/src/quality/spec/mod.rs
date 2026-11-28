//! The measures of the specification question: one module file per story that builds them. This
//! file only declares them; a story that needs a helper adds it as a child module of its own
//! file.

pub mod conformance;
pub mod drift;
pub mod layering;
pub mod mutation;
pub mod refusal;
pub mod static_measures;
