//! specapp-core: the manifest of a specification pack (`specapp-pack/1`), loading a pack, and
//! the report of `specapp pack check` (`specapp-pack-check/1`).
//!
//! This crate knows no pack. What a pack calls its things comes from its manifest and its
//! files at run time.

pub mod graph;
pub mod manifest;
pub mod pack;
pub mod paths;
pub mod report;
pub mod tool;
pub mod words;

pub use manifest::Manifest;
pub use pack::{Pack, ess_compile, load, load_with};
pub use report::{Issue, Report};
