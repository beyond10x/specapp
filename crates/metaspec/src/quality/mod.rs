//! `metaspec quality`: the verbs `kpis`, `spec` and `ratchet`, each in its own file. This file
//! holds their arguments and the dispatch.

mod kpis;
mod ratchet;
mod spec;

use crate::PackArgs;
use clap::Subcommand;
use metaspec_core::quality::catalogue;
use metaspec_core::quality::registry::{self, Input};
use metaspec_core::quality::report::{self, QualityReport, Question};
use metaspec_core::{Issue, tool};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

#[derive(Subcommand)]
pub enum QualityCommand {
    /// Measure the pack's KPI catalogue: every measure of the question `kpi_catalogue` of
    /// metaspec's catalogue, as a value with the names behind it or as a gap naming the input
    /// the pack lacks.
    ///
    /// Executes nothing of the pack but `ess specify compile`, except where a measure reads the
    /// check of a trusted pack's tool (`metaspec trust`). Prints the report, format
    /// metaspec-quality/1 with --json. Exits 0 when the report is printed (gaps included), 1 when
    /// the pack or metaspec's catalogue is refused, 2 on bad usage.
    Kpis {
        #[command(flatten)]
        pack: PackArgs,
        /// Print one JSON document, format metaspec-quality/1.
        #[arg(long)]
        json: bool,
        /// Seconds one run of the tool may take before it is killed.
        #[arg(long, value_name = "SECONDS", default_value_t = 30.0)]
        deadline: f64,
    },
    /// Measure the pack's specification: every measure of the question `specification` of
    /// metaspec's catalogue.
    Spec {
        #[command(flatten)]
        pack: PackArgs,
        /// Print one JSON document, format metaspec-quality/1.
        #[arg(long)]
        json: bool,
        /// Seconds one run of the tool may take before it is killed.
        #[arg(long, value_name = "SECONDS", default_value_t = 30.0)]
        deadline: f64,
    },
    /// Hold every value of one or more reports against its baseline per pack and measure.
    Ratchet {
        /// Reports `metaspec quality kpis --json` or `metaspec quality spec --json` wrote.
        #[arg(value_name = "REPORT", required = true)]
        reports: Vec<PathBuf>,
        /// The directory of the baselines, instead of the one the pack's visibility names.
        #[arg(long, value_name = "DIR")]
        baselines: Option<PathBuf>,
    },
}

/// Measures one question over a pack and prints the report: the text of
/// [`QualityReport::text`], or its JSON. The report is validated against its schema before it
/// is printed. Exit 0 when printed, 1 when the pack, metaspec's catalogue or the registry is
/// refused, 2 on bad usage.
fn measure(question: Question, pack: &PackArgs, json: bool, deadline: f64) -> ExitCode {
    if !(deadline.is_finite() && deadline > 0.0) {
        eprintln!("metaspec: --deadline takes a number of seconds above 0");
        return ExitCode::from(2);
    }
    let (report, loaded) = match pack.load() {
        Ok(loaded) => loaded,
        Err(why) => return crate::usage(why),
    };
    let Some(loaded) = loaded else {
        eprint!("{}", report.text());
        return ExitCode::from(1);
    };
    let refused = |issues: Vec<Issue>| {
        for e in issues {
            eprintln!("metaspec: {} {}: {}", e.code, e.subject, e.message);
        }
        ExitCode::from(1)
    };
    let catalogue = match catalogue::embedded() {
        Ok(catalogue) => catalogue,
        Err(issues) => return refused(issues),
    };
    let state = tool::state_dir();
    let input = Input::new(&loaded, state.as_deref(), Duration::from_secs_f64(deadline));
    let report: QualityReport = match registry::run(question, &catalogue, &input) {
        Ok(report) => report,
        Err(issues) => return refused(issues),
    };
    let value = match serde_json::to_value(&report) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("metaspec: the report does not serialize: {error}");
            return ExitCode::from(1);
        }
    };
    if let Err(issues) = report::validate(&value) {
        return refused(issues);
    }
    if json {
        match serde_json::to_string_pretty(&value) {
            Ok(text) => println!("{text}"),
            Err(error) => {
                eprintln!("metaspec: the report does not serialize: {error}");
                return ExitCode::from(1);
            }
        }
    } else {
        print!("{}", report.text());
    }
    ExitCode::SUCCESS
}

/// Runs one verb.
pub fn run(command: QualityCommand) -> ExitCode {
    match command {
        QualityCommand::Kpis {
            pack,
            json,
            deadline,
        } => kpis::run(&pack, json, deadline),
        QualityCommand::Spec {
            pack,
            json,
            deadline,
        } => spec::run(&pack, json, deadline),
        QualityCommand::Ratchet { reports, baselines } => {
            ratchet::run(&reports, baselines.as_deref())
        }
    }
}
