//! specapp: one docs site and one studio for every specification pack.

use clap::{Args, Parser, Subcommand};
use specapp_core::{ess_compile, load, words};
use std::path::PathBuf;
use std::process::ExitCode;

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
    let Command::Pack { command } = Cli::parse().command;
    match command {
        PackCommand::Check { pack, json } => {
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
        PackCommand::Words { pack } => {
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
    }
}
