//! specapp-docs: the generator of the docs site of a pack.
//!
//! `build` reads a loaded pack and writes, in memory, every file of `specapp docs build`:
//!
//! - `docs/<id>.mdx`: one page each, front matter and Markdown that draws its components with
//!   the data of its page;
//! - `pages/<id>.json`: the same page as data, format `specapp-page/1` (front matter,
//!   Markdown, the names of its components and their data);
//! - `site.json`: format `specapp-site/1`, the pack's identity, its counts, the sidebar, the
//!   list of pages and the presentation table every component reads;
//! - `palette.css`: specapp's palette as CSS variables per theme.
//!
//! The output is a function of the pack's files and its tool's answers: no clock, no path of the
//! machine, no hash order. This crate knows no pack.

pub mod blocks;
pub mod catalogue;
pub mod document;
pub mod documents;
pub mod kpis;
pub mod layermap;
pub mod md;
pub mod model;
pub mod site;

pub use site::{Built, Page, build, check, write};
