//! `metaspec quality spec`: the specification question over one pack, every measure of
//! metaspec's catalogue whose question is `specification`, and the gaps of kind `verb` of the
//! adoption level (`metaspec_core::quality::spec::static_measures::report`).
//!
//! `ess` is the program on PATH, or the one `METASPEC_ESS` names; it runs in the pack's blocks
//! directory and is killed after 600 s, or the seconds `METASPEC_ESS_DEADLINE` names. Its answers
//! are cached in the user's cache directory by the digest of the pack's blocks
//! (`metaspec_core::quality::spec::static_measures::ess_answer`). Exit 0 when the report is
//! printed (gaps included), 1 when the pack, metaspec's catalogue or the registry is refused or
//! the report breaks its schema, 2 on bad usage.

use crate::PackArgs;
use metaspec_core::quality::catalogue;
use metaspec_core::quality::registry::Input;
use metaspec_core::quality::report::{self, QualityReport};
use metaspec_core::quality::spec::static_measures::{
    self,
    ess_answer::{self, Ess},
};
use metaspec_core::{Issue, tool};
use std::process::ExitCode;
use std::time::Duration;

pub fn run(pack: &PackArgs, json: bool, deadline: f64) -> ExitCode {
    if !(deadline.is_finite() && deadline > 0.0) {
        eprintln!("metaspec: --deadline takes a number of seconds above 0");
        return ExitCode::from(2);
    }
    let (checked, loaded) = match pack.load() {
        Ok(loaded) => loaded,
        Err(why) => return crate::usage(why),
    };
    let Some(loaded) = loaded else {
        eprint!("{}", checked.text());
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
    let measured = ess_answer::scoped(Ess::from_env(), || {
        static_measures::report(&catalogue, &input)
    });
    let report: QualityReport = match measured {
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
        for gap in report.gaps.iter().filter(|g| g.kind == "verb") {
            println!(
                "gap: verb {} needs {}: {}",
                gap.feature,
                gap.needs.as_deref().unwrap_or("-"),
                gap.summary
            );
        }
    }
    ExitCode::SUCCESS
}
