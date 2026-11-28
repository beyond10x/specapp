//! `metaspec quality spec`: the specification question over one pack. A stub, filled by
//! `story:spec-quality-static`; until then it exits 2 with "not built yet".

use crate::PackArgs;
use metaspec_core::quality::report::NOT_BUILT;
use std::process::ExitCode;

pub fn run(_pack: &PackArgs, _json: bool, _deadline: f64) -> ExitCode {
    eprintln!("metaspec: quality spec: {NOT_BUILT}");
    ExitCode::from(2)
}
