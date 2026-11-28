//! `metaspec quality ratchet`: every value of the given reports held against its baseline. A
//! stub, filled by `story:quality-ratchet`; until then it exits 2 with "not built yet".

use metaspec_core::quality::report::NOT_BUILT;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn run(_reports: &[PathBuf], _baselines: Option<&Path>) -> ExitCode {
    eprintln!("metaspec: quality ratchet: {NOT_BUILT}");
    ExitCode::from(2)
}
