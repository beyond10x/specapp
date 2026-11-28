//! `specapp docs build`: the generated pages of a pack's docs site, or their comparison with an
//! output directory (`--check`).

use specapp_core::pack::Pack;
use specapp_core::tool;
use std::path::Path;
use std::process::ExitCode;
use std::time::{Duration, Instant};

/// Builds the docs of a loaded pack and writes them to `out`, or with `check` compares them
/// with `out` byte for byte. Exit 0: written, or equal. Exit 1: a file cannot be written, or a
/// file differs.
pub fn build(pack: &Pack, out: &Path, check: bool, deadline: f64, started: Instant) -> ExitCode {
    let state = tool::state_dir();
    let built = specapp_docs::build(pack, state.as_deref(), Duration::from_secs_f64(deadline));
    let kinds: Vec<String> = built
        .kinds
        .iter()
        .map(|(kind, n)| format!("{n} {kind}"))
        .collect();
    let secs = started.elapsed().as_secs_f64();
    if check {
        let diff = specapp_docs::check(out, &built);
        if diff.is_empty() {
            println!(
                "docs: {} pages ({}), {} files, equal to {} ({secs:.1} s)",
                built.pages(),
                kinds.join(", "),
                built.files.len(),
                out.display()
            );
            ExitCode::SUCCESS
        } else {
            for line in &diff {
                println!("docs: {line}");
            }
            println!(
                "docs: {} of {} files differ from a fresh generation; run specapp docs build without --check",
                diff.len(),
                built.files.len()
            );
            ExitCode::from(1)
        }
    } else {
        match specapp_docs::write(out, &built) {
            Ok(changed) => {
                let removed: Vec<&String> = changed
                    .iter()
                    .filter(|l| l.starts_with("removed:"))
                    .collect();
                for line in &removed {
                    println!("docs: {line}");
                }
                println!(
                    "docs: {} pages ({}), {} files in {}: {} written, {} removed ({secs:.1} s)",
                    built.pages(),
                    kinds.join(", "),
                    built.files.len(),
                    out.display(),
                    changed.len() - removed.len(),
                    removed.len()
                );
                ExitCode::SUCCESS
            }
            Err(why) => {
                eprintln!("specapp: {why}");
                ExitCode::from(1)
            }
        }
    }
}
