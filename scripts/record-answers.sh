#!/bin/sh
# record-answers.sh: record what a known pack's own tool answers to the `check` of each case.
#
#   scripts/record-answers.sh <pack> <pack-dir>
#
# The cases are invented documents, fixtures/answers/<pack>/<class>/<case>.yaml. For each, the
# tool of packs/<pack>.yaml runs as metaspec runs it, `<tool.run> <verbs.check.args> <case>`, in
# <pack-dir> (a checkout of the pack with its tool built), and its answer is written beside the
# case: <case>.stdout.json (stdout, byte for byte) and <case>.exit (the exit code). The tests read
# them through the manifest's adapter and compare the result with <case>.findings.json
# (`METASPEC_BLESS=1 cargo nextest run` rewrites that file). `task check` runs no pack's tool.
#
# Not part of `task check`: it needs a checkout of the pack. Needs yq.
#
# Exit 0: every case recorded. Exit 2: bad usage, or the tool cannot be started.

set -eu

[ $# -eq 2 ] || { echo "usage: scripts/record-answers.sh <pack> <pack-dir>" >&2; exit 2; }
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
pack=$1
dir=$(CDPATH= cd -- "$2" && pwd)
manifest=$root/packs/$pack.yaml
cases=$root/fixtures/answers/$pack
[ -f "$manifest" ] || { echo "record-answers: no manifest packs/$pack.yaml" >&2; exit 2; }
[ -d "$cases" ] || { echo "record-answers: no directory fixtures/answers/$pack" >&2; exit 2; }

run=$(yq -r '.tool.run | join(" ")' "$manifest")
args=$(yq -r '.tool.verbs.check.args | join(" ")' "$manifest")
[ -n "$run" ] || { echo "record-answers: packs/$pack.yaml names no tool" >&2; exit 2; }
program=${run%% *}
case "$program" in
  */*) [ -x "$dir/$program" ] || { echo "record-answers: no tool at $program in the pack" >&2; exit 2; } ;;
esac

n=0
for file in "$cases"/*/*.yaml; do
  [ -f "$file" ] || continue
  stem=${file%.yaml}
  code=0
  # shellcheck disable=SC2086 # run and args are lists of words
  (cd "$dir" && $run $args "$file") > "$stem.stdout.json" 2> /dev/null || code=$?
  echo "$code" > "$stem.exit"
  echo "record-answers: ${file#"$root"/} exit $code"
  n=$((n + 1))
done
[ "$n" -gt 0 ] || { echo "record-answers: no case under fixtures/answers/$pack" >&2; exit 2; }
echo "record-answers: $n case(s) of $pack recorded"
