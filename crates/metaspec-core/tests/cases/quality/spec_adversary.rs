//! Cases added by the adversarial pass on `story:spec-quality-static`: what `metaspec quality
//! spec` measures when `ess` does not answer with JSON, and when the pack's blocks pin another
//! release of `ess` than the one that runs.
//!
//! Each `ess` here is a POSIX sh stand-in written into the test's scratch directory; none runs
//! synthesis and nothing is cached.

use super::super::common::{load_pack, packs, scratch};
use metaspec_core::Pack;
use metaspec_core::quality::catalogue;
use metaspec_core::quality::registry::Input;
use metaspec_core::quality::report::{QualityReport, Reading};
use metaspec_core::quality::spec::static_measures::{
    self,
    ess_answer::{self, Ess},
};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

fn first_pack() -> Pack {
    let (_, dir) = &packs()[0];
    let (loaded, pack) = load_pack(dir);
    pack.unwrap_or_else(|| panic!("{} loads: {:?}", dir.display(), loaded.errors))
}

/// Writes `script` as an executable `ess` under `dir`.
fn program(dir: &Path, script: &str) -> PathBuf {
    let path = dir.join("ess");
    std::fs::write(&path, script).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    path
}

fn spec_report(pack: &Pack, ess: Ess) -> QualityReport {
    let catalogue = catalogue::embedded().expect("metaspec's catalogue loads");
    let input = Input::new(pack, None, Duration::from_secs(5));
    ess_answer::scoped(ess, || static_measures::report(&catalogue, &input))
        .unwrap_or_else(|issues| panic!("the specification question runs: {issues:?}"))
}

fn reading<'a>(report: &'a QualityReport, id: &str) -> &'a Reading {
    &report
        .measurements
        .iter()
        .find(|m| m.measure == id)
        .unwrap_or_else(|| panic!("the report holds {id}"))
        .reading
}

/// `ess specify validate` exits 1 for a refusal, and also when it refuses to run at all (an
/// unfollowed pin, an input it cannot acquire, an unknown component): then it prints nothing on
/// stdout and says why on stderr. Exit 1 is one of the codes `validate` accepts, so the gap is
/// the JSON parser's "EOF while parsing" and ess's reason is dropped; the doc of `answer` and
/// the story's constraint ("a gap reading naming the missing input") promise the reason.
#[test]
fn adversary_an_ess_that_exits_1_without_json_names_its_stderr_in_the_gap() {
    let work = scratch("spec-adversary-exit-1-no-json");
    let ess = program(
        &work,
        "#!/bin/sh\n\
         if [ \"${1-}\" = --version ]; then echo 'ess stand-in'; exit 0; fi\n\
         echo 'error: the reason this adversary case writes' >&2\n\
         exit 1\n",
    );
    let report = spec_report(&first_pack(), Ess::new(ess, None));
    match reading(&report, "validity.refusals") {
        Reading::Gap { gap } => assert!(
            gap.contains("the reason this adversary case writes"),
            "the gap names what ess said on stderr: {gap}"
        ),
        Reading::Value { value } => panic!("validity.refusals is the value {value}, not a gap"),
    }
}

/// The pin rule of `ess` 0.99.11 (`toolchain::unfollowed_pin`; measured with the release itself
/// on a copy of the first fixture pack: exit 1, stdout empty, stderr "blocks/ess-inputs.yaml
/// requires ess 0.99.2 and this is ess 0.99.11, and no ess-inputs.yaml at or above the working
/// directory pins an exact release, so ess did not switch to it; `cd blocks` and run ess
/// there, or run it as `ESS_TOOLCHAIN=0.99.2 ess …`"). The repository pins ess 0.99.11
/// (spec/ess-inputs.yaml); both fixture packs and both real packs pin 0.99.2 in
/// `blocks/ess-inputs.yaml`. The same release loads the pack (`ess specify compile` from a
/// directory that is not above the pack warns and goes on), yet `validate`, run in the pack
/// directory, is refused, so every ess measure of every such pack is a gap.
///
/// The stand-in answers when ESS_TOOLCHAIN names a release, or when an `ess-inputs.yaml` at or
/// above its working directory pins one (ess delegates to that release), and refuses like
/// 0.99.11 when the manifest of `--path` lies below the working directory and pins another.
#[test]
fn adversary_a_pack_whose_blocks_pin_another_ess_release_is_still_measured() {
    let work = scratch("spec-adversary-unfollowed-pin");
    let ess = program(
        &work,
        r#"#!/bin/sh
set -eu
this=0.99.11
pinned() { sed -n 's/^requires: ess \([0-9][0-9.]*\)[[:space:]]*$/\1/p' "$1" 2>/dev/null | head -n 1; }
above=
d=$(pwd -P)
while :; do
  if [ -f "$d/ess-inputs.yaml" ] && [ -n "$(pinned "$d/ess-inputs.yaml")" ]; then
    above=$(pinned "$d/ess-inputs.yaml"); break
  fi
  [ "$d" = / ] && break
  d=$(dirname -- "$d")
done
if [ "${1-}" = --version ]; then echo "ess ${ESS_TOOLCHAIN:-${above:-$this}}"; exit 0; fi
path=. prev=
for w in "$@"; do
  [ "$prev" = --path ] && path=$w
  prev=$w
done
if [ -z "${ESS_TOOLCHAIN-}" ] && [ -z "$above" ] && [ -f "$path/ess-inputs.yaml" ]; then
  want=$(pinned "$path/ess-inputs.yaml")
  here=$(pwd -P)
  there=$(cd -- "$path" && pwd -P)
  case "$there/" in
    "$here"/*)
      if [ -n "$want" ] && [ "$want" != "$this" ]; then
        echo "error: $path/ess-inputs.yaml requires ess $want and this is ess $this, and no ess-inputs.yaml at or above the working directory pins an exact release, so ess did not switch to it" >&2
        exit 1
      fi ;;
  esac
fi
echo '{"valid": true, "system": "s", "version": "v1", "files_read": 1, "domains": 1, "commands": 0, "events": 0, "components": 0, "unresolved_references": []}'
"#,
    );
    let pack = first_pack();
    let blocks = pack.root.join(&pack.manifest.blocks.path);
    let pin = std::fs::read_to_string(blocks.join("ess-inputs.yaml")).unwrap();
    assert!(
        pin.lines()
            .any(|l| l.starts_with("requires: ess ") && l != "requires: ess 0.99.11"),
        "the fixture's blocks pin another release than 0.99.11: {pin}"
    );
    let report = spec_report(&pack, Ess::new(ess, None));
    for id in ["validity.refusals", "validity.warnings"] {
        match reading(&report, id) {
            Reading::Value { value } => assert_eq!(value.as_u64(), Some(0), "{id}"),
            Reading::Gap { gap } => panic!("{id} is a gap on a pack that loads: {gap}"),
        }
    }
}

// --- pass 2: the runner of ess, the cache key and the stub ---

/// A stand-in that prints a valid report with no warning after `before`, then runs `after`.
fn report_script(before: &str, after: &str) -> String {
    format!(
        "#!/bin/sh\n\
         if [ \"${{1-}}\" = --version ]; then echo 'ess stand-in'; exit 0; fi\n\
         {before}\n\
         echo '{{\"valid\": true, \"system\": \"s\", \"version\": \"v1\", \"warnings\": []}}'\n\
         {after}\n"
    )
}

/// ess exits at once, but a process it started in its group keeps stdout open (a server, a
/// child left behind). `run` reaps ess and then joins the reader of stdout with no bound, so
/// the deadline the module promises ("the run is killed with everything it started after its
/// deadline") does not bound the run: it lasts as long as the process left behind.
#[test]
fn adversary_a_process_ess_leaves_behind_holding_stdout_does_not_outlast_the_deadline() {
    let work = scratch("spec-adversary-left-behind");
    let ess = program(&work, &report_script("sleep 15 &", "exit 0"));
    let started = std::time::Instant::now();
    let _ = spec_report(
        &first_pack(),
        Ess::new(ess, None).with_deadline(Duration::from_secs(1)),
    );
    assert!(
        started.elapsed() < Duration::from_secs(8),
        "a 1 s deadline bounds the run; it took {:?}",
        started.elapsed()
    );
}

/// ess runs over its deadline and has started a process in a group of its own, as `ess verify
/// conform run --target process` does with its target (own process group, stderr inherited).
/// SIGKILL to ess's group leaves that process alive with the pipe, and `run` joins the readers
/// with no bound after the kill.
#[test]
fn adversary_a_run_killed_at_its_deadline_ends_though_a_process_it_started_left_its_group() {
    let work = scratch("spec-adversary-own-group");
    let ess = program(
        &work,
        &report_script("setsid sleep 15 &\nsleep 30", "exit 0"),
    );
    let started = std::time::Instant::now();
    let report = spec_report(
        &first_pack(),
        Ess::new(ess, None).with_deadline(Duration::from_secs(1)),
    );
    let elapsed = started.elapsed();
    match reading(&report, "validity.refusals") {
        Reading::Gap { gap } => assert!(gap.contains("ran over 1 s"), "{gap}"),
        Reading::Value { value } => panic!("validity.refusals is the value {value}, not a gap"),
    }
    assert!(
        elapsed < Duration::from_secs(8),
        "killed at the 1 s deadline, the run ends; it took {elapsed:?}"
    );
}

/// The stub answers whatever `--path` names: run in the blocks directory with a path that does
/// not exist, it still prints the recorded answer, so a test using it passes when metaspec runs
/// ess with a wrong `--path` (e.g. `--path blocks` from the blocks directory), which the real
/// ess refuses.
#[test]
fn adversary_the_stub_refuses_a_path_that_is_not_its_blocks_directory() {
    let (_, dir) = &packs()[0];
    let stub = super::super::common::repo().join("fixtures/ess/ess");
    let out = std::process::Command::new(stub)
        .args([
            "specify",
            "validate",
            "--path",
            "no-such-directory",
            "--format",
            "json",
        ])
        .current_dir(dir.join("blocks"))
        .output()
        .expect("the stub starts");
    assert_ne!(
        out.status.code(),
        Some(0),
        "the stub answered for a --path that does not exist: {}",
        String::from_utf8_lossy(&out.stdout)
            .chars()
            .take(80)
            .collect::<String>()
    );
}

/// `deadline_of` promises "a finite number of seconds above 0, else DEADLINE", but a positive
/// value below one nanosecond becomes `Duration::ZERO`: every run is killed at once ("ran over
/// 0 s and was killed", measured through the CLI with METASPEC_ESS_DEADLINE=1e-300).
#[test]
fn adversary_a_deadline_of_metaspec_ess_deadline_is_never_zero() {
    for tiny in ["1e-300", "0.0000000001"] {
        let deadline = ess_answer::deadline_of(Some(std::ffi::OsStr::new(tiny)));
        assert!(deadline > Duration::ZERO, "{tiny:?} gives {deadline:?}");
    }
}
