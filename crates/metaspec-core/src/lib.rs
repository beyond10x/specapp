//! metaspec-core: the manifest of a specification pack (`metaspec-pack/1`), loading a pack, and
//! the report of `metaspec pack check` (`metaspec-pack-check/1`).
//!
//! Its code names no pack. What a pack calls its things comes from its manifest (metaspec's
//! `packs/`, or a path given at run time) and its files.

pub mod adapter;
pub mod graph;
pub mod known;
pub mod manifest;
pub mod pack;
pub mod paths;
pub mod quality;
pub mod report;
pub mod tool;
pub mod words;

pub use manifest::Manifest;
pub use pack::{Pack, ess_compile, load, load_with};
pub use report::{Issue, Report};
