#!/bin/sh
# Validate metaspec's own ESS specification (spec/) and run its conformance suite on ESS's
# interpreter, inside the gate's budget.
#
# Every ess command runs inside spec/: spec/ess-inputs.yaml pins the release (`requires: ess
# X.Y.Z`), and ess resolves that pin from the working directory upward, not from --path.
#
# Synthesis is the expensive part and running the suite the cheap one. The suite is synthesized
# once per state of its sources and kept in a cache on disk, keyed by their digest:
#
#   target/spec-conform/<digest>.json
#
# The digest is the SHA-256 over the synthesis line below, the `ess --version` lines (the
# dispatcher and the pinned release it runs), spec/ess-inputs.yaml and every file it lists under
# `specification:` and `scenarios:`, each with its path. The suite is not committed.
#
# Known failures. spec/known-failing.txt lists, by exact id, the synthesized scenarios that fail
# on the interpreter because the scenario is wrong, each group under a `# root:` line that is its
# reason. The list is not part of the digest. For each run it is bound to the suite, the
# specification and the `ess` executable as an `ess-known-failures/1` declaration
# (target/spec-conform/known-failures.json) and passed with `--known-failing`; `ess` writes which
# failures it names and which it does not (target/spec-conform/accounting.json) and keeps its own
# verdict and exit status. A listed scenario is never counted as passed. SPEC_CONFORM_KNOWN names
# another list.
#
# The verdict is read from the report and the accounting, not from the exit status of `ess`.
# All four hold, or the script fails:
#   - the run covered the suite: as many scenarios reported as the suite holds, and more than 0;
#   - no scenario ended error, unsupported or skipped;
#   - every failed scenario is on the list;
#   - every other scenario passed.
#
#   check-spec-conform.sh           validate, then run the cached suite; synthesize it first when
#                                   there is none
#   check-spec-conform.sh --slow    synthesize afresh, compare bytes with the cached suite of the
#                                   same digest when there is one, keep it, then run it
#   check-spec-conform.sh --digest  print the digest
#
# Exit status:
#   0  valid, and every scenario passed or is a listed known failure
#   1  a scenario that is not listed failed, or one ended error, unsupported or skipped, or the
#      run did not cover the suite; or --slow found a cached suite that differs from a fresh
#      synthesis
#   2  validation or synthesis failed; the list is malformed; or `ess` refused the list
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root/spec"

work=$root/target/spec-conform
jobs=${SPEC_CONFORM_JOBS:-4}
known=${SPEC_CONFORM_KNOWN:-$root/spec/known-failing.txt}
synth="ess verify conform synthesize --path . --compact --strict-requires"
# `--scenarios .` reads the `scenarios:` list of spec/ess-inputs.yaml; ess refuses it while the list
# is empty.
if ! grep -q '^scenarios: \[\]' ess-inputs.yaml; then
  synth="$synth --scenarios ."
fi

mode=${1:-run}
case $mode in
  run | --slow | --digest) ;;
  *) echo "check-spec-conform: unknown argument $mode" >&2; exit 2 ;;
esac

# The files the suite is made of, relative to spec/: ess-inputs.yaml and its two lists.
sources() {
  echo ess-inputs.yaml
  awk '/^(specification|scenarios):/ { on = 1; next } /^[a-z_]+:/ { on = 0 } on && /^  - / { sub(/^  - /, ""); print }' \
    ess-inputs.yaml
}

digest() {
  {
    printf '%s\n' "$synth"
    ess --version 2>/dev/null
    sources | while IFS= read -r file; do
      printf '%s %s\n' "$(sha256sum < "$file" | cut -d' ' -f1)" "$file"
    done
  } | sha256sum | cut -d' ' -f1
}

missing=$(sources | while IFS= read -r file; do [ -f "$file" ] || echo "spec/$file"; done)
if [ -n "$missing" ]; then
  echo "check-spec-conform: listed in spec/ess-inputs.yaml and missing: $missing" >&2
  exit 2
fi
key=$(digest)
if [ "$mode" = --digest ]; then
  echo "$key"
  exit 0
fi

mkdir -p "$work"
if ! ess specify validate --path . --strict-requires > "$work/validate.txt" 2>&1; then
  grep -v 'is the dispatcher' "$work/validate.txt" >&2
  echo "check-spec-conform: ess specify validate --path spec --strict-requires refused the specification" >&2
  exit 2
fi
if grep -v 'is the dispatcher' "$work/validate.txt" | grep -q 'warning'; then
  grep -v 'is the dispatcher' "$work/validate.txt" >&2
  echo "check-spec-conform: ess specify validate warned; a warning is not clean" >&2
  exit 2
fi
echo "check-spec-conform: $(grep -v 'is the dispatcher' "$work/validate.txt" | tail -n 1)"

suite=$work/$key.json
report=$work/report.json
listed=$work/known-failing.tsv
declaration=$work/known-failures.json
accounting=$work/accounting.json

# synthesize <file>: write a fresh suite into <file>, its log beside it.
synthesize() {
  start=$(date +%s)
  if ! $synth --jobs "$jobs" --out "$1" > "$1.log" 2>&1; then
    tail -n 20 "$1.log" >&2
    echo "check-spec-conform: synthesis failed: $synth" >&2
    exit 2
  fi
  echo "check-spec-conform: synthesized in $(( $(date +%s) - start )) s: $(tail -n 1 "$1.log")"
}

if [ "$mode" = --slow ]; then
  synthesize "$work/fresh.json"
  if [ -f "$suite" ]; then
    if cmp -s "$work/fresh.json" "$suite"; then
      echo "check-spec-conform: the cached suite $suite is byte for byte a fresh synthesis"
    else
      echo "check-spec-conform: the cached suite $suite differs from a fresh synthesis of the same sources" >&2
      exit 1
    fi
  fi
  mv "$work/fresh.json" "$suite"
  mv "$work/fresh.json.log" "$suite.log"
elif [ -f "$suite" ]; then
  echo "check-spec-conform: suite $key from the cache"
else
  synthesize "$work/next.json"
  mv "$work/next.json" "$suite"
  mv "$work/next.json.log" "$suite.log"
fi

# Keep one suite: the cache of other digests is stale.
for old in "$work"/*.json; do
  case $old in
    "$suite" | "$report" | "$declaration" | "$accounting") ;;
    *) rm -f "$old" "$old.log" ;;
  esac
done

# The list as `<id><TAB><reason>` lines in id order. An id needs a `# root:` line above it.
: > "$listed.raw"
if [ -f "$known" ]; then
  if ! awk -v file="$known" '
    /^# root: / { reason = substr($0, 9); next }
    /^#/ || /^[[:space:]]*$/ { next }
    reason == "" { printf "check-spec-conform: %s:%d: an id above the first `# root:` line\n", file, NR | "cat 1>&2"; bad = 1; next }
    /[[:space:]]/ { printf "check-spec-conform: %s:%d: an id holds no blank\n", file, NR | "cat 1>&2"; bad = 1; next }
    { printf "%s\t%s\n", $0, reason }
    END { exit bad }
  ' "$known" > "$listed.raw"; then
    exit 2
  fi
fi
LC_ALL=C sort "$listed.raw" > "$listed"
rm -f "$listed.raw"
twice=$(cut -f1 "$listed" | uniq -d)
if [ -n "$twice" ]; then
  echo "check-spec-conform: listed twice in $known: $twice" >&2
  exit 2
fi

# One read of the suite: the specification digest it was synthesized from, and its size.
set -- $(jq -r '"\(.provenance.spec_digest) \(.scenarios | length)"' "$suite")
spec_digest=$1
in_suite=$2

# The executable that runs: the pinned release, not the dispatcher on PATH.
binary=$(ess specify toolchain which 2>/dev/null | sed -n 's/^binary: \([^ ]*\).*/\1/p')
[ -n "$binary" ] || binary=$(command -v ess)
release=$(ess specify toolchain which 2>/dev/null | sed -n '1s/^ess //p')

rm -f "$declaration" "$accounting"
status=0
if [ -s "$listed" ]; then
  jq -Rn \
    --arg implementation "interpreted $release" \
    --arg build "sha256:$(sha256sum < "$binary" | cut -d' ' -f1)" \
    --arg spec_digest "$spec_digest" \
    --arg suite_digest "sha256:$(sha256sum < "$suite" | cut -d' ' -f1)" \
    --arg tracking "spec/known-failing.txt" '
      {
        failures: [inputs | split("\t") | {reason: .[1], scenario: .[0], tracking: $tracking}],
        format: "ess-known-failures/1",
        implementation: $implementation,
        implementation_build: $build,
        spec_digest: $spec_digest,
        suite_digest: $suite_digest
      }' < "$listed" > "$declaration"
  ess verify conform run --target interpreted --path . --suite "$suite" --strict-requires \
    --report-format 2 --report-out "$report" \
    --known-failing "$declaration" --accounting-out "$accounting" > "$work/run.txt" 2>&1 || status=$?
else
  ess verify conform run --target interpreted --path . --suite "$suite" --strict-requires \
    --report-format 2 --report-out "$report" > "$work/run.txt" 2>&1 || status=$?
fi

# 0 and 1 are a run with a report. Anything else wrote none: `ess` refused the list or the suite.
case $status in
  0 | 1) ;;
  *)
    grep -v '^Requires an empty logical' "$work/run.txt" | tail -n 5 >&2
    echo "check-spec-conform: exit $status: no run; when a scenario of $known passed, take it off the list; output $work/run.txt" >&2
    exit 2
    ;;
esac

set -- $(jq -r '.counts | "\(.total) \(.passed) \(.failed) \(.error) \(.unsupported) \(.skipped)"' "$report")
total=$1
passed=$2
failed=$3
error=$4
unsupported=$5
skipped=$6
if [ -f "$accounting" ]; then
  known_failed=$(jq '.known_failed | length' "$accounting")
  jq -r '.unexpected_failed[] | "  failed \(if type == "object" then .scenario else . end)"' "$accounting"
else
  known_failed=0
  jq -r '.outcomes.failed[] | "  failed \(.)"' "$report"
fi
unlisted=$(( failed - known_failed ))
jq -r '(.outcomes.error[] | "  error \(.)"), (.outcomes.unsupported[] | "  unsupported \(.)"), (.outcomes.skipped[] | "  skipped \(.)")' "$report"

verdict=0
if [ "$total" -eq 0 ] || [ "$total" -ne "$in_suite" ]; then
  echo "check-spec-conform: the run reported $total scenario(s) and the suite holds $in_suite" >&2
  verdict=1
fi
if [ $(( error + unsupported + skipped )) -ne 0 ]; then
  echo "check-spec-conform: $error error, $unsupported unsupported, $skipped skipped: none of them is a pass" >&2
  verdict=1
fi
if [ "$unlisted" -ne 0 ]; then
  echo "check-spec-conform: $unlisted failed scenario(s) not listed in $known" >&2
  verdict=1
fi
if [ $(( passed + known_failed )) -ne "$total" ]; then
  echo "check-spec-conform: $passed passed and $known_failed known failing do not make $total" >&2
  verdict=1
fi
[ "$verdict" = 0 ] || echo "check-spec-conform: exit $verdict; report $report, output $work/run.txt" >&2
echo "check-spec-conform: $total scenarios: $passed passed, $known_failed known failing (${known#"$root"/}), $unlisted failed and not listed, $error error, $unsupported unsupported"
exit "$verdict"
