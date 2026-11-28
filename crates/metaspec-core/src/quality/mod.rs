//! The quality of every pack's KPI catalogue and of every specification, measured by metaspec.
//! The measures are data: rows of metaspec's own KPI catalogue (`kpis/catalogue.yaml`), read
//! by [`catalogue`]. [`registry`] maps each row to the function that measures it, in the module
//! file of the story that builds it ([`kpis`] for the KPI question, [`spec`] for the
//! specification question), and runs a question into a report of [`report`]'s format,
//! `metaspec-quality/1`. [`check_pass`] runs the pack's check over every document once for the
//! measures that read it; [`ratchet`] holds each value against a baseline.

pub mod catalogue;
pub mod check_pass;
pub mod kpis;
pub mod ratchet;
pub mod registry;
pub mod report;
pub mod spec;
