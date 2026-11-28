//! `metaspec quality kpis`: the KPI question over one pack, every measure of metaspec's
//! catalogue whose question is `kpi_catalogue`.

use crate::PackArgs;
use metaspec_core::quality::report::Question;
use std::process::ExitCode;

pub fn run(pack: &PackArgs, json: bool, deadline: f64) -> ExitCode {
    super::measure(Question::KpiCatalogue, pack, json, deadline)
}
