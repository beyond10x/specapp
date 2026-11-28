//! The pack's tool: trust, and one run of a verb under a deadline (see "The tool adapter" in
//! `design/metaspec.md`).
//!
//! metaspec runs a pack's tool only for a pack the user trusted: `metaspec trust <dir>` records
//! the digest of the manifest's `tool` section in the user's state directory. A changed `tool`
//! section is not trusted until it is trusted again. The tool answers in the pack's own format;
//! the manifest's adapter maps that answer (`crate::adapter`).

use crate::adapter::{self, Exit, Mapped};
use crate::graph::Graph;
use crate::manifest::Tool;
use crate::pack::Pack;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// The format of the trust file.
pub const TRUST_FORMAT: &str = "metaspec-trust/1";

/// The schema of the trust file.
pub const TRUST_SCHEMA: &str = include_str!("../../../schema/metaspec-trust-1.schema.json");

/// The format of a check result.
pub const FINDINGS_FORMAT: &str = "metaspec-findings/1";

/// The schema of a check result.
pub const FINDINGS_SCHEMA: &str = include_str!("../../../schema/metaspec-findings-1.schema.json");

/// The name of the trust file in the state directory.
pub const TRUST_FILE: &str = "trust.json";

/// The deadline of one run when none is given.
pub const DEADLINE: Duration = Duration::from_secs(30);

/// The user's state directory of metaspec: `$XDG_STATE_HOME/metaspec`, else
/// `$HOME/.local/state/metaspec`.
pub fn state_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("XDG_STATE_HOME").filter(|d| !d.is_empty()) {
        return Some(PathBuf::from(dir).join("metaspec"));
    }
    std::env::var_os("HOME")
        .filter(|d| !d.is_empty())
        .map(|home| PathBuf::from(home).join(".local/state/metaspec"))
}

/// The digest of a manifest's `tool` section: SHA-256 of its JSON, `sha256:<hex>`.
pub fn digest(tool: &Tool) -> String {
    let text = serde_json::to_string(tool).unwrap_or_default();
    let hash = Sha256::digest(text.as_bytes());
    let hex: String = hash.iter().map(|b| format!("{b:02x}")).collect();
    format!("sha256:{hex}")
}

/// The trust file: per pack directory (resolved) the digest of the `tool` section the user
/// trusted.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrustFile {
    pub format: String,
    pub packs: BTreeMap<String, String>,
}

fn read_trust(state: &Path) -> TrustFile {
    std::fs::read_to_string(state.join(TRUST_FILE))
        .ok()
        .and_then(|text| serde_json::from_str::<TrustFile>(&text).ok())
        .filter(|file| file.format == TRUST_FORMAT)
        .unwrap_or_else(|| TrustFile {
            format: TRUST_FORMAT.to_string(),
            packs: BTreeMap::new(),
        })
}

/// Records that the user trusts the `tool` section of the pack in `root`. Returns the digest.
pub fn trust(state: &Path, root: &Path, tool: &Tool) -> Result<String, String> {
    let mut file = read_trust(state);
    let digest = digest(tool);
    file.packs
        .insert(root.to_string_lossy().into_owned(), digest.clone());
    std::fs::create_dir_all(state)
        .map_err(|e| format!("{} cannot be made: {e}", state.display()))?;
    let text = serde_json::to_string_pretty(&file).map_err(|e| e.to_string())?;
    let partial = state.join(format!("{TRUST_FILE}.{}.tmp", std::process::id()));
    std::fs::write(&partial, format!("{text}\n"))
        .map_err(|e| format!("{} cannot be written: {e}", partial.display()))?;
    std::fs::rename(&partial, state.join(TRUST_FILE))
        .map_err(|e| format!("the trust file cannot be written: {e}"))?;
    Ok(digest)
}

/// Whether the user trusts this `tool` section of the pack in `root`.
pub fn trusted(state: &Path, root: &Path, tool: &Tool) -> bool {
    read_trust(state).packs.get(root.to_string_lossy().as_ref()) == Some(&digest(tool))
}

/// What one run of the tool came to.
#[derive(Debug, Clone, PartialEq)]
pub enum Answer {
    /// The tool ended by itself; the adapter reads its exit code and stdout.
    Exited(Exit),
    /// The run went over its deadline and was killed.
    TimedOut(Duration),
    /// The tool could not be started or waited for.
    Failed(String),
}

/// Runs `<argv> <args>` with the pack directory as working directory, no stdin, and kills it
/// with everything it started when it runs over `deadline`.
pub fn run(root: &Path, argv: &[String], args: &[&str], deadline: Duration) -> Answer {
    use std::os::unix::process::CommandExt;
    let Some((program, prefix)) = argv.split_first() else {
        return Answer::Failed(String::from("tool.run is empty"));
    };
    let program = if program.contains('/') {
        root.join(program).to_string_lossy().into_owned()
    } else {
        program.clone()
    };
    let mut child = match Command::new(&program)
        .args(prefix)
        .args(args)
        .current_dir(root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0)
        .spawn()
    {
        Ok(child) => child,
        Err(error) => return Answer::Failed(format!("{program} cannot be started: {error}")),
    };
    let reader = |pipe: Option<Box<dyn Read + Send>>| {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            if let Some(mut pipe) = pipe {
                let _ = pipe.read_to_end(&mut bytes);
            }
            bytes
        })
    };
    let stdout = reader(
        child
            .stdout
            .take()
            .map(|p| Box::new(p) as Box<dyn Read + Send>),
    );
    let stderr = reader(
        child
            .stderr
            .take()
            .map(|p| Box::new(p) as Box<dyn Read + Send>),
    );
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if start.elapsed() >= deadline => break None,
            Ok(None) => std::thread::sleep(Duration::from_millis(5)),
            Err(error) => return Answer::Failed(format!("the run cannot be waited for: {error}")),
        }
    };
    let Some(status) = status else {
        let group = child.id() as i32;
        // SAFETY: kill(2) with a negative pid signals the process group the child leads; it
        // touches no memory of this process.
        unsafe {
            libc::kill(-group, libc::SIGKILL);
        }
        let _ = child.wait();
        let _ = stdout.join();
        let _ = stderr.join();
        return Answer::TimedOut(deadline);
    };
    let out = stdout.join().unwrap_or_default();
    let err = stderr.join().unwrap_or_default();
    Answer::Exited(Exit {
        code: status.code(),
        stdout: out,
        stderr: String::from_utf8_lossy(&err)
            .lines()
            .next()
            .unwrap_or("")
            .trim_end()
            .to_string(),
    })
}

/// One finding of a check: a rule, whether it is a gate, what it is about, a sentence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub rule: String,
    pub gate: bool,
    /// `part:<id>`, `link:<id>` or `document`.
    pub subject: String,
    pub message: String,
}

/// What a subject names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subject<'a> {
    Part(&'a str),
    Link(&'a str),
    Document,
}

impl Finding {
    pub fn subject(&self) -> Subject<'_> {
        if let Some(id) = self.subject.strip_prefix("part:") {
            Subject::Part(id)
        } else if let Some(id) = self.subject.strip_prefix("link:") {
            Subject::Link(id)
        } else {
            Subject::Document
        }
    }
}

/// A check result, `metaspec-findings/1`: what the adapter reads from the tool's answer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Findings {
    pub format: String,
    pub valid: bool,
    pub findings: Vec<Finding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counts: Option<BTreeMap<String, u64>>,
    /// KPI values of the document as the tool measured it, by KPI id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kpis: Option<BTreeMap<String, serde_json::Number>>,
}

/// Validates a check result against `metaspec-findings/1` and decodes it.
pub fn findings(value: &Value) -> Result<Findings, String> {
    if let Some(format) = value.get("format").and_then(Value::as_str)
        && format != FINDINGS_FORMAT
    {
        return Err(format!(
            "the result is in {format}; this metaspec reads {FINDINGS_FORMAT}"
        ));
    }
    let schema: Value = serde_json::from_str(FINDINGS_SCHEMA).expect("the findings schema is JSON");
    let validator = jsonschema::validator_for(&schema).expect("the findings schema compiles");
    let errors: Vec<String> = validator
        .iter_errors(value)
        .take(5)
        .map(|e| format!("at {}: {e}", e.instance_path()))
        .collect();
    if !errors.is_empty() {
        return Err(format!(
            "the answer is no {FINDINGS_FORMAT}: {}",
            errors.join("; ")
        ));
    }
    serde_json::from_value(value.clone()).map_err(|e| e.to_string())
}

/// The state of the check of one document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckState {
    /// The tool answered; the findings are its answer.
    Ran,
    /// The manifest names no tool.
    NoTool,
    /// The tool has no verb `check`.
    NoCheck,
    /// The user has not trusted this `tool` section of the pack.
    NotTrusted,
    /// The run went over its deadline and was killed.
    TimedOut,
    /// The tool refused the document: its answer holds the text the adapter's `error` points at.
    Refused,
    /// The tool could not be started, or its answer is not what the adapter declares.
    Failed,
}

/// The check of one document.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Check {
    pub state: CheckState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Findings>,
}

impl Check {
    fn without(state: CheckState, message: impl Into<String>) -> Check {
        Check {
            state,
            message: Some(message.into()),
            result: None,
        }
    }

    /// The findings, none when the tool did not answer.
    pub fn findings(&self) -> &[Finding] {
        self.result.as_ref().map_or(&[], |r| r.findings.as_slice())
    }
}

/// Checks one document of a loaded pack with the pack's tool, when the manifest declares the
/// verb `check` and the user trusts the tool (`state` is the state directory; none trusts
/// nothing): runs `<tool.run> <args> <document>` and maps the answer with the manifest's
/// adapter. `graph` is the document as metaspec drew it.
pub fn check(
    pack: &Pack,
    document: &str,
    graph: Option<&Graph>,
    state: Option<&Path>,
    deadline: Duration,
) -> Check {
    let Some(tool) = &pack.manifest.tool else {
        return Check::without(CheckState::NoTool, "the manifest names no tool");
    };
    let Some(spec) = &tool.verbs.check else {
        return Check::without(CheckState::NoCheck, "the manifest declares no verb check");
    };
    if !state.is_some_and(|s| trusted(s, &pack.root, tool)) {
        return Check::without(
            CheckState::NotTrusted,
            "the pack's tool is not trusted; metaspec trust trusts it",
        );
    }
    let mut args: Vec<&str> = spec.args.iter().map(String::as_str).collect();
    args.push(document);
    match run(&pack.root, &tool.run, &args, deadline) {
        Answer::Exited(exit) => match adapter::check(spec, &exit, graph) {
            Mapped::Findings(result) => Check {
                state: CheckState::Ran,
                message: None,
                result: Some(result),
            },
            Mapped::Refused(error) => Check::without(CheckState::Refused, error),
            Mapped::Failed(why) => Check::without(CheckState::Failed, why),
        },
        Answer::TimedOut(after) => Check::without(
            CheckState::TimedOut,
            format!(
                "the check ran over its deadline of {} s and was killed",
                after.as_secs_f64()
            ),
        ),
        Answer::Failed(why) => Check::without(CheckState::Failed, why),
    }
}
