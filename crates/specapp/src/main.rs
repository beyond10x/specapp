//! specapp: one docs site and one studio for every specification pack.

mod page;
mod render;

use clap::{Args, Parser, Subcommand};
use specapp_core::{ess_compile, load, pack, tool, words};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

#[derive(Parser)]
#[command(
    name = "specapp",
    version,
    about = "One docs site and one studio for every specification pack."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Work with a pack.
    Pack {
        #[command(subcommand)]
        command: PackCommand,
    },
    /// Trust the tool a pack's manifest names, so that specapp may run it.
    ///
    /// Records the digest of the manifest's `tool` section for the pack directory in
    /// `$XDG_STATE_HOME/specapp/trust.json` (default `~/.local/state/specapp`). A changed `tool`
    /// section is not trusted until it is trusted again. Exits 0 when recorded, 1 when the
    /// manifest names no tool or cannot be read.
    Trust {
        #[command(flatten)]
        pack: PackArgs,
    },
    /// Write one self-contained HTML page per document that is drawn (`parts-links/1`), with
    /// each view of its class and the findings of the pack's tool, and an index page.
    ///
    /// The tool's `check` runs only for a trusted pack (`specapp trust`); a run over the
    /// deadline is killed and the page says so. Exits 0 when the pages are written, 1 when the
    /// pack is refused or a page cannot be written, 2 on bad usage.
    Render {
        /// The pack directory.
        #[arg(long = "pack", value_name = "DIR")]
        dir: PathBuf,
        /// The manifest to read instead of DIR/specapp.yaml. It may lie outside the pack.
        #[arg(long, value_name = "FILE")]
        manifest: Option<PathBuf>,
        /// The directory the pages are written to.
        #[arg(long, value_name = "DIR")]
        out: PathBuf,
        /// Seconds one run of the tool may take before it is killed.
        #[arg(long, value_name = "SECONDS", default_value_t = 30.0)]
        deadline: f64,
    },
}

#[derive(Subcommand)]
enum PackCommand {
    /// Load a pack and print its counts and what specapp has to say about it.
    ///
    /// Compiles the blocks with `ess specify compile` and executes nothing of the pack. Exits 0
    /// when the pack loads (warnings included), 1 when it is refused, 2 on bad usage.
    Check {
        #[command(flatten)]
        pack: PackArgs,
        /// Print one JSON document, format specapp-pack-check/1.
        #[arg(long)]
        json: bool,
    },
    /// Print the words of a pack, one `<kind> <word>` per line: its block, layer, link, KPI and
    /// rule names.
    ///
    /// Exits 0 when the pack loads, 1 when it is refused.
    Words {
        #[command(flatten)]
        pack: PackArgs,
    },
}

#[derive(Args)]
struct PackArgs {
    /// The pack directory. Every path of the manifest is relative to it.
    #[arg(value_name = "DIR")]
    dir: PathBuf,
    /// The manifest to read instead of DIR/specapp.yaml. It may lie outside the pack.
    #[arg(long, value_name = "FILE")]
    manifest: Option<PathBuf>,
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Pack {
            command: PackCommand::Check { pack, json },
        } => {
            let (report, _) = load(&pack.dir, pack.manifest.as_deref(), &ess_compile);
            if json {
                match serde_json::to_string_pretty(&report) {
                    Ok(text) => println!("{text}"),
                    Err(error) => {
                        eprintln!("specapp: the report does not serialize: {error}");
                        return ExitCode::from(2);
                    }
                }
            } else {
                print!("{}", report.text());
            }
            if report.valid {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Command::Pack {
            command: PackCommand::Words { pack },
        } => {
            let (report, loaded) = load(&pack.dir, pack.manifest.as_deref(), &ess_compile);
            let Some(loaded) = loaded else {
                eprint!("{}", report.text());
                return ExitCode::from(1);
            };
            for (kind, word) in words::words(&loaded) {
                println!("{} {word}", kind.as_str());
            }
            ExitCode::SUCCESS
        }
        Command::Trust { pack } => trust(&pack),
        Command::Render {
            dir,
            manifest,
            out,
            deadline,
        } => {
            if !(deadline.is_finite() && deadline > 0.0) {
                eprintln!("specapp: --deadline takes a number of seconds above 0");
                return ExitCode::from(2);
            }
            let (report, loaded) = load(&dir, manifest.as_deref(), &ess_compile);
            let Some(loaded) = loaded else {
                eprint!("{}", report.text());
                return ExitCode::from(1);
            };
            let state = tool::state_dir();
            let rendered =
                render::build(&loaded, state.as_deref(), Duration::from_secs_f64(deadline));
            match render::write(&loaded, &rendered, &out) {
                Ok(lines) => {
                    for line in &lines {
                        println!("{line}");
                    }
                    let documents: usize = rendered
                        .classes
                        .iter()
                        .filter(|c| c.shape == specapp_core::manifest::Shape::PartsLinks)
                        .map(|c| c.documents)
                        .sum();
                    println!(
                        "render: {} page(s) for {documents} drawn document(s), and index.html",
                        lines.len()
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
}

fn trust(args: &PackArgs) -> ExitCode {
    let path = pack::manifest_path(&args.dir, args.manifest.as_deref());
    let manifest = match pack::read_manifest(&path) {
        Ok(manifest) => manifest,
        Err(errors) => {
            for e in errors {
                eprintln!("specapp: {} {}: {}", e.code, e.subject, e.message);
            }
            return ExitCode::from(1);
        }
    };
    let Some(spec) = &manifest.tool else {
        eprintln!("specapp: the manifest names no tool; there is nothing to trust");
        return ExitCode::from(1);
    };
    let Ok(root) = args.dir.canonicalize() else {
        eprintln!("specapp: the pack directory does not exist");
        return ExitCode::from(1);
    };
    let Some(state) = tool::state_dir() else {
        eprintln!("specapp: no state directory: set XDG_STATE_HOME or HOME");
        return ExitCode::from(1);
    };
    match tool::trust(&state, &root, spec) {
        Ok(digest) => {
            println!(
                "trusted: {} runs `{}` ({digest})",
                manifest.pack,
                spec.run.join(" ")
            );
            ExitCode::SUCCESS
        }
        Err(why) => {
            eprintln!("specapp: {why}");
            ExitCode::from(1)
        }
    }
}
