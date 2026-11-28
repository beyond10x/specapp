//! specapp: one docs site and one studio for every specification pack.

mod docs;
mod page;
mod render;

use clap::{Args, Parser, Subcommand};
use specapp_core::{Issue, Manifest, Report, ess_compile, known, load_with, pack, tool, words};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{Duration, Instant};

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
    /// manifest names no tool or cannot be read, 2 on bad usage.
    Trust {
        #[command(flatten)]
        pack: PackArgs,
    },
    /// Write one self-contained HTML page per document that is drawn (`parts-links/1`), with
    /// each view of its class and the findings of the pack's tool, and an index page.
    ///
    /// The tool's check runs only for a trusted pack (`specapp trust`); a run over the
    /// deadline is killed and the page says so. Exits 0 when the pages are written, 1 when the
    /// pack is refused or a page cannot be written, 2 on bad usage.
    Render {
        #[command(flatten)]
        pack: PackArgs,
        /// The directory the pages are written to.
        #[arg(long, value_name = "DIR")]
        out: PathBuf,
        /// Seconds one run of the tool may take before it is killed.
        #[arg(long, value_name = "SECONDS", default_value_t = 30.0)]
        deadline: f64,
    },
    /// The docs site of a pack.
    Docs {
        #[command(subcommand)]
        command: DocsCommand,
    },
}

#[derive(Subcommand)]
enum DocsCommand {
    /// Write the generated pages of a pack's docs site: one MDX page and its data
    /// (specapp-page/1) per page, site.json (specapp-site/1: the sidebar, the pages, the
    /// presentation table) and palette.css.
    ///
    /// Pages: the pack, the layer map, one per block, planned blocks, the KPI catalogue, per
    /// document class an index and one per document, the rules. The tool's `check` runs only
    /// for a trusted pack (`specapp trust`). OUT is specapp's: a file there that the build does
    /// not write is removed, and a directory that holds other files is refused. With --check
    /// nothing is written: exit 1 lists every file that differs, is missing or is not owned.
    /// Exit 0 when written or equal, 1 when the pack is refused or a file differs, 2 on bad
    /// usage.
    Build {
        #[command(flatten)]
        pack: PackArgs,
        /// The output directory.
        #[arg(long, value_name = "DIR")]
        out: PathBuf,
        /// Compare with OUT byte for byte instead of writing.
        #[arg(long)]
        check: bool,
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
    /// Exits 0 when the pack loads, 1 when it is refused, 2 on bad usage.
    Words {
        #[command(flatten)]
        pack: PackArgs,
    },
    /// Print the names of the packs specapp knows, one per line: the manifests under its
    /// `packs/`.
    Known,
}

/// Which pack: its manifest (specapp's, by the name of a pack specapp knows or by a path) and
/// its directory. A pack directory holds nothing of specapp.
#[derive(Args)]
struct PackArgs {
    /// The name of a pack specapp knows; its manifest is specapp's `packs/<NAME>.yaml`.
    #[arg(
        value_name = "NAME",
        conflicts_with = "manifest",
        required_unless_present = "manifest"
    )]
    name: Option<String>,
    /// The manifest of a pack specapp does not know, such as a private pack. It lies outside
    /// the pack.
    #[arg(long, value_name = "FILE")]
    manifest: Option<PathBuf>,
    /// The pack directory. Every path of the manifest is relative to it.
    #[arg(long = "pack", value_name = "DIR")]
    dir: PathBuf,
    /// The manifest of the pack this pack's manifest `extends`, when specapp does not know it.
    #[arg(long, value_name = "FILE")]
    extends: Option<PathBuf>,
}

/// Why a pack was not read.
enum NotRead {
    /// Bad usage: a name specapp does not know.
    Usage(String),
    /// The manifest is refused.
    Refused(Vec<Issue>),
}

impl PackArgs {
    /// The manifest the arguments name.
    fn manifest(&self) -> Result<Manifest, NotRead> {
        let read = match (&self.name, &self.manifest) {
            (_, Some(path)) => pack::read_manifest(path),
            (Some(name), None) => match pack::known_manifest(name) {
                Some(read) => read,
                None => {
                    return Err(NotRead::Usage(format!(
                        "specapp knows no pack {name}; it knows: {}. A pack it does not know is read with --manifest <file>",
                        known::names().join(", ")
                    )));
                }
            },
            (None, None) => return Err(NotRead::Usage(String::from("a pack name or --manifest"))),
        };
        read.map_err(NotRead::Refused)
    }

    /// Loads the pack: the report, with what specapp says about `extends`, and the pack when it
    /// loads.
    fn load(&self) -> Result<(Report, Option<pack::Pack>), NotRead> {
        let manifest = match self.manifest() {
            Ok(manifest) => manifest,
            Err(NotRead::Refused(errors)) => {
                return Ok((Report::refused(errors, Vec::new()), None));
            }
            Err(usage) => return Err(usage),
        };
        let extends = pack::extends_issues(&manifest, self.extends.as_deref());
        let (mut report, loaded) = load_with(&self.dir, manifest, &ess_compile);
        report.warnings.extend(extends);
        Ok((report, loaded))
    }
}

fn usage(why: NotRead) -> ExitCode {
    match why {
        NotRead::Usage(message) => eprintln!("specapp: {message}"),
        NotRead::Refused(errors) => eprint!("{}", Report::refused(errors, Vec::new()).text()),
    }
    ExitCode::from(2)
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Pack {
            command: PackCommand::Check { pack, json },
        } => {
            let (report, _) = match pack.load() {
                Ok(loaded) => loaded,
                Err(why) => return usage(why),
            };
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
            let (report, loaded) = match pack.load() {
                Ok(loaded) => loaded,
                Err(why) => return usage(why),
            };
            let Some(loaded) = loaded else {
                eprint!("{}", report.text());
                return ExitCode::from(1);
            };
            for (kind, word) in words::words(&loaded) {
                println!("{} {word}", kind.as_str());
            }
            ExitCode::SUCCESS
        }
        Command::Pack {
            command: PackCommand::Known,
        } => {
            for name in known::names() {
                println!("{name}");
            }
            ExitCode::SUCCESS
        }
        Command::Trust { pack } => trust(&pack),
        Command::Docs {
            command:
                DocsCommand::Build {
                    pack,
                    out,
                    check,
                    deadline,
                },
        } => {
            if !(deadline.is_finite() && deadline > 0.0) {
                eprintln!("specapp: --deadline takes a number of seconds above 0");
                return ExitCode::from(2);
            }
            let started = Instant::now();
            let (report, loaded) = match pack.load() {
                Ok(loaded) => loaded,
                Err(why) => return usage(why),
            };
            let Some(loaded) = loaded else {
                eprint!("{}", report.text());
                return ExitCode::from(1);
            };
            docs::build(&loaded, &out, check, deadline, started)
        }
        Command::Render {
            pack,
            out,
            deadline,
        } => {
            if !(deadline.is_finite() && deadline > 0.0) {
                eprintln!("specapp: --deadline takes a number of seconds above 0");
                return ExitCode::from(2);
            }
            let (report, loaded) = match pack.load() {
                Ok(loaded) => loaded,
                Err(why) => return usage(why),
            };
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
    let manifest = match args.manifest() {
        Ok(manifest) => manifest,
        Err(NotRead::Refused(errors)) => {
            for e in errors {
                eprintln!("specapp: {} {}: {}", e.code, e.subject, e.message);
            }
            return ExitCode::from(1);
        }
        Err(usage_error) => return usage(usage_error),
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
