//! The answer of `ess` on a pack's blocks, cached by the digest of the blocks.
//!
//! An answer such as `ess specify validate --format json` runs conformance synthesis, which
//! takes as long as `ess verify conform synthesize`, so it is kept in the user's cache directory
//! (`$XDG_CACHE_HOME/metaspec/ess`, else `$HOME/.cache/metaspec/ess`), never in the metaspec
//! checkout: the blocks may be a private pack's. The key is the SHA-256 of what `ess --version`
//! prints, the command words and options, and the path and content of every file under the
//! manifest's `blocks.path`. Without a cache directory every run asks `ess`.
//!
//! The program is `ess` on PATH, or the one the environment variable `METASPEC_ESS` names (the
//! tests name the stub `fixtures/ess/ess`). `ess` runs with the pack's blocks directory as its
//! working directory and `--path .`, as ess asks: there a release that dispatches hands the run
//! to the release the blocks' `ess-inputs.yaml` pins, and `ess --version`, run in the same
//! directory, names the release that answers. Stdin is closed; after its deadline ([`DEADLINE`],
//! or the seconds of `METASPEC_ESS_DEADLINE`, at least [`DEADLINE_FLOOR`]) the run's process
//! group gets SIGTERM and, 2 s later, SIGKILL, and the run ends without waiting for a process that
//! left the group and still holds a pipe.
//! Within [`scoped`] the program, the cache and the deadline are given, and each answer is asked
//! once per scope.
//!
//! A run that gives no answer is an error naming what ess said on stderr (its first lines,
//! without `note:` lines when it said more).
//!
//! Later stories ask their own commands through [`answer`] and do not change this file.

use crate::pack::Pack;
use crate::paths::{resolved_inside, shown};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex, PoisonError, mpsc};
use std::time::{Duration, Instant};

/// The environment variable that names the `ess` program.
pub const PROGRAM_VAR: &str = "METASPEC_ESS";

/// How long one run of `ess` may take when nothing else is named: synthesis of a large
/// specification takes minutes.
pub const DEADLINE: Duration = Duration::from_secs(600);

/// The environment variable that names the deadline of one run of `ess`, in seconds.
pub const DEADLINE_VAR: &str = "METASPEC_ESS_DEADLINE";

/// The shortest deadline: a value of [`DEADLINE_VAR`] above 0 and below it is raised to it.
pub const DEADLINE_FLOOR: Duration = Duration::from_millis(1);

/// The deadline a value of [`DEADLINE_VAR`] names: a finite number of seconds above 0, at least
/// [`DEADLINE_FLOOR`], else [`DEADLINE`].
pub fn deadline_of(text: Option<&OsStr>) -> Duration {
    text.and_then(OsStr::to_str)
        .and_then(|t| t.trim().parse::<f64>().ok())
        .filter(|s| s.is_finite() && *s > 0.0)
        .and_then(|s| Duration::try_from_secs_f64(s).ok())
        .map(|d| d.max(DEADLINE_FLOOR))
        .unwrap_or(DEADLINE)
}

/// How long `ess --version` may take.
const VERSION_DEADLINE: Duration = Duration::from_secs(30);

/// How many lines of ess's stderr an error names.
const STDERR_LINES: usize = 5;

/// How long a stopped run has between SIGTERM and SIGKILL, and how long the pipes of an `ess`
/// that has exited may stay open (held by a process it left behind) before its group is stopped.
const GRACE: Duration = Duration::from_secs(2);

/// How long the pipes are read after SIGKILL; a process outside the group that still holds one
/// is not waited for.
const SETTLE: Duration = Duration::from_millis(200);

/// Which `ess` runs, where its answers are kept and how long one run may take.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ess {
    /// The program: a name looked up on PATH, or a path.
    pub program: PathBuf,
    /// The cache directory; `None` keeps nothing.
    pub cache: Option<PathBuf>,
    /// How long one run may take before it is killed.
    pub deadline: Duration,
}

impl Ess {
    /// The program `program` with the cache directory `cache`. A relative path with a slash is
    /// taken from the current directory.
    pub fn new(program: impl Into<PathBuf>, cache: Option<PathBuf>) -> Ess {
        let program: PathBuf = program.into();
        let program = if program.is_relative() && program.components().count() > 1 {
            std::env::current_dir()
                .map(|dir| dir.join(&program))
                .unwrap_or(program)
        } else {
            program
        };
        Ess {
            program,
            cache,
            deadline: DEADLINE,
        }
    }

    /// The same `ess` with another deadline for one run.
    pub fn with_deadline(self, deadline: Duration) -> Ess {
        Ess { deadline, ..self }
    }

    /// The program of `METASPEC_ESS`, else `ess`; the user's cache directory; the deadline of
    /// `METASPEC_ESS_DEADLINE`, else [`DEADLINE`].
    pub fn from_env() -> Ess {
        let program = std::env::var_os(PROGRAM_VAR)
            .filter(|p| !p.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("ess"));
        let xdg = std::env::var_os("XDG_CACHE_HOME");
        let home = std::env::var_os("HOME");
        let deadline = std::env::var_os(DEADLINE_VAR);
        Ess::new(program, cache_dir(xdg.as_deref(), home.as_deref()))
            .with_deadline(deadline_of(deadline.as_deref()))
    }
}

/// The cache directory of the answers: `<XDG_CACHE_HOME>/metaspec/ess` when that is an
/// absolute path, else `<HOME>/.cache/metaspec/ess`; `None` without either.
pub fn cache_dir(xdg_cache_home: Option<&OsStr>, home: Option<&OsStr>) -> Option<PathBuf> {
    let xdg = xdg_cache_home
        .map(Path::new)
        .filter(|p| p.is_absolute())
        .map(Path::to_path_buf);
    let home = home
        .map(Path::new)
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.join(".cache"));
    xdg.or(home).map(|dir| dir.join("metaspec").join("ess"))
}

/// The `ess` of a scope and the answers asked in it.
struct Scope {
    ess: Ess,
    answers: HashMap<String, Result<Value, String>>,
}

thread_local! {
    static SCOPE: RefCell<Option<Scope>> = const { RefCell::new(None) };
}

/// Runs `f` with `ess` as the program and cache of every [`answer`] it asks on this thread;
/// within it each answer is asked once, and a run that gave no answer is not run again. Outside
/// a scope, [`Ess::from_env`] is used and nothing is kept in memory.
pub fn scoped<R>(ess: Ess, f: impl FnOnce() -> R) -> R {
    let outer = SCOPE.with(|scope| {
        scope.borrow_mut().replace(Scope {
            ess,
            answers: HashMap::new(),
        })
    });
    let result = f();
    SCOPE.with(|scope| *scope.borrow_mut() = outer);
    result
}

/// The `ess` of the current scope, or of the environment.
pub fn current() -> Ess {
    SCOPE
        .with(|scope| scope.borrow().as_ref().map(|s| s.ess.clone()))
        .unwrap_or_else(Ess::from_env)
}

/// `ess specify validate --path . --format json` in the blocks directory: a report of a valid
/// specification (exit 0, or 1 with scenario refusals) or a refusal (exit 1).
pub fn validate(pack: &Pack) -> Result<Value, String> {
    answer(
        pack,
        &["specify", "validate"],
        &["--format", "json"],
        &[0, 1],
    )
}

/// The JSON answer of `ess <words> --path . <options>`, run in the pack's blocks directory,
/// from the cache when the digest of the blocks has one. An exit code outside `exit`, a run over
/// the deadline, output that is no JSON and a program that cannot be started are errors naming
/// what ess said on stderr, and are not cached. Within a scope the same request is answered once,
/// error included.
pub fn answer(
    pack: &Pack,
    words: &[&str],
    options: &[&str],
    exit: &[i32],
) -> Result<Value, String> {
    let ess = current();
    let request = format!(
        "{}\0{}\0{}\0{}",
        ess.program.display(),
        pack.root.join(&pack.manifest.blocks.path).display(),
        words.join(" "),
        options.join(" ")
    );
    if let Some(result) = remembered(&request) {
        return result;
    }
    let result = ask(&ess, pack, words, options, exit);
    remember(&request, &result);
    result
}

/// One request: the cached answer of its digest, else a run of ess.
fn ask(
    ess: &Ess,
    pack: &Pack,
    words: &[&str],
    options: &[&str],
    exit: &[i32],
) -> Result<Value, String> {
    let command = format!("ess {}", words.join(" "));
    let blocks = resolved_inside(&pack.root, &pack.root.join(&pack.manifest.blocks.path))?;
    let program = ess.program.to_string_lossy().into_owned();
    let version = match run(&blocks, &ess.program, &["--version"], VERSION_DEADLINE) {
        Ran::Exited(run) if run.code == Some(0) => run.stdout,
        Ran::Exited(run) => {
            return Err(format!(
                "{program} --version exited {}{}",
                code_of(run.code),
                said(&run.stderr)
            ));
        }
        Ran::TimedOut(after) => {
            return Err(format!(
                "{program} --version ran over {} s and was killed",
                after.as_secs_f64()
            ));
        }
        Ran::Failed(why) => return Err(why),
    };
    let key = digest(&version, words, options, &pack.root, &blocks);
    let file = ess
        .cache
        .as_ref()
        .map(|dir| dir.join(format!("{key}.json")));
    if let Some(value) = file
        .as_ref()
        .and_then(|f| std::fs::read_to_string(f).ok())
        .and_then(|text| serde_json::from_str::<Value>(&text).ok())
    {
        return Ok(value);
    }
    let mut args: Vec<&str> = words.to_vec();
    args.extend_from_slice(&["--path", "."]);
    args.extend_from_slice(options);
    let value = match run(&blocks, &ess.program, &args, ess.deadline) {
        Ran::Exited(run) if run.code.is_some_and(|c| exit.contains(&c)) => {
            serde_json::from_slice::<Value>(&run.stdout).map_err(|error| {
                format!(
                    "{command} exited {} and printed no JSON ({error}){}",
                    code_of(run.code),
                    said(&run.stderr)
                )
            })?
        }
        Ran::Exited(run) => {
            return Err(format!(
                "{command} exited {}{}",
                code_of(run.code),
                said(&run.stderr)
            ));
        }
        Ran::TimedOut(after) => {
            return Err(format!(
                "{command} ran over {} s and was killed",
                after.as_secs_f64()
            ));
        }
        Ran::Failed(why) => return Err(why),
    };
    if let Some(file) = &file {
        // A cache that cannot be written costs the next run its time, nothing else.
        let _ = keep(file, &value);
    }
    Ok(value)
}

fn code_of(code: Option<i32>) -> String {
    code.map_or_else(|| String::from("by a signal"), |c| c.to_string())
}

/// `: <what ess said>` from its stderr: the first [`STDERR_LINES`] lines that are not blank,
/// leaving out `note:` lines when it said anything else; empty when it said nothing.
fn said(stderr: &str) -> String {
    let lines: Vec<&str> = stderr
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    let told: Vec<&str> = lines
        .iter()
        .copied()
        .filter(|l| !l.starts_with("note:"))
        .collect();
    let chosen = if told.is_empty() { lines } else { told };
    if chosen.is_empty() {
        return String::new();
    }
    let mut text = chosen
        .iter()
        .take(STDERR_LINES)
        .copied()
        .collect::<Vec<_>>()
        .join(" / ");
    if chosen.len() > STDERR_LINES {
        text.push_str(" / ...");
    }
    format!(": {text}")
}

/// One run of ess.
struct Exited {
    code: Option<i32>,
    stdout: Vec<u8>,
    stderr: String,
}

enum Ran {
    Exited(Exited),
    TimedOut(Duration),
    Failed(String),
}

/// Runs `<program> <args>` in `dir` with stdin closed, in a process group of its own, and keeps
/// stdout and the whole of stderr. Over `deadline` the run is stopped ([`stop`]) and timed out.
/// When ess exits but a process it left behind keeps a pipe open, the pipes are read until the
/// deadline, at most [`GRACE`] after the exit, and then the group is stopped: the run lasts at
/// most `deadline` + [`GRACE`] + [`SETTLE`], whatever ess started.
fn run(dir: &Path, program: &Path, args: &[&str], deadline: Duration) -> Ran {
    use std::os::unix::process::CommandExt;
    let mut child = match Command::new(program)
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0)
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            return Ran::Failed(format!("{} cannot be started: {error}", program.display()));
        }
    };
    let mut stdout = Drain::start(child.stdout.take());
    let mut stderr = Drain::start(child.stderr.take());
    let group = child.id() as i32;
    let start = Instant::now();
    let end = start
        .checked_add(deadline)
        .unwrap_or_else(|| start + Duration::from_secs(u64::from(u32::MAX)));
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if Instant::now() >= end => break None,
            Ok(None) => std::thread::sleep(Duration::from_millis(5)),
            Err(error) => {
                stop(group, Some(&mut child), &mut stdout, &mut stderr);
                return Ran::Failed(format!("ess cannot be waited for: {error}"));
            }
        }
    };
    let Some(status) = status else {
        stop(group, Some(&mut child), &mut stdout, &mut stderr);
        return Ran::TimedOut(deadline);
    };
    let exited = Instant::now();
    let until = (exited + GRACE).min(end).max(exited + SETTLE);
    if !(stdout.closed_by(until) && stderr.closed_by(until)) {
        stop(group, None, &mut stdout, &mut stderr);
    }
    Ran::Exited(Exited {
        code: status.code(),
        stdout: stdout.bytes(),
        stderr: String::from_utf8_lossy(&stderr.bytes()).into_owned(),
    })
}

/// Stops the process group of a run: SIGTERM, then SIGKILL once ess has ended and its pipes are
/// closed or after [`GRACE`]; then the pipes are read for at most [`SETTLE`]. A process that left
/// the group and holds a pipe is not waited for: its reader is left to end with it.
fn stop(group: i32, mut child: Option<&mut Child>, stdout: &mut Drain, stderr: &mut Drain) {
    signal(group, libc::SIGTERM);
    let until = Instant::now() + GRACE;
    while let Some(c) = child.as_mut() {
        match c.try_wait() {
            Ok(None) if Instant::now() < until => std::thread::sleep(Duration::from_millis(5)),
            _ => break,
        }
    }
    let _ = stdout.closed_by(until) && stderr.closed_by(until);
    signal(group, libc::SIGKILL);
    if let Some(c) = child {
        let _ = c.wait();
    }
    let settle = Instant::now() + SETTLE;
    let _ = stdout.closed_by(settle) && stderr.closed_by(settle);
}

/// Sends `signal` to the process group `group`.
fn signal(group: i32, signal: libc::c_int) {
    // SAFETY: kill(2) with a negative pid signals a process group; it touches no memory of this
    // process. A group without processes left answers ESRCH, which is ignored.
    unsafe {
        libc::kill(-group, signal);
    }
}

/// A pipe read to its end on a thread of its own; what it read so far can be taken at any time.
struct Drain {
    bytes: Arc<Mutex<Vec<u8>>>,
    done: mpsc::Receiver<()>,
    closed: bool,
}

impl Drain {
    fn start(pipe: Option<impl Read + Send + 'static>) -> Drain {
        let bytes = Arc::new(Mutex::new(Vec::new()));
        let (done, receiver) = mpsc::channel::<()>();
        let kept = Arc::clone(&bytes);
        std::thread::spawn(move || {
            // Dropping `done` at the end tells the receiver the pipe is closed.
            let _done = done;
            let Some(mut pipe) = pipe else { return };
            let mut chunk = [0u8; 8192];
            loop {
                match pipe.read(&mut chunk) {
                    Ok(0) => break,
                    Ok(n) => kept
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner)
                        .extend_from_slice(&chunk[..n]),
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(_) => break,
                }
            }
        });
        Drain {
            bytes,
            done: receiver,
            closed: false,
        }
    }

    /// Whether the pipe is closed, waiting for it until `until` at most.
    fn closed_by(&mut self, until: Instant) -> bool {
        if !self.closed {
            let left = until.saturating_duration_since(Instant::now());
            self.closed = !matches!(
                self.done.recv_timeout(left),
                Err(mpsc::RecvTimeoutError::Timeout)
            );
        }
        self.closed
    }

    /// What was read so far.
    fn bytes(&self) -> Vec<u8> {
        self.bytes
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

fn remembered(key: &str) -> Option<Result<Value, String>> {
    SCOPE.with(|scope| {
        scope
            .borrow()
            .as_ref()
            .and_then(|s| s.answers.get(key).cloned())
    })
}

fn remember(key: &str, value: &Result<Value, String>) {
    SCOPE.with(|scope| {
        if let Some(s) = scope.borrow_mut().as_mut() {
            s.answers.insert(key.to_string(), value.clone());
        }
    });
}

/// Writes the answer whole: a partial file is renamed into place.
fn keep(file: &Path, value: &Value) -> std::io::Result<()> {
    let dir = file.parent().unwrap_or(Path::new("."));
    std::fs::create_dir_all(dir)?;
    let partial = file.with_extension(format!("{}.tmp", std::process::id()));
    std::fs::write(&partial, value.to_string())?;
    std::fs::rename(&partial, file)
}

/// The key of an answer: `ess --version`, the command, and every file under the blocks.
fn digest(version: &[u8], words: &[&str], options: &[&str], root: &Path, blocks: &Path) -> String {
    let mut hasher = Sha256::new();
    hasher.update(version);
    hasher.update([0]);
    for part in words.iter().chain([&"--"]).chain(options) {
        hasher.update(part.as_bytes());
        hasher.update([0]);
    }
    for file in files(blocks) {
        hasher.update(shown(root, &file).as_bytes());
        hasher.update([0]);
        hasher.update(std::fs::read(&file).unwrap_or_default());
        hasher.update([0]);
    }
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Every regular file under `dir`, hidden ones included, sorted by path; symbolic links are not
/// followed.
pub fn files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            match entry.file_type() {
                Ok(kind) if kind.is_dir() => stack.push(entry.path()),
                Ok(kind) if kind.is_file() => out.push(entry.path()),
                _ => {}
            }
        }
    }
    out.sort();
    out
}
