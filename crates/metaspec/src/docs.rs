//! `metaspec docs build`: the generated pages of one docs site over one or more packs, or their
//! comparison with an output directory (`--check`).

use metaspec_core::pack::Pack;
use metaspec_core::tool;
use std::collections::BTreeMap;
use std::path::Path;
use std::process::ExitCode;
use std::time::{Duration, Instant};

fn kinds(kinds: &BTreeMap<&'static str, usize>) -> String {
    kinds
        .iter()
        .map(|(kind, n)| format!("{n} {kind}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Builds the site of the loaded packs and writes it to `out`, or with `check` compares it with
/// `out` byte for byte. Exit 0: written, or equal. Exit 1: a file cannot be written, or a file
/// differs. Exit 2: two packs of one name.
pub fn build(packs: &[Pack], out: &Path, check: bool, deadline: f64, started: Instant) -> ExitCode {
    let state = tool::state_dir();
    let refs: Vec<&Pack> = packs.iter().collect();
    let built =
        match metaspec_docs::build(&refs, state.as_deref(), Duration::from_secs_f64(deadline)) {
            Ok(built) => built,
            Err(why) => {
                eprintln!("metaspec: {why}");
                return ExitCode::from(2);
            }
        };
    for (name, pack_kinds) in &built.packs {
        println!(
            "docs: {name}: {} pages ({})",
            pack_kinds.values().sum::<usize>(),
            kinds(pack_kinds)
        );
    }
    let secs = started.elapsed().as_secs_f64();
    let summary = format!(
        "{} pages of {} pack(s) and the page /",
        built.pages(),
        built.packs.len()
    );
    if check {
        let diff = metaspec_docs::check(out, &built);
        if diff.is_empty() {
            println!(
                "docs: {summary}, {} files, equal to {} ({secs:.1} s)",
                built.files.len(),
                out.display()
            );
            ExitCode::SUCCESS
        } else {
            for line in &diff {
                println!("docs: {line}");
            }
            println!(
                "docs: {} of {} files differ from a fresh generation; run metaspec docs build without --check",
                diff.len(),
                built.files.len()
            );
            ExitCode::from(1)
        }
    } else {
        match metaspec_docs::write(out, &built) {
            Ok(changed) => {
                let removed: Vec<&String> = changed
                    .iter()
                    .filter(|l| l.starts_with("removed:"))
                    .collect();
                for line in &removed {
                    println!("docs: {line}");
                }
                println!(
                    "docs: {summary}, {} files in {}: {} written, {} removed ({secs:.1} s)",
                    built.files.len(),
                    out.display(),
                    changed.len() - removed.len(),
                    removed.len()
                );
                ExitCode::SUCCESS
            }
            Err(why) => {
                eprintln!("metaspec: {why}");
                ExitCode::from(1)
            }
        }
    }
}
