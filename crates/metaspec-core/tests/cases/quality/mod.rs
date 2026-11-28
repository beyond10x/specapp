//! The quality measures of every pack: metaspec's own KPI catalogue of them, and each measure.
//! One module per story of the quality initiative, each filled by its story only; a test
//! module's filter is `quality::<module>`.

mod catalogue;
mod catalogue_adversary;
mod conformance;
mod drift;
mod grounding;
mod kpis;
mod kpis_adversary;
mod layering;
mod mutation;
mod ratchet;
mod refusal;
mod rules;
mod spec;
mod spec_adversary;
mod values;
