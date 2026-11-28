//! metaspec: one docs site and one studio for every specification pack.

mod docs;
mod page;
mod render;

use clap::{Args, Parser, Subcommand};
use metaspec_core::{Issue, Manifest, Report, ess_compile, known, load_with, pack, tool, words};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(
    name = "metaspec",
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
    /// Trust the tool a pack's manifest names, so that metaspec may run it.
    ///
    /// Records the digest of the manifest's `tool` section for the pack directory in
    /// `$XDG_STATE_HOME/metaspec/trust.json` (default `~/.local/state/metaspec`). A changed `tool`
    /// section is not trusted until it is trusted again. Exits 0 when recorded, 1 when the
    /// manifest names no tool or cannot be read, 2 on bad usage.
    Trust {
        #[command(flatten)]
        pack: PackArgs,
    },
    /// Write one self-contained HTML page per document that is drawn (`parts-links/1`), with
    /// each view of its class and the findings of the pack's tool, and an index page.
    ///
    /// The tool's check runs only for a trusted pack (`metaspec trust`); a run over the
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
    /// Write the generated pages of one docs site over one or more packs.
    ///
    /// One pack: `[NAME | --manifest FILE] --pack DIR`. Several: `--pack NAME=DIR` or
    /// `--pack FILE=DIR` once per pack, where FILE is a manifest (a path with a `/`, or ending
    /// in `.yaml` or `.yml`) and NAME a pack metaspec knows.
    ///
    /// Per pack under OUT/<pack>/ (routed below /<pack>): one MDX page and its data
    /// (metaspec-page/1) per page and site.json (metaspec-site/1: the sidebar, the pages, the
    /// presentation table). For the site: index.json (metaspec-index/1: per pack its counts and
    /// the check of its documents), _index/index.mdx (the page /, the packs in a table) and
    /// palette.css.
    ///
    /// Pages of a pack: the pack, the layer map, one per block, planned blocks, the KPI
    /// catalogue, per document class an index and one per document, the rules. The tool's
    /// `check` runs only for a trusted pack (`metaspec trust`). OUT is metaspec's: a file there
    /// that the build does not write is removed, and a directory that holds other files is
    /// refused. With --check nothing is written: exit 1 lists every file that differs, is
    /// missing or is not owned. Exit 0 when written or equal, 1 when a pack is refused or a
    /// file differs, 2 on bad usage.
    Build {
        /// The name of a pack metaspec knows, when the site holds one pack.
        #[arg(value_name = "NAME", conflicts_with = "manifest")]
        name: Option<String>,
        /// The manifest of the one pack of the site, when metaspec does not know it.
        #[arg(long, value_name = "FILE")]
        manifest: Option<PathBuf>,
        /// The pack directory, after NAME or --manifest; else NAME=DIR or FILE=DIR, once per
        /// pack of the site.
        #[arg(
            long = "pack",
            value_name = "DIR | NAME=DIR | FILE=DIR",
            required = true
        )]
        packs: Vec<String>,
        /// The manifest the one pack's manifest `extends`, when metaspec does not know it.
        #[arg(long, value_name = "FILE")]
        extends: Option<PathBuf>,
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
    /// Load a pack and print its counts and what metaspec has to say about it.
    ///
    /// Compiles the blocks with `ess specify compile` and executes nothing of the pack. Exits 0
    /// when the pack loads (warnings included), 1 when it is refused, 2 on bad usage.
    Check {
        #[command(flatten)]
        pack: PackArgs,
        /// Print one JSON document, format metaspec-pack-check/1.
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
    /// Print the names of the packs metaspec knows, one per line: the manifests under its
    /// `packs/`.
    Known,
}

/// Which pack: its manifest (metaspec's, by the name of a pack metaspec knows or by a path) and
/// its directory. A pack directory holds nothing of metaspec.
#[derive(Args)]
struct PackArgs {
    /// The name of a pack metaspec knows; its manifest is metaspec's `packs/<NAME>.yaml`.
    #[arg(
        value_name = "NAME",
        conflicts_with = "manifest",
        required_unless_present = "manifest"
    )]
    name: Option<String>,
    /// The manifest of a pack metaspec does not know, such as a private pack. It lies outside
    /// the pack.
    #[arg(long, value_name = "FILE")]
    manifest: Option<PathBuf>,
    /// The pack directory. Every path of the manifest is relative to it.
    #[arg(long = "pack", value_name = "DIR")]
    dir: PathBuf,
    /// The manifest of the pack this pack's manifest `extends`, when metaspec does not know it.
    #[arg(long, value_name = "FILE")]
    extends: Option<PathBuf>,
}

/// Why a pack was not read.
enum NotRead {
    /// Bad usage: a name metaspec does not know.
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
                        "metaspec knows no pack {name}; it knows: {}. A pack it does not know is read with --manifest <file>",
                        known::names().join(", ")
                    )));
                }
            },
            (None, None) => return Err(NotRead::Usage(String::from("a pack name or --manifest"))),
        };
        read.map_err(NotRead::Refused)
    }

    /// Loads the pack: the report, with what metaspec says about `extends`, and the pack when it
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

/// The packs of one docs site: the one pack of `[NAME | --manifest FILE] --pack DIR`, or one per
/// `--pack NAME=DIR` and `--pack FILE=DIR`.
fn site_packs(
    name: Option<String>,
    manifest: Option<PathBuf>,
    packs: Vec<String>,
    extends: Option<PathBuf>,
) -> Result<Vec<PackArgs>, String> {
    if name.is_some() || manifest.is_some() {
        let [dir] = <[String; 1]>::try_from(packs).map_err(|_| {
            String::from(
                "after a pack name or --manifest, --pack takes the one pack's directory; for several packs give --pack NAME=DIR or --pack FILE=DIR for each",
            )
        })?;
        return Ok(vec![PackArgs {
            name,
            manifest,
            dir: PathBuf::from(dir),
            extends,
        }]);
    }
    if extends.is_some() {
        return Err(String::from(
            "--extends is for one pack, named by NAME or --manifest",
        ));
    }
    packs
        .iter()
        .map(|spec| {
            let Some((which, dir)) = spec.split_once('=') else {
                return Err(format!(
                    "--pack {spec}: give NAME=DIR or FILE=DIR, or name the one pack of the site by NAME or --manifest"
                ));
            };
            if which.is_empty() || dir.is_empty() {
                return Err(format!("--pack {spec}: give NAME=DIR or FILE=DIR"));
            }
            let is_file = which.contains('/') || which.ends_with(".yaml") || which.ends_with(".yml");
            Ok(PackArgs {
                name: (!is_file).then(|| which.to_string()),
                manifest: is_file.then(|| PathBuf::from(which)),
                dir: PathBuf::from(dir),
                extends: None,
            })
        })
        .collect()
}

fn usage(why: NotRead) -> ExitCode {
    match why {
        NotRead::Usage(message) => eprintln!("metaspec: {message}"),
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
                        eprintln!("metaspec: the report does not serialize: {error}");
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
                    name,
                    manifest,
                    packs,
                    extends,
                    out,
                    check,
                    deadline,
                },
        } => {
            if !(deadline.is_finite() && deadline > 0.0) {
                eprintln!("metaspec: --deadline takes a number of seconds above 0");
                return ExitCode::from(2);
            }
            let started = Instant::now();
            let args = match site_packs(name, manifest, packs, extends) {
                Ok(args) => args,
                Err(why) => return usage(NotRead::Usage(why)),
            };
            let mut loaded = Vec::new();
            for pack in &args {
                let (report, one) = match pack.load() {
                    Ok(one) => one,
                    Err(why) => return usage(why),
                };
                let Some(one) = one else {
                    eprint!("{}", report.text());
                    return ExitCode::from(1);
                };
                loaded.push(one);
            }
            docs::build(&loaded, &out, check, deadline, started)
        }
        Command::Render {
            pack,
            out,
            deadline,
        } => {
            if !(deadline.is_finite() && deadline > 0.0) {
                eprintln!("metaspec: --deadline takes a number of seconds above 0");
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
                        .filter(|c| c.shape == metaspec_core::manifest::Shape::PartsLinks)
                        .map(|c| c.documents)
                        .sum();
                    println!(
                        "render: {} page(s) for {documents} drawn document(s), and index.html",
                        lines.len()
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
}

fn trust(args: &PackArgs) -> ExitCode {
    let manifest = match args.manifest() {
        Ok(manifest) => manifest,
        Err(NotRead::Refused(errors)) => {
            for e in errors {
                eprintln!("metaspec: {} {}: {}", e.code, e.subject, e.message);
            }
            return ExitCode::from(1);
        }
        Err(usage_error) => return usage(usage_error),
    };
    let Some(spec) = &manifest.tool else {
        eprintln!("metaspec: the manifest names no tool; there is nothing to trust");
        return ExitCode::from(1);
    };
    let Ok(root) = args.dir.canonicalize() else {
        eprintln!("metaspec: the pack directory does not exist");
        return ExitCode::from(1);
    };
    let Some(state) = tool::state_dir() else {
        eprintln!("metaspec: no state directory: set XDG_STATE_HOME or HOME");
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
            eprintln!("metaspec: {why}");
            ExitCode::from(1)
        }
    }
}
