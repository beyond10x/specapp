//! metaspec-docs: the generator of the one docs site of every pack it is given.
//!
//! `build` reads loaded packs and writes, in memory, every file of `metaspec docs build`. Per
//! pack, under `<pack>/` (routed below `/<pack>`):
//!
//! - `docs/<id>.mdx`: one page each, front matter and Markdown that draws its components with
//!   the data of its page;
//! - `pages/<id>.json`: the same page as data, format `metaspec-page/1` (front matter,
//!   Markdown, the names of its components and their data);
//! - `site.json`: format `metaspec-site/1`, the pack's identity, its counts, the sidebar, the
//!   list of pages and the presentation table every component reads.
//!
//! For the site:
//!
//! - `index.json`: format `metaspec-index/1`, the site's title and per pack its identity, route,
//!   counts and the check of its documents;
//! - `_index/index.mdx`: the page `/`, the packs in a table (a directory no pack can be named);
//! - `palette.css`: metaspec's palette as CSS variables per theme.
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
